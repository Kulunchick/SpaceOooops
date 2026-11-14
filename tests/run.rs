//! The Run through the public `sim` interface: timeline, Random Events, Success and determinism.
mod common;

use common::{CROP_WATCH, crop_watch_known_good};
use spaceooops::sim::*;

fn fly(design: &Design, seed: u64) -> RunResult {
    let catalog = load_catalog().unwrap();
    run(&catalog, design, CROP_WATCH, seed)
}

#[test]
fn known_good_crop_watch_design_reaches_success() {
    let result = fly(&crop_watch_known_good(), 1);
    let d = &result.debrief;
    assert!(d.success, "{:#?}", d.stats);
    assert!(d.stats.science_value >= d.stats.required_science_value);
    assert_eq!(d.stats.days_survived, 365);
    assert_eq!(result.timeline.len(), 365);
    assert_eq!(result.timeline[0].day, 1);
    assert_eq!(result.timeline.last().unwrap().day, 365);
}

#[test]
fn same_design_scenario_and_seed_give_an_identical_run() {
    let a = fly(&crop_watch_known_good(), 42);
    let b = fly(&crop_watch_known_good(), 42);
    assert_eq!(a, b);
}

fn event_days(result: &RunResult) -> Vec<(u32, EventKind)> {
    result.timeline.iter().flat_map(|d| d.events.iter().map(move |e| (d.day, e.kind))).collect()
}

#[test]
fn different_seeds_can_produce_different_random_events() {
    let first = event_days(&fly(&crop_watch_known_good(), 0));
    assert!((1..20).any(|seed| event_days(&fly(&crop_watch_known_good(), seed)) != first));
}

#[test]
fn random_events_explain_themselves() {
    let mut kinds = Vec::new();
    for seed in 0..30 {
        for day in fly(&crop_watch_known_good(), seed).timeline {
            for e in day.events {
                assert!(e.text.len() > 20, "{e:?}");
                kinds.push(e.kind);
            }
        }
    }
    assert!(kinds.contains(&EventKind::SolarFlare));
    assert!(kinds.contains(&EventKind::GroundStationOutage));
}

// ---------------------------------------------------------------- balance over a fixed seed range

const SEEDS: std::ops::Range<u64> = 0..300;

fn success_rate(design: &Design) -> f64 {
    let catalog = load_catalog().unwrap();
    let wins = SEEDS.filter(|&seed| run(&catalog, design, CROP_WATCH, seed).debrief.success).count();
    let rate = wins as f64 / SEEDS.count() as f64;
    eprintln!("success rate {rate:.3} for {design:?}");
    rate
}

fn all_basic() -> Design {
    let mut d = crop_watch_known_good();
    for pick in [&mut d.solar, &mut d.battery, &mut d.attitude, &mut d.comms] {
        pick.as_mut().unwrap().tier = ReliabilityTier::Basic;
    }
    d.computer_tier = ReliabilityTier::Basic;
    d
}

fn small_battery() -> Design {
    let mut d = crop_watch_known_good();
    d.battery = Some(Pick::new("battery-s", ReliabilityTier::FlightProven));
    d
}

fn uhf_only() -> Design {
    let mut d = crop_watch_known_good();
    d.comms = Some(Pick::new("comms-uhf", ReliabilityTier::FlightProven));
    d
}

#[test]
fn known_good_design_succeeds_on_the_large_majority_of_seeds() {
    let rate = success_rate(&crop_watch_known_good());
    assert!(rate >= 0.85, "known-good success rate {rate}");
}

#[test]
fn a_battery_too_small_for_its_eclipses_never_reaches_success() {
    assert!(success_rate(&small_battery()) <= 0.01);
}

#[test]
fn a_battery_that_cannot_cover_the_eclipse_costs_operating_time_not_the_mission() {
    let result = fly(&small_battery(), 3);
    // Wear alone never ends the Mission: the Run reaches its planned end unless a Component fails at random.
    assert!(SEEDS.take(20).any(|seed| fly(&small_battery(), seed).debrief.stats.days_survived == 365));
    let shortfall = result.timeline.iter().find(|d| d.events.iter().any(|e| e.kind == EventKind::BatteryShortfall));
    let day = shortfall.expect("the Run says when the battery first fails to cover the eclipse");
    let e = day.events.iter().find(|e| e.kind == EventKind::BatteryShortfall).unwrap();
    assert!(e.text.contains("Battery S"), "{}", e.text);
    assert!(!e.text.contains("Mission ends"), "{}", e.text);
    assert_eq!(day.battery_state, GaugeState::Violated);
    assert!(day.operating_fraction < 1.0 && day.operating_fraction > 0.0, "{day:?}");
    let shortfalls = result.timeline.iter().flat_map(|d| &d.events).filter(|e| e.kind == EventKind::BatteryShortfall);
    assert_eq!(shortfalls.count(), 1, "reported once, when it starts");
}

