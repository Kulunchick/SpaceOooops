//! Which new Random Events a Run gets, and on which days, seen through the Run interface (More Random Events,
//! ticket 01): a count from the Scenario, each event at most once, on distinct days, from their own random
//! stream, so the built-in events fall exactly as before.
mod common;

use std::collections::BTreeSet;

use common::*;
use spaceooops::sim::*;
use ReliabilityTier::Basic;

/// Crop Watch's known-good Design with every Component Basic, so Component failures happen in a sweep.
fn basic_design() -> Design {
    let mut d = crop_watch_known_good();
    for pick in [&mut d.solar, &mut d.battery, &mut d.attitude, &mut d.comms] {
        pick.as_mut().unwrap().tier = Basic;
    }
    d.computer_tier = Basic;
    d
}

/// The new Random Events of a Run, with their days.
fn new_events(r: &RunResult) -> Vec<(u32, &RandomEvent)> {
    r.timeline.iter().flat_map(|t| t.events.iter().filter(|e| e.kind == EventKind::Catalog).map(move |e| (t.day, e))).collect()
}

/// The built-in Random Events of a Run (battery shortfalls aside, which depend on the state), with their days.
fn built_in_events(r: &RunResult) -> Vec<(u32, EventKind, Option<String>)> {
    r.timeline
        .iter()
        .flat_map(|t| {
            t.events
                .iter()
                .filter(|e| matches!(e.kind, EventKind::SolarFlare | EventKind::GroundStationOutage | EventKind::ComponentFailure))
                .map(move |e| (t.day, e.kind, e.component.clone()))
        })
        .collect()
}

#[test]
fn with_new_events_off_a_run_is_as_before_them() {
    let off = legacy_catalog();
    let mut no_pool = with_new_events(load_catalog().unwrap(), 3, 3);
    no_pool.events.clear();
    let mut none = legacy_catalog();
    none.events.clear();
    for design in [crop_watch_known_good(), basic_design()] {
        for seed in 0..20 {
            let before = run(&none, &design, CROP_WATCH, seed);
            assert_eq!(run(&off, &design, CROP_WATCH, seed), before, "seed {seed}");
            assert_eq!(run(&no_pool, &design, CROP_WATCH, seed), before, "seed {seed}");
        }
    }
}

#[test]
fn flares_outages_and_failures_fall_on_the_same_days_with_new_events_on() {
    let off = legacy_catalog();
    let on = with_new_events(load_catalog().unwrap(), 3, 3);
    let (mut failures, mut with_new) = (0, 0);
    for seed in 0..120 {
        let d = basic_design();
        let (before, after) = (run(&off, &d, CROP_WATCH, seed), run(&on, &d, CROP_WATCH, seed));
        assert_eq!(built_in_events(&after), built_in_events(&before), "seed {seed}");
        failures += built_in_events(&before).iter().filter(|e| e.1 == EventKind::ComponentFailure).count();
        with_new += usize::from(!new_events(&after).is_empty());
    }
    assert!(failures > 0, "the sweep sees Component failures");
    assert!(with_new > 80, "only {with_new} of 120 Runs had new events");
}

#[test]
fn the_same_seed_gives_the_same_new_events_and_other_seeds_give_others() {
    let c = load_catalog().unwrap();
    let d = crop_watch_known_good();
    let mut sets = BTreeSet::new();
    for seed in 0..20 {
        let a = run(&c, &d, CROP_WATCH, seed);
        assert_eq!(run(&c, &d, CROP_WATCH, seed), a, "seed {seed}");
        let set: Vec<(u32, String)> =
            new_events(&a).iter().map(|(day, e)| (*day, e.catalog.as_ref().unwrap().id.clone())).collect();
        sets.insert(set);
    }
    assert!(sets.len() >= 15, "20 seeds gave only {} different sets", sets.len());
}

#[test]
fn a_run_gets_its_count_of_new_events_each_once_on_distinct_days_and_none_ends_the_mission() {
    let on = with_new_events(load_catalog().unwrap(), 6, 8); // the shipped 2 to 3 are all questions (03), so passive events would not show
    let off = legacy_catalog();
    let tuning = &on.scenario(CROP_WATCH).unwrap().events;
    let (min, max) = (tuning.new_events_min as usize, tuning.new_events_max as usize);
    let mut seen = BTreeSet::new();
    for seed in 0..200 {
        let d = basic_design();
        let r = run(&on, &d, CROP_WATCH, seed);
        let before = run(&off, &d, CROP_WATCH, seed);
        let events = new_events(&r);
        let ended_early = before.debrief.stats.days_survived < before.timeline.len() as u32;
        if !ended_early {
            assert!((min..=max).contains(&events.len()), "seed {seed}: {} new events", events.len());
        }
        let ids: BTreeSet<&str> = events.iter().map(|(_, e)| e.catalog.as_ref().unwrap().id.as_str()).collect();
        assert_eq!(ids.len(), events.len(), "seed {seed}: an event repeats");
        let days: BTreeSet<u32> = events.iter().map(|(day, _)| *day).collect();
        assert_eq!(days.len(), events.len(), "seed {seed}: two events on one day");
        // A new event never ends the Run sooner; "Quiet Sun" (a lower failure chance) may let it last longer.
        assert!(r.debrief.stats.days_survived >= before.debrief.stats.days_survived, "seed {seed}: the Run ended sooner");
        assert!(r.timeline.len() >= before.timeline.len(), "seed {seed}");
        seen.extend(ids.into_iter().map(str::to_string));
    }
    // Questions fill a Run first (up to 3 or 4), so while the Scenarios give 2 to 3 events and the pool has
    // three or more questions, only those show up; ticket 07 raises the count and checks the whole pool.
    assert!(seen.len() >= 3, "a sweep shows several different events: {seen:?}");
}

#[test]
fn a_new_event_carries_its_catalog_id_group_and_scene_and_logs_its_text() {
    let c = load_catalog().unwrap();
    let r = run(&c, &crop_watch_known_good(), CROP_WATCH, 7);
    let events = new_events(&r);
    assert!(!events.is_empty());
    for (_, e) in events {
        let catalog = e.catalog.as_ref().expect("a new event names its catalog event");
        let spec = c.events.iter().find(|s| s.id == catalog.id).expect("a catalog id");
        assert_eq!(catalog.group, spec.group);
        assert_eq!(catalog.scene, spec.scene);
        assert!((1..=5).contains(&catalog.scene_days));
        assert!(e.component.is_none());
        match &spec.question {
            // a question event asks, and logs the text of the option that applied (ticket 03)
            Some(q) => {
                assert!(e.prompt.is_some());
                let option = q.options.iter().find(|o| Some(&o.id) == e.choice.as_ref()).expect("an option id");
                assert_eq!(e.text, option.log);
            }
            None => {
                match &spec.mitigation {
                    Some(m) if catalog.softened => assert_eq!(e.text, m.log),
                    _ => assert_eq!(e.text, spec.log),
                }
                assert!(e.prompt.is_none() && e.choice.is_none());
            }
        }
    }
}

#[test]
fn built_in_events_carry_no_catalog_event() {
    let c = load_catalog().unwrap();
    let r = run(&c, &basic_design(), CROP_WATCH, 3);
    for e in r.timeline.iter().flat_map(|t| &t.events).filter(|e| e.kind != EventKind::Catalog) {
        assert!(e.catalog.is_none());
    }
}
