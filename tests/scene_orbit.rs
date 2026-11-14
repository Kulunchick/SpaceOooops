//! Scene Spec: the Run's orbit view (low-poly ticket 03). The spacecraft's place, Earth's turn and the eclipse
//! come only from the playback position and the Run timeline, so the 3D scene, the SVG fallback and the
//! dashboards always agree.
mod common;

use common::*;
use spaceooops::scene::{self, OrbitView, SceneInputs, Screen};
use spaceooops::sim::{load_catalog, run, Catalog, Design, RunResult};

fn flown(c: &Catalog, d: &Design, scenario: &str) -> RunResult {
    let r = run(c, d, scenario, 7);
    assert!(!r.timeline.is_empty(), "the known-good Design flies");
    r
}

fn view_at(c: &Catalog, d: &Design, r: &RunResult, playback: f64) -> OrbitView {
    scene::build(&SceneInputs { catalog: Some(c), design: Some(d), run: Some(r), playback, ..SceneInputs::on(Screen::Run) })
        .orbit
        .expect("the Run shows the orbit view")
}

#[test]
fn each_playback_day_shows_that_days_shadow_from_the_timeline() {
    let c = load_catalog().unwrap();
    let d = crop_watch_known_good();
    let r = flown(&c, &d, CROP_WATCH);
    for (i, day) in r.timeline.iter().enumerate() {
        let v = view_at(&c, &d, &r, i as f64 + 0.5);
        assert_eq!(v.day, day.day as usize, "playback {} is in day {}", i as f64 + 0.5, day.day);
        assert_eq!(v.shadow_fraction, day.in_eclipse_fraction, "day {}", day.day);
    }
}

