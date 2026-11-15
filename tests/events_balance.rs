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

// ---------------------------------------------------------------- every Scenario's Runs

#[test]
fn every_scenario_gives_a_run_5_to_8_new_events() {
    let c = load_catalog().unwrap();
    for s in &c.scenarios {
        assert_eq!((s.events.new_events_min, s.events.new_events_max), (5, 8), "{}", s.id);
    }
}

/// The reference Design of a Scenario: its known-good Design from `tests/common`.
fn reference(scenario: &str) -> Design {
    match scenario {
        CROP_WATCH => crop_watch_known_good(),
        CONNECT_THE_VILLAGES => connect_the_villages_known_good(),
        SPACE_WEATHER_SENTINEL => space_weather_sentinel_known_good(),
        AMAZON_WATCH => amazon_watch_known_good(),
        GLACIER_WATCH => glacier_watch_known_good(),
        JAPAN_DISASTER_RESPONSE => japan_disaster_response_known_good(),
        CYCLONE_WATCH => cyclone_watch_known_good(),
        other => panic!("Scenario '{other}' has no reference Design: add its known-good Design to `reference` in tests/events_balance.rs"),
    }
}

/// Every Scenario of the shipped catalog with its reference Design.
fn references(c: &Catalog) -> Vec<(String, Design)> {
    c.scenarios.iter().map(|s| (s.id.clone(), reference(&s.id))).collect()
}

/// The new Random Events of a Run, with their days.
fn new_events(r: &RunResult) -> Vec<(u32, &RandomEvent)> {
    r.timeline.iter().flat_map(|t| t.events.iter().filter(|e| e.kind == EventKind::Catalog).map(move |e| (t.day, e))).collect()
}

/// A small deterministic generator, so the sweep is the same on every machine.
struct Lcg(u64);

impl Lcg {
    fn below(&mut self, n: usize) -> usize {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        ((self.0 >> 33) as usize) % n
    }
}

/// Random launchable Designs, a few in every Scenario of the catalog.
fn launchable_designs(c: &Catalog, per_scenario: usize) -> Vec<(String, Design)> {
    let tiers = [ReliabilityTier::Basic, ReliabilityTier::FlightProven];
    let mut l = Lcg(11);
    let mut out = Vec::new();
    for s in &c.scenarios {
        let mut found = 0;
        for _ in 0..50_000 {
            if found == per_scenario {
                break;
            }
            let mut instruments: Vec<String> =
                (0..1 + l.below(3)).map(|_| c.instruments[l.below(c.instruments.len())].id.clone()).collect();
            instruments.sort();
            instruments.dedup();
            let design = Design {
                instruments,
                structure: Some(c.structures[l.below(c.structures.len())].id.clone()),
                solar: Some(Pick::new(&c.solar_panels[l.below(c.solar_panels.len())].id, tiers[l.below(2)])),
                battery: Some(Pick::new(&c.batteries[l.below(c.batteries.len())].id, tiers[l.below(2)])),
                attitude: Some(Pick::new(&c.attitude_controls[l.below(c.attitude_controls.len())].id, tiers[l.below(2)])),
                comms: Some(Pick::new(&c.comms[l.below(c.comms.len())].id, tiers[l.below(2)])),
                computer_tier: tiers[l.below(2)],
                orbit: Some(c.orbits[l.below(c.orbits.len())].id.clone()),
                launch_vehicle: Some(c.launch_vehicles[l.below(c.launch_vehicles.len())].id.clone()),
            };
            if launch_review(c, &design, &s.id).can_launch {
                found += 1;
                out.push((s.id.clone(), design));
            }
        }
        assert_eq!(found, per_scenario, "{}: too few launchable Designs found", s.id);
    }
    out
}

#[test]
fn every_run_gets_5_to_8_new_events_3_or_4_with_a_question_and_none_ends_the_mission() {
    let on = load_catalog().unwrap();
    let off = legacy_catalog();
    let mut flown = references(&on);
    flown.extend(launchable_designs(&on, 12));
    for (scenario, design) in &flown {
        for seed in 0..15 {
            let at = format!("{scenario} seed {seed} {design:?}");
            let (r, before) = (run(&on, design, scenario, seed), run(&off, design, scenario, seed));
            // A new event never ends a Run, nor ends it sooner ("Quiet Sun" may let it last longer).
            assert!(r.timeline.len() >= before.timeline.len(), "{at}: the Run ended sooner");
            assert!(new_events(&r).iter().all(|(_, e)| !e.text.contains("Mission ends")), "{at}");
            // Events are planned over the whole Run; one cut short by a failure meets only the earlier ones.
            if before.timeline.len() < r.timeline.len() || !r.debrief.stats.lifetime_met {
                continue;
            }
            let events = new_events(&r);
            assert!((5..=8).contains(&events.len()), "{at}: {} new events", events.len());
            let asked = events.iter().filter(|(_, e)| e.prompt.is_some()).count();
            assert!((3..=4).contains(&asked), "{at}: {asked} questions");
        }
    }
}

