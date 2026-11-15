//! The Debrief teaches why: causes with this Run's numbers, the Score breakdown and the Real-Mission Match.
//! All through the public `sim` interface.
mod common;

use common::*;
use spaceooops::sim::*;
use ReliabilityTier::{Basic, FlightProven};

fn fly(design: &Design, scenario: &str, seed: u64) -> Debrief {
    run(&legacy_catalog(), design, scenario, seed).debrief
}

fn cause(d: &Debrief, kind: CauseKind) -> &Cause {
    d.causes.iter().find(|c| c.kind == kind).unwrap_or_else(|| panic!("no {kind:?} cause in {:#?}", d.causes))
}

fn texts(d: &Debrief) -> String {
    d.causes.iter().map(|c| format!("[{:?}] {}", c.kind, c.text)).collect::<Vec<_>>().join("\n")
}

// ---------------------------------------------------------------- battery

#[test]
fn an_undersized_battery_produces_a_battery_cause_with_the_eclipse_and_the_time_lost() {
    let mut design = crop_watch_known_good();
    design.battery = Some(Pick::new("battery-s", FlightProven));
    let d = fly(&design, CROP_WATCH, 3);
    let c = cause(&d, CauseKind::BatteryTooSmall);
    assert!(c.text.contains("Battery S"), "{}", c.text);
    assert!(c.text.contains("35 min"), "names the minutes in Earth's shadow: {}", c.text);
    assert!(c.text.contains("% of"), "says how much operating time was lost: {}", c.text);
    assert!(c.text.contains("Battery M") || c.text.contains("Battery L"), "says what would fix it: {}", c.text);
    assert!(c.day.is_some());
}

// ---------------------------------------------------------------- Data

#[test]
fn data_above_downlink_produces_a_lost_data_cause_naming_the_band_the_share_and_a_fix() {
    let mut design = space_weather_sentinel_known_good();
    design.comms = Some(Pick::new("comms-s-band", FlightProven));
    let d = fly(&design, SPACE_WEATHER_SENTINEL, 1);
    let c = cause(&d, CauseKind::DataLost);
    assert!(c.text.contains("S-band radio"), "{}", c.text);
    let lost = format!("{:.0}%", d.stats.lost_data_fraction * 100.0);
    assert!(c.text.contains(&lost), "names the lost share {lost}: {}", c.text);
    assert!(c.text.contains("X-band radio"), "says what would fix it: {}", c.text);
}

#[test]
fn a_uhf_radio_on_an_imager_puts_lost_data_first_even_when_bad_luck_also_strikes() {
    let mut design = crop_watch_known_good();
    design.comms = Some(Pick::new("comms-uhf", FlightProven));
    for seed in 0..100 {
        let d = fly(&design, CROP_WATCH, seed);
        assert_eq!(d.causes[0].kind, CauseKind::DataLost, "seed {seed}:\n{}", texts(&d));
        assert!(d.causes[0].text.contains("UHF radio"), "{}", texts(&d));
    }
}

// ---------------------------------------------------------------- Component failure

fn all_basic() -> Design {
    let mut d = crop_watch_known_good();
    for pick in [&mut d.solar, &mut d.battery, &mut d.attitude, &mut d.comms] {
        pick.as_mut().unwrap().tier = Basic;
    }
    d.computer_tier = Basic;
    d
}

#[test]
fn a_component_failure_that_ends_the_run_early_names_the_component_its_tier_and_the_day() {
    let catalog = legacy_catalog();
    let r = (0..300)
        .map(|seed| run(&catalog, &all_basic(), CROP_WATCH, seed))
        .find(|r| r.timeline.len() < 365)
        .expect("some all-Basic Run ends early");
    let last = r.timeline.last().unwrap();
    let failed = last.events.iter().find(|e| e.kind == EventKind::ComponentFailure).unwrap();
    let c = cause(&r.debrief, CauseKind::ComponentFailure);
    assert_eq!(c.day, Some(last.day));
    assert!(c.text.contains(&format!("day {}", last.day)), "{}", c.text);
    assert!(c.text.contains("Basic"), "names the Reliability Tier: {}", c.text);
    let names = [catalog.computer.name.clone()]
        .into_iter()
        .chain(catalog.batteries.iter().map(|o| o.name.clone()))
        .chain(catalog.solar_panels.iter().map(|o| o.name.clone()))
        .chain(catalog.comms.iter().map(|o| o.name.clone()));
    let name = names.filter(|n| failed.text.contains(n.as_str())).max_by_key(|n| n.len()).unwrap();
    assert!(c.text.contains(name.as_str()), "names the {name}: {}", c.text);
    assert_eq!(r.debrief.causes[0].kind, CauseKind::ComponentFailure, "{}", texts(&r.debrief));
}

