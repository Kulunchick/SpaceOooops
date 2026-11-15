//! The Run plan of the new Random Events: how many, which ones and on which days, drawn once when the Run
//! starts from a random stream of their own, so the Run's main stream (flares, outages, failures) keeps every
//! draw it had before.
//!
//! Each catalog event has its own stream derived from the seed and its id, and always takes the same draws,
//! so adding, removing or reordering events never moves another event's pick, day or gamble roll.
use super::design_facts::Facts;
use super::event_catalog::EventSpec;
use super::rng::Rng;

const SALT: u64 = 0x6E65_775F_6576_7473;

/// The new events of one Run, sorted by day.
#[derive(Debug, Clone, PartialEq, Default)]
pub(crate) struct Plan {
    pub events: Vec<Planned>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Planned {
    /// 1-based Run day.
    pub day: u32,
    /// Index into `Catalog::events`.
    pub event: usize,
    /// One uniform roll per question option, for a gamble: drawn now, before any answer.
    pub rolls: [f64; 2],
}

/// Plans `min..=max` events (fewer if the Run or the eligible pool is shorter), at most `interactive target`
/// (3 or 4) of them with a question, on distinct days spread over `planned_days`.
pub(crate) fn plan(events: &[EventSpec], min: u32, max: u32, planned_days: u32, seed: u64, facts: &Facts) -> Plan {
    let mut run_wide = Rng::new(Rng::new(seed ^ SALT).next_u64());
    let max = max.max(min);
    let count_draw = run_wide.next_f64();
    let target_draw = run_wide.next_f64();
    let n = (min + (count_draw * f64::from(max - min + 1)) as u32).min(max).min(planned_days) as usize;
    let target = if target_draw < 0.5 { 3 } else { 4 };
    if n == 0 {
        return Plan::default();
    }

    struct Candidate {
        event: usize,
        key: f64,
        slot: f64,
        day: f64,
        rolls: [f64; 2],
        question: bool,
    }
    let mut candidates: Vec<Candidate> = Vec::new();
    for (i, e) in events.iter().enumerate() {
        let mut rng = Rng::new(Rng::new(seed ^ SALT ^ fnv1a64(&e.id)).next_u64());
        let keys = [rng.next_f64(), rng.next_f64(), rng.next_f64(), rng.next_f64()];
        let slot = rng.next_f64();
        let day = rng.next_f64();
        let rolls = [rng.next_f64(), rng.next_f64()];
        if e.only.is_some_and(|f| !facts.contains(&f)) {
            continue;
        }
        // The max of `w` uniform draws: picking by the largest key is then weighted sampling with weight `w`.
        let w = match &e.more_likely {
            Some(m) if facts.contains(&m.fact) => m.weight.clamp(1, 4) as usize,
            _ => 1,
        };
        let key = keys[..w].iter().copied().fold(0.0, f64::max);
        candidates.push(Candidate { event: i, key, slot, day, rolls, question: e.question.is_some() });
    }
    candidates.sort_by(|a, b| b.key.total_cmp(&a.key).then(a.event.cmp(&b.event)));
    let questions = candidates.iter().filter(|c| c.question).count();
    let k = target.min(n).min(questions);
    let mut chosen: Vec<&Candidate> = candidates.iter().filter(|c| c.question).take(k).collect();
    chosen.extend(candidates.iter().filter(|c| !c.question).take(n - k));

    // Spread over the Run: the chosen events, ordered by their slot draw, each get one equal stretch of days.
    chosen.sort_by(|a, b| a.slot.total_cmp(&b.slot).then(a.event.cmp(&b.event)));
    let (m, days) = (chosen.len() as u64, u64::from(planned_days));
    let mut planned: Vec<Planned> = chosen
        .iter()
        .enumerate()
        .map(|(i, c)| {
            let first = 1 + i as u64 * days / m;
            let last = (i as u64 + 1) * days / m;
            let span = last - first + 1;
            let day = first + ((c.day * span as f64) as u64).min(span - 1);
            Planned { day: day as u32, event: c.event, rolls: c.rolls }
        })
        .collect();
    planned.sort_by_key(|p| p.day);
    Plan { events: planned }
}

/// FNV-1a, 64 bit: a stable hash of an event id (the same on every platform and build).
fn fnv1a64(text: &str) -> u64 {
    text.bytes().fold(0xCBF2_9CE4_8422_2325, |h, b| (h ^ u64::from(b)).wrapping_mul(0x0000_0100_0000_01B3))
}
