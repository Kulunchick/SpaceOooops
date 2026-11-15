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
