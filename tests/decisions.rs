//! Interactive Random Events through the public `sim` interface: decision prompts in the timeline, choices
//! passed back into `run_with_decisions`, determinism, defaults and the Debrief's notes on decisions.
mod common;

use common::{CROP_WATCH, SPACE_WEATHER_SENTINEL, crop_watch_known_good, space_weather_sentinel_known_good};
use spaceooops::sim::*;

fn catalog() -> Catalog {
    load_catalog().unwrap()
}

/// Every decision prompt of a Run, in timeline order.
fn prompts(result: &RunResult) -> Vec<DecisionPrompt> {
    result.timeline.iter().flat_map(|d| d.events.iter().filter_map(|e| e.prompt.clone())).collect()
}

/// The first seed (from 0) whose Run with default choices asks a question about `kind`.
fn seed_with_prompt(design: &Design, scenario: &str, kind: EventKind) -> (u64, DecisionPrompt) {
    let catalog = catalog();
    (0..500)
        .find_map(|seed| {
            let r = run(&catalog, design, scenario, seed);
            r.timeline
                .iter()
                .flat_map(|d| d.events.iter())
                .find(|e| e.kind == kind && e.prompt.is_some())
                .map(|e| (seed, e.prompt.clone().unwrap()))
        })
        .expect("some seed asks this question")
}

fn other_option(prompt: &DecisionPrompt) -> String {
    prompt.options.iter().find(|o| o.id != prompt.default).unwrap().id.clone()
}

fn decide(prompt: &DecisionPrompt, choice: &str) -> Decision {
    Decision { prompt_id: prompt.id.clone(), choice: choice.to_string() }
}

#[test]
fn solar_flares_and_ground_station_outages_ask_the_player_with_two_choices() {
    for kind in [EventKind::SolarFlare, EventKind::GroundStationOutage] {
        let (_, prompt) = seed_with_prompt(&crop_watch_known_good(), CROP_WATCH, kind);
        assert_eq!(prompt.options.len(), 2, "{prompt:#?}");
        assert!(prompt.options.iter().any(|o| o.id == prompt.default));
        assert!(prompt.question.len() > 20);
        for o in &prompt.options {
            assert!(!o.label.is_empty() && o.consequence.len() > 20, "{o:#?}");
        }
    }
}

#[test]
fn run_without_decisions_is_run_with_default_choices() {
    let catalog = catalog();
    let design = crop_watch_known_good();
    let (seed, prompt) = seed_with_prompt(&design, CROP_WATCH, EventKind::SolarFlare);
    let plain = run(&catalog, &design, CROP_WATCH, seed);
    assert_eq!(plain, run_with_decisions(&catalog, &design, CROP_WATCH, seed, &[]));
    let explicit_default = [decide(&prompt, &prompt.default)];
    let with_default = run_with_decisions(&catalog, &design, CROP_WATCH, seed, &explicit_default);
    assert_eq!(plain.timeline, with_default.timeline);
    assert_eq!(plain.debrief.stats, with_default.debrief.stats);
}

#[test]
fn same_seed_and_same_decisions_give_an_identical_run() {
    let catalog = catalog();
    let design = crop_watch_known_good();
    for kind in [EventKind::SolarFlare, EventKind::GroundStationOutage] {
        let (seed, prompt) = seed_with_prompt(&design, CROP_WATCH, kind);
        let decisions = [decide(&prompt, &other_option(&prompt))];
        let a = run_with_decisions(&catalog, &design, CROP_WATCH, seed, &decisions);
        let b = run_with_decisions(&catalog, &design, CROP_WATCH, seed, &decisions);
        assert_eq!(a, b);
    }
}

#[test]
fn the_other_choice_changes_the_outcome() {
    let catalog = catalog();
    let design = crop_watch_known_good();
    for kind in [EventKind::SolarFlare, EventKind::GroundStationOutage] {
        let (seed, prompt) = seed_with_prompt(&design, CROP_WATCH, kind);
        let default = run(&catalog, &design, CROP_WATCH, seed);
        let other = run_with_decisions(&catalog, &design, CROP_WATCH, seed, &[decide(&prompt, &other_option(&prompt))]);
        assert_ne!(default.debrief.stats, other.debrief.stats, "{kind:?}");
    }
}

#[test]
fn riding_out_a_flare_keeps_the_instruments_running_and_safe_mode_does_not() {
    let catalog = catalog();
    let design = crop_watch_known_good();
    let (seed, prompt) = seed_with_prompt(&design, CROP_WATCH, EventKind::SolarFlare);
    let day = prompt.day as usize;
    let safe = run_with_decisions(&catalog, &design, CROP_WATCH, seed, &[decide(&prompt, "safe-mode")]);
    let ride = run_with_decisions(&catalog, &design, CROP_WATCH, seed, &[decide(&prompt, "ride-out")]);
    assert_eq!(safe.timeline[day - 1].operating_fraction, 0.0);
    assert!(ride.timeline[day - 1].operating_fraction > 0.0);
}

