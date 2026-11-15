//! Personal Objectives: evaluated in the Debrief, paid out through the Score, previewed before launch.
mod common;

use common::*;
use spaceooops::sim::*;

fn catalog() -> Catalog {
    legacy_catalog()
}

/// Same Design with every Component on the cheaper, less reliable Basic tier.
fn basic_tier(mut d: Design) -> Design {
    for p in [&mut d.solar, &mut d.battery, &mut d.attitude, &mut d.comms].into_iter().flatten() {
        p.tier = ReliabilityTier::Basic;
    }
    d.computer_tier = ReliabilityTier::Basic;
    d
}

fn result(d: &Debrief, role: Role) -> &ObjectiveResult {
    d.objectives.iter().find(|o| o.role == role).unwrap_or_else(|| panic!("no objective for {role:?}"))
}

fn flown(design: &Design, scenario: &str, seed: u64) -> Debrief {
    run(&catalog(), design, scenario, seed).debrief
}

#[test]
fn every_scenario_gives_each_role_exactly_one_objective() {
    let c = catalog();
    for scenario in &c.scenarios {
        let list = scenario_objectives(&c, &scenario.id);
        let roles: Vec<Role> = list.iter().map(|o| o.role).collect();
        assert_eq!(roles, Role::ALL.to_vec(), "{}", scenario.id);
        assert!(list.iter().all(|o| !o.description.is_empty() && o.bonus > 0.0));
    }
}

#[test]
fn a_scenario_can_replace_a_roles_default_objective() {
    let c = catalog();
    let science = |id: &str| scenario_objectives(&c, id).into_iter().find(|o| o.role == Role::Science).unwrap();
    assert_ne!(science(CROP_WATCH).description, science(SPACE_WEATHER_SENTINEL).description);
}

#[test]
fn known_good_crop_watch_meets_some_objectives_and_misses_others() {
    let d = flown(&crop_watch_known_good(), CROP_WATCH, 1);
    assert!(d.success);
    assert!(result(&d, Role::Science).met, "{:?}", result(&d, Role::Science));
    assert!(result(&d, Role::Spacecraft).met, "{:?}", result(&d, Role::Spacecraft));
    // Spare capacity is 22% and the spend is 75% of the Budget: both just short of their goals.
    assert!(!result(&d, Role::LaunchOrbit).met);
    assert!(!result(&d, Role::BudgetComms).met);
    assert!(d.objectives.iter().all(|o| !o.detail.is_empty()));
}

#[test]
fn a_cheaper_design_trades_reliability_for_the_budget_objective() {
    let d = flown(&basic_tier(crop_watch_known_good()), CROP_WATCH, 1);
    assert!(d.success);
    assert!(result(&d, Role::BudgetComms).met, "{:?}", result(&d, Role::BudgetComms));
}

#[test]
fn a_big_battery_trades_mass_and_budget_for_battery_health() {
    // The villages design leaves 26% of the capacity unused but its battery sags to 82%.
    let d = flown(&connect_the_villages_known_good(), CONNECT_THE_VILLAGES, 1);
    assert!(d.success);
    assert!(result(&d, Role::LaunchOrbit).met);
    assert!(!result(&d, Role::Spacecraft).met);
    assert!(!result(&d, Role::BudgetComms).met);
}

#[test]
fn sentinel_science_objective_is_about_lost_data() {
    let d = flown(&space_weather_sentinel_known_good(), SPACE_WEATHER_SENTINEL, 1);
    assert!(d.success);
    let science = result(&d, Role::Science);
    assert!(science.description.contains("Data"), "{science:?}");
    assert!(science.met, "{science:?}");

    // A weaker radio cannot get the heavy Data down.
    let mut weak = space_weather_sentinel_known_good();
    weak.comms = Some(Pick::new("comms-s-band", ReliabilityTier::FlightProven));
    let d = flown(&weak, SPACE_WEATHER_SENTINEL, 1);
    assert!(!result(&d, Role::Science).met);
}

