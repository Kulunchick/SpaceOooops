//! The Debrief's real-life lines, the separate Score line of a Score bonus, and the shipped rescue event (More
//! Random Events, ticket 06).
mod common;

use common::*;
use spaceooops::sim::*;

fn event(id: &str, group: &str, real_life: Option<&str>) -> String {
    let rl = real_life.map(|t| format!("real_life = \"{t}\"")).unwrap_or_default();
    format!(
        r#"
[[events]]
id = "{id}"
group = "{group}"
title = "Title {id}"
log = "Log {id}."
scene = "hit"
{rl}
effects = [{{ mechanic = "power", strength = -0.01, days = 1 }}]
"#
    )
}

fn bonus_event(points: f64) -> String {
    format!(
        r#"
[[events]]
id = "bonus-event"
group = "lucky"
title = "A good deed"
log = "You did a good deed."
scene = "bonus-glow"
effects = [{{ mechanic = "score-bonus", strength = {points:?}, label = "Good deed" }}]
"#
    )
}

#[test]
fn the_debrief_lists_the_real_life_line_of_each_real_event_once() {
    let toml = format!(
        "{}{}{}",
        event("real-a", "real", Some("Real A happens.")),
        event("fantasy-a", "fantasy", None),
        event("lucky-a", "lucky", Some("Lucky A happens."))
    );
    let c = catalog_with_events(&toml, 3, 3);
    let r = run(&c, &crop_watch_known_good(), CROP_WATCH, 3);
    assert_eq!(r.debrief.real_life, vec!["Title real-a: Real A happens.".to_string()]);
}

#[test]
fn an_event_that_did_not_happen_has_no_line() {
    let c = catalog_with_events(&event("real-a", "real", Some("Real A happens.")), 0, 0);
    let r = run(&c, &crop_watch_known_good(), CROP_WATCH, 3);
    assert!(r.debrief.real_life.is_empty());
}

#[test]
fn a_score_bonus_is_its_own_score_line_and_leaves_science_value_alone() {
    let d = crop_watch_known_good();
    let plain = run(&legacy_catalog(), &d, CROP_WATCH, 3);
    let c = catalog_with_events(&bonus_event(250.0), 1, 1);
    let r = run(&c, &d, CROP_WATCH, 3);
    let line = r.debrief.score.lines.iter().find(|l| l.kind == ScoreLineKind::EventBonus).expect("a bonus line");
    assert_eq!(line.label, "Good deed");
    assert_eq!(line.points, 250.0);
    let sum: f64 = r.debrief.score.lines.iter().map(|l| l.points).sum();
    assert_eq!(r.debrief.score.total, sum);
    assert_eq!(r.debrief.success, plain.debrief.success);
    assert_eq!(r.debrief.stats.science_value, plain.debrief.stats.science_value);
}

#[test]
fn a_score_bonus_needs_a_label() {
    let msg = parse_events(&bonus_event(5.0).replace(", label = \"Good deed\"", "")).unwrap_err().to_string();
    assert!(msg.contains("bonus-event") && msg.contains("label"), "{msg}");
}

#[test]
fn every_shipped_real_event_has_a_real_life_line_and_no_fantasy_event_has() {
    let c = load_catalog().unwrap();
    for e in &c.events {
        match e.group {
            EventGroup::Real => assert!(e.real_life.as_ref().is_some_and(|t| !t.trim().is_empty()), "{}", e.id),
            EventGroup::Fantasy => assert!(e.real_life.is_none(), "{}", e.id),
            EventGroup::Lucky => {}
        }
    }
}

#[test]
fn the_rescue_event_costs_two_days_of_science_and_pays_a_separate_score_line() {
    let c = load_catalog().unwrap();
    let e = c.events.iter().find(|e| e.id == "rescuers-ask-for-flood-images").expect("event 37");
    let q = e.question.as_ref().expect("a question");
    let help = q.options.iter().find(|o| o.id == "help").expect("help option");
    assert!(help.effects.iter().any(|f| f.mechanic == Mechanic::InstrumentsOff && f.days == Some(2)));
    assert!(help.effects.iter().any(|f| f.mechanic == Mechanic::ScoreBonus && f.label.is_some() && f.strength > 0.0));
    let free = q.options.iter().find(|o| o.id == q.default).unwrap();
    assert!(free.effects.iter().all(|f| f.mechanic != Mechanic::ScoreBonus));
}

#[test]
fn helping_the_rescuers_is_worth_40_points_less_than_the_whole_budget_bonus() {
    let c = load_catalog().unwrap();
    let d = crop_watch_known_good();
    let (seed, prompt) = (0..200)
        .find_map(|seed| {
            let r = run(&c, &d, CROP_WATCH, seed);
            let p = r.timeline.iter().flat_map(|t| &t.events).filter_map(|e| e.prompt.clone()).find(|p| p.id.ends_with("rescuers-ask-for-flood-images"));
            p.map(|p| (seed, p))
        })
        .expect("a Run that meets the rescuers");
    let help = prompt.options.iter().find(|o| o.id == "help").unwrap();
    assert!(help.consequence.contains("40 points"), "{}", help.consequence);
    let r = run_with_decisions(&c, &d, CROP_WATCH, seed, &[Decision { prompt_id: prompt.id.clone(), choice: "help".into() }]);
    let line = r.debrief.score.lines.iter().find(|l| l.kind == ScoreLineKind::EventBonus).expect("the rescue line");
    assert_eq!(line.points, 40.0);
    assert!(line.points < c.scoring.budget_margin_points);
}