// ---------------------------------------------------------------- Score

#[test]
fn the_score_breakdown_adds_up_to_the_score_and_an_early_end_is_a_negative_line() {
    let catalog = legacy_catalog();
    for seed in 0..60 {
        for design in [all_basic(), crop_watch_known_good()] {
            let d = run(&catalog, &design, CROP_WATCH, seed).debrief;
            let sum: f64 = d.score.lines.iter().map(|l| l.points).sum();
            assert!((d.score.total - sum).abs() < 1e-9, "{:#?}", d.score);
            if !d.stats.lifetime_met {
                let penalty = d.score.lines.iter().find(|l| l.kind == ScoreLineKind::Penalty).expect("penalty line");
                assert!(penalty.points < 0.0, "{penalty:?}");
            }
        }
    }
}

// ---------------------------------------------------------------- not launched

#[test]
fn a_design_that_cannot_launch_gives_its_launch_review_violations_as_causes() {
    let catalog = legacy_catalog();
    let mut design = crop_watch_known_good();
    design.attitude = Some(Pick::new("magnetorquers", FlightProven));
    let violations = launch_review(&catalog, &design, CROP_WATCH).violations;
    let d = fly(&design, CROP_WATCH, 1);
    assert!(!d.causes.is_empty());
    assert!(d.causes.iter().all(|c| c.kind == CauseKind::NotLaunched));
    assert!(d.causes[0].text.contains(&violations[0].reason), "{}", texts(&d));
}

// ---------------------------------------------------------------- Success

#[test]
fn a_success_still_says_what_went_right_and_what_held_the_score_down() {
    for (scenario, design) in [
        (CROP_WATCH, crop_watch_known_good()),
        (CONNECT_THE_VILLAGES, connect_the_villages_known_good()),
        (SPACE_WEATHER_SENTINEL, space_weather_sentinel_known_good()),
    ] {
        let d = fly(&design, scenario, 1);
        assert!(d.success);
        assert_eq!(d.causes[0].kind, CauseKind::WentWell, "{}", texts(&d));
        assert!(d.causes.iter().any(|c| c.kind != CauseKind::WentWell), "{}", texts(&d));
    }
}

// ---------------------------------------------------------------- Orbit and Instruments

#[test]
fn the_wrong_orbit_for_the_objective_produces_a_coverage_cause_pointing_to_a_better_orbit() {
    let mut design = connect_the_villages_known_good();
    design.solar = Some(Pick::new("solar-deployable", FlightProven));
    design.orbit = Some("sso-600".into());
    design.launch_vehicle = Some("smallsat-rideshare".into());
    let d = fly(&design, CONNECT_THE_VILLAGES, 1);
    let c = cause(&d, CauseKind::PoorCoverage);
    assert!(c.text.contains("Sun-synchronous") && c.text.contains("5%"), "{}", c.text);
    assert!(c.text.contains("Geostationary"), "{}", c.text);
    assert_eq!(d.causes[0].kind, CauseKind::PoorCoverage, "{}", texts(&d));
}

#[test]
fn an_instrument_that_suits_the_scenario_poorly_produces_a_fit_cause() {
    let mut design = crop_watch_known_good();
    design.instruments = vec!["wide-camera".into()];
    let d = fly(&design, CROP_WATCH, 1);
    let c = cause(&d, CauseKind::PoorInstrumentFit);
    assert!(c.text.contains("Wide-field camera") && c.text.contains("30%"), "{}", c.text);
    assert!(c.text.contains("Multispectral imager"), "{}", c.text);
}

