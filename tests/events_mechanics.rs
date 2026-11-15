//! What each mechanic of the new Random Events does to a Run, seen in the day snapshots (More Random Events,
//! tickets 01 and 02). Each test flies a catalog with one event against the same Run without it: the days the
//! event holds move as it states, every other day is as before.
mod common;

use common::*;
use spaceooops::sim::*;

/// A catalog whose only new event has these `effects`, and every Run gets it.
fn one_event(effects: &str) -> Catalog {
    catalog_with_events(
        &format!(
            r#"
[[events]]
id = "test-event"
group = "lucky"
title = "A test event"
log = "Something happened."
scene = "hit"
effects = [{effects}]
"#
        ),
        1,
        1,
    )
}

/// The day of the Run's one new event.
fn event_day(r: &RunResult) -> u32 {
    let days: Vec<u32> =
        r.timeline.iter().filter(|t| t.events.iter().any(|e| e.kind == EventKind::Catalog)).map(|t| t.day).collect();
    assert_eq!(days.len(), 1, "one new event: {days:?}");
    days[0]
}

/// Data delivered on `day` alone (1-based).
fn delivered_on(r: &RunResult, day: u32) -> f64 {
    let at = |d: u32| if d == 0 { 0.0 } else { r.timeline[d as usize - 1].delivered_data_mb };
    at(day) - at(day - 1)
}

/// A Crop Watch Run with the event and the same Run without it, for a seed whose days around the event
/// (from the day before to `quiet` days after) have no built-in Random Event and deliver Data every day.
fn flown(c: &Catalog, quiet: u32) -> (RunResult, RunResult, u32) {
    let d = crop_watch_known_good();
    let off = legacy_catalog();
    for seed in 0..500 {
        let with = run(c, &d, CROP_WATCH, seed);
        let without = run(&off, &d, CROP_WATCH, seed);
        let day = event_day(&with);
        let window = day.saturating_sub(1).max(1)..=(day + quiet).min(without.timeline.len() as u32);
        let calm = window.clone().all(|t| {
            let s = &without.timeline[t as usize - 1];
            s.events.is_empty() && s.operating_fraction > 0.0 && delivered_on(&without, t) > 0.0
        });
        if calm && day + quiet < without.timeline.len() as u32 && day > 1 {
            return (with, without, day);
        }
    }
    panic!("no calm seed");
}

/// Asserts that every day in `held` satisfies `moved` against the Run without the event, and that every other
/// day before the end of `held` is exactly as before.
fn holds_on(with: &RunResult, without: &RunResult, held: std::ops::RangeInclusive<u32>, field: fn(&DaySnapshot) -> f64,
    moved: fn(f64, f64) -> bool)
{
    for (a, b) in with.timeline.iter().zip(&without.timeline) {
        let (on, off) = (field(a), field(b));
        if held.contains(&a.day) {
            assert!(moved(on, off), "day {}: {on} against {off} without the event", a.day);
        } else {
            assert_eq!(on, off, "day {} is as before", a.day);
        }
    }
}

fn operating(s: &DaySnapshot) -> f64 {
    s.operating_fraction
}

