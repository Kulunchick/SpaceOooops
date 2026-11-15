//! New Random Events that ask the player a question (More Random Events): the prompt, the choice
//! applying that option's mechanic, defaults, unaffordable options, the count per Run and the Debrief notes.
//! Test catalogs use only the first mechanics (instruments off, downlink, power).
mod common;

use common::*;
use spaceooops::sim::*;

/// A question event: pay `cost` for nothing to happen, or (default) wait with Instruments off for 2 days.
fn question_event(id: &str, cost: f64) -> String {
    format!(
        r#"
[[events]]
id = "{id}"
group = "lucky"
title = "Question {id}"
scene = "bonus-glow"

[events.question]
text = "Question {id}: do you pay?"
default = "wait"

[[events.question.options]]
id = "pay"
label = "Pay up"
consequence = "The problem goes away."
cost_usd = {cost:?}
log = "You paid and all was well."
note = "the problem went away"
effects = []

[[events.question.options]]
id = "wait"
label = "Wait it out"
consequence = "Free, but the Instruments are off for 2 days."
log = "You waited and the Instruments went off."
note = "the Instruments were off for 2 days"
effects = [{{ mechanic = "instruments-off", days = 2 }}]
"#
    )
}

fn passive_event(id: &str) -> String {
    format!(
        r#"
[[events]]
id = "{id}"
group = "lucky"
title = "Passive {id}"
log = "Something happened."
scene = "hit"
effects = [{{ mechanic = "power", strength = -0.01, days = 1 }}]
"#
    )
}

fn one_question(cost: f64) -> Catalog {
    catalog_with_events(&question_event("q-one", cost), 1, 1)
}

/// The prompts of new events (the built-in flare and outage prompts are left out).
fn event_prompts(r: &RunResult) -> Vec<DecisionPrompt> {
    r.timeline
        .iter()
        .flat_map(|d| d.events.iter().filter_map(|e| e.prompt.clone()))
        .filter(|p| p.id.contains("-event-"))
        .collect()
}

fn decide(p: &DecisionPrompt, choice: &str) -> Decision {
    Decision { prompt_id: p.id.clone(), choice: choice.to_string() }
}

#[test]
fn a_question_event_carries_a_prompt_with_two_options_and_a_default() {
    let c = one_question(15000.0);
    let r = run(&c, &crop_watch_known_good(), CROP_WATCH, 3);
    let ps = event_prompts(&r);
    assert_eq!(ps.len(), 1, "{ps:#?}");
    let p = &ps[0];
    assert_eq!(p.id, format!("day-{}-event-q-one", p.day));
    assert_eq!(p.question, "Question q-one: do you pay?");
    assert_eq!(p.default, "wait");
    assert_eq!(p.options.iter().map(|o| o.id.as_str()).collect::<Vec<_>>(), ["pay", "wait"]);
    assert!(p.options[0].consequence.starts_with("The problem goes away."));
    assert!(p.options[0].consequence.contains("Costs $15k"), "{}", p.options[0].consequence);
    assert_eq!(p.options[1].consequence, "Free, but the Instruments are off for 2 days.");
    assert!(p.options.iter().all(|o| o.available));
}

#[test]
fn the_players_choice_applies_that_options_mechanic_and_no_answer_applies_the_default() {
    let c = one_question(1000.0);
    let d = crop_watch_known_good();
    let plain = run(&c, &d, CROP_WATCH, 3);
    let p = event_prompts(&plain).remove(0);
    assert_eq!(operating_on(&plain, p.day), 0.0, "the default has the Instruments off");
    let waited = run_with_decisions(&c, &d, CROP_WATCH, 3, &[decide(&p, "wait")]);
    assert_eq!(plain.timeline, waited.timeline);
    let paid = run_with_decisions(&c, &d, CROP_WATCH, 3, &[decide(&p, "pay")]);
    assert_ne!(paid.timeline, plain.timeline);
    // paying changes nothing but the Budget: the same Run as without the event on the days it would have held
    let none = run(&legacy_catalog(), &d, CROP_WATCH, 3);
    assert_eq!(operating_on(&paid, p.day), operating_on(&none, p.day));
    assert_eq!(operating_on(&paid, p.day + 1), operating_on(&none, p.day + 1));
    // an invalid answer, or one to another question, is the default
    assert_eq!(plain, run_with_decisions(&c, &d, CROP_WATCH, 3, &[decide(&p, "nonsense")]));
    let elsewhere = Decision { prompt_id: "elsewhere".into(), choice: "pay".into() };
    assert_eq!(plain, run_with_decisions(&c, &d, CROP_WATCH, 3, &[elsewhere]));
}

#[test]
fn days_before_a_question_are_the_same_whatever_its_answer() {
    let c = one_question(1000.0);
    let d = crop_watch_known_good();
    let plain = run(&c, &d, CROP_WATCH, 5);
    let p = event_prompts(&plain).remove(0);
    let paid = run_with_decisions(&c, &d, CROP_WATCH, 5, &[decide(&p, "pay")]);
    let before = p.day as usize - 1;
    assert_eq!(plain.timeline[..before], paid.timeline[..before]);
    let asked = |r: &RunResult| r.timeline[before].events.iter().filter_map(|e| e.prompt.clone()).collect::<Vec<_>>();
    assert_eq!(asked(&plain), asked(&paid));
}

