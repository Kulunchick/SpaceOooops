//! The Score breakdown in the Debrief.
mod common;

use common::{CROP_WATCH, crop_watch_known_good};
use spaceooops::sim::*;

fn debrief(design: &Design, seed: u64) -> Debrief {
    let catalog = load_catalog().unwrap();
    run(&catalog, design, CROP_WATCH, seed).debrief
}

fn line(d: &Debrief, kind: ScoreLineKind) -> Option<&ScoreLine> {
    d.score.lines.iter().find(|l| l.kind == kind)
}

#[test]
fn score_lines_add_up_to_the_score() {
    let mut small_battery = crop_watch_known_good();
    small_battery.battery = Some(Pick::new("battery-s", ReliabilityTier::FlightProven));
    for design in [crop_watch_known_good(), small_battery, Design::default()] {
        for seed in 0..5 {
            let d = debrief(&design, seed);
            let sum: f64 = d.score.lines.iter().map(|l| l.points).sum();
            assert_eq!(d.score.total, sum, "{:#?}", d.score);
        }
    }
}

#[test]
fn a_successful_run_scores_science_and_margin_bonuses_without_penalty() {
    let d = debrief(&crop_watch_known_good(), 1);
    assert!(d.success);
    let science = line(&d, ScoreLineKind::ScienceValue).expect("science line");
    assert!(science.points >= 100.0, "{science:?}");
    // The known-good Design leaves 25% of the Budget and 22% of the rideshare's Mass unused.
    assert!(line(&d, ScoreLineKind::BudgetMargin).unwrap().points > 0.0);
    assert!(line(&d, ScoreLineKind::MassMargin).unwrap().points > 0.0);
    assert!(line(&d, ScoreLineKind::Penalty).is_none());
    assert!(d.score.lines.iter().all(|l| !l.label.is_empty()));
}

#[test]
fn ending_before_the_minimum_lifetime_costs_a_penalty() {
    let mut small_battery = crop_watch_known_good();
    small_battery.battery = Some(Pick::new("battery-s", ReliabilityTier::FlightProven));
    let d = debrief(&small_battery, 1);
    let penalty = line(&d, ScoreLineKind::Penalty).expect("penalty line");
    assert!(penalty.points < 0.0);
    assert!(penalty.label.contains("365"), "{}", penalty.label);
}

#[test]
fn later_ticket_debrief_parts_start_empty() {
    let d = debrief(&crop_watch_known_good(), 1);
    assert!(d.causes.is_empty() && d.objectives.is_empty() && d.decisions.is_empty());
    assert!(d.real_mission_match.is_none());
    assert_eq!(d.stats.min_lifetime_days, 365);
    assert_eq!(d.stats.required_science_value, 100.0);
}
