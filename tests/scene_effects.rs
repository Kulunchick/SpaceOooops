//! Scene Spec: Random Events and Decisions as effects in the Run scene (low-poly ticket 05). Effects come only
//! from the Run timeline (which carries the Decisions taken) and the playback position, so pausing, speeding up
//! and skipping can never put the scene out of step with the event log and the dashboards.
mod common;

use common::*;
use spaceooops::scene::effects::{BACKUP, PRIMARY};
use spaceooops::scene::{self, Cue, EffectKind, SceneInputs, Screen};
use spaceooops::sim::{load_catalog, run, run_with_decisions, Catalog, Decision, Design, EventKind, ReliabilityTier, RunResult};

/// The Scene Spec's effects at a playback position of a Crop Watch Run.
fn effects_at(c: &Catalog, d: &Design, r: &RunResult, playback: f64) -> Vec<EffectKind> {
    scene::build(&SceneInputs {
        catalog: Some(c),
        design: Some(d),
        chosen: Some(CROP_WATCH),
        run: Some(r),
        playback,
        ..SceneInputs::on(Screen::Run)
    })
    .effects
    .into_iter()
    .map(|e| e.kind)
    .collect()
}

/// Playback positions inside day `day` (1-based): just after it starts, its middle, and its very end.
fn inside(day: u32) -> [f64; 3] {
    let d = f64::from(day);
    [d - 1.0 + 1e-6, d - 0.5, d]
}

/// The days (1-based) on which `kind` is active somewhere in the day, sampled at the positions of `inside`.
fn days_with(c: &Catalog, d: &Design, r: &RunResult, kind: impl Fn(&EffectKind) -> bool) -> Vec<u32> {
    (1..=r.timeline.len() as u32)
        .filter(|&day| inside(day).iter().any(|&p| effects_at(c, d, r, p).iter().any(&kind)))
        .collect()
}

/// Seeds (from 0) whose Crop Watch Run with default choices has at least one Random Event of `kind`.
fn seeds_with(c: &Catalog, d: &Design, kind: EventKind, how_many: usize) -> Vec<(u64, RunResult)> {
    (0..2000)
        .map(|seed| (seed, run(c, d, CROP_WATCH, seed)))
        .filter(|(_, r)| r.timeline.iter().any(|t| t.events.iter().any(|e| e.kind == kind)))
        .take(how_many)
        .collect()
}

fn event_days(r: &RunResult, kind: EventKind) -> Vec<u32> {
    r.timeline.iter().filter(|t| t.events.iter().any(|e| e.kind == kind)).map(|t| t.day).collect()
}

#[test]
fn a_solar_flare_wave_is_in_the_scene_on_the_flare_day_and_on_no_other_day() {
    let c = load_catalog().unwrap();
    let d = crop_watch_known_good();
    for (seed, r) in seeds_with(&c, &d, EventKind::SolarFlare, 5) {
        let flares = days_with(&c, &d, &r, |k| *k == EffectKind::Flare);
        assert_eq!(flares, event_days(&r, EventKind::SolarFlare), "seed {seed}");
    }
}

/// The days covered by windows of `len` days starting on each of `starts`, cut at the end of the Run.
fn windows(starts: &[u32], len: u32, run_days: usize) -> Vec<u32> {
    let mut days: Vec<u32> =
        starts.iter().flat_map(|&s| s..s + len).filter(|&d| d as usize <= run_days).collect();
    days.sort();
    days.dedup();
    days
}

fn decide_all(r: &RunResult, choice: &str) -> Vec<Decision> {
    r.timeline
        .iter()
        .flat_map(|t| &t.events)
        .filter_map(|e| e.prompt.as_ref())
        .filter(|p| p.options.iter().any(|o| o.id == choice))
        .map(|p| Decision { prompt_id: p.id.clone(), choice: choice.to_string() })
        .collect()
}

#[test]
fn safe_mode_lasts_from_the_flare_day_for_the_scenarios_safe_mode_days() {
    let c = load_catalog().unwrap();
    let d = crop_watch_known_good();
    let safe_days = c.scenario(CROP_WATCH).unwrap().events.flare_safe_mode_days;
    assert!(safe_days > 1, "the check needs a safe mode longer than the flare day");
    for (seed, r) in seeds_with(&c, &d, EventKind::SolarFlare, 5) {
        // Safe mode is the default answer to a flare warning.
        let expected = windows(&event_days(&r, EventKind::SolarFlare), safe_days, r.timeline.len());
        assert_eq!(days_with(&c, &d, &r, |k| *k == EffectKind::SafeMode), expected, "seed {seed}");
    }
}

