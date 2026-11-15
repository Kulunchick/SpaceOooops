//! What the new Random Events do to a Run. A timed mechanic never touches the Run state: it becomes an active
//! effect, and `EventState::modifiers` turns the effects active on a day into factors the day's `step` applies.
//! An instant mechanic comes back from the day as numbers (`DayOutcome`) the `step` adds.
use super::design_facts::Facts;
use super::event_catalog::{EffectSpec, EventSpec, Mechanic};
use super::score::{ScoreLine, ScoreLineKind};

/// Everything the new Random Events keep over the Run.
#[derive(Debug, Clone, PartialEq, Default)]
pub(crate) struct EventState {
    /// Timed effects, in the order they started.
    active: Vec<Active>,
    /// US$ gained during the Run; it raises the Budget left.
    pub budget_gained_usd: f64,
    /// Score lines from `score-bonus` effects.
    pub score_lines: Vec<ScoreLine>,
    /// Every new event that fired, in day order, for the Debrief.
    pub fired: Vec<Fired>,
    /// The Design facts of this Run.
    pub facts: Facts,
}

/// One new Random Event that happened.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Fired {
    pub day: u32,
    pub spec: EventSpec,
    /// For an event with a question: the index of the option that applied.
    pub option: Option<usize>,
    /// The player chose the option (rather than the default).
    pub by_player: bool,
    /// For a gamble: the good outcome came up.
    pub good: bool,
    /// A Design choice softened the event (its mitigation applied).
    pub softened: bool,
}

/// One timed effect in force.
#[derive(Debug, Clone, PartialEq)]
struct Active {
    mechanic: Mechanic,
    /// 1 + strength.
    factor: f64,
    /// First and last day (inclusive); `None` = to the end of the Run.
    from: u32,
    to: Option<u32>,
}

/// The factors of one day: neutral (1.0, false) unless an effect is active.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct DayModifiers {
    pub power: f64,
    pub instruments_off: bool,
    pub downlink: f64,
    pub data: f64,
    pub storage: f64,
    pub pointing: f64,
    pub battery_capacity: f64,
    pub failure_chance: f64,
}

impl Default for DayModifiers {
    fn default() -> Self {
        DayModifiers {
            power: 1.0,
            instruments_off: false,
            downlink: 1.0,
            data: 1.0,
            storage: 1.0,
            pointing: 1.0,
            battery_capacity: 1.0,
            failure_chance: 1.0,
        }
    }
}

/// What a day's instant effects add, for the `step` to apply.
#[derive(Debug, Clone, PartialEq, Default)]
pub(crate) struct DayOutcome {
    /// US$ spent (before the cap at the Budget left).
    pub cost_usd: f64,
    /// Share of the Scenario's required Science Value earned.
    pub science_bonus_share: f64,
}

impl EventState {
    pub fn new(facts: Facts) -> Self {
        EventState { facts, ..EventState::default() }
    }

    /// The factors of the effects active on `day`. Several active effects multiply; a factor is never negative.
    pub fn modifiers(&self, day: u32) -> DayModifiers {
        let mut m = DayModifiers::default();
        for a in self.active.iter().filter(|a| a.from <= day && a.to.is_none_or(|to| day <= to)) {
            let factor = a.factor.max(0.0);
            match a.mechanic {
                Mechanic::Power => m.power *= factor,
                Mechanic::InstrumentsOff => m.instruments_off = true,
                Mechanic::Downlink => m.downlink *= factor,
                Mechanic::Data | Mechanic::ExtraDataDay => m.data *= factor,
                Mechanic::Storage => m.storage *= factor,
                Mechanic::Pointing => m.pointing *= factor,
                Mechanic::BatteryCapacity => m.battery_capacity *= factor,
                Mechanic::FailureChance => m.failure_chance *= factor,
                Mechanic::Budget | Mechanic::ScienceBonus | Mechanic::ScoreBonus => {}
            }
        }
        m
    }

    /// Puts a timed effect in force from `day + delay`, for `days` days or to the end of the Run.
    fn timed(&mut self, effect: &EffectSpec, day: u32) {
        let from = day + effect.delay;
        let to = if effect.to_end { None } else { Some(from + effect.days.unwrap_or(1).max(1) - 1) };
        self.active.push(Active { mechanic: effect.mechanic, factor: 1.0 + effect.strength, from, to });
    }
}

/// Starts one effect of an event that fired on `day`. One arm per mechanic; a later ticket fills its own arm.
pub(crate) fn start(effect: &EffectSpec, day: u32, st: &mut EventState, out: &mut DayOutcome) {
    match effect.mechanic {
        // ---- 01
        Mechanic::Power => st.timed(effect, day),
        Mechanic::InstrumentsOff => st.timed(effect, day),
        Mechanic::Downlink => st.timed(effect, day),
        // ---- 02, timed
        Mechanic::Data => st.timed(effect, day),
        Mechanic::Storage => st.timed(effect, day),
        Mechanic::Pointing => st.timed(effect, day),
        Mechanic::BatteryCapacity => st.timed(effect, day),
        Mechanic::FailureChance => st.timed(effect, day),
        // ---- 02, instant
        Mechanic::Budget => {
            if effect.strength >= 0.0 {
                st.budget_gained_usd += effect.strength;
            } else {
                out.cost_usd -= effect.strength;
            }
        }
        Mechanic::ScienceBonus => out.science_bonus_share += effect.strength,
        Mechanic::ExtraDataDay => {
            st.timed(&EffectSpec { strength: 1.0, days: Some(1), to_end: false, ..effect.clone() }, day)
        }
        // ---- 06
        Mechanic::ScoreBonus => st.score_lines.push(ScoreLine {
            kind: ScoreLineKind::EventBonus,
            label: effect.label.clone().unwrap_or_default(),
            points: effect.strength,
        }),
    }
}

/// How many days the scene shows an event whose `effects` applied: the longest timed effect that starts on the
/// event's day, at least 1, at most 5. A "to the end" effect is not a year-long animation: it counts as the cap
/// only when the event has no effect of a set length, so an event never shows longer than it lasts (the aliens'
/// two-day visit is not stretched by the change they leave behind).
pub(crate) fn scene_days(effects: &[EffectSpec]) -> u32 {
    const CAP: u32 = 5;
    let timed = || effects.iter().filter(|e| e.mechanic.is_timed() && e.delay == 0);
    let set = timed().filter_map(|e| e.days).max();
    let to_end = timed().any(|e| e.to_end).then_some(CAP);
    set.or(to_end).unwrap_or(1).clamp(1, CAP)
}
