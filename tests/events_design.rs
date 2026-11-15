//! New Random Events that depend on the Design (More Random Events, ticket 05): conditions, "more likely",
//! mitigation, and what the Debrief says about the choice behind an event. Seen through `run` and `parse_events`.
mod common;

use common::*;
use spaceooops::sim::*;

/// One passive event with its own Design fields; `extra` is spliced in after the scene.
fn event(id: &str, extra: &str) -> String {
    format!(
        "[[events]]\nid = \"{id}\"\ngroup = \"real\"\ntitle = \"Title of {id}\"\nlog = \"Log of {id}.\"\nscene = \"hit\"\nreal_life = \"It happens.\"\n{extra}\n\
         effects = [{{ mechanic = \"instruments-off\", days = 2 }}]\n"
    )
}

fn error(toml: &str) -> String {
    parse_events(toml).expect_err("should be rejected").to_string()
}

fn new_events(r: &RunResult) -> Vec<&RandomEvent> {
    r.timeline.iter().flat_map(|t| t.events.iter().filter(|e| e.kind == EventKind::Catalog)).collect()
}

fn ids(r: &RunResult) -> Vec<String> {
    new_events(r).iter().map(|e| e.catalog.as_ref().unwrap().id.clone()).collect()
}

fn final_science(r: &RunResult) -> f64 {
    r.timeline.last().unwrap().science_value
}

/// The known-good Design with a Basic computer: no Flight-proven computer (a Basic one may fail and end the
/// Run early, so tests of an event on it skip Runs where the event never came).
fn basic_computer() -> Design {
    let mut d = crop_watch_known_good();
    d.computer_tier = ReliabilityTier::Basic;
    d
}

// ---------------------------------------------------------------- validation

#[test]
fn an_unknown_design_fact_is_rejected_with_the_fact_named() {
    for (field, named) in [
        ("only = \"low-orbits\"", "low-orbits"),
        ("more_likely = { fact = \"nonsense\", weight = 2 }", "nonsense"),
        ("mitigation = { fact = \"large-computer\", scale = 0.0, log = \"Fine.\" }", "large-computer"),
    ] {
        let msg = error(&event("odd-one", field));
        assert!(msg.contains(named), "{msg}");
    }
    assert!(parse_events(&event("fine-one", "only = \"low-orbit\"")).is_ok());
}

#[test]
fn a_more_likely_weight_is_two_to_four() {
    for weight in [0, 1, 5] {
        let msg = error(&event("odd-one", &format!("more_likely = {{ fact = \"low-orbit\", weight = {weight} }}")));
        assert!(msg.contains("odd-one") && msg.contains("weight"), "{msg}");
    }
    for weight in [2, 3, 4] {
        assert!(parse_events(&event("fine-one", &format!("more_likely = {{ fact = \"low-orbit\", weight = {weight} }}"))).is_ok());
    }
}

#[test]
fn a_mitigation_scale_lies_in_zero_to_one_and_is_zero_for_instruments_off() {
    let m = |scale: &str| format!("mitigation = {{ fact = \"computer-flight-proven\", scale = {scale}, log = \"Shrugged off.\" }}");
    for scale in ["-0.1", "1.0", "1.5"] {
        let msg = error(&event("odd-one", &m(scale)));
        assert!(msg.contains("odd-one") && msg.contains("scale"), "{scale}: {msg}");
    }
    // Instruments off cannot be scaled, so a mitigation of it must remove it.
    let msg = error(&event("odd-one", &m("0.5")));
    assert!(msg.contains("odd-one") && msg.contains("instruments-off"), "{msg}");
    assert!(parse_events(&event("fine-one", &m("0.0"))).is_ok());
    let power = event("fine-two", &m("0.5")).replace("{ mechanic = \"instruments-off\", days = 2 }", "{ mechanic = \"power\", strength = -0.1, to_end = true }");
    assert!(parse_events(&power).is_ok());
}

// ---------------------------------------------------------------- the shipped Design-aware events