#[test]
fn ten_runs_show_most_of_the_pool_a_design_can_meet() {
    let c = load_catalog().unwrap();
    let ids = |seeds: std::ops::Range<u64>, design: &Design, scenario: &str| -> BTreeSet<String> {
        seeds
            .flat_map(|seed| {
                let r = run(&c, design, scenario, seed);
                new_events(&r).into_iter().map(|(_, e)| e.catalog.as_ref().unwrap().id.clone()).collect::<Vec<_>>()
            })
            .collect()
    };
    for (scenario, design) in references(&c) {
        let possible = ids(0..300, &design, &scenario);
        let ten = ids(100..110, &design, &scenario);
        eprintln!("{scenario}: ten Runs show {} of the {} events this Design can meet", ten.len(), possible.len());
        assert!(possible.len() >= 40, "{scenario}: only {} events possible", possible.len());
        assert!(ten.len() * 2 > possible.len(), "{scenario}: ten Runs show only {} of {}", ten.len(), possible.len());
    }
}

// ---------------------------------------------------------------- the Debrief names them

#[test]
fn a_new_event_that_cost_science_value_is_named_among_the_causes() {
    let c = catalog_with_events(
        r#"
[[events]]
id = "long-nap"
group = "fantasy"
title = "A long nap"
log = "The spacecraft took a long nap: Instruments off for 5 days."
scene = "visitor"
effects = [{ mechanic = "instruments-off", days = 5 }]
"#,
        1,
        1,
    );
    let design = space_weather_sentinel_known_good();
    // A seed whose built-in events stay clear of the nap (an outage during it would explain some of its days).
    let seed = (0..100)
        .find(|&seed| {
            let r = run(&c, &design, SPACE_WEATHER_SENTINEL, seed);
            let Some(&(day, _)) = new_events(&r).first() else { return false };
            r.timeline.iter().all(|t| t.day + 3 < day || t.day > day + 8 || t.events.iter().all(|e| e.kind == EventKind::Catalog))
        })
        .expect("a calm seed");
    let r = run(&c, &design, SPACE_WEATHER_SENTINEL, seed);
    let (day, _) = new_events(&r)[0];
    let cause = r.debrief.causes.iter().find(|c| c.kind == CauseKind::RandomEvents).expect("a Random Events cause");
    assert!(cause.text.contains(&format!("A long nap (day {day})")), "{}", cause.text);
    assert!(cause.text.contains("Science Value"), "{}", cause.text);
    assert_eq!(cause.day, Some(day));
    // Without the event there is no such cause.
    let quiet = run(&legacy_catalog(), &design, SPACE_WEATHER_SENTINEL, seed);
    assert!(!quiet.debrief.causes.iter().any(|c| c.text.contains("A long nap")));
}

/// The titles the Random Events cause of a Run names.
fn blamed(c: &Catalog, r: &RunResult) -> Vec<String> {
    let cause = r.debrief.causes.iter().filter(|c| c.kind == CauseKind::RandomEvents).map(|c| c.text.as_str()).collect::<Vec<_>>().join(" ");
    c.events.iter().filter(|e| cause.contains(&format!("{} (day", e.title))).map(|e| e.title.clone()).collect()
}

#[test]
fn the_debrief_never_blames_a_lucky_event_or_one_a_design_choice_shrugged_off() {
    let c = load_catalog().unwrap();
    // Seed 71 met "A neighbouring station gives away passes" (lucky), seed 32 a bit flip the Flight-proven computer
    // corrected; both were once named as costing Science Value.
    let r = run(&c, &crop_watch_known_good(), CROP_WATCH, 71);
    assert!(new_events(&r).iter().any(|(_, e)| e.catalog.as_ref().unwrap().id == "neighbouring-station-gives-away-passes"));
    assert!(!blamed(&c, &r).iter().any(|t| t.contains("neighbouring station")), "{:?}", r.debrief.causes);
    let r = run(&c, &crop_watch_known_good(), CROP_WATCH, 32);
    assert!(new_events(&r).iter().any(|(_, e)| e.catalog.as_ref().unwrap().softened));
    assert!(!blamed(&c, &r).iter().any(|t| t.contains("cosmic ray")), "{:?}", r.debrief.causes);
    // And over many Runs: only events that could cost Science Value are ever named.
    for (scenario, design) in references(&c) {
        for seed in 0..100 {
            let r = run(&c, &design, &scenario, seed);
            for title in blamed(&c, &r) {
                let (_, e) = new_events(&r).into_iter().find(|(_, e)| c.events.iter().any(|s| s.title == title && Some(&s.id) == e.catalog.as_ref().map(|k| &k.id))).unwrap();
                let k = e.catalog.as_ref().unwrap();
                assert!(k.group != EventGroup::Lucky && !k.softened, "{scenario} seed {seed}: blames {title}: {}", e.text);
            }
        }
    }
}

