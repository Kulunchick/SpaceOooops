//! Gamble options of new Random Events (More Random Events, ticket 04): the catalog rules, the roll planned
//! with the Run, both outcomes through the Run interface, the consequence text and the Debrief note.
mod common;

use common::*;
use spaceooops::sim::*;

/// A question whose second option is a gamble with the given chance: the good outcome changes nothing,
/// the bad one switches the Instruments off for 3 days. The default (free, safe) option is "safe".
fn gamble_event(id: &str, chance: f64) -> String {
    format!(
        r#"
[[events]]
id = "{id}"
group = "lucky"
title = "Gamble {id}"
scene = "bonus-glow"

[events.question]
text = "Gamble {id}: take the risk?"
default = "safe"

[[events.question.options]]
id = "safe"
label = "Play safe"
consequence = "Nothing happens."
log = "You played safe."
note = "nothing happened"
effects = []

[[events.question.options]]
id = "risk"
label = "Risk it"
consequence = "Maybe nothing happens."
log = "The risk paid off."
note = "the risk paid off"
effects = []
chance = {chance:?}
bad = {{ log = "The risk failed.", note = "the Instruments went off for 3 days", effects = [{{ mechanic = "instruments-off", days = 3 }}] }}
"#
    )
}

fn one_gamble(chance: f64) -> Catalog {
    catalog_with_events(&gamble_event("g-one", chance), 1, 1)
}

fn prompt_of(r: &RunResult) -> DecisionPrompt {
    r.timeline
        .iter()
        .flat_map(|d| d.events.iter().filter_map(|e| e.prompt.clone()))
        .find(|p| p.id.contains("-event-"))
        .expect("the event asked a question")
}

fn risk(p: &DecisionPrompt) -> Decision {
    Decision { prompt_id: p.id.clone(), choice: "risk".to_string() }
}

fn operating(r: &RunResult, day: u32) -> f64 {
    r.timeline[day as usize - 1].operating_fraction
}

fn event_text(r: &RunResult, day: u32) -> String {
    r.timeline[day as usize - 1].events.iter().find(|e| e.catalog.is_some()).unwrap().text.clone()
}

#[test]
fn a_gamble_that_always_goes_well_applies_the_good_outcome() {
    let c = one_gamble(1.0);
    let d = crop_watch_known_good();
    let plain = run(&c, &d, CROP_WATCH, 3);
    let p = prompt_of(&plain);
    let r = run_with_decisions(&c, &d, CROP_WATCH, 3, &[risk(&p)]);
    assert_eq!(event_text(&r, p.day), "The risk paid off.");
    assert_eq!(operating(&r, p.day), 1.0);
    let note = r.debrief.decisions.iter().find(|n| n.contains("Gamble g-one")).unwrap();
    assert!(note.contains("you chose \"Risk it\". The gamble paid off: the risk paid off."), "{note}");
}

#[test]
fn a_gamble_that_always_fails_applies_the_bad_outcome() {
    let c = one_gamble(0.0);
    let d = crop_watch_known_good();
    let plain = run(&c, &d, CROP_WATCH, 3);
    let p = prompt_of(&plain);
    let r = run_with_decisions(&c, &d, CROP_WATCH, 3, &[risk(&p)]);
    assert_eq!(event_text(&r, p.day), "The risk failed.");
    assert_eq!(operating(&r, p.day), 0.0);
    assert_eq!(operating(&r, p.day + 2), 0.0);
    let note = r.debrief.decisions.iter().find(|n| n.contains("Gamble g-one")).unwrap();
    assert!(
        note.contains("you chose \"Risk it\". The gamble failed: the Instruments went off for 3 days."),
        "{note}"
    );
}

#[test]
fn the_safe_option_of_the_same_question_has_no_gamble_note() {
    let c = one_gamble(0.0);
    let r = run(&c, &crop_watch_known_good(), CROP_WATCH, 3);
    let note = r.debrief.decisions.iter().find(|n| n.contains("Gamble g-one")).unwrap();
    assert!(!note.contains("gamble"), "{note}");
}