#[test]
fn met_objectives_appear_as_score_lines_and_the_score_still_adds_up() {
    for seed in 0..3 {
        for (design, scenario) in [
            (crop_watch_known_good(), CROP_WATCH),
            (connect_the_villages_known_good(), CONNECT_THE_VILLAGES),
            (space_weather_sentinel_known_good(), SPACE_WEATHER_SENTINEL),
            (basic_tier(crop_watch_known_good()), CROP_WATCH),
        ] {
            let d = flown(&design, scenario, seed);
            let lines: Vec<&ScoreLine> =
                d.score.lines.iter().filter(|l| l.kind == ScoreLineKind::PersonalObjective).collect();
            let met: Vec<&ObjectiveResult> = d.objectives.iter().filter(|o| o.met).collect();
            assert_eq!(lines.len(), met.len());
            for o in met {
                assert!(lines.iter().any(|l| l.points == o.bonus && l.label == o.description), "{o:?}");
            }
            let sum: f64 = d.score.lines.iter().map(|l| l.points).sum();
            assert_eq!(d.score.total, sum);
        }
    }
}

#[test]
fn objective_bonuses_require_a_successful_run() {
    // Seed 0 loses the cheap Basic-tier design early: its spend is under 70% of the Budget, yet no bonus.
    let d = flown(&basic_tier(crop_watch_known_good()), CROP_WATCH, 0);
    assert!(!d.success);
    assert!(d.objectives.iter().all(|o| !o.met));
    assert!(d.score.lines.iter().all(|l| l.kind != ScoreLineKind::PersonalObjective));
    // The rule itself held for the Budget objective: the Debrief can tell "held, but not counted" from "missed".
    let budget = d.objectives.iter().find(|o| o.detail.contains("Budget")).unwrap();
    assert!(budget.held, "{budget:#?}");
    assert!(d.objectives.iter().all(|o| o.held == o.detail.contains("only count when the Mission succeeds")));

    let d = flown(&Design::default(), CROP_WATCH, 1);
    assert!(!d.success);
    assert!(d.objectives.iter().all(|o| !o.met));
    assert!(d.score.lines.iter().all(|l| l.kind != ScoreLineKind::PersonalObjective));
}

#[test]
fn outlook_flags_roles_whose_objective_looks_at_risk() {
    let c = catalog();
    let outlook = objective_outlook(&c, &crop_watch_known_good(), CROP_WATCH);
    assert_eq!(outlook.iter().map(|o| o.role).collect::<Vec<_>>(), Role::ALL.to_vec());
    let of = |role| outlook.iter().find(|o| o.role == role).unwrap();
    assert!(!of(Role::Science).at_risk);
    assert!(of(Role::LaunchOrbit).at_risk);
    assert!(of(Role::BudgetComms).at_risk);
    assert!(outlook.iter().all(|o| !o.reason.is_empty() && !o.description.is_empty()));
    assert!(of(Role::BudgetComms).reason.contains("75%"), "{}", of(Role::BudgetComms).reason);
}

#[test]
fn outlook_for_an_unflyable_design_puts_every_role_at_risk_without_panicking() {
    let c = catalog();
    let outlook = objective_outlook(&c, &Design::default(), CROP_WATCH);
    assert_eq!(outlook.len(), 4);
    assert!(outlook.iter().all(|o| o.at_risk && !o.reason.is_empty()));
    // An unknown Scenario does not panic either.
    assert!(objective_outlook(&c, &Design::default(), "no-such-scenario").iter().all(|o| o.at_risk));
}

#[test]
fn a_no_go_on_an_objective_that_looks_safe_reads_as_the_players_own_call() {
    let c = catalog();
    let outlook = objective_outlook(&c, &crop_watch_known_good(), CROP_WATCH);
    assert!(outlook.iter().any(|o| !o.at_risk) && outlook.iter().any(|o| o.at_risk), "{outlook:#?}");
    for o in &outlook {
        if o.at_risk {
            assert_eq!(o.objection, o.reason);
        } else {
            assert!(!o.objection.contains("On track"), "{}", o.objection);
            assert!(o.objection.contains("looks safe") && o.objection.contains("own"), "{}", o.objection);
        }
    }
}
