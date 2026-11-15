//! Every Scenario is playable and has a real trade-off: a known-good Design reaches Success and tempting but
//! wrong Designs fail Launch Review or the Run. All through the public `sim` interface.
mod common;

use common::*;
use spaceooops::sim::*;
use ReliabilityTier::FlightProven;

const SEEDS: std::ops::Range<u64> = 0..300;

fn review(design: &Design, scenario: &str) -> LaunchReview {
    launch_review(&load_catalog().unwrap(), design, scenario)
}

fn success_rate(design: &Design, scenario: &str) -> f64 {
    let catalog = load_catalog().unwrap();
    let wins = SEEDS.filter(|&seed| run(&catalog, design, scenario, seed).debrief.success).count();
    let rate = wins as f64 / SEEDS.count() as f64;
    eprintln!("{scenario}: success rate {rate:.3} for {design:?}");
    rate
}

fn has_violation(r: &LaunchReview, kind: FindingKind) -> bool {
    r.violations.iter().any(|f| f.kind == kind)
}

fn known_goods() -> [(&'static str, Design); 4] {
    [
        (CROP_WATCH, crop_watch_known_good()),
        (CONNECT_THE_VILLAGES, connect_the_villages_known_good()),
        (SPACE_WEATHER_SENTINEL, space_weather_sentinel_known_good()),
        (AMAZON_WATCH, amazon_watch_known_good()),
    ]
}

fn ids<T>(items: &[T], id: impl Fn(&T) -> &str) -> Vec<String> {
    items.iter().map(|o| id(o).to_string()).collect()
}

// ---------------------------------------------------------------- catalog breadth

#[test]
fn every_scenario_has_difficulty_key_constraints_and_a_target_country() {
    let catalog = load_catalog().unwrap();
    for id in [CROP_WATCH, CONNECT_THE_VILLAGES, SPACE_WEATHER_SENTINEL, AMAZON_WATCH] {
        let s = catalog.scenario(id).unwrap_or_else(|| panic!("{id} is bundled"));
        assert!(!s.difficulty.is_empty() && !s.key_constraints.is_empty() && !s.why_it_matters.is_empty(), "{s:?}");
        assert!(!s.target_country.id.is_empty() && !s.target_country.name.is_empty(), "{s:?}");
    }
    let angolan: Vec<&str> =
        catalog.scenarios.iter().filter(|s| s.target_country.id == "angola").map(|s| s.difficulty.as_str()).collect();
    assert_eq!(angolan, ["Easy", "Medium", "Hard"]);
    let amazon = catalog.scenario(AMAZON_WATCH).unwrap();
    assert_eq!((amazon.difficulty.as_str(), amazon.target_country.name.as_str()), ("Medium", "Brazil"));
}

#[test]
fn the_full_catalog_is_bundled() {
    let catalog = load_catalog().unwrap();
    assert_eq!(ids(&catalog.orbits, |o| &o.id), ["leo-400", "sso-600", "geo"]);
    assert_eq!(ids(&catalog.structures, |o| &o.id), ["cubesat-3u", "cubesat-6u", "cubesat-12u", "smallsat"]);
    assert_eq!(ids(&catalog.batteries, |o| &o.id), ["battery-s", "battery-m", "battery-l"]);
    assert_eq!(ids(&catalog.comms, |o| &o.id), ["comms-uhf", "comms-s-band", "comms-x-band"]);
    assert!((6..=8).contains(&catalog.instruments.len()), "{}", catalog.instruments.len());
    assert!(catalog.launch_vehicles.len() >= 5);
    assert!(catalog.launch_vehicles.iter().all(|lv| !lv.real_example.is_empty() && !lv.why.is_empty()));
    // every Orbit is reachable and rated for every Scenario; every Scenario has Instruments that suit it
    for orbit in &catalog.orbits {
        assert!(catalog.launch_vehicles.iter().any(|lv| lv.capacity_kg.contains_key(&orbit.id)), "{}", orbit.id);
        for s in &catalog.scenarios {
            assert!(orbit.coverage.contains_key(&s.id), "{} has no coverage for {}", orbit.id, s.id);
        }
    }
    for s in &catalog.scenarios {
        assert!(catalog.instruments.iter().any(|i| i.fit.get(&s.id).copied().unwrap_or(0.0) >= 0.8), "{}", s.id);
    }
}

// ---------------------------------------------------------------- every Scenario is winnable

#[test]
fn every_scenario_has_a_known_good_design_that_passes_launch_review_cleanly() {
    for (scenario, design) in known_goods() {
        let r = review(&design, scenario);
        assert!(r.violations.is_empty(), "{scenario}: {:#?}", r.violations);
        assert!(r.warnings.is_empty(), "{scenario}: {:#?}", r.warnings);
    }
}

#[test]
fn every_scenario_has_a_known_good_design_that_reaches_success_with_a_fixed_seed() {
    let catalog = load_catalog().unwrap();
    for (scenario, design) in known_goods() {
        let d = run(&catalog, &design, scenario, 1).debrief;
        assert!(d.success, "{scenario}: {:#?}", d.stats);
    }
}

#[test]
fn every_known_good_design_succeeds_on_the_large_majority_of_seeds() {
    for (scenario, design) in known_goods() {
        let rate = success_rate(&design, scenario);
        assert!(rate >= 0.85, "{scenario}: known-good success rate {rate}");
    }
}

// ---------------------------------------------------------------- Connect the Villages: GEO is needed, and costly

#[test]
fn villages_a_smallsat_is_too_heavy_for_the_geo_rideshare() {
    let mut d = connect_the_villages_known_good();
    d.structure = Some("smallsat".into());
    let r = review(&d, CONNECT_THE_VILLAGES);
    assert!(has_violation(&r, FindingKind::OverMass), "{:#?}", r.violations);
    assert!(!r.can_launch);
}

#[test]
fn villages_a_big_launcher_to_geo_breaks_the_budget() {
    let mut d = connect_the_villages_known_good();
    d.launch_vehicle = Some("medium-launcher".into());
    let r = review(&d, CONNECT_THE_VILLAGES);
    assert!(has_violation(&r, FindingKind::OverBudget), "{:#?}", r.violations);
}

#[test]
fn villages_a_cubesat_rideshare_cannot_reach_geo() {
    let mut d = connect_the_villages_known_good();
    d.launch_vehicle = Some("cubesat-rideshare".into());
    let r = review(&d, CONNECT_THE_VILLAGES);
    assert!(has_violation(&r, FindingKind::OrbitUnreachable), "{:#?}", r.violations);
}

/// The cheap way: the same relay in a sun-synchronous Orbit. It launches, but a low satellite is over the
/// villages only minutes a day, so the service never earns enough.
fn villages_from_low_orbit() -> Design {
    let mut d = connect_the_villages_known_good();
    d.solar = Some(Pick::new("solar-deployable", FlightProven));
    d.orbit = Some("sso-600".into());
    d.launch_vehicle = Some("smallsat-rideshare".into());
    d
}

#[test]
fn villages_a_relay_in_low_orbit_launches_but_never_succeeds() {
    let r = review(&villages_from_low_orbit(), CONNECT_THE_VILLAGES);
    assert!(r.can_launch, "{:#?}", r.violations);
    assert_eq!(success_rate(&villages_from_low_orbit(), CONNECT_THE_VILLAGES), 0.0);
}

// ---------------------------------------------------------------- Space Weather Sentinel: heavy Data, small Budget

#[test]
fn sentinel_the_cheaper_s_band_radio_cannot_keep_up_with_the_data() {
    let mut d = space_weather_sentinel_known_good();
    d.comms = Some(Pick::new("comms-s-band", FlightProven));
    let r = review(&d, SPACE_WEATHER_SENTINEL);
    assert!(r.warnings.iter().any(|f| f.kind == FindingKind::DataAboveDownlink), "{:#?}", r.warnings);
    assert!(r.can_launch);
    assert!(success_rate(&d, SPACE_WEATHER_SENTINEL) < 0.05);
}

#[test]
fn sentinel_one_instrument_is_not_enough() {
    let mut d = space_weather_sentinel_known_good();
    d.instruments = vec!["particle-detector".into()];
    assert!(review(&d, SPACE_WEATHER_SENTINEL).can_launch);
    assert_eq!(success_rate(&d, SPACE_WEATHER_SENTINEL), 0.0);
}

#[test]
fn sentinel_a_higher_orbit_flies_above_most_of_the_ionosphere_and_falls_short() {
    let mut d = space_weather_sentinel_known_good();
    d.orbit = Some("sso-600".into());
    d.launch_vehicle = Some("cubesat-rideshare".into());
    assert!(review(&d, SPACE_WEATHER_SENTINEL).can_launch);
    assert_eq!(success_rate(&d, SPACE_WEATHER_SENTINEL), 0.0);
}

#[test]
fn sentinel_a_bigger_launcher_breaks_the_small_budget() {
    let mut d = space_weather_sentinel_known_good();
    d.launch_vehicle = Some("small-launcher".into());
    assert!(has_violation(&review(&d, SPACE_WEATHER_SENTINEL), FindingKind::OverBudget));
}

// ---------------------------------------------------------------- Amazon Watch: optics are blind under cloud

/// The tempting cheap way: Crop Watch's imager spacecraft over the Amazon. It launches, but clouds hide the
/// forest on most passes, so it never earns the Science Value.
#[test]
fn amazon_an_optical_imager_on_sso_launches_but_falls_short() {
    let d = crop_watch_known_good();
    let r = review(&d, AMAZON_WATCH);
    assert!(r.can_launch, "{:#?}", r.violations);
    assert_eq!(success_rate(&d, AMAZON_WATCH), 0.0);
}

#[test]
fn amazon_every_optical_instrument_together_still_falls_short() {
    let mut d = amazon_watch_known_good();
    d.instruments = vec!["multispectral-imager".into(), "thermal-imager".into(), "wide-camera".into()];
    assert!(review(&d, AMAZON_WATCH).can_launch, "{:#?}", review(&d, AMAZON_WATCH).violations);
    assert_eq!(success_rate(&d, AMAZON_WATCH), 0.0);
}

#[test]
fn amazon_the_radar_is_too_heavy_for_a_6u_rideshare_slot() {
    let mut d = amazon_watch_known_good();
    d.launch_vehicle = Some("cubesat-rideshare".into());
    assert!(has_violation(&review(&d, AMAZON_WATCH), FindingKind::OverMass));
}

#[test]
fn amazon_the_radar_needs_deployable_panels() {
    let mut d = amazon_watch_known_good();
    d.solar = Some(Pick::new("solar-body", FlightProven));
    assert!(!review(&d, AMAZON_WATCH).can_launch);
}

#[test]
fn amazon_an_s_band_radio_cannot_keep_up_with_the_radar() {
    let mut d = amazon_watch_known_good();
    d.comms = Some(Pick::new("comms-s-band", FlightProven));
    let r = review(&d, AMAZON_WATCH);
    assert!(r.warnings.iter().any(|f| f.kind == FindingKind::DataAboveDownlink), "{:#?}", r.warnings);
}

/// The radar is no shortcut in Angola: there it is both unaffordable and poorly suited.
#[test]
fn the_radar_is_not_the_answer_in_the_angolan_scenarios() {
    let catalog = load_catalog().unwrap();
    let radar = catalog.instrument("sar-radar").unwrap();
    for (scenario, good) in known_goods().into_iter().filter(|(s, _)| *s != AMAZON_WATCH) {
        let best_other = good.instruments.iter().map(|i| catalog.instrument(i).unwrap().fit[scenario]).fold(0.0, f64::max);
        assert!(radar.fit[scenario] < best_other / 2.0, "{scenario}: radar fit {}", radar.fit[scenario]);
    }
    let mut d = amazon_watch_known_good();
    for scenario in [CROP_WATCH, SPACE_WEATHER_SENTINEL] {
        assert!(has_violation(&review(&d, scenario), FindingKind::OverBudget), "{scenario}");
    }
    d.orbit = Some("geo".into());
    d.launch_vehicle = Some("geo-rideshare".into());
    assert!(!review(&d, CONNECT_THE_VILLAGES).can_launch);
}

// ---------------------------------------------------------------- the Orbit drives Lifetime, Power and Data

#[test]
fn changing_the_orbit_changes_lifetime_power_and_data_gauges() {
    let in_orbit = |orbit: &str| {
        let mut d = crop_watch_known_good();
        d.orbit = Some(orbit.into());
        review(&d, CROP_WATCH).gauges
    };
    let (leo, sso, geo) = (in_orbit("leo-400"), in_orbit("sso-600"), in_orbit("geo"));

    // Lifetime: drag brings a CubeSat at 400 km down within months; Crop Watch needs a year.
    assert!(leo.lifetime.value < sso.lifetime.value, "{:?} vs {:?}", leo.lifetime, sso.lifetime);
    assert_eq!(leo.lifetime.state, GaugeState::Violated);
    assert_eq!(sso.lifetime.state, GaugeState::Ok);
    // Power: GEO is almost always in sunlight; low orbits spend over a third of each orbit in shadow.
    assert!(geo.power.limit > sso.power.limit && sso.power.limit > leo.power.limit);
    // Data: GEO is always in view of its ground station; LEO 400 km gets the fewest minutes of contact.
    assert!(geo.data.limit > sso.data.limit && sso.data.limit > leo.data.limit);
    assert!(leo.lifetime.detail != sso.lifetime.detail && leo.data.detail != sso.data.detail);
}

#[test]
fn crop_watch_from_leo_400_comes_down_before_the_season_ends() {
    let mut d = crop_watch_known_good();
    d.orbit = Some("leo-400".into());
    d.launch_vehicle = Some("station-deploy".into());
    let result = run(&load_catalog().unwrap(), &d, CROP_WATCH, 1);
    assert!(!result.debrief.success);
    assert!(result.debrief.stats.days_survived < 365);
}