#[test]
fn the_debrief_does_not_charge_an_event_for_an_outage_or_a_component_failure() {
    // A real event that costs almost nothing: the downlink 1% lower for a day. Days of an outage after it are the
    // outage's, not the event's.
    let c = catalog_with_events(
        r#"
[[events]]
id = "small-hiccup"
group = "real"
title = "A small hiccup"
log = "A small hiccup: the downlink was 1% lower for 1 day."
scene = "link-interference"
real_life = "It happens."
effects = [{ mechanic = "downlink", strength = -0.01, days = 1 }]
"#,
        1,
        1,
    );
    let design = crop_watch_known_good();
    let mut checked = 0;
    for seed in 0..400 {
        let r = run(&c, &design, CROP_WATCH, seed);
        // A Run a failure ended early may not reach the event.
        let Some(&(day, _)) = new_events(&r).first() else { continue };
        let outage_soon = r.timeline.iter().any(|t| {
            t.day >= day.saturating_sub(2) && t.day <= day + 2 && t.events.iter().any(|e| e.kind == EventKind::GroundStationOutage && e.choice.as_deref() != Some("buy-station-time"))
        });
        if !outage_soon {
            continue;
        }
        checked += 1;
        assert!(blamed(&c, &r).is_empty(), "seed {seed}: {:?}", r.debrief.causes);
    }
    assert!(checked > 0, "no seed puts an outage beside the event");
    // Nor for what a Component that failed shortly before took away.
    let basic = |p: Option<Pick>| p.map(|p| Pick::new(&p.id, ReliabilityTier::Basic));
    let d = crop_watch_known_good();
    let design = Design { solar: basic(d.solar.clone()), comms: basic(d.comms.clone()), attitude: basic(d.attitude.clone()), ..d };
    let mut checked = 0;
    for seed in 0..2000 {
        let r = run(&c, &design, CROP_WATCH, seed);
        let Some(&(day, _)) = new_events(&r).first() else { continue };
        let failed_before = r.timeline.iter().any(|t| {
            t.day + 20 >= day && t.day < day && t.events.iter().any(|e| e.kind == EventKind::ComponentFailure)
        });
        if !failed_before || (r.timeline.len() as u32) < day + 5 {
            continue;
        }
        checked += 1;
        assert!(blamed(&c, &r).is_empty(), "seed {seed}: {:?}", r.debrief.causes);
    }
    assert!(checked > 0, "no seed puts a failure shortly before the event");
}

// ---------------------------------------------------------------- balance

/// The player answers every new event's question with the option that is not the default (where affordable).
fn against_the_default(c: &Catalog, design: &Design, scenario: &str, seed: u64) -> RunResult {
    let mut decisions: Vec<Decision> = Vec::new();
    let mut r = run(c, design, scenario, seed);
    loop {
        let next = new_events(&r)
            .into_iter()
            .filter_map(|(_, e)| e.prompt.clone())
            .find(|p| !decisions.iter().any(|d| d.prompt_id == p.id));
        let Some(p) = next else { return r };
        let pick = p.options.iter().find(|o| o.id != p.default && o.available).unwrap_or(&p.options[0]);
        decisions.push(Decision { prompt_id: p.id.clone(), choice: pick.id.clone() });
        r = run_with_decisions(c, design, scenario, seed, &decisions);
    }
}

#[test]
fn new_events_move_science_value_by_about_a_tenth_at_most_and_leave_success_to_the_design() {
    let on = load_catalog().unwrap();
    let off = legacy_catalog();
    const SEEDS: u64 = 200;
    let mut over = Vec::new();
    for (scenario, design) in references(&on) {
        let (mut wins_on, mut wins_off, mut wins_against) = (0, 0, 0);
        let (mut sum, mut sum_against, mut worst) = (0.0, 0.0, 0.0f64);
        for seed in 0..SEEDS {
            let before = run(&off, &design, &scenario, seed);
            let r = run(&on, &design, &scenario, seed);
            let against = against_the_default(&on, &design, &scenario, seed);
            let base = before.debrief.stats.science_value;
            let shift = r.debrief.stats.science_value / base - 1.0;
            sum += shift;
            sum_against += against.debrief.stats.science_value / base - 1.0;
            worst = if shift.abs() > worst.abs() { shift } else { worst };
            if shift.abs() > 0.10 {
                over.push(format!("{scenario} seed {seed}: new events moved Science Value by {:+.1}%", shift * 100.0));
            }
            wins_off += usize::from(before.debrief.success);
            wins_on += usize::from(r.debrief.success);
            wins_against += usize::from(against.debrief.success);
        }
        let n = SEEDS as f64;
        let (mean, mean_against) = (sum / n, sum_against / n);
        eprintln!(
            "{scenario}: Science Value shift mean {:+.1}% (worst Run {:+.1}%), answering against the default {:+.1}%; \
             success {wins_off}/{SEEDS} off, {wins_on}/{SEEDS} on, {wins_against}/{SEEDS} on against the default",
            mean * 100.0,
            worst * 100.0,
            mean_against * 100.0
        );
        assert!(mean.abs() <= 0.05 && mean_against.abs() <= 0.05, "{scenario}: mean shift {mean:+.3} / {mean_against:+.3}");
        for wins in [wins_on, wins_against] {
            assert!(wins.abs_diff(wins_off) * 20 <= SEEDS as usize, "{scenario}: success {wins} with new events, {wins_off} without");
        }
    }
    assert!(over.is_empty(), "{}", over.join("\n"));
}
