//! Launch Review through the public `sim` interface.
mod common;

use common::{CROP_WATCH, crop_watch_known_good};
use spaceooops::sim::*;

fn review(design: &Design) -> LaunchReview {
    let catalog = load_catalog().unwrap();
    launch_review(&catalog, design, CROP_WATCH)
}

fn violation(r: &LaunchReview, kind: FindingKind) -> &Finding {
    r.violations
        .iter()
        .find(|f| f.kind == kind)
        .unwrap_or_else(|| panic!("expected a {kind:?} violation, got {:#?}", r.violations))
}

fn warning(r: &LaunchReview, kind: FindingKind) -> &Finding {
    r.warnings
        .iter()
        .find(|f| f.kind == kind)
        .unwrap_or_else(|| panic!("expected a {kind:?} warning, got {:#?}", r.warnings))
}

#[test]
fn known_good_crop_watch_design_passes() {
    let r = review(&crop_watch_known_good());
    assert!(r.violations.is_empty(), "{:#?}", r.violations);
    assert!(r.warnings.is_empty(), "{:#?}", r.warnings);
    assert!(r.can_launch);
    for gauge in [&r.gauges.budget, &r.gauges.mass, &r.gauges.power, &r.gauges.data, &r.gauges.lifetime] {
        assert_eq!(gauge.state, GaugeState::Ok, "{gauge:?}");
    }
}

#[test]
fn known_good_totals_match_the_worked_example() {
    let r = review(&crop_watch_known_good());
    // 4.0 structure + 1.2 panels + 0.45 battery + 1.5 wheels + 0.5 S-band + 0.2 computer + 1.5 imager
    assert!((r.gauges.mass.value - 9.35).abs() < 1e-9, "{:?}", r.gauges.mass);
    assert_eq!(r.gauges.mass.limit, 12.0);
    // spacecraft 60k + 1.6 x (80k + 15k + 120k + 60k + 40k); imager 350k; rideshare 330k; S-band 300/day x 365
    assert!((r.cost.spacecraft_usd - 564_000.0).abs() < 1e-6);
    assert!((r.cost.instruments_usd - 350_000.0).abs() < 1e-6);
    assert!((r.cost.launch_usd - 330_000.0).abs() < 1e-6);
    assert!((r.cost.operations_usd - 109_500.0).abs() < 1e-6);
    assert!((r.cost.total_usd - 1_353_500.0).abs() < 1e-6);
    assert_eq!(r.gauges.budget.limit, 1_800_000.0);
    // 2.5 wheels + 2.0 S-band + 1.0 computer + 3.0 imager; 6 W/U x 6U x 64% sunlit
    assert!((r.gauges.power.value - 8.5).abs() < 1e-9);
    assert!((r.gauges.power.limit - 23.04).abs() < 1e-9);
    // 2 Mbps x 40 min/day
    assert!((r.gauges.data.limit - 600.0).abs() < 1e-9);
    assert_eq!(r.gauges.data.value, 300.0);
}

// ---------------------------------------------------------------- hard constraints

#[test]
fn too_heavy_for_the_launch_vehicle_blocks_launch() {
    let mut d = crop_watch_known_good();
    d.instruments.push("thermal-imager".into());
    let r = review(&d);
    let f = violation(&r, FindingKind::OverMass);
    assert!(f.reason.contains("12.4 kg") && f.reason.contains("12.0 kg"), "{}", f.reason);
    assert!(f.reason.contains("CubeSat rideshare"), "{}", f.reason);
    assert_eq!(f.role, Role::LaunchOrbit);
    assert_eq!(r.violations.len(), 1, "{:#?}", r.violations);
    assert_eq!(r.gauges.mass.state, GaugeState::Violated);
    assert!(!r.can_launch);
}

#[test]
fn cost_over_budget_blocks_launch() {
    let mut d = crop_watch_known_good();
    d.launch_vehicle = Some("small-launcher".into());
    let r = review(&d);
    let f = violation(&r, FindingKind::OverBudget);
    assert!(f.reason.contains("Budget") && f.reason.contains("$1.80M"), "{}", f.reason);
    assert_eq!(f.role, Role::BudgetComms);
    assert_eq!(r.gauges.budget.state, GaugeState::Violated);
    assert!(!r.can_launch);
}