/// The three known-good Designs, one per Orbit, each with its Scenario.
fn one_per_orbit() -> [(Design, &'static str); 3] {
    [
        (space_weather_sentinel_known_good(), SPACE_WEATHER_SENTINEL), // LEO 400 km
        (crop_watch_known_good(), CROP_WATCH),                         // SSO 600 km
        (connect_the_villages_known_good(), CONNECT_THE_VILLAGES),     // GEO
    ]
}

/// Share of evenly spaced playback positions over the whole Run at which the spacecraft is in Earth's shadow.
fn share_in_shadow(c: &Catalog, d: &Design, r: &RunResult) -> f64 {
    let days = r.timeline.len() as f64;
    let samples = 20_000;
    (0..samples).filter(|k| view_at(c, d, r, days * (*k as f64 + 0.5) / samples as f64).in_shadow).count() as f64
        / samples as f64
}

#[test]
fn low_orbits_spend_the_timelines_share_of_the_run_in_earths_shadow() {
    let c = load_catalog().unwrap();
    // LEO and SSO circle Earth many times over a Run, so the time in shadow adds up to the day's share.
    for (d, scenario) in one_per_orbit().into_iter().take(2) {
        let r = flown(&c, &d, scenario);
        // Every day of these Runs has the Orbit's own shadow share (LEO 0.38, SSO 0.36).
        let expected = r.timeline[0].in_eclipse_fraction;
        assert!(r.timeline.iter().all(|t| t.in_eclipse_fraction == expected));
        let shaded = share_in_shadow(&c, &d, &r);
        assert!((shaded - expected).abs() < 0.02, "{:?}: in shadow {shaded:.3} of the Run, timeline says {expected}", d.orbit);
    }
}

#[test]
fn geo_is_almost_always_in_sunlight_but_crosses_the_shadow() {
    let c = load_catalog().unwrap();
    let d = connect_the_villages_known_good();
    let r = flown(&c, &d, CONNECT_THE_VILLAGES);
    let shaded = share_in_shadow(&c, &d, &r);
    assert!(shaded > 0.0 && shaded < 0.06, "GEO in shadow {shaded:.3} of the Run");
}

#[test]
fn leo_sso_and_geo_differ_in_altitude_inclination_and_speed() {
    let c = load_catalog().unwrap();
    let views: Vec<(OrbitView, OrbitView)> = one_per_orbit()
        .into_iter()
        .map(|(d, s)| {
            let r = flown(&c, &d, s);
            (view_at(&c, &d, &r, 0.0), view_at(&c, &d, &r, 0.002 * r.timeline.len() as f64))
        })
        .collect();
    let [(leo, leo_later), (sso, sso_later), (geo, geo_later)] = [&views[0], &views[1], &views[2]].map(|v| v.clone());
    assert_eq!([leo.class.as_str(), sso.class.as_str(), geo.class.as_str()], ["leo", "sso", "geo"]);
    // Altitude: higher Orbits are drawn further out, GEO far beyond the two low ones, all off the surface.
    assert!(1.0 < leo.radius && leo.radius < sso.radius && sso.radius < geo.radius);
    assert!(geo.radius > 2.0 * leo.radius - 0.2, "GEO {} should be far out next to LEO {}", geo.radius, leo.radius);
    // Inclination from the catalog: the station's 51.6, near-polar SSO, equatorial GEO.
    assert_eq!([leo.inclination_deg, sso.inclination_deg, geo.inclination_deg], [51.6, 97.8, 0.0]);
    // Speed: over the same share of the Run, the low Orbits sweep many times the angle GEO does.
    let swept = |a: &OrbitView, b: &OrbitView| (b.angle - a.angle).rem_euclid(std::f64::consts::TAU);
    assert!(swept(&leo, &leo_later) > swept(&sso, &sso_later));
    assert!(swept(&sso, &sso_later) > 10.0 * swept(&geo, &geo_later));
}

#[test]
fn the_spacecraft_starts_in_the_middle_of_the_shadow_and_moves_only_with_playback() {
    let c = load_catalog().unwrap();
    let d = crop_watch_known_good();
    let r = flown(&c, &d, CROP_WATCH);
    let start = view_at(&c, &d, &r, 0.0);
    assert_eq!((start.angle, start.earth_turn, start.day), (0.0, 0.0, 0));
    assert!(start.in_shadow);
    // Paused: the same playback position gives the very same view, however often it is asked for.
    let a = view_at(&c, &d, &r, 12.3);
    assert_eq!(a, view_at(&c, &d, &r, 12.3));
    // Playing on moves the spacecraft and turns Earth.
    let b = view_at(&c, &d, &r, 12.4);
    assert_ne!(a.angle, b.angle);
    assert_ne!(a.earth_turn, b.earth_turn);
    assert_eq!(b.playback, 12.4);
}

#[test]
fn skipping_to_the_end_shows_the_last_day() {
    let c = load_catalog().unwrap();
    let d = crop_watch_known_good();
    let r = flown(&c, &d, CROP_WATCH);
    let days = r.timeline.len();
    let end = view_at(&c, &d, &r, days as f64);
    assert_eq!(end.day, days);
    // Skipping straight to the end lands where playing through would.
    assert_eq!(end, view_at(&c, &d, &r, days as f64));
    assert_eq!(end.shadow_fraction, r.timeline[days - 1].in_eclipse_fraction);
}

#[test]
fn a_geo_spacecraft_stays_over_the_same_place_on_the_turning_earth() {
    let c = load_catalog().unwrap();
    let d = connect_the_villages_known_good();
    let r = flown(&c, &d, CONNECT_THE_VILLAGES);
    let days = r.timeline.len() as f64;
    for k in 0..=20 {
        let v = view_at(&c, &d, &r, days * k as f64 / 20.0);
        let gap = (v.angle - v.earth_turn).rem_euclid(std::f64::consts::TAU);
        assert!(gap < 0.01 || gap > std::f64::consts::TAU - 0.01, "GEO drifts {gap} rad from its ground point");
    }
}

#[test]
fn the_orbit_view_is_only_on_the_run() {
    let c = load_catalog().unwrap();
    let d = crop_watch_known_good();
    let r = flown(&c, &d, CROP_WATCH);
    use Screen::*;
    for screen in [Title, Sources, TeamSetup, ScenarioSelect, Brief, Design, LaunchReview, Debrief, History] {
        let spec = scene::build(&SceneInputs { catalog: Some(&c), design: Some(&d), run: Some(&r), playback: 3.0, ..SceneInputs::on(screen) });
        assert!(spec.orbit.is_none(), "{screen:?} has an orbit view");
    }
}

#[test]
fn the_run_frames_the_orbit_over_a_clear_scene_with_the_designed_spacecraft() {
    let c = load_catalog().unwrap();
    let d = crop_watch_known_good();
    let r = flown(&c, &d, CROP_WATCH);
    let spec = scene::build(&SceneInputs { catalog: Some(&c), design: Some(&d), run: Some(&r), playback: 3.0, ..SceneInputs::on(Screen::Run) });
    assert!(!spec.dimmed, "the Run shows the scene clearly");
    let parts = spec.spacecraft.expect("the Run shows the Design's spacecraft").parts;
    assert!(parts.iter().any(|p| p.id == "cubesat-6u"));
    let json: serde_json::Value = serde_json::from_str(&scene::to_json(&scene::build(&SceneInputs {
        catalog: Some(&c),
        design: Some(&d),
        run: Some(&r),
        playback: 3.0,
        ..SceneInputs::on(Screen::Run)
    })))
    .unwrap();
    assert_eq!(json["shot"]["kind"], "orbit");
    assert_eq!(json["shot"]["radius"], json["orbit"]["radius"]);
    for key in ["radius", "inclinationDeg", "angle", "earthTurn", "shadowFraction", "inShadow", "day", "playback"] {
        assert!(!json["orbit"][key].is_null(), "orbit view JSON lacks {key}: {}", json["orbit"]);
    }
}