#[test]
fn buying_station_time_costs_budget_and_shows_in_the_total_cost() {
    let catalog = catalog();
    let design = crop_watch_known_good();
    let (seed, prompt) = seed_with_prompt(&design, CROP_WATCH, EventKind::GroundStationOutage);
    let wait = run_with_decisions(&catalog, &design, CROP_WATCH, seed, &[decide(&prompt, "wait")]);
    let buy = run_with_decisions(&catalog, &design, CROP_WATCH, seed, &[decide(&prompt, "buy-station-time")]);
    assert!(buy.debrief.stats.total_cost_usd > wait.debrief.stats.total_cost_usd);
    let day = prompt.day as usize;
    assert!(buy.timeline[day - 1].delivered_data_mb > wait.timeline[day - 1].delivered_data_mb);
    let sum: f64 = buy.debrief.score.lines.iter().map(|l| l.points).sum();
    assert!((buy.debrief.score.total - sum).abs() < 1e-9);
}

#[test]
fn days_before_a_decision_are_identical_whatever_the_choice() {
    let catalog = catalog();
    let design = space_weather_sentinel_known_good();
    for kind in [EventKind::SolarFlare, EventKind::GroundStationOutage] {
        let (seed, prompt) = seed_with_prompt(&design, SPACE_WEATHER_SENTINEL, kind);
        let before = prompt.day as usize - 1;
        let a = run_with_decisions(&catalog, &design, SPACE_WEATHER_SENTINEL, seed, &[decide(&prompt, &prompt.default)]);
        let b = run_with_decisions(&catalog, &design, SPACE_WEATHER_SENTINEL, seed, &[decide(&prompt, &other_option(&prompt))]);
        assert_eq!(a.timeline[..before], b.timeline[..before]);
        // The same question is asked on the decision day itself.
        assert!(prompts(&b).contains(&prompt));
    }
}

#[test]
fn a_choice_does_not_change_when_later_random_events_happen() {
    let catalog = catalog();
    let design = crop_watch_known_good();
    let (seed, prompt) = seed_with_prompt(&design, CROP_WATCH, EventKind::GroundStationOutage);
    let ids = |r: &RunResult| prompts(r).into_iter().map(|p| p.id).collect::<Vec<_>>();
    let a = run_with_decisions(&catalog, &design, CROP_WATCH, seed, &[decide(&prompt, "wait")]);
    let b = run_with_decisions(&catalog, &design, CROP_WATCH, seed, &[decide(&prompt, "buy-station-time")]);
    assert_eq!(ids(&a), ids(&b));
}

#[test]
fn the_debrief_mentions_each_decision_and_its_effect() {
    let catalog = catalog();
    let design = crop_watch_known_good();
    let (seed, prompt) = seed_with_prompt(&design, CROP_WATCH, EventKind::GroundStationOutage);
    let buy = run_with_decisions(&catalog, &design, CROP_WATCH, seed, &[decide(&prompt, "buy-station-time")]);
    let notes = &buy.debrief.decisions;
    assert_eq!(notes.len(), prompts(&buy).len());
    let note = notes.iter().find(|n| n.contains(&format!("Day {}", prompt.day))).expect("a note on the decision day");
    assert!(note.contains("station time"), "{note}");
    assert!(note.contains('$'), "the note states what it cost: {note}");

    let defaults = run(&catalog, &design, CROP_WATCH, seed);
    assert!(defaults.debrief.decisions.iter().any(|n| n.contains("default")), "{:?}", defaults.debrief.decisions);
}

#[test]
fn a_run_without_interactive_events_has_no_decision_notes() {
    let catalog = catalog();
    let design = crop_watch_known_good();
    let seed = (0..500).find(|&s| prompts(&run(&catalog, &design, CROP_WATCH, s)).is_empty()).unwrap();
    assert!(run(&catalog, &design, CROP_WATCH, seed).debrief.decisions.is_empty());
}

#[test]
fn flares_ridden_out_are_not_blamed_on_safe_mode() {
    let catalog = catalog();
    let design = crop_watch_known_good();
    let blames_safe_mode =
        |r: &RunResult| r.debrief.causes.iter().any(|c| c.kind == CauseKind::RandomEvents && c.text.contains("safe mode"));
    let defaults = (0..500)
        .map(|seed| (seed, run(&catalog, &design, CROP_WATCH, seed)))
        .find(|(_, r)| blames_safe_mode(r))
        .expect("some Run lists its flares among the causes");
    let (seed, defaults) = defaults;
    // Ride out every flare: none of them sends the spacecraft into safe mode any more.
    let ride_all: Vec<Decision> = prompts(&defaults)
        .iter()
        .filter(|p| p.options.iter().any(|o| o.id == "ride-out"))
        .map(|p| decide(p, "ride-out"))
        .collect();
    let ridden = run_with_decisions(&catalog, &design, CROP_WATCH, seed, &ride_all);
    assert!(!blames_safe_mode(&ridden), "{:#?}", ridden.debrief.causes);
}

#[test]
fn unknown_decisions_are_ignored() {
    let catalog = catalog();
    let design = crop_watch_known_good();
    let (seed, prompt) = seed_with_prompt(&design, CROP_WATCH, EventKind::SolarFlare);
    let junk = [
        Decision { prompt_id: "no-such-prompt".into(), choice: "ride-out".into() },
        decide(&prompt, "not-an-option"),
    ];
    let plain = run(&catalog, &design, CROP_WATCH, seed);
    assert_eq!(plain.timeline, run_with_decisions(&catalog, &design, CROP_WATCH, seed, &junk).timeline);
}