fn spec_at(c: &Catalog, d: &Design, r: &RunResult, playback: f64) -> scene::SceneSpec {
    scene::build(&SceneInputs {
        catalog: Some(c),
        design: Some(d),
        chosen: Some(CROP_WATCH),
        run: Some(r),
        playback,
        ..SceneInputs::on(Screen::Run)
    })
}

#[test]
fn the_run_shows_a_main_ground_station_in_angola_and_a_backup_station() {
    let c = load_catalog().unwrap();
    let d = crop_watch_known_good();
    let r = run(&c, &d, CROP_WATCH, 1);
    let stations = spec_at(&c, &d, &r, 3.0).stations;
    assert_eq!(stations.iter().map(|s| s.id.as_str()).collect::<Vec<_>>(), [PRIMARY, BACKUP]);
    let main = &stations[0];
    assert!((-18.0..-4.5).contains(&main.lat) && (11.5..24.0).contains(&main.lon), "{main:?} is not in Angola");
    let backup = &stations[1];
    assert!((-90.0..90.0).contains(&backup.lat) && (-180.0..180.0).contains(&backup.lon));
    assert_ne!((main.lat, main.lon), (backup.lat, backup.lon));
    // Only the Run has ground stations.
    let elsewhere = scene::build(&SceneInputs { run: Some(&r), ..SceneInputs::on(Screen::Debrief) });
    assert!(elsewhere.stations.is_empty() && elsewhere.effects.is_empty());
}

#[test]
fn the_main_stations_beacon_is_dark_for_the_whole_outage_and_on_no_other_day() {
    let c = load_catalog().unwrap();
    let d = crop_watch_known_good();
    let outage_days = c.scenario(CROP_WATCH).unwrap().events.outage_days;
    for (seed, plain) in seeds_with(&c, &d, EventKind::GroundStationOutage, 5) {
        // Whether the player waits or buys time on another station, the main station is offline.
        let bought = run_with_decisions(&c, &d, CROP_WATCH, seed, &decide_all(&plain, "buy-station-time"));
        for r in [&plain, &bought] {
            let expected = windows(&event_days(r, EventKind::GroundStationOutage), outage_days, r.timeline.len());
            let dark = days_with(&c, &d, r, |k| *k == EffectKind::StationDark { station: PRIMARY.into() });
            assert_eq!(dark, expected, "seed {seed}");
            assert!(days_with(&c, &d, r, |k| *k == EffectKind::StationDark { station: BACKUP.into() }).is_empty());
        }
    }
}

/// Days on which Data came down (the cumulative delivered Data grew).
fn downlink_days(r: &RunResult) -> Vec<u32> {
    let mut before = 0.0;
    r.timeline
        .iter()
        .filter_map(|t| {
            let grew = t.delivered_data_mb > before;
            before = t.delivered_data_mb;
            grew.then_some(t.day)
        })
        .collect()
}

#[test]
fn a_contact_beam_links_the_spacecraft_to_a_station_on_each_day_data_comes_down() {
    let c = load_catalog().unwrap();
    let d = crop_watch_known_good();
    let outage_days = c.scenario(CROP_WATCH).unwrap().events.outage_days;
    for (seed, plain) in seeds_with(&c, &d, EventKind::GroundStationOutage, 5) {
        let contact = |r: &RunResult, station: &str| {
            days_with(&c, &d, r, |k| *k == EffectKind::Contact { station: station.to_string() })
        };
        // Waiting the outage out: no Data comes down, so no beam, until the main station is back.
        assert_eq!(contact(&plain, PRIMARY), downlink_days(&plain), "seed {seed}");
        assert!(contact(&plain, BACKUP).is_empty());
        let outage = windows(&event_days(&plain, EventKind::GroundStationOutage), outage_days, plain.timeline.len());
        assert!(outage.iter().all(|day| !contact(&plain, PRIMARY).contains(day)));

        // Time bought on another station: the beam goes to the backup station during the outage.
        let bought = run_with_decisions(&c, &d, CROP_WATCH, seed, &decide_all(&plain, "buy-station-time"));
        let outage = windows(&event_days(&bought, EventKind::GroundStationOutage), outage_days, bought.timeline.len());
        let (during, outside): (Vec<u32>, Vec<u32>) = downlink_days(&bought).into_iter().partition(|d| outage.contains(d));
        assert!(!during.is_empty(), "seed {seed}: bought time brings Data down during the outage");
        assert_eq!(contact(&bought, BACKUP), during, "seed {seed}");
        assert_eq!(contact(&bought, PRIMARY), outside, "seed {seed}");
    }
}

fn all_basic() -> Design {
    let mut d = crop_watch_known_good();
    for pick in [&mut d.solar, &mut d.battery, &mut d.attitude, &mut d.comms] {
        pick.as_mut().unwrap().tier = ReliabilityTier::Basic;
    }
    d.computer_tier = ReliabilityTier::Basic;
    d
}