#[test]
fn too_few_instruments_produce_a_science_shortfall_cause() {
    let mut design = space_weather_sentinel_known_good();
    design.instruments = vec!["particle-detector".into()];
    let d = fly(&design, SPACE_WEATHER_SENTINEL, 1);
    let c = cause(&d, CauseKind::ScienceShortfall);
    assert!(c.text.contains("70 required"), "{}", c.text);
}

#[test]
fn an_orbit_that_comes_down_too_soon_produces_a_drag_cause() {
    let mut design = crop_watch_known_good();
    design.orbit = Some("leo-400".into());
    design.launch_vehicle = Some("station-deploy".into());
    let d = fly(&design, CROP_WATCH, 1);
    let c = cause(&d, CauseKind::LifetimeShortfall);
    assert!(c.text.contains("drag") && c.text.contains("300 days") && c.text.contains("65 days"), "{}", c.text);
}

#[test]
fn solar_flares_that_cost_science_are_told() {
    let catalog = legacy_catalog();
    let design = space_weather_sentinel_known_good();
    let flare_runs = (0..100)
        .map(|seed| run(&catalog, &design, SPACE_WEATHER_SENTINEL, seed).debrief)
        .filter(|d| d.causes.iter().any(|c| c.kind == CauseKind::RandomEvents))
        .count();
    assert!(flare_runs > 0);
}

// ---------------------------------------------------------------- Real-Mission Match

#[test]
fn every_known_good_design_is_matched_to_a_sensible_real_mission() {
    let expected = [
        (CROP_WATCH, crop_watch_known_good(), "Planet Dove"),
        (CONNECT_THE_VILLAGES, connect_the_villages_known_good(), "Arcturus"),
        (SPACE_WEATHER_SENTINEL, space_weather_sentinel_known_good(), "Dellingr"),
        (AMAZON_WATCH, amazon_watch_known_good(), "Amazonia-1"),
    ];
    for (scenario, design, name) in expected {
        let m = fly(&design, scenario, 1).real_mission_match.expect("a match");
        assert!(m.name.contains(name), "{scenario}: {m:#?}");
        assert!(m.url.starts_with("https://") && !m.blurb.is_empty() && !m.why.is_empty(), "{m:#?}");
    }
}

#[test]
fn the_match_follows_the_objective_first_then_the_spacecraft_size() {
    // A 3U space-weather CubeSat matches the 3U student radiation CubeSat rather than the 6U one.
    let mut design = space_weather_sentinel_known_good();
    design.structure = Some("cubesat-3u".into());
    design.instruments = vec!["particle-detector".into()];
    assert!(launch_review(&legacy_catalog(), &design, SPACE_WEATHER_SENTINEL).can_launch);
    let m = fly(&design, SPACE_WEATHER_SENTINEL, 1).real_mission_match.unwrap();
    assert!(m.name.starts_with("CSSWE"), "{m:#?}");
    // The relay flown low still matches by objective (communications), and says the Orbit differs.
    let mut low = connect_the_villages_known_good();
    low.solar = Some(Pick::new("solar-deployable", FlightProven));
    low.orbit = Some("sso-600".into());
    low.launch_vehicle = Some("smallsat-rideshare".into());
    let m = fly(&low, CONNECT_THE_VILLAGES, 1).real_mission_match.unwrap();
    assert!(m.why.contains("geostationary"), "{m:#?}");
}

#[test]
fn every_real_mission_in_the_catalog_has_a_source_link_and_a_fact() {
    let catalog = legacy_catalog();
    assert!(catalog.real_missions.len() >= 6);
    for m in &catalog.real_missions {
        assert!(m.url.starts_with("https://") && m.blurb.len() > 30 && m.year >= 1990, "{m:#?}");
    }
}

// ---------------------------------------------------------------- "why it matters" tooltips