#[test]
fn an_option_the_budget_cannot_pay_for_is_unavailable_and_a_decision_naming_it_takes_the_default() {
    let c = one_question(1.0e12);
    let d = crop_watch_known_good();
    let plain = run(&c, &d, CROP_WATCH, 3);
    let p = event_prompts(&plain).remove(0);
    assert!(!p.options[0].available);
    let reason = p.options[0].unavailable_reason.as_deref().unwrap();
    assert!(reason.contains("Not enough Budget") && reason.contains("left"), "{reason}");
    assert!(p.options[1].available);
    assert_eq!(plain, run_with_decisions(&c, &d, CROP_WATCH, 3, &[decide(&p, "pay")]));
}

#[test]
fn a_run_has_three_or_four_interactive_new_events_and_never_more_than_four() {
    let mut toml = String::new();
    for i in 0..8 {
        toml += &question_event(&format!("q-{i}"), 1000.0);
    }
    for i in 0..5 {
        toml += &passive_event(&format!("p-{i}"));
    }
    let c = catalog_with_events(&toml, 7, 7);
    let d = crop_watch_known_good();
    let runs: Vec<_> = (0..60).map(|seed| run(&c, &d, CROP_WATCH, seed)).collect();
    // a Run that ended early never reached its later events
    let full = runs.iter().map(|r| r.timeline.len()).max().unwrap();
    let mut seen = std::collections::BTreeSet::new();
    for (seed, r) in runs.iter().enumerate().filter(|(_, r)| r.timeline.len() == full) {
        let n = event_prompts(r).len();
        assert!((3..=4).contains(&n), "seed {seed}: {n} interactive events");
        seen.insert(n);
    }
    assert_eq!(seen.len(), 2, "both 3 and 4 occur: {seen:?}");
    assert!(runs.iter().all(|r| event_prompts(r).len() <= 4));
}

#[test]
fn the_debrief_notes_each_question_in_day_order_with_who_chose() {
    let toml = question_event("q-a", 1000.0) + &question_event("q-b", 1000.0);
    let c = catalog_with_events(&toml, 2, 2);
    let d = crop_watch_known_good();
    let plain = run(&c, &d, CROP_WATCH, 7);
    let ps = event_prompts(&plain);
    assert_eq!(ps.len(), 2);
    let notes: Vec<_> = plain.debrief.decisions.iter().filter(|n| n.contains("Question q-")).collect();
    assert_eq!(notes.len(), 2, "{:#?}", plain.debrief.decisions);
    for (n, p) in notes.iter().zip(&ps) {
        let title = format!("Question {}", p.id.rsplit_once("-event-").unwrap().1);
        let want = format!(
            "Day {}, {title}: no answer was given, so by default the spacecraft chose \"Wait it out\". The Instruments were off for 2 days.",
            p.day
        );
        assert_eq!(**n, want);
    }
    let chosen = run_with_decisions(&c, &d, CROP_WATCH, 7, &[decide(&ps[0], "pay")]);
    let first = chosen.debrief.decisions.iter().find(|n| n.contains("Question q-")).unwrap();
    assert!(first.contains(": you chose \"Pay up\". The problem went away."), "{first}");
}

#[test]
fn a_badly_formed_question_is_refused_when_the_catalog_loads() {
    let base = question_event("q-x", 5.0);
    let err = |from: &str, to: &str| parse_events(&base.replace(from, to)).unwrap_err().to_string();
    assert!(err("default = \"wait\"", "default = \"nope\"").contains("default"));
    assert!(err("default = \"wait\"", "default = \"pay\"").contains("free"));
    assert!(err("id = \"wait\"", "id = \"pay\"").contains("distinct"));
    let effects = "scene = \"bonus-glow\"\neffects = [{ mechanic = \"power\", strength = -0.1, days = 1 }]";
    assert!(err("scene = \"bonus-glow\"", effects).contains("question"));
    let one = base.split("[[events.question.options]]").take(2).collect::<Vec<_>>().join("[[events.question.options]]");
    assert!(parse_events(&one).unwrap_err().to_string().contains("two options"));
    assert!(parse_events(&base).is_ok());
}

#[test]
fn the_shipped_catalog_has_the_eight_question_events() {
    let c = load_catalog().unwrap();
    for id in [
        "digger-cut-the-cable",
        "birds-in-the-dish",
        "lost-frequency-licence",
        "rival-jams-the-link",
        "journalists-want-a-picture",
        "cloud-archive-unpaid",
        "space-cat-chases-the-beam",
        "university-joint-observations",
    ] {
        let e = c.events.iter().find(|e| e.id == id).unwrap_or_else(|| panic!("{id} missing"));
        let q = e.question.as_ref().expect("has a question");
        assert_eq!(q.options.len(), 2);
        assert!(q.options.iter().any(|o| o.id == q.default && o.cost_usd == 0.0));
    }
}
