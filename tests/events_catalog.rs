//! The new Random Events are data (`data/events.toml`): the shipped file loads, and a broken entry fails with a
//! message that names the file, the event and what is wrong (More Random Events, ticket 01).
use std::collections::BTreeSet;

use spaceooops::sim::*;

/// One well-formed passive event, to break one field at a time.
const GOOD: &str = r#"
[[events]]
id = "test-event"
group = "real"
title = "A test event"
log = "Something happened."
scene = "hit"
real_life = "It happens."
effects = [{ mechanic = "instruments-off", days = 2 }]
"#;

fn error(text: &str) -> String {
    parse_events(text).expect_err("the entry should be rejected").to_string()
}

#[test]
fn the_shipped_events_load_into_the_catalog() {
    let c = load_catalog().unwrap();
    assert!(c.events.len() >= 8, "{} events", c.events.len());
    let ids: BTreeSet<&str> = c.events.iter().map(|e| e.id.as_str()).collect();
    assert_eq!(ids.len(), c.events.len(), "ids are unique");
    for e in &c.events {
        assert!(!e.title.is_empty(), "{}", e.id);
        assert!(e.question.is_some() || !e.log.is_empty(), "{} has log text", e.id);
    }
}

#[test]
fn a_well_formed_event_parses() {
    let events = parse_events(GOOD).unwrap();
    assert_eq!(events.len(), 1);
    let e = &events[0];
    assert_eq!(e.id, "test-event");
    assert_eq!(e.group, EventGroup::Real);
    assert_eq!(e.scene, SceneEffect::Hit);
    assert_eq!(e.effects[0].mechanic, Mechanic::InstrumentsOff);
    assert_eq!(e.effects[0].days, Some(2));
}

#[test]
fn an_unknown_key_names_the_file_and_the_key() {
    let msg = error(&GOOD.replace("scene = ", "scenery = \"x\"\nscene = "));
    assert!(msg.contains("data/events.toml") && msg.contains("scenery"), "{msg}");
}

#[test]
fn an_unknown_mechanic_is_rejected() {
    let msg = error(&GOOD.replace("instruments-off", "teleport"));
    assert!(msg.contains("data/events.toml") && msg.contains("teleport"), "{msg}");
}

#[test]
fn a_duplicate_id_is_rejected() {
    let msg = error(&format!("{GOOD}{GOOD}"));
    assert!(msg.contains("test-event") && msg.contains("twice"), "{msg}");
}

#[test]
fn an_id_must_be_kebab_case_and_not_an_existing_event() {
    for id in ["Test_Event", "solar-flare", "ground-station-outage", ""] {
        let msg = error(&GOOD.replace("\"test-event\"", &format!("\"{id}\"")));
        assert!(msg.contains("data/events.toml"), "{id}: {msg}");
    }
}

#[test]
fn a_passive_event_needs_log_text_and_an_effect() {
    let msg = error(&GOOD.replace("log = \"Something happened.\"", ""));
    assert!(msg.contains("test-event") && msg.contains("log"), "{msg}");
    let msg = error(&GOOD.replace("effects = [{ mechanic = \"instruments-off\", days = 2 }]", "effects = []"));
    assert!(msg.contains("test-event") && msg.contains("effect"), "{msg}");
}

#[test]
fn a_timed_effect_lasts_either_n_days_or_to_the_end() {
    for broken in ["{ mechanic = \"power\", strength = -0.1 }", "{ mechanic = \"power\", strength = -0.1, days = 2, to_end = true }",
        "{ mechanic = \"power\", strength = -0.1, days = 0 }"]
    {
        let msg = error(&GOOD.replace("{ mechanic = \"instruments-off\", days = 2 }", broken));
        assert!(msg.contains("test-event") && msg.contains("power"), "{broken}: {msg}");
    }
    let ok = GOOD.replace("{ mechanic = \"instruments-off\", days = 2 }", "{ mechanic = \"power\", strength = -0.1, to_end = true }");
    assert!(parse_events(&ok).is_ok());
}

#[test]
fn an_instant_effect_has_no_length() {
    let msg = error(&GOOD.replace("{ mechanic = \"instruments-off\", days = 2 }", "{ mechanic = \"budget\", strength = 1000.0, days = 2 }"));
    assert!(msg.contains("test-event") && msg.contains("budget"), "{msg}");
}

#[test]
fn a_share_stays_within_minus_one_and_one() {
    let msg = error(&GOOD.replace("{ mechanic = \"instruments-off\", days = 2 }", "{ mechanic = \"downlink\", strength = -1.5, days = 1 }"));
    assert!(msg.contains("test-event") && msg.contains("downlink"), "{msg}");
}

#[test]
fn a_real_event_needs_a_real_life_line_and_a_fantasy_event_has_none() {
    let msg = error(&GOOD.replace("real_life = \"It happens.\"", ""));
    assert!(msg.contains("test-event") && msg.contains("real_life"), "{msg}");
    let msg = error(&GOOD.replace("group = \"real\"", "group = \"fantasy\""));
    assert!(msg.contains("test-event") && msg.contains("real_life"), "{msg}");
    assert!(parse_events(&GOOD.replace("group = \"real\"", "group = \"lucky\"")).is_ok());
}

#[test]
fn every_scenario_sets_how_many_new_events_a_run_gets() {
    let c = load_catalog().unwrap();
    for s in &c.scenarios {
        let t = &s.events;
        assert!(t.new_events_min <= t.new_events_max, "{}", s.id);
        assert!(t.new_events_max > 0, "{}: new events are on in the shipped game", s.id);
    }
}
