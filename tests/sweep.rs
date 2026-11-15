//! Properties that must hold for every Run, checked over a deterministic sweep of random launchable Designs in
//! every Scenario and several seeds, answering every interactive question the way the UI does (one at a time).
//! All through the public `sim` interface.
mod common;

use common::*;
use spaceooops::sim::*;
use ReliabilityTier::{Basic, FlightProven};

/// A small deterministic generator, so the sweep is the same on every machine.
struct Lcg(u64);

impl Lcg {
    fn below(&mut self, n: usize) -> usize {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        ((self.0 >> 33) as usize) % n
    }
}

/// One flown Run of the sweep, with the inputs that produced it.
struct Flown {
    scenario: String,
    design: Design,
    seed: u64,
    result: RunResult,
}

/// Random launchable Designs in every Scenario, each flown with a few seeds: once with the defaults and once
/// answering every question at random, one question at a time.
fn sweep() -> Vec<Flown> {
    let c = load_catalog().unwrap();
    let mut l = Lcg(7);
    let tiers = [Basic, FlightProven];
    let mut out = Vec::new();
    let mut launchable = 0;
    while launchable < 600 {
        let sc = c.scenarios[l.below(c.scenarios.len())].id.clone();
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
        if !launch_review(&c, &design, &sc).can_launch {
            continue;
        }
        launchable += 1;
        for seed in 0..3u64 {
            let first = run(&c, &design, &sc, seed);
            let mut decisions: Vec<Decision> = Vec::new();
            let mut answered = first.clone();
            loop {
                let next = answered
                    .timeline
                    .iter()
                    .flat_map(|d| d.events.iter())
                    .filter_map(|e| e.prompt.as_ref())
                    .find(|p| !decisions.iter().any(|d| d.prompt_id == p.id))
                    .cloned();
                let Some(p) = next else { break };
                let pick = &p.options[l.below(p.options.len())];
                decisions.push(Decision { prompt_id: p.id.clone(), choice: pick.id.clone() });
                answered = run_with_decisions(&c, &design, &sc, seed, &decisions);
            }
            out.push(Flown { scenario: sc.clone(), design: design.clone(), seed, result: first });
            out.push(Flown { scenario: sc.clone(), design: design.clone(), seed, result: answered });
        }
    }
    out
}

// ---------------------------------------------------------------- causes

#[test]
fn every_failed_run_has_a_cause_that_explains_why_it_failed() {
    for f in sweep() {
        let d = &f.result.debrief;
        if d.success {
            continue;
        }
        let at = format!("{} seed {} {:?}", f.scenario, f.seed, f.design);
        assert!(!d.causes.is_empty(), "{at}: a failed Run with no cause");
        if !d.stats.lifetime_met {
            assert!(
                d.causes.iter().any(|c| matches!(c.kind, CauseKind::ComponentFailure | CauseKind::LifetimeShortfall)),
                "{at}: the Run ended early but no cause says why: {:#?}",
                d.causes
            );
        }
        if !d.stats.science_met {
            assert!(
                d.causes.iter().any(|c| c.kind != CauseKind::LifetimeShortfall),
                "{at}: the Science Value was missed but no cause says why: {:#?}",
                d.causes
            );
        }
    }
}

/// A critical failure a few days before the minimum Lifetime costs little Science Value, but it is still the
/// reason the Mission failed, so it is reported.
#[test]
fn a_critical_failure_just_before_the_minimum_lifetime_is_still_reported() {
    let c = load_catalog().unwrap();
    let mut design = crop_watch_known_good();
    design.computer_tier = Basic;
    design.comms = Some(Pick::new("comms-s-band", Basic));
    design.attitude = Some(Pick::new("reaction-wheels", Basic));
    let mut found = 0;
    for seed in 0..3000 {
        let d = run(&c, &design, CROP_WATCH, seed).debrief;
        let s = &d.stats;
        if s.days_survived + 15 < s.min_lifetime_days || s.lifetime_met {
            continue;
        }
        found += 1;
        let failed_on = s.days_survived + 1;
        assert!(
            d.causes.iter().any(|c| c.kind == CauseKind::ComponentFailure && c.day == Some(failed_on)),
            "seed {seed}: the failure on day {failed_on} is not reported: {:#?}",
            d.causes
        );
    }
    assert!(found > 0, "no seed ended just short of the minimum Lifetime");
}