#[test]
fn a_failed_component_is_marked_on_its_part_from_the_failure_day_to_the_end_of_the_run() {
    let c = load_catalog().unwrap();
    let d = all_basic();
    for (seed, r) in seeds_with(&c, &d, EventKind::ComponentFailure, 8) {
        let failed: Vec<String> = r.timeline.last().unwrap().failed_components.clone();
        assert!(!failed.is_empty());
        let parts = spec_at(&c, &d, &r, 1.0).spacecraft.unwrap().parts;
        for id in &failed {
            assert!(parts.iter().any(|p| &p.id == id), "seed {seed}: {id} is a part of the model");
            let expected: Vec<u32> =
                r.timeline.iter().filter(|t| t.failed_components.contains(id)).map(|t| t.day).collect();
            let marked = days_with(&c, &d, &r, |k| *k == EffectKind::Failure { part: id.clone() });
            assert_eq!(marked, expected, "seed {seed}: {id}");
        }
    }
}

#[test]
fn a_battery_shortfall_is_marked_on_the_battery_on_the_day_it_starts() {
    let c = load_catalog().unwrap();
    let mut d = crop_watch_known_good();
    d.battery = Some(spaceooops::sim::Pick::new("battery-s", ReliabilityTier::FlightProven));
    let r = run(&c, &d, CROP_WATCH, 3);
    let days = event_days(&r, EventKind::BatteryShortfall);
    assert_eq!(days.len(), 1);
    let marked = days_with(&c, &d, &r, |k| *k == EffectKind::BatteryShortfall { part: "battery-s".into() });
    assert_eq!(marked, days);
}

fn cue_for(kind: EventKind) -> Cue {
    match kind {
        EventKind::SolarFlare => Cue::Flare,
        EventKind::GroundStationOutage => Cue::Outage,
        EventKind::ComponentFailure => Cue::Failure,
        EventKind::BatteryShortfall => Cue::BatteryShortfall,
    }
}

/// Plays a Run back in small steps, as the playback driver does, and collects the one-shot Cues with the day
/// the playback was on when each was heard.
fn heard(c: &Catalog, d: &Design, r: &RunResult, step: f64) -> Vec<(u32, Cue)> {
    let mut out = Vec::new();
    let mut previous = spec_at(c, d, r, 0.0);
    let mut p = 0.0;
    while p < r.timeline.len() as f64 {
        p = (p + step).min(r.timeline.len() as f64);
        let current = spec_at(c, d, r, p);
        out.extend(scene::one_shots(&previous, &current).into_iter().map(|cue| (p.ceil() as u32, cue)));
        previous = current;
    }
    out
}

fn expected_cues(r: &RunResult) -> Vec<(u32, Cue)> {
    let mut v: Vec<(u32, Cue)> =
        r.timeline.iter().flat_map(|t| t.events.iter().map(move |e| (t.day, cue_for(e.kind)))).collect();
    v.dedup();
    v
}

#[test]
fn each_random_event_kind_is_heard_with_its_own_cue_when_the_playback_reaches_its_day() {
    let c = load_catalog().unwrap();
    let names: std::collections::BTreeSet<&str> =
        [Cue::Flare, Cue::Outage, Cue::Failure, Cue::BatteryShortfall].iter().map(|c| c.name()).collect();
    assert_eq!(names.len(), 4, "every Random Event kind has its own sound");
    assert!(!names.contains("snap") && !names.contains("launch"));

    let mut kinds_heard = std::collections::BTreeSet::new();
    let mut small_battery = all_basic();
    small_battery.battery = Some(spaceooops::sim::Pick::new("battery-s", ReliabilityTier::Basic));
    for d in [crop_watch_known_good(), all_basic(), small_battery] {
        for (seed, r) in seeds_with(&c, &d, EventKind::GroundStationOutage, 3) {
            let mut got = heard(&c, &d, &r, 0.3);
            got.dedup();
            assert_eq!(got, expected_cues(&r), "seed {seed}");
            kinds_heard.extend(got.into_iter().map(|x| x.1.name()));
        }
    }
    assert_eq!(kinds_heard.into_iter().collect::<std::collections::BTreeSet<_>>(), names);
}

#[test]
fn a_paused_or_skipped_run_plays_no_event_cues() {
    let c = load_catalog().unwrap();
    let d = all_basic();
    let (_, r) = seeds_with(&c, &d, EventKind::ComponentFailure, 1).remove(0);
    let first = event_days(&r, EventKind::ComponentFailure)[0];
    let on_the_day = spec_at(&c, &d, &r, f64::from(first) - 0.5);
    assert!(scene::one_shots(&on_the_day, &on_the_day).is_empty(), "paused on an event day");
    // Skipping from the start to the end jumps past every event: nothing replays.
    let end = r.timeline.len() as f64;
    let skipped = scene::one_shots(&spec_at(&c, &d, &r, 0.5), &spec_at(&c, &d, &r, end));
    let last_day_events = r.timeline.last().unwrap().events.len();
    assert!(skipped.len() <= last_day_events, "{skipped:?}");
}

