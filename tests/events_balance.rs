//! The full pool of new Random Events and its balance (More Random Events, ticket 07): the shipped pool has the
//! agreed size and split and names no real country or agency; every Scenario gives a Run 5 to 8 new events, 3 or
//! 4 of them with a question, none of them ending the Mission; and together they move Science Value by about a
//! tenth at most, so the Design still decides Success. All through the public `sim` interface.
mod common;

use std::collections::BTreeSet;

use common::*;
use spaceooops::sim::*;

// ---------------------------------------------------------------- the pool

#[test]
fn the_shipped_pool_holds_59_events_40_real_10_fantasy_9_lucky_21_with_a_question() {
    let c = load_catalog().unwrap();
    let count = |g: EventGroup| c.events.iter().filter(|e| e.group == g).count();
    assert_eq!(c.events.len(), 59);
    assert_eq!((count(EventGroup::Real), count(EventGroup::Fantasy), count(EventGroup::Lucky)), (40, 10, 9));
    assert_eq!(c.events.iter().filter(|e| e.question.is_some()).count(), 21);
}

/// Every player-facing text of one event.
fn texts(e: &EventSpec) -> Vec<&str> {
    let mut out = vec![e.title.as_str(), e.log.as_str()];
    out.extend(e.real_life.as_deref());
    out.extend(e.mitigation.as_ref().map(|m| m.log.as_str()));
    let options = e.question.iter().flat_map(|q| &q.options);
    out.extend(e.question.iter().map(|q| q.text.as_str()));
    for o in options {
        out.extend([o.label.as_str(), o.consequence.as_str(), o.log.as_str(), o.note.as_str()]);
        out.extend(o.bad.iter().flat_map(|b| [b.log.as_str(), b.note.as_str()]));
        out.extend(o.effects.iter().chain(o.bad.iter().flat_map(|b| &b.effects)).filter_map(|f| f.label.as_deref()));
    }
    out.extend(e.effects.iter().filter_map(|f| f.label.as_deref()));
    out
}

#[test]
fn no_event_text_names_a_real_country_or_agency_and_the_rival_is_a_rival_agency() {
    let c = load_catalog().unwrap();
    let deny = ["NASA", "ESA", "Russia", "China", "America", "Japan", "India", "Brazil", "Angola", "Soviet"];
    for e in &c.events {
        for text in texts(e) {
            for name in deny {
                assert!(!text.contains(name), "{}: names {name}: {text}", e.id);
            }
        }
    }
    let rival: Vec<&str> = c.events.iter().flat_map(texts).filter(|t| t.contains("rival")).collect();
    assert!(!rival.is_empty());
    assert!(rival.iter().any(|t| t.contains("rival agency")), "{rival:?}");
}

#[test]
fn an_event_naming_a_real_agency_or_country_fails_to_load() {
    let event = |text: &str| {
        format!(
            "[[events]]\nid = \"x\"\ngroup = \"real\"\ntitle = \"X\"\nlog = \"{text}\"\nscene = \"hit\"\n\
             real_life = \"It happens.\"\neffects = [{{ mechanic = \"instruments-off\", days = 1 }}]\n"
        )
    };
    assert!(parse_events(&event("A rival agency did it.")).is_ok());
    for text in ["NASA called.", "A Russian satellite passed.", "The ISRO team waved.", "Over Japan the link failed."] {
        let msg = parse_events(&event(text)).expect_err(text).to_string();
        assert!(msg.contains("data/events.toml") && msg.contains("'x'"), "{text}: {msg}");
    }
}

#[test]
fn every_gamble_states_its_odds_between_1_and_9_in_10() {
    let c = load_catalog().unwrap();
    let mut gambles = 0;
    for e in &c.events {
        for o in e.question.iter().flat_map(|q| &q.options).filter(|o| o.chance.is_some()) {
            gambles += 1;
            let n = (o.chance.unwrap() * 10.0).round();
            assert!((1.0..=9.0).contains(&n), "{} / {}: about {n} in 10", e.id, o.id);
            assert!(!o.consequence.to_lowercase().contains("gamble"), "{} / {}: the sim says it is a gamble", e.id, o.id);
        }
    }
    assert!(gambles >= 9);
}
