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

fn all_basic() -> Design {
    let mut d = crop_watch_known_good();
    for pick in [&mut d.solar, &mut d.battery, &mut d.attitude, &mut d.comms] {
        pick.as_mut().unwrap().tier = ReliabilityTier::Basic;
    }
    d.computer_tier = ReliabilityTier::Basic;
    d
}

#[test]
fn ending_before_the_minimum_lifetime_costs_a_penalty() {
    let d = (0..300)
        .map(|seed| debrief(&all_basic(), seed))
        .find(|d| d.stats.days_survived < 365)
        .expect("some all-Basic Run ends early");
    let penalty = line(&d, ScoreLineKind::Penalty).expect("penalty line");
    assert!(penalty.points < 0.0);
    assert!(penalty.label.contains("365"), "{}", penalty.label);
}

#[test]
fn margin_bonuses_are_paid_only_on_success() {
    // Battery S survives the year but browns out every eclipse, so it misses the required Science Value.
    let mut small_battery = crop_watch_known_good();
    small_battery.battery = Some(Pick::new("battery-s", ReliabilityTier::FlightProven));
    let d = debrief(&small_battery, 1);
    assert!(!d.success);
    for kind in [ScoreLineKind::BudgetMargin, ScoreLineKind::MassMargin] {
        let l = line(&d, kind).expect("the margin line is still shown");
        assert_eq!(l.points, 0.0, "{l:?}");
        assert!(l.label.contains("Success"), "{}", l.label);
    }
}

#[test]
fn later_ticket_debrief_parts_start_empty() {
    let d = debrief(&crop_watch_known_good(), 1);
    // Causes and the Real-Mission Match are filled by ticket 04 (tests/debrief.rs), objectives by ticket 06.
    assert!(d.decisions.is_empty());
    assert_eq!(d.stats.min_lifetime_days, 365);
    assert_eq!(d.stats.required_science_value, 100.0);
}