/// The day a critical Component failed and the days the Mission survived are told the same way everywhere:
/// the Mission fails on day N, so N-1 full days were flown.
#[test]
fn the_failure_day_and_the_days_survived_agree_in_every_text() {
    for f in sweep() {
        let d = &f.result.debrief;
        let s = &d.stats;
        let ended = f.result.timeline.last().is_some_and(|day| {
            day.events.iter().any(|e| e.kind == EventKind::ComponentFailure && e.text.contains("Mission ends"))
        });
        if !ended {
            continue;
        }
        let at = format!("{} seed {} {:?}", f.scenario, f.seed, f.design);
        let failed_on = f.result.timeline.len() as u32;
        assert_eq!(s.days_survived, failed_on - 1, "{at}");
        let flown = format!("{} of the {} days", s.days_survived, s.min_lifetime_days);
        assert!(d.ending.contains(&format!("day {failed_on}")), "{at}: {}", d.ending);
        assert!(d.ending.contains(&format!("{} of the", s.days_survived)), "{at}: {}", d.ending);
        let penalty = d.score.lines.iter().find(|l| l.kind == ScoreLineKind::Penalty).expect("an early-end line");
        assert!(penalty.label.contains(&flown), "{at}: {}", penalty.label);
        assert!(!penalty.label.contains(&format!("day {}", s.days_survived)), "{at}: {}", penalty.label);
        if let Some(c) = d.causes.iter().find(|c| c.kind == CauseKind::ComponentFailure && c.day == Some(failed_on)) {
            assert!(c.text.contains(&format!("day {failed_on}")), "{at}: {}", c.text);
            assert!(c.text.contains(&flown), "{at}: {}", c.text);
        }
    }
}

/// The battery cause follows what happened: it only blames deep discharges when the battery was drained past the
/// healthy depth, and the battery it suggests is a bigger one than the player's.
#[test]
fn the_battery_cause_matches_the_discharge_and_suggests_a_bigger_battery() {
    let c = load_catalog().unwrap();
    let healthy = c.review.battery_max_depth_of_discharge * 100.0;
    let biggest = c.batteries.iter().map(|b| b.capacity_wh).fold(0.0, f64::max);
    let mut seen = 0;
    for f in sweep() {
        for cause in f.result.debrief.causes.iter().filter(|c| c.kind == CauseKind::BatteryTooSmall) {
            seen += 1;
            let own = c.batteries.iter().find(|b| Some(&b.id) == f.design.battery.as_ref().map(|p| &p.id)).unwrap();
            let text = &cause.text;
            assert_eq!(text.matches(own.name.as_str()).count(), 1, "names only the player's battery as the problem: {text}");
            for other in c.batteries.iter().filter(|b| text.contains(b.name.as_str()) && b.id != own.id) {
                assert!(other.capacity_wh > own.capacity_wh, "suggests a smaller battery: {text}");
            }
            if own.capacity_wh < biggest {
                assert!(c.batteries.iter().any(|b| b.id != own.id && text.contains(b.name.as_str())), "names a bigger battery: {text}");
            } else {
                assert!(text.contains("Instruments"), "says what else to change: {text}");
            }
            if let Some(depth) = text.split("drained ").nth(1).and_then(|r| r.split('%').next()).and_then(|n| n.parse::<f64>().ok())
                && depth <= healthy {
                    assert!(!text.contains("wore out fast") && !text.contains("so it wore out"), "{text}");
                }
        }
    }
    assert!(seen > 0, "the sweep has battery causes");
}