/// (Design, Scenario, the facts that hold for it, written out by hand from the Design).
fn designs() -> Vec<(Design, &'static str, Vec<&'static str>)> {
    vec![
        // sso-600, wings, S-band, reaction wheels, an imager
        (crop_watch_known_good(), CROP_WATCH, vec!["low-orbit", "deployable-panels", "camera", "precise-attitude"]),
        // geo, body panels, S-band, reaction wheels, a relay (not an imager)
        (connect_the_villages_known_good(), CONNECT_THE_VILLAGES, vec!["high-orbit", "precise-attitude"]),
        // leo-400, wings, X-band, magnetorquers, particle detector and probe
        (space_weather_sentinel_known_good(), SPACE_WEATHER_SENTINEL, vec!["low-orbit", "deployable-panels"]),
    ]
}

fn only_of(e: &EventSpec) -> Option<String> {
    e.only.map(|f| serde_json::to_value(f).unwrap().as_str().unwrap().to_string())
}

fn seen_over_seeds(c: &Catalog, design: &Design, scenario: &str) -> std::collections::BTreeSet<String> {
    (0..60).flat_map(|seed| ids(&run(c, design, scenario, seed))).collect()
}

#[test]
fn an_event_never_appears_in_a_run_whose_design_does_not_meet_its_condition() {
    let c = with_new_events(load_catalog().unwrap(), 8, 8);
    let conditioned: Vec<&EventSpec> = c.events.iter().filter(|e| e.only.is_some()).collect();
    assert!(conditioned.len() >= 9, "the Design-aware events are in the catalog: {}", conditioned.len());
    for (design, scenario, facts) in designs() {
        let seen = seen_over_seeds(&c, &design, scenario);
        for e in &conditioned {
            let need = only_of(e).unwrap();
            if !facts.contains(&need.as_str()) {
                assert!(!seen.contains(&e.id), "{} needs {need}, but happened in a Run of the {scenario} Design", e.id);
            }
        }
    }
}

#[test]
fn an_event_that_needs_a_choice_does_appear_when_the_design_has_it() {
    let c = with_new_events(load_catalog().unwrap(), 8, 8);
    for (design, scenario, facts) in designs() {
        let seen = seen_over_seeds(&c, &design, scenario);
        for e in c.events.iter().filter(|e| e.only.is_some()) {
            if facts.contains(&only_of(e).unwrap().as_str()) {
                assert!(seen.contains(&e.id), "{} never happened in 60 Runs of the {scenario} Design", e.id);
            }
        }
    }
}

#[test]
fn the_shipped_design_aware_events_are_all_there() {
    let c = load_catalog().unwrap();
    for id in [
        "micrometeoroid-through-a-panel", "bit-flipped-by-a-cosmic-ray", "a-bad-week-over-the-south-atlantic",
        "geomagnetic-storm-puffs-up-the-atmosphere", "atomic-oxygen-eats-the-coating", "static-discharge-on-the-hull",
        "the-moon-blinds-the-star-tracker", "a-panel-did-not-open-all-the-way", "storage-full-of-debug-logs",
        "a-reaction-wheel-starts-to-vibrate", "the-lens-cover-froze-shut", "condensation-on-the-lens",
        "radio-amateur-jams-the-frequency", "a-space-whale-swims-past", "a-busload-of-alien-tourists",
        "radio-amateurs-pick-up-your-signal", "a-rare-sight-lands-in-frame", "a-photo-goes-viral",
    ] {
        let e = c.events.iter().find(|e| e.id == id).unwrap_or_else(|| panic!("{id} missing"));
        assert!(
            e.only.is_some() || e.more_likely.is_some() || e.mitigation.is_some() || e.effects.iter().any(|f| f.when.is_some()),
            "{id} has no Design field"
        );
    }
}

// ---------------------------------------------------------------- "more likely"