#[test]
fn battery_charge_falls_as_a_small_battery_wears() {
    let timeline = fly(&small_battery(), 3).timeline;
    assert!(timeline.last().unwrap().battery_charge < timeline[0].battery_charge);
    assert_eq!(timeline[0].battery_state, GaugeState::Thin, "a deep but covering discharge is flagged");
}

#[test]
fn a_well_sized_battery_reports_a_healthy_state_every_day() {
    let timeline = fly(&crop_watch_known_good(), 1).timeline;
    assert!(timeline.iter().all(|d| d.battery_state == GaugeState::Ok));
}

#[test]
fn a_uhf_only_design_loses_its_data_and_never_reaches_success() {
    assert_eq!(success_rate(&uhf_only()), 0.0);
    let d = fly(&uhf_only(), 1).debrief;
    assert!(d.stats.lost_data_fraction > 0.9, "{:?}", d.stats);
}

#[test]
fn an_all_basic_design_does_visibly_worse_than_flight_proven() {
    let good = success_rate(&crop_watch_known_good());
    let basic = success_rate(&all_basic());
    assert!(basic < good - 0.2, "basic {basic} vs flight-proven {good}");
}

#[test]
fn flight_proven_components_fail_less_often_than_basic() {
    let catalog = load_catalog().unwrap();
    let failures = |design: &Design| -> usize {
        SEEDS
            .map(|seed| {
                run(&catalog, design, CROP_WATCH, seed)
                    .timeline
                    .iter()
                    .flat_map(|d| &d.events)
                    .filter(|e| e.kind == EventKind::ComponentFailure)
                    .count()
            })
            .sum()
    };
    let flight_proven = failures(&crop_watch_known_good());
    let basic = failures(&all_basic());
    assert!(flight_proven * 3 < basic, "flight-proven {flight_proven} vs basic {basic} failures");
}

#[test]
fn a_fatal_failure_ends_the_run_early() {
    let catalog = load_catalog().unwrap();
    let ended_early = SEEDS
        .map(|seed| run(&catalog, &all_basic(), CROP_WATCH, seed))
        .find(|r| r.timeline.len() < 365)
        .expect("some all-Basic Run ends early");
    let last = ended_early.timeline.last().unwrap();
    assert!(last.events.iter().any(|e| e.kind == EventKind::ComponentFailure && e.text.contains("Mission ends")));
    assert!(!last.failed_components.is_empty());
    assert!(!ended_early.debrief.success);
}

#[test]
fn an_empty_design_or_unknown_scenario_flies_badly_without_panicking() {
    let catalog = load_catalog().unwrap();
    assert!(!run(&catalog, &Design::default(), CROP_WATCH, 7).debrief.success);
    let r = run(&catalog, &crop_watch_known_good(), "no-such-scenario", 7);
    assert!(!r.debrief.success);
    assert!(r.timeline.is_empty());
}

fn unlaunchable() -> Design {
    // Magnetorquers cannot point the multispectral imager, and the small launcher blows the Budget.
    let mut d = crop_watch_known_good();
    d.attitude = Some(Pick::new("magnetorquers", ReliabilityTier::FlightProven));
    d.launch_vehicle = Some("small-launcher".to_string());
    d
}

#[test]
fn a_design_that_fails_launch_review_never_flies_scores_or_succeeds() {
    let catalog = load_catalog().unwrap();
    for design in [unlaunchable(), Design::default()] {
        assert!(!launch_review(&catalog, &design, CROP_WATCH).can_launch);
        for seed in 0..20 {
            let r = run(&catalog, &design, CROP_WATCH, seed);
            assert!(r.timeline.is_empty());
            let d = &r.debrief;
            assert!(!d.success);
            assert_eq!(d.score.total, 0.0, "{:#?}", d.score);
            assert_eq!(d.score.total, d.score.lines.iter().map(|l| l.points).sum::<f64>());
            assert_eq!(d.stats.science_value, 0.0);
            assert_eq!(d.stats.days_survived, 0);
            assert_eq!(d.stats.delivered_data_mb, 0.0);
            assert_eq!(d.stats.required_science_value, 100.0);
            assert_eq!(d.stats.min_lifetime_days, 365);
            assert!(!d.ending.is_empty());
        }
    }
    let r = run(&catalog, &unlaunchable(), CROP_WATCH, 1);
    assert!(r.debrief.stats.total_cost_usd > 0.0 && r.debrief.stats.total_mass_kg > 0.0);
}

#[test]
fn the_debrief_says_how_the_run_ended() {
    let done = fly(&crop_watch_known_good(), 1).debrief;
    assert!(done.ending.to_lowercase().contains("planned mission complete"), "{}", done.ending);
    assert!(done.stats.science_met && done.stats.lifetime_met);
    let catalog = load_catalog().unwrap();
    let early = SEEDS
        .map(|seed| run(&catalog, &all_basic(), CROP_WATCH, seed))
        .find(|r| r.timeline.len() < 365)
        .unwrap()
        .debrief;
    assert!(!early.ending.to_lowercase().contains("planned mission complete"), "{}", early.ending);
    assert!(!early.stats.lifetime_met);
}