/// A battery drained within the healthy depth every orbit still wears a little each time; when that slow wear
/// finally leaves it short, the cause says so instead of blaming a deep discharge.
#[test]
fn slow_wear_within_the_healthy_depth_is_told_as_slow_wear() {
    let mut found = 0;
    for f in sweep() {
        for cause in f.result.debrief.causes.iter().filter(|c| c.kind == CauseKind::BatteryTooSmall) {
            if cause.text.contains("within the healthy") {
                found += 1;
                assert!(cause.text.contains("wear"), "{}", cause.text);
            }
        }
    }
    assert!(found > 0, "the sweep has a battery that wore out slowly within the healthy depth");
}

// ---------------------------------------------------------------- texts

/// Every problem found in one player-facing text.
fn malformed(text: &str) -> Vec<String> {
    let mut found = Vec::new();
    if text.trim().is_empty() {
        found.push("empty text".to_string());
    }
    for bad in [",.", "..", "  ", " ,", " .", ",,", "()", "{", "}", "NaN", "inf ", ": .", "the .", " a ."] {
        if text.contains(bad) {
            found.push(format!("contains {bad:?}"));
        }
    }
    if text != text.trim() {
        found.push("leading or trailing space".to_string());
    }
    // "1 days", "1 minutes", "0 days" left over from a number dropped into a sentence.
    let words: Vec<&str> = text.split_whitespace().collect();
    for pair in words.windows(2) {
        let number = pair[0].trim_start_matches(['(', '~']);
        let unit = pair[1].trim_end_matches([',', '.', ')', ';', ':']);
        let plural = ["days", "minutes", "orbits", "flares", "Components", "hours", "months"].contains(&unit);
        if number == "1" && plural {
            found.push(format!("plural slip {:?}", pair.join(" ")));
        }
    }
    if text.contains("last 0 days") || text.contains("for 0 days") {
        found.push("zero days".to_string());
    }
    // Percentages in a Run's texts are shares of something whole: never above 100%.
    for word in text.split(|c: char| c.is_whitespace() || c == '(') {
        if let Some(n) = word.split('%').next().filter(|_| word.contains('%')).and_then(|n| n.parse::<f64>().ok())
            && n > 100.0 {
                found.push(format!("over 100%: {word}"));
            }
    }
    found
}

#[test]
fn player_facing_texts_of_every_run_are_well_formed() {
    let mut problems: Vec<String> = Vec::new();
    let mut check = |what: &str, text: &str| {
        for p in malformed(text) {
            if problems.len() < 30 {
                problems.push(format!("[{what}] {p}: {text}"));
            }
        }
    };
    for f in sweep() {
        let d = &f.result.debrief;
        check("ending", &d.ending);
        for c in &d.causes {
            check("cause", &c.text);
        }
        for n in &d.decisions {
            check("decision", n);
        }
        for o in &d.objectives {
            check("objective", &o.detail);
        }
        for l in &d.score.lines {
            check("score", &l.label);
        }
        for day in &f.result.timeline {
            for e in &day.events {
                check("event", &e.text);
                if let Some(p) = &e.prompt {
                    check("question", &p.question);
                    for o in &p.options {
                        check("option", &o.label);
                        check("consequence", &o.consequence);
                    }
                }
            }
        }
    }
    assert!(problems.is_empty(), "{}", problems.join("\n"));
}

/// The reviewer's repro: the store-and-forward messenger in GEO flies all 365 days, reaches almost no Science
/// Value, passes Launch Review cleanly, and must say why it failed.
#[test]
fn a_store_and_forward_messenger_in_geo_explains_its_science_shortfall() {
    let c = load_catalog().unwrap();
    let mut design = connect_the_villages_known_good();
    design.instruments = vec!["store-and-forward".to_string()];
    for seed in 0..20 {
        let d = run(&c, &design, CONNECT_THE_VILLAGES, seed).debrief;
        assert!(!d.success, "seed {seed}");
        assert!(!d.stats.science_met, "seed {seed}");
        assert!(
            d.causes.iter().any(|c| matches!(
                c.kind,
                CauseKind::ScienceShortfall | CauseKind::PoorInstrumentFit | CauseKind::PoorCoverage
            )),
            "seed {seed}: no cause explains the missing Science Value: {:#?}",
            d.causes
        );
    }
}