#[test]
fn negative_average_power_blocks_launch() {
    let mut d = crop_watch_known_good();
    d.solar = Some(Pick::new("solar-body", ReliabilityTier::FlightProven));
    let r = review(&d);
    let f = violation(&r, FindingKind::NegativePower);
    // 2 W/U x 6U x 64% sunlit = 7.7 W made, 8.5 W used
    assert!(f.reason.contains("8.5 W") && f.reason.contains("7.7 W"), "{}", f.reason);
    assert!(f.reason.contains("shadow"), "{}", f.reason);
    assert_eq!(r.gauges.power.state, GaugeState::Violated);
    assert!(!r.can_launch);
}

#[test]
fn pointing_instrument_without_reaction_wheels_blocks_launch() {
    let mut d = crop_watch_known_good();
    d.attitude = Some(Pick::new("magnetorquers", ReliabilityTier::FlightProven));
    let r = review(&d);
    let f = violation(&r, FindingKind::NeedsReactionWheels);
    assert!(f.reason.contains("Multispectral imager") && f.reason.contains("Reaction wheels"), "{}", f.reason);
    assert!(!r.can_launch);
}

#[test]
fn structure_volume_exceeded_blocks_launch() {
    let mut d = crop_watch_known_good();
    d.structure = Some("cubesat-3u".into());
    let r = review(&d);
    let f = violation(&r, FindingKind::VolumeExceeded);
    // 0.1 panels + 0.3 battery + 0.8 wheels + 0.4 S-band + 0.2 computer + 2.0 imager = 3.8U in 2.4U
    assert!(f.reason.contains("3.8U") && f.reason.contains("2.4U"), "{}", f.reason);
    assert!(f.reason.contains("CubeSat 3U"), "{}", f.reason);
    assert!(!r.can_launch);
}

#[test]
fn no_comms_blocks_launch() {
    let mut d = crop_watch_known_good();
    d.comms = None;
    let r = review(&d);
    let f = violation(&r, FindingKind::NoComms);
    assert!(f.reason.contains("no comms"), "{}", f.reason);
    assert!(!r.can_launch);
}

#[test]
fn empty_design_is_incomplete_not_a_panic() {
    let r = review(&Design::default());
    assert!(!r.can_launch);
    assert!(r.violations.iter().filter(|f| f.kind == FindingKind::Incomplete).count() >= 5);
    for g in [&r.gauges.budget, &r.gauges.mass, &r.gauges.power, &r.gauges.data, &r.gauges.lifetime] {
        assert!(g.value.is_finite() && g.limit.is_finite(), "{g:?}");
    }
}

#[test]
fn unknown_ids_and_scenario_do_not_panic() {
    let catalog = load_catalog().unwrap();
    let mut d = crop_watch_known_good();
    d.instruments.push("no-such-instrument".into());
    d.orbit = Some("no-such-orbit".into());
    let r = launch_review(&catalog, &d, "no-such-scenario");
    assert!(!r.can_launch);
    violation(&r, FindingKind::Incomplete);
}

// ---------------------------------------------------------------- soft warnings

#[test]
fn data_above_downlink_warns_without_blocking() {
    let mut d = crop_watch_known_good();
    d.comms = Some(Pick::new("comms-uhf", ReliabilityTier::FlightProven));
    let r = review(&d);
    let f = warning(&r, FindingKind::DataAboveDownlink);
    // 9.6 kbps x 40 min = 2.9 MB per day
    assert!(f.reason.contains("300 MB") && f.reason.contains("2.9 MB"), "{}", f.reason);
    assert_eq!(r.gauges.data.state, GaugeState::Violated);
    assert!(r.can_launch, "{:#?}", r.violations);
}

#[test]
fn battery_that_only_just_covers_the_eclipse_warns() {
    let mut d = crop_watch_known_good();
    d.battery = Some(Pick::new("battery-s", ReliabilityTier::FlightProven));
    let r = review(&d);
    let f = warning(&r, FindingKind::BatteryBarelyCoversEclipse);
    // 8.5 W x 35 min = 4.9 Wh of 20 Wh
    assert!(f.reason.contains("25%") && f.reason.contains("35 min"), "{}", f.reason);
    assert!(r.can_launch);
}