#[test]
fn every_design_parameter_gauge_and_component_category_has_a_why_it_matters_text() {
    let catalog = legacy_catalog();
    let labels = [
        // gauges and resources
        "Budget", "Mass", "Power", "Data", "Lifetime", "Cost", "Volume",
        // numbers on the option cards
        "Reliability Tier", "Time in shadow", "Ground contact", "Room inside", "Altitude", "Lifetime limit",
        "Scenario coverage", "Scenario fit", "Pointing", "Capacity", "Ground stations", "Real example",
        // Component categories
        "Structure", "Solar panels", "Battery", "Attitude control", "Onboard computer", "Comms band", "Instruments",
        "Orbit", "Launch Vehicle",
    ];
    for label in labels {
        let why = catalog.glossary_why(label).unwrap_or_else(|| panic!("no tooltip for {label}"));
        assert!(why.len() > 40, "{label}: {why}");
    }
    assert_eq!(catalog.glossary_why("eclipse"), catalog.glossary_why("Time in shadow"));
}

#[test]
fn crop_watch_and_every_scenario_outside_angola_carry_first_time_hints() {
    let catalog = legacy_catalog();
    for s in &catalog.scenarios {
        let expected = s.id == CROP_WATCH || s.target_country.id != "angola";
        assert_eq!(!s.tutorial_hints.is_empty(), expected, "{}", s.id);
    }
}

#[test]
#[ignore = "prints example Debriefs for review: cargo test --test debrief -- --ignored --nocapture"]
fn print_examples() {
    let catalog = legacy_catalog();
    let mut battery = crop_watch_known_good();
    battery.battery = Some(Pick::new("battery-s", FlightProven));
    let mut uhf = crop_watch_known_good();
    uhf.comms = Some(Pick::new("comms-uhf", FlightProven));
    let mut leo = crop_watch_known_good();
    leo.orbit = Some("leo-400".into());
    leo.launch_vehicle = Some("station-deploy".into());
    let mut wide = crop_watch_known_good();
    wide.instruments = vec!["wide-camera".into()];
    let mut one = space_weather_sentinel_known_good();
    one.instruments = vec!["particle-detector".into()];
    let mut sband = space_weather_sentinel_known_good();
    sband.comms = Some(Pick::new("comms-s-band", FlightProven));
    let mut villages_sso = connect_the_villages_known_good();
    villages_sso.solar = Some(Pick::new("solar-deployable", FlightProven));
    villages_sso.orbit = Some("sso-600".into());
    villages_sso.launch_vehicle = Some("smallsat-rideshare".into());
    let mut blocked = crop_watch_known_good();
    blocked.attitude = Some(Pick::new("magnetorquers", FlightProven));
    let early = (0..300).find(|&s| run(&catalog, &all_basic(), CROP_WATCH, s).timeline.len() < 365).unwrap();
    let cases: Vec<(&str, &str, Design, u64)> = vec![
        ("good crop", CROP_WATCH, crop_watch_known_good(), 1),
        ("good villages", CONNECT_THE_VILLAGES, connect_the_villages_known_good(), 1),
        ("good sentinel", SPACE_WEATHER_SENTINEL, space_weather_sentinel_known_good(), 1),
        ("battery S", CROP_WATCH, battery, 3),
        ("uhf", CROP_WATCH, uhf, 1),
        ("leo crop", CROP_WATCH, leo, 1),
        ("wide camera", CROP_WATCH, wide, 1),
        ("one instrument", SPACE_WEATHER_SENTINEL, one, 1),
        ("s-band sentinel", SPACE_WEATHER_SENTINEL, sband, 1),
        ("villages sso", CONNECT_THE_VILLAGES, villages_sso, 1),
        ("blocked", CROP_WATCH, blocked, 1),
        ("all basic early end", CROP_WATCH, all_basic(), early),
        ("all basic seed 1", CROP_WATCH, all_basic(), 1),
    ];
    for (label, scenario, design, seed) in cases {
        let d = run(&catalog, &design, scenario, seed).debrief;
        eprintln!("== {label} (success {}, score {:.1})\n{}", d.success, d.score.total, texts(&d));
        eprintln!("   match: {:?}\n", d.real_mission_match);
    }
}

#[test]
fn causes_are_capped_to_the_few_that_mattered() {
    let catalog = legacy_catalog();
    for seed in 0..40 {
        let d = run(&catalog, &all_basic(), CROP_WATCH, seed).debrief;
        assert!(d.causes.len() <= catalog.cause_rules.max_causes, "{}", texts(&d));
    }
}
