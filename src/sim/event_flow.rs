//! A planned new Random Event on its day: which text and effects apply (the event's own, a Design
//! mitigation's, or the outcome of the option chosen for its question), the effects started, and the
//! Random Event for the timeline.
use super::decisions::{Decision, DecisionOption, DecisionPrompt, chosen};
use super::review::money;
use super::design_facts::Facts;
use super::event_catalog::{EffectSpec, EventSpec, Mechanic, Mitigation, OptionSpec, QuestionSpec};
use super::event_effects::{DayOutcome, EventState, Fired, scene_days, start};
use super::event_plan::Planned;
use super::events::{CatalogEvent, EventKind, RandomEvent};
use super::run::RunContext;

/// What the new events of one day add, for the `step` to apply.
#[derive(Debug, Default)]
pub(crate) struct DayFiring {
    pub events: Vec<RandomEvent>,
    /// US$ spent today, already capped at the Budget left.
    pub cost_usd: f64,
    /// Science Value earned today.
    pub science_bonus: f64,
}

/// Fires the events planned for `day`. `budget_left_usd` is the Budget left before today's events.
pub(crate) fn on_day(
    ctx: &RunContext,
    day: u32,
    budget_left_usd: f64,
    pool: &mut EventState,
    decisions: &[Decision],
) -> DayFiring {
    let mut firing = DayFiring::default();
    for planned in ctx.plan.events.iter().filter(|p| p.day == day) {
        let Some(spec) = ctx.event_specs.get(planned.event) else { continue };
        let mut out = DayOutcome::default();
        let mut event = fire(spec, planned, budget_left_usd - firing.cost_usd, pool, decisions, &mut out);
        let left = (budget_left_usd - firing.cost_usd).max(0.0);
        // A spend is capped at the Budget left, so the log must not claim more than was taken.
        if out.cost_usd > left + 0.5 {
            let took = if left < 0.5 {
                "The Budget was already spent, so it took nothing.".to_string()
            } else {
                format!("Only {} of the Budget was left, so that is all it took.", money(left))
            };
            event.text = format!("{} {took}", event.text.trim_end());
        }
        firing.cost_usd += out.cost_usd.min(left);
        firing.science_bonus += out.science_bonus_share * ctx.required_science_value;
        firing.events.push(event);
    }
    firing
}

fn fire(
    spec: &EventSpec,
    planned: &Planned,
    budget_left_usd: f64,
    pool: &mut EventState,
    decisions: &[Decision],
    out: &mut DayOutcome,
) -> RandomEvent {
    let day = planned.day;
    let mut text = spec.log.clone();
    let mut effects = spec.effects.clone();
    let (mut prompt, mut choice, mut option, mut by_player, mut good) = (None, None, None, false, true);
    if let Some(q) = &spec.question {
        let asked = ask(spec, q, day, budget_left_usd, decisions);
        if let Some(o) = q.options.get(asked.option) {
            let (log, applied, came_good) = outcome(o, planned.rolls[asked.option.min(1)]);
            (text, effects, good) = (log, applied, came_good);
            out.cost_usd += o.cost_usd;
            (prompt, choice, option, by_player) = (asked.prompt, Some(o.id.clone()), Some(asked.option), asked.by_player);
        }
    }
    effects.retain(|e| e.when.is_none_or(|f| pool.facts.contains(&f)));
    let softened = match mitigated(spec, &pool.facts) {
        Some(m) => {
            text = m.log.clone();
            effects.retain(|_| m.scale > 0.0);
            for e in effects.iter_mut().filter(|e| e.mechanic != Mechanic::InstrumentsOff) {
                e.strength *= m.scale;
            }
            true
        }
        None => false,
    };
    for effect in &effects {
        start(effect, day, pool, out);
    }
    pool.fired.push(Fired { day, spec: spec.clone(), option, by_player, good, softened });
    RandomEvent {
        kind: EventKind::Catalog,
        text,
        component: None,
        prompt,
        choice,
        catalog: Some(CatalogEvent {
            id: spec.id.clone(),
            group: spec.group,
            scene: spec.scene,
            model: spec.model.clone(),
            scene_days: scene_days(&effects),
            softened,
            can_cost_science: effects.iter().any(EffectSpec::can_cost_science),
        }),
    }
}

/// The answer to an event's question.
struct Asked {
    /// Shown to the player.
    prompt: Option<DecisionPrompt>,
    /// Index into the question's options.
    option: usize,
    by_player: bool,
}

/// The question as a `DecisionPrompt` (id `day-{day}-event-{event id}`, consequences from
/// `consequence`, an option the Budget left cannot pay for unavailable), and the player's answer by
/// `decisions::chosen`, or the default.
fn ask(spec: &EventSpec, q: &QuestionSpec, day: u32, budget_left_usd: f64, decisions: &[Decision]) -> Asked {
    let left = budget_left_usd.max(0.0);
    let options = q
        .options
        .iter()
        .map(|o| {
            let affordable = o.cost_usd <= left;
            DecisionOption {
                id: o.id.clone(),
                label: o.label.clone(),
                consequence: consequence(o, budget_left_usd),
                available: affordable,
                unavailable_reason: (!affordable).then(|| {
                    format!(
                        "Not enough Budget: this costs {}, but only {} of the Budget is left.",
                        money(o.cost_usd),
                        money(left)
                    )
                }),
            }
        })
        .collect();
    let prompt = DecisionPrompt {
        id: super::events::catalog_event_id(day, &spec.id),
        day,
        question: q.text.clone(),
        options,
        default: q.default.clone(),
    };
    let (id, by_player) = chosen(&prompt, decisions);
    let option = q.options.iter().position(|o| o.id == id).unwrap_or(0);
    Asked { prompt: Some(prompt), option, by_player }
}

/// An option's consequence as the player reads it: the data text, the price against the Budget
/// left, and for a gamble its odds.
fn consequence(o: &OptionSpec, budget_left_usd: f64) -> String {
    let mut text = o.consequence.clone();
    if o.cost_usd > 0.0 {
        let left = budget_left_usd.max(0.0);
        text = if o.cost_usd > left {
            format!("{text} Would cost {} of Budget.", money(o.cost_usd))
        } else {
            format!("{text} Costs {} of the {} of Budget left.", money(o.cost_usd), money(left))
        };
    }
    match (o.chance, &o.bad) {
        (Some(chance), Some(_)) => format!("{text} A gamble: about {} in 10 it goes well.", (chance * 10.0).round() as u32),
        _ => text,
    }
}

/// The outcome of the chosen option: its own log and effects, or for a gamble whose planned
/// `roll` missed its `chance`, the `bad` ones. Returns (log, effects, good).
fn outcome(o: &OptionSpec, roll: f64) -> (String, Vec<EffectSpec>, bool) {
    match (&o.bad, o.chance) {
        (Some(bad), Some(chance)) if roll >= chance => (bad.log.clone(), bad.effects.clone(), false),
        _ => (o.log.clone(), o.effects.clone(), true),
    }
}

/// The event's mitigation when its Design fact holds.
fn mitigated<'a>(spec: &'a EventSpec, facts: &Facts) -> Option<&'a Mitigation> {
    spec.mitigation.as_ref().filter(|m| facts.contains(&m.fact))
}