#[test]
fn a_more_likely_condition_raises_how_often_the_event_is_planned() {
    // Two events and room for one: the favoured one is picked about 80% of the time with the fact, 50% without.
    let toml = format!("{}{}", event("favoured", "more_likely = { fact = \"camera\", weight = 4 }"), event("plain", ""));
    let c = catalog_with_events(&toml, 1, 1);
    let with_fact = (0..400).filter(|&s| ids(&run(&c, &crop_watch_known_good(), CROP_WATCH, s)) == ["favoured"]).count();
    let without = (0..400)
        .filter(|&s| ids(&run(&c, &space_weather_sentinel_known_good(), SPACE_WEATHER_SENTINEL, s)) == ["favoured"])
        .count();
    assert!(with_fact > without + 60, "with the fact {with_fact}, without {without}, of 400");
    assert!((140..260).contains(&without), "without the fact it is a coin toss: {without}");
}

// ---------------------------------------------------------------- mitigation

#[test]
fn a_mitigated_event_is_in_the_log_with_its_own_text_and_does_nothing() {
    let toml = event("bit-flip", "mitigation = { fact = \"computer-flight-proven\", scale = 0.0, log = \"The Flight-proven computer shrugged it off.\" }");
    let c = catalog_with_events(&toml, 1, 1);
    let mut none = c.clone();
    none.events.clear();
    let (proven, basic) = (crop_watch_known_good(), basic_computer());
    let mut checked = 0;
    for seed in 0..30 {
        let r = run(&c, &proven, CROP_WATCH, seed);
        let events = new_events(&r);
        if events.is_empty() {
            continue; // the Run ended before the event's day
        }
        assert_eq!(events.len(), 1, "seed {seed}");
        assert_eq!(events[0].text, "The Flight-proven computer shrugged it off.");
        assert!(events[0].catalog.as_ref().unwrap().softened);
        assert_eq!(final_science(&r), final_science(&run(&none, &proven, CROP_WATCH, seed)), "seed {seed}: no effect");

        let r = run(&c, &basic, CROP_WATCH, seed);
        let events = new_events(&r);
        if events.is_empty() {
            continue; // the Basic computer failed before the event's day
        }
        checked += 1;
        assert_eq!(events[0].text, "Log of bit-flip.");
        assert!(!events[0].catalog.as_ref().unwrap().softened);
        assert!(final_science(&r) < final_science(&run(&none, &basic, CROP_WATCH, seed)), "seed {seed}: Instruments off costs Science Value");
    }
    assert!(checked >= 10, "only {checked} Runs reached the event");
}

#[test]
fn a_mitigation_with_a_scale_reduces_the_effect() {
    let toml = event("wear", "mitigation = { fact = \"computer-flight-proven\", scale = 0.5, log = \"Softened.\" }")
        .replace("{ mechanic = \"instruments-off\", days = 2 }", "{ mechanic = \"data\", strength = -0.8, to_end = true }");
    let c = catalog_with_events(&toml, 1, 1);
    let mut none = c.clone();
    none.events.clear();
    // The same Design twice, differing only in the fact: compare with and without the event on each.
    let loss = |design: &Design, seed| {
        let r = run(&c, design, CROP_WATCH, seed);
        (!new_events(&r).is_empty()).then(|| final_science(&run(&none, design, CROP_WATCH, seed)) - final_science(&r))
    };
    let (mut checked, mut sum) = (0, (0.0, 0.0));
    for seed in 0..30 {
        if let (Some(soft), Some(hard)) = (loss(&crop_watch_known_good(), seed), loss(&basic_computer(), seed)) {
            checked += 1;
            sum = (sum.0 + soft, sum.1 + hard);
        }
    }
    assert!(checked >= 10, "only {checked} Runs reached the event");
    // The event does 80% of Data loss without the fact and 40% with it, so about half the loss.
    assert!(sum.0 > 0.0 && sum.0 < sum.1 * 0.75, "soft {}, hard {}", sum.0, sum.1);
}

// ---------------------------------------------------------------- the Debrief