/// The playback driver's steps for a Run: 30 s per Run at 1x in 50 ms ticks, and 1.5 s per event day.
fn steps(r: &RunResult, speed: f64) -> (f64, f64) {
    (r.timeline.len() as f64 / 600.0 * speed, speed * 0.05 / 1.5)
}

/// Every playback position the driver passes through, from launch to the end of the Run.
fn positions(r: &RunResult, speed: f64) -> Vec<f64> {
    let (step, slow) = steps(r, speed);
    let mut out = Vec::new();
    let mut p = 0.0;
    while p < r.timeline.len() as f64 {
        p = scene::effects::advance(r, p, step, slow);
        out.push(p);
    }
    out
}

#[test]
fn the_playback_lingers_on_random_event_days_at_any_speed_so_every_effect_is_seen() {
    let c = load_catalog().unwrap();
    let d = all_basic();
    for (seed, r) in seeds_with(&c, &d, EventKind::GroundStationOutage, 4) {
        for speed in [1.0, 4.0] {
            let ps = positions(&r, speed);
            for t in r.timeline.iter().filter(|t| !t.events.is_empty()) {
                let k = f64::from(t.day);
                let ticks = ps.iter().filter(|&&p| k - 1.0 < p && p <= k).count();
                assert!(ticks >= 6, "seed {seed} at {speed}x: day {} shown for {ticks} ticks", t.day);
            }
            // Ordinary days keep the normal pace: the Run takes little longer than its 30 s.
            let event_days = r.timeline.iter().filter(|t| !t.events.is_empty()).count() as f64;
            let ticks = ps.len() as f64;
            // (Each event day costs its 1.5 s plus up to two short steps where the playback lines up with it.)
            assert!(ticks <= 600.0 / speed + 1.0 + event_days * (30.0 / speed + 2.0), "seed {seed}: {ticks} ticks");
            assert_eq!(*ps.last().unwrap(), r.timeline.len() as f64);
        }
    }
}

#[test]
fn at_four_times_speed_every_random_event_is_still_heard_on_its_day() {
    let c = load_catalog().unwrap();
    let d = all_basic();
    for (seed, r) in seeds_with(&c, &d, EventKind::SolarFlare, 4) {
        let mut previous = spec_at(&c, &d, &r, 0.0);
        let mut got = Vec::new();
        for p in positions(&r, 4.0) {
            let current = spec_at(&c, &d, &r, p);
            got.extend(scene::one_shots(&previous, &current).into_iter().map(|cue| (p.ceil() as u32, cue)));
            previous = current;
        }
        got.dedup();
        assert_eq!(got, expected_cues(&r), "seed {seed}");
    }
}

#[test]
fn the_bridge_receives_effects_with_their_kind_target_start_and_length() {
    let c = load_catalog().unwrap();
    let d = crop_watch_known_good();
    let (_, r) = seeds_with(&c, &d, EventKind::GroundStationOutage, 1).remove(0);
    let day = event_days(&r, EventKind::GroundStationOutage)[0];
    let json: serde_json::Value =
        serde_json::from_str(&scene::to_json(&spec_at(&c, &d, &r, f64::from(day) - 0.5))).unwrap();
    let dark = json["effects"].as_array().unwrap().iter().find(|e| e["kind"] == "stationDark").expect("{json}");
    assert_eq!(dark["station"], "primary");
    assert_eq!(dark["start"], f64::from(day) - 1.0);
    assert_eq!(dark["days"], 3.0);
    assert!(dark["id"].is_string());
    assert_eq!(json["stations"][0]["id"], "primary");
    assert!(json["stations"][0]["lat"].is_number() && json["stations"][0]["lon"].is_number());
}

#[test]
fn riding_out_a_flare_shows_the_wave_but_no_safe_mode() {
    let c = load_catalog().unwrap();
    let d = crop_watch_known_good();
    for (seed, plain) in seeds_with(&c, &d, EventKind::SolarFlare, 5) {
        let ridden = run_with_decisions(&c, &d, CROP_WATCH, seed, &decide_all(&plain, "ride-out"));
        assert_eq!(days_with(&c, &d, &ridden, |k| *k == EffectKind::Flare), event_days(&ridden, EventKind::SolarFlare));
        assert!(days_with(&c, &d, &ridden, |k| *k == EffectKind::SafeMode).is_empty(), "seed {seed}");
    }
}