#[test]
fn the_roll_is_fixed_by_the_seed_whatever_else_was_answered() {
    // a half chance: across seeds both outcomes occur, and each seed repeats its own outcome
    let c = one_gamble(0.5);
    let d = crop_watch_known_good();
    let mut seen = std::collections::BTreeSet::new();
    for seed in 0..30 {
        let plain = run(&c, &d, CROP_WATCH, seed);
        // a Run that ended before the event's day never asked
        let asked = plain.timeline.iter().flat_map(|t| &t.events).filter_map(|e| e.prompt.as_ref());
        if !asked.into_iter().any(|p| p.id.contains("-event-")) {
            continue;
        }
        let p = prompt_of(&plain);
        let a =run_with_decisions(&c, &d, CROP_WATCH, seed, &[risk(&p)]);
        let b = run_with_decisions(&c, &d, CROP_WATCH, seed, &[risk(&p)]);
        assert_eq!(a, b);
        // answering another, unrelated question as well changes nothing about this roll
        let noise = Decision { prompt_id: "elsewhere".into(), choice: "risk".into() };
        let c2 = run_with_decisions(&c, &d, CROP_WATCH, seed, &[noise, risk(&p)]);
        assert_eq!(a, c2);
        seen.insert(event_text(&a, p.day));
    }
    assert_eq!(seen.len(), 2, "both outcomes occur: {seen:?}");
}

#[test]
fn days_before_the_gamble_are_the_same_whatever_its_answer_or_outcome() {
    let d = crop_watch_known_good();
    for chance in [0.0, 1.0] {
        let c = one_gamble(chance);
        let plain = run(&c, &d, CROP_WATCH, 5);
        let p = prompt_of(&plain);
        let r = run_with_decisions(&c, &d, CROP_WATCH, 5, &[risk(&p)]);
        let before = p.day as usize - 1;
        assert_eq!(plain.timeline[..before], r.timeline[..before]);
    }
}

#[test]
fn the_consequence_says_it_is_a_gamble_and_states_the_odds() {
    let r = run(&one_gamble(0.7), &crop_watch_known_good(), CROP_WATCH, 3);
    let p = prompt_of(&r);
    assert_eq!(p.options[0].consequence, "Nothing happens.");
    assert_eq!(p.options[1].consequence, "Maybe nothing happens. A gamble: about 7 in 10 it goes well.");
}

#[test]
fn a_badly_formed_gamble_is_refused_when_the_catalog_loads() {
    let base = gamble_event("g-x", 0.5);
    let err = |from: &str, to: &str| parse_events(&base.replace(from, to)).unwrap_err().to_string();
    assert!(parse_events(&base).is_ok());
    assert!(err("chance = 0.5", "chance = 1.5").contains("chance"));
    assert!(err("chance = 0.5", "chance = -0.1").contains("chance"));
    assert!(err("chance = 0.5", "").contains("bad"));
    let no_bad = base.lines().filter(|l| !l.starts_with("bad = ")).collect::<Vec<_>>().join("\n");
    assert!(parse_events(&no_bad).unwrap_err().to_string().contains("chance"));
    assert!(err("The risk failed.", "").contains("bad"));
    assert!(err("mechanic = \"instruments-off\", days = 3", "mechanic = \"instruments-off\"").contains("days"));
}

#[test]
fn the_shipped_catalog_has_the_gamble_events_of_this_ticket() {
    let c = load_catalog().unwrap();
    for (id, model) in [
        ("debris-on-a-collision-course", None),
        ("meteor-shower", None),
        ("software-update-on-a-friday-evening", None),
        ("neighbour-on-the-same-frequency", None),
        ("inspector-satellite-comes-close", Some("inspector")),
        ("schoolchildren-hack-the-link", None),
        ("scientists-argue-where-to-look", None),
        ("ufo-parks-alongside", Some("ufo")),
        ("dragon-takes-the-satellite-for-something-shiny", Some("dragon")),
    ] {
        let e = c.events.iter().find(|e| e.id == id).unwrap_or_else(|| panic!("{id} missing"));
        assert_eq!(e.model.as_deref(), model, "{id}");
        let q = e.question.as_ref().expect("has a question");
        let gamble = q.options.iter().filter(|o| o.chance.is_some()).count();
        assert_eq!(gamble, 1, "{id} has one gamble option");
        assert!(q.options.iter().any(|o| o.id == q.default && o.chance.is_none() && o.cost_usd == 0.0));
    }
}