#[test]
fn the_debrief_names_the_design_choice_behind_each_conditioned_or_mitigated_event() {
    let toml = format!(
        "{}{}",
        event("needs-low", "only = \"low-orbit\""),
        event("bit-flip", "mitigation = { fact = \"computer-flight-proven\", scale = 0.0, log = \"Shrugged off.\" }")
    );
    let c = catalog_with_events(&toml, 2, 2);
    let r = run(&c, &crop_watch_known_good(), CROP_WATCH, 1);
    let notes = &r.debrief.design_notes;
    assert_eq!(notes.len(), 2, "{notes:?}");
    let low = notes.iter().find(|n| n.contains("Title of needs-low")).unwrap();
    assert!(low.contains("a low Orbit"), "{low}");
    let flip = notes.iter().find(|n| n.contains("Title of bit-flip")).unwrap();
    assert!(flip.contains("a Flight-proven computer"), "{flip}");

    // Without the choice, the Debrief says which choice would have prevented or softened it.
    let notes = run(&c, &basic_computer(), CROP_WATCH, 1).debrief.design_notes;
    assert_eq!(notes.len(), 2, "{notes:?}");
    let flip = notes.iter().find(|n| n.contains("Title of bit-flip")).unwrap();
    assert!(flip.contains("a Flight-proven computer would have prevented it"), "{flip}");
    let toml = format!(
        "[[events]]\nid = \"glare\"\ngroup = \"real\"\ntitle = \"Glare\"\nlog = \"Glare.\"\nscene = \"hit\"\nreal_life = \"It happens.\"\n\
         mitigation = {{ fact = \"computer-flight-proven\", scale = 0.5, log = \"Half the glare.\" }}\n\
         effects = [{{ mechanic = \"data\", strength = -0.4, days = 2 }}]\n"
    );
    let c = catalog_with_events(&toml, 1, 1);
    let notes = run(&c, &basic_computer(), CROP_WATCH, 1).debrief.design_notes;
    assert!(notes.len() == 1 && notes[0].contains("a Flight-proven computer would have softened it"), "{notes:?}");
}

#[test]
fn a_more_likely_event_is_named_in_the_debrief_only_when_the_fact_held() {
    let c = catalog_with_events(&event("leaky", "more_likely = { fact = \"uhf-radio\", weight = 4 }"), 1, 1);
    let r = run(&c, &crop_watch_known_good(), CROP_WATCH, 1);
    assert_eq!(ids(&r), ["leaky"]);
    assert!(r.debrief.design_notes.is_empty(), "S-band is not UHF: {:?}", r.debrief.design_notes);
    let mut uhf = crop_watch_known_good();
    uhf.comms = Some(Pick::new("comms-uhf", ReliabilityTier::FlightProven));
    let r = run(&c, &uhf, CROP_WATCH, 1);
    assert_eq!(r.debrief.design_notes.len(), 1);
    assert!(r.debrief.design_notes[0].contains("a UHF radio"), "{:?}", r.debrief.design_notes);
}

// ---------------------------------------------------------------- the count

#[test]
fn a_run_gets_as_many_events_as_are_eligible_when_conditions_shrink_the_pool() {
    let toml = format!(
        "{}{}{}{}",
        event("a", "only = \"low-orbit\""),
        event("b", "only = \"high-orbit\""),
        event("c", "only = \"high-orbit\""),
        event("d", "")
    );
    let c = catalog_with_events(&toml, 3, 3);
    for seed in 0..20 {
        let low = ids(&run(&c, &crop_watch_known_good(), CROP_WATCH, seed));
        assert_eq!(low.len(), 2, "a and d only: {low:?}");
        let high = ids(&run(&c, &connect_the_villages_known_good(), CONNECT_THE_VILLAGES, seed));
        assert_eq!(high.len(), 3, "b, c and d: {high:?}");
    }
    let wants_four = catalog_with_events(&toml, 4, 4);
    assert_eq!(ids(&run(&wants_four, &crop_watch_known_good(), CROP_WATCH, 0)).len(), 2);
}