#[test]
fn basic_tier_on_a_critical_component_warns() {
    let mut d = crop_watch_known_good();
    d.battery = Some(Pick::new("battery-m", ReliabilityTier::Basic));
    let r = review(&d);
    let f = warning(&r, FindingKind::BasicTierOnCritical);
    assert!(f.reason.contains("Battery M"), "{}", f.reason);
    assert!(r.can_launch);
}

#[test]
fn thin_mass_margin_warns_and_turns_the_gauge_amber() {
    let mut d = crop_watch_known_good();
    d.instruments = vec!["thermal-imager".into()];
    let r = review(&d);
    let f = warning(&r, FindingKind::ThinMargin);
    assert_eq!(f.role, Role::LaunchOrbit);
    assert_eq!(r.gauges.mass.state, GaugeState::Thin);
    assert!(r.can_launch);
}

#[test]
fn all_basic_tier_shortens_lifetime_to_a_thin_margin() {
    let mut d = crop_watch_known_good();
    for pick in [&mut d.solar, &mut d.battery, &mut d.attitude, &mut d.comms] {
        pick.as_mut().unwrap().tier = ReliabilityTier::Basic;
    }
    d.computer_tier = ReliabilityTier::Basic;
    let r = review(&d);
    assert_eq!(r.gauges.lifetime.state, GaugeState::Thin, "{:?}", r.gauges.lifetime);
    assert!(r.warnings.iter().filter(|f| f.kind == FindingKind::BasicTierOnCritical).count() >= 4);
    assert!(r.can_launch);
}

#[test]
fn only_critical_components_count_towards_the_expected_lifetime() {
    // Attitude control is not critical: a Basic one degrades pointing but never ends the Mission.
    let good = review(&crop_watch_known_good()).gauges.lifetime.value;
    let mut d = crop_watch_known_good();
    d.attitude = Some(Pick::new("reaction-wheels", ReliabilityTier::Basic));
    assert_eq!(review(&d).gauges.lifetime.value, good);
}

// ---------------------------------------------------------------- incompatible options

#[test]
fn magnetorquers_mark_pointing_instruments_incompatible() {
    let mut d = crop_watch_known_good();
    d.instruments = vec!["wide-camera".into()];
    d.attitude = Some(Pick::new("magnetorquers", ReliabilityTier::FlightProven));
    let r = review(&d);
    let clash = r.incompatible.iter().find(|i| i.option_id == "multispectral-imager").expect("imager flagged");
    assert!(clash.reason.contains("Magnetorquers"), "{}", clash.reason);
    assert!(!r.incompatible.iter().any(|i| i.option_id == "wide-camera"));
}

#[test]
fn a_pointing_instrument_marks_magnetorquers_incompatible() {
    let r = review(&crop_watch_known_good());
    assert!(r.incompatible.iter().any(|i| i.option_id == "magnetorquers"));
}

#[test]
fn obvious_cheap_and_max_designs_are_not_clean() {
    use ReliabilityTier::*;
    let cheap = Design {
        instruments: vec!["wide-camera".into()],
        structure: Some("cubesat-3u".into()),
        solar: Some(Pick::new("solar-body", Basic)),
        battery: Some(Pick::new("battery-s", Basic)),
        attitude: Some(Pick::new("magnetorquers", Basic)),
        comms: Some(Pick::new("comms-uhf", Basic)),
        computer_tier: Basic,
        orbit: Some("sso-600".into()),
        launch_vehicle: Some("cubesat-rideshare".into()),
    };
    let r = review(&cheap);
    warning(&r, FindingKind::DataAboveDownlink);

    let mut max = crop_watch_known_good();
    max.instruments = vec!["multispectral-imager".into(), "thermal-imager".into(), "wide-camera".into()];
    max.battery = Some(Pick::new("battery-l", FlightProven));
    let r = review(&max);
    assert!(!r.can_launch);
}