#[test]
fn instruments_off_stops_the_instruments_for_its_days() {
    let (with, without, day) = flown(&one_event(r#"{ mechanic = "instruments-off", days = 2 }"#), 3);
    holds_on(&with, &without, day..=day + 1, operating, |on, _| on == 0.0);
    assert!(with.debrief.stats.science_value < without.debrief.stats.science_value);
}

#[test]
fn a_power_cut_for_n_days_lowers_the_operating_time_then_returns_to_normal() {
    let (with, without, day) = flown(&one_event(r#"{ mechanic = "power", strength = -0.9, days = 3 }"#), 4);
    holds_on(&with, &without, day..=day + 2, operating, |on, off| on < off);
    holds_on(&with, &without, day..=day + 2, |s| s.battery_charge, |on, off| on < off);
}

#[test]
fn a_power_change_to_the_end_lasts_to_the_end_of_the_run() {
    let (with, without, day) = flown(&one_event(r#"{ mechanic = "power", strength = -0.9, to_end = true }"#), 1);
    let last = with.timeline.len() as u32;
    // Over the rest of the Run a flare's safe mode may switch both Runs off on a day.
    holds_on(&with, &without, day..=last, operating, |on, off| on < off || on == off && off == 0.0);
}

#[test]
fn a_delayed_effect_starts_days_after_the_event() {
    let (with, without, day) = flown(&one_event(r#"{ mechanic = "instruments-off", days = 1, delay = 2 }"#), 3);
    holds_on(&with, &without, day + 2..=day + 2, operating, |on, _| on == 0.0);
}

#[test]
fn downlink_off_brings_no_data_down_that_day_and_then_resumes() {
    let (with, without, day) = flown(&one_event(r#"{ mechanic = "downlink", strength = -1.0, days = 1 }"#), 2);
    assert_eq!(delivered_on(&with, day), 0.0);
    assert!(delivered_on(&without, day) > 0.0);
    assert!(delivered_on(&with, day + 1) > 0.0, "contact returns the next day");
    for t in 1..day {
        assert_eq!(with.timeline[t as usize - 1], without.timeline[t as usize - 1], "day {t} is as before");
    }
}

#[test]
fn a_lower_downlink_share_brings_less_data_down() {
    let (with, without, day) = flown(&one_event(r#"{ mechanic = "downlink", strength = -0.99, days = 2 }"#), 3);
    for t in day..=day + 1 {
        assert!(delivered_on(&with, t) < delivered_on(&without, t), "day {t}");
    }
    for t in 1..day {
        assert_eq!(with.timeline[t as usize - 1], without.timeline[t as usize - 1], "day {t} is as before");
    }
}

#[test]
fn the_event_log_shows_the_event_text_on_its_day() {
    let (with, _, day) = flown(&one_event(r#"{ mechanic = "instruments-off", days = 1 }"#), 1);
    let e = with.timeline[day as usize - 1].events.iter().find(|e| e.kind == EventKind::Catalog).unwrap();
    assert_eq!(e.text, "Something happened.");
    let catalog = e.catalog.as_ref().unwrap();
    assert_eq!((catalog.id.as_str(), catalog.group, catalog.scene), ("test-event", EventGroup::Lucky, SceneEffect::Hit));
}

#[test]
fn playback_slows_on_a_day_with_a_new_event() {
    use spaceooops::scene::effects::advance;
    let (with, without, day) = flown(&one_event(r#"{ mechanic = "instruments-off", days = 1 }"#), 1);
    let start = f64::from(day) - 1.0;
    // A long step stops at the start of the event day, then crosses it slowly.
    assert_eq!(advance(&with, start - 1.0, 5.0, 0.25), start);
    assert_eq!(advance(&with, start, 5.0, 0.25), start + 0.25);
    // Without the event the same day is crossed at full speed.
    assert!(advance(&without, start, 5.0, 0.25) > start + 1.0);
}

// ---- ticket 02: the remaining mechanics

/// Data made so far (stored + delivered + lost).
fn made(s: &DaySnapshot) -> f64 {
    s.stored_data_mb + s.delivered_data_mb + s.lost_data_mb
}

fn made_on(r: &RunResult, day: u32) -> f64 {
    let at = |d: u32| if d == 0 { 0.0 } else { made(&r.timeline[d as usize - 1]) };
    at(day) - at(day - 1)
}

#[test]
fn data_lost_for_a_day_makes_no_data_that_day_and_nothing_changes_before() {
    let (with, without, day) = flown(&one_event(r#"{ mechanic = "data", strength = -1.0, days = 1 }"#), 2);
    assert_eq!(made_on(&with, day), 0.0);
    assert!(made_on(&without, day) > 0.0);
    assert!(made_on(&with, day + 1) > 0.0, "Data is made again the next day");
    for t in 1..day {
        assert_eq!(with.timeline[t as usize - 1], without.timeline[t as usize - 1], "day {t} is as before");
    }
}

#[test]
fn an_extra_day_of_data_doubles_that_days_data() {
    let (with, without, day) = flown(&one_event(r#"{ mechanic = "extra-data-day" }"#), 2);
    let (a, b) = (made_on(&with, day), made_on(&without, day));
    assert!((a - 2.0 * b).abs() < 1e-6 * b.max(1.0), "{a} against {b}");
    assert!((made_on(&with, day + 1) - made_on(&without, day + 1)).abs() < 1e-9, "only that day");
}

#[test]
fn less_room_for_data_loses_data_while_it_lasts() {
    // Data is only held overnight when the downlink cannot take it all, so the link is cut too.
    let (with, without, day) = flown(
        &one_event(r#"{ mechanic = "storage", strength = -1.0, days = 2 }, { mechanic = "downlink", strength = -1.0, days = 2 }"#),
        3,
    );
    assert!(with.timeline[day as usize + 1].lost_data_mb > without.timeline[day as usize + 1].lost_data_mb);
    for t in 1..day {
        assert_eq!(with.timeline[t as usize - 1], without.timeline[t as usize - 1], "day {t} is as before");
    }
}

#[test]
fn worse_pointing_makes_less_data_from_pointing_instruments() {
    let (with, without, day) = flown(&one_event(r#"{ mechanic = "pointing", strength = -1.0, days = 2 }"#), 3);
    assert!(made_on(&with, day) < made_on(&without, day), "day {day}");
    assert!(made_on(&with, day + 1) < made_on(&without, day + 1));
    assert_eq!(made_on(&with, day + 2), made_on(&without, day + 2), "back to normal");
}

#[test]
fn a_smaller_battery_drains_deeper_for_its_days_and_never_raises_the_one_time_shortfall() {
    let (with, without, day) =
        flown(&one_event(r#"{ mechanic = "battery-capacity", strength = -0.5, days = 3 }"#), 4);
    holds_on(&with, &without, day..=day + 2, |s| s.battery_charge, |on, off| on < off);
    assert!(!with.timeline.iter().flat_map(|t| &t.events).any(|e| e.kind == EventKind::BatteryShortfall));
}

#[test]
fn a_lower_failure_chance_changes_only_the_threshold_not_the_random_draws() {
    let c = one_event(r#"{ mechanic = "failure-chance", strength = -1.0, to_end = true }"#);
    let d = crop_watch_known_good();
    let off = legacy_catalog();
    let days_of = |r: &RunResult, kind: fn(EventKind) -> bool, from: u32, to: u32| -> Vec<u32> {
        r.timeline
            .iter()
            .filter(|t| t.day >= from && t.day <= to && t.events.iter().any(|e| kind(e.kind)))
            .map(|t| t.day)
            .collect()
    };
    for seed in 0..40 {
        let with = run(&c, &d, CROP_WATCH, seed);
        let without = run(&off, &d, CROP_WATCH, seed);
        let day = event_day(&with);
        let last = with.timeline.len().min(without.timeline.len()) as u32;
        let flare = |k: EventKind| k == EventKind::SolarFlare;
        let outage = |k: EventKind| k == EventKind::GroundStationOutage;
        let fail = |k: EventKind| k == EventKind::ComponentFailure;
        assert_eq!(days_of(&with, flare, 1, last), days_of(&without, flare, 1, last), "seed {seed}");
        assert_eq!(days_of(&with, outage, 1, last), days_of(&without, outage, 1, last), "seed {seed}");
        assert_eq!(days_of(&with, fail, 1, day - 1), days_of(&without, fail, 1, day - 1), "seed {seed}");
        assert!(days_of(&with, fail, day, u32::MAX).is_empty(), "seed {seed}");
    }
}

#[test]
fn money_spent_goes_through_the_extra_cost_path() {
    let (with, without, _) = flown(&one_event(r#"{ mechanic = "budget", strength = -20000.0 }"#), 1);
    let spent = with.debrief.stats.total_cost_usd - without.debrief.stats.total_cost_usd;
    assert!((spent - 20000.0).abs() < 1e-6, "{spent}");
}

#[test]
fn money_gained_raises_the_budget_left_and_shows_in_the_score() {
    let (with, without, _) = flown(&one_event(r#"{ mechanic = "budget", strength = 20000.0 }"#), 1);
    assert_eq!(with.debrief.stats.total_cost_usd, without.debrief.stats.total_cost_usd, "gained money is not a cost");
    let margin = |r: &RunResult| {
        r.debrief.score.lines.iter().find(|l| l.kind == ScoreLineKind::BudgetMargin).unwrap().points
    };
    assert!(margin(&with) > margin(&without), "{} against {}", margin(&with), margin(&without));
}

#[test]
fn a_science_bonus_adds_its_share_of_the_required_science_value() {
    let (with, without, day) = flown(&one_event(r#"{ mechanic = "science-bonus", strength = 0.03 }"#), 1);
    let extra = with.timeline[day as usize - 1].science_value - without.timeline[day as usize - 1].science_value;
    let want = 0.03 * with.debrief.stats.required_science_value;
    assert!((extra - want).abs() < 1e-6, "{extra} against {want}");
}

#[test]
fn an_event_with_two_effects_applies_both() {
    let (with, without, day) = flown(
        &one_event(
            r#"{ mechanic = "instruments-off", days = 1 }, { mechanic = "data", strength = 1.0, to_end = true }"#,
        ),
        2,
    );
    assert_eq!(with.timeline[day as usize - 1].operating_fraction, 0.0);
    assert!(without.timeline[day as usize - 1].operating_fraction > 0.0);
    assert!(made_on(&with, day + 1) > made_on(&without, day + 1) * 1.5, "the Data gain lasts");
}

#[test]
fn no_new_mechanic_fails_a_component_or_ends_the_run() {
    let d = crop_watch_known_good();
    let off = legacy_catalog();
    for effects in [
        r#"{ mechanic = "data", strength = -1.0, days = 1 }"#,
        r#"{ mechanic = "storage", strength = -1.0, days = 2 }"#,
        r#"{ mechanic = "pointing", strength = -1.0, days = 2 }"#,
        r#"{ mechanic = "battery-capacity", strength = -0.9, days = 3 }"#,
        r#"{ mechanic = "budget", strength = -1000000.0 }"#,
        r#"{ mechanic = "science-bonus", strength = 0.03 }"#,
        r#"{ mechanic = "extra-data-day" }"#,
    ] {
        let c = one_event(effects);
        for seed in 0..10 {
            let with = run(&c, &d, CROP_WATCH, seed);
            let without = run(&off, &d, CROP_WATCH, seed);
            assert_eq!(
                with.timeline.last().unwrap().failed_components,
                without.timeline.last().unwrap().failed_components,
                "{effects} seed {seed}"
            );
            assert_eq!(with.timeline.len(), without.timeline.len(), "{effects} seed {seed}");
        }
    }
}

// ---- every shipped event changes the Run

/// Every way a shipped event can play out, as a passive event of its own: its effects (or those of an option, or of
/// a gamble's bad outcome), with an option's price as money spent. The mitigation is left out, so this is the event
/// on a Design without the choice that softens it.
fn outcomes(e: &EventSpec) -> Vec<(String, EventSpec)> {
    let passive = |name: String, mut effects: Vec<EffectSpec>, cost: f64| {
        if cost > 0.0 {
            effects.push(EffectSpec {
                mechanic: Mechanic::Budget,
                strength: -cost,
                days: None,
                to_end: false,
                delay: 0,
                when: None,
                label: None,
            });
        }
        let spec = EventSpec { log: "x".to_string(), effects, question: None, mitigation: None, ..e.clone() };
        (name, spec)
    };
    let mut out = Vec::new();
    match &e.question {
        None => out.push(passive(e.id.clone(), e.effects.clone(), 0.0)),
        Some(q) => {
            for o in &q.options {
                out.push(passive(format!("{} / {}", e.id, o.id), o.effects.clone(), o.cost_usd));
                if let Some(bad) = &o.bad {
                    out.push(passive(format!("{} / {} (bad)", e.id, o.id), bad.effects.clone(), o.cost_usd));
                }
            }
        }
    }
    out.retain(|(_, spec)| !spec.effects.is_empty());
    out
}

fn crop_watch_with_basic_parts() -> Design {
    let basic = |p: Option<Pick>| p.map(|p| Pick::new(&p.id, ReliabilityTier::Basic));
    let d = crop_watch_known_good();
    Design { solar: basic(d.solar.clone()), battery: basic(d.battery.clone()), attitude: basic(d.attitude.clone()), comms: basic(d.comms.clone()), ..d }
}

/// What a player can see of a Run, without the event log itself.
fn seen(r: &RunResult) -> (Vec<DaySnapshot>, f64, f64, f64) {
    let days = r.timeline.iter().map(|t| DaySnapshot { events: Vec::new(), ..t.clone() }).collect();
    let s = &r.debrief.stats;
    (days, s.science_value, s.total_cost_usd, r.debrief.score.total)
}

#[test]
fn every_shipped_event_flown_alone_changes_the_run_on_a_design_it_can_meet() {
    let shipped = load_catalog().unwrap();
    let uhf = Design {
        comms: Some(Pick::new("comms-uhf", ReliabilityTier::FlightProven)),
        ..space_weather_sentinel_known_good()
    };
    let designs = [
        (CROP_WATCH, crop_watch_known_good()),
        (SPACE_WEATHER_SENTINEL, space_weather_sentinel_known_good()),
        (CONNECT_THE_VILLAGES, connect_the_villages_known_good()),
        (SPACE_WEATHER_SENTINEL, uhf),
    ];
    // Basic parts fail now and then, so a lower failure chance has failures to prevent.
    let basic = [(CROP_WATCH, Design { computer_tier: ReliabilityTier::Basic, ..crop_watch_with_basic_parts() })];
    let off = legacy_catalog();
    let mut failures = Vec::new();
    for e in &shipped.events {
        let ways = outcomes(e);
        assert!(!ways.is_empty(), "{}: no outcome does anything", e.id);
        for (name, spec) in ways {
            let mut c = with_new_events(load_catalog().unwrap(), 1, 1);
            c.events = vec![spec.clone()];
            // A lower failure chance changes a Run only when a failure was due: it must save some of 200 Runs.
            // Any other outcome changes nearly every Run; one that falls on a day a flare or an outage already
            // stopped the same thing may change nothing.
            let failure_only = spec.effects.iter().all(|f| f.mechanic == Mechanic::FailureChance);
            let (flown, seeds) = if failure_only { (&basic[..], 200) } else { (&designs[..], 20) };
            let mut met = false;
            for (scenario, design) in flown {
                assert!(launch_review(&c, design, scenario).can_launch, "{scenario}");
                let (mut fired, mut changed) = (0, 0);
                for seed in 0..seeds {
                    let with = run(&c, design, scenario, seed);
                    if !with.timeline.iter().flat_map(|t| &t.events).any(|e| e.kind == EventKind::Catalog) {
                        continue;
                    }
                    fired += 1;
                    changed += usize::from(seen(&with) != seen(&run(&off, design, scenario, seed)));
                }
                if fired == 0 {
                    continue;
                }
                met = true;
                let enough = if failure_only { changed >= 1 } else { changed * 5 >= fired * 4 };
                if !enough {
                    failures.push(format!("{name} in {scenario}: changed {changed} of {fired} Runs"));
                }
                break;
            }
            assert!(met, "{name}: no test Design meets its condition");
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}
