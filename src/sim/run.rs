//! The Run: a day-by-day flight of a Design, driven by a seed. Each day is one pure `step` over explicit
//! `RunState`, so the loop can later take player decisions between days (ticket 10).
use serde::{Deserialize, Serialize};

use super::catalog::*;
use super::causes::not_launched_causes;
use super::debrief::{Debrief, DebriefContext, RunStats, debrief, not_launched};
use super::decisions::{self, Decision, Taken};
use super::design::*;
use super::events::{self, ComponentKind, RandomEvent};
use super::resources::{Resources, resolve, resources};
use super::review::{GaugeState, launch_review};
use super::rng::Rng;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RunResult {
    /// One snapshot per simulated day, in order.
    pub timeline: Vec<DaySnapshot>,
    pub debrief: Debrief,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DaySnapshot {
    /// 1-based.
    pub day: u32,
    /// Battery charge at the worst point of the day (end of an eclipse), 0.0..=1.0.
    pub battery_charge: f64,
    /// How the battery is coping, from the healthy depth of discharge in `data/review.toml`: `Ok` within it,
    /// `Thin` drained deeper (it wears fast), `Violated` it cannot cover the eclipse, so Instruments go off.
    pub battery_state: GaugeState,
    pub in_eclipse_fraction: f64,
    /// Share of the day the Instruments could run.
    pub operating_fraction: f64,
    /// Data waiting on board for the next ground contact.
    pub stored_data_mb: f64,
    /// Cumulative.
    pub delivered_data_mb: f64,
    /// Cumulative.
    pub lost_data_mb: f64,
    /// Cumulative.
    pub science_value: f64,
    /// Catalog ids of every Component that has failed so far.
    pub failed_components: Vec<String>,
    /// Random Events that happened on this day.
    pub events: Vec<RandomEvent>,
}

/// Flies a Design through a Scenario. Same inputs and seed give an identical result.
/// Never panics. A Design that fails Launch Review never flies: empty timeline, no Success, Score 0.
/// Interactive Random Events take their default choice; see `run_with_decisions`.
pub fn run(catalog: &Catalog, design: &Design, scenario_id: &str, seed: u64) -> RunResult {
    run_with_decisions(catalog, design, scenario_id, seed, &[])
}

/// Flies a Design like `run`, applying the player's answers to interactive Random Events. A question with no
/// (valid) answer takes its default choice. Same inputs, seed and decisions give an identical result, and
/// the days before a question never depend on its answer, so the UI can re-run after each choice and carry on.
pub fn run_with_decisions(
    catalog: &Catalog,
    design: &Design,
    scenario_id: &str,
    seed: u64,
    decisions: &[Decision],
) -> RunResult {
    let r = resolve(catalog, design, scenario_id);
    let res = resources(catalog, design, &r);
    let required_science_value = r.scenario.map_or(0.0, |s| s.required_science_value);
    let review = launch_review(catalog, design, scenario_id);
    if !review.can_launch {
        let stats = RunStats {
            science_value: 0.0,
            required_science_value,
            days_survived: 0,
            min_lifetime_days: res.min_lifetime_days,
            total_cost_usd: res.cost.total_usd,
            total_mass_kg: res.mass_kg,
            lost_data_fraction: 0.0,
            delivered_data_mb: 0.0,
            science_met: false,
            lifetime_met: false,
        };
        let causes = not_launched_causes(&catalog.cause_rules, &review.violations);
        return RunResult { timeline: Vec::new(), debrief: not_launched(stats, causes) };
    }
    let ctx = RunContext::new(catalog, design, scenario_id, &r, &res, decisions);
    let mut state = RunState::new(seed, &ctx);
    let mut timeline = Vec::with_capacity(ctx.planned_days as usize);
    while state.day < ctx.planned_days && !state.ended {
        timeline.push(step(&ctx, &mut state));
    }

    let days_survived = if state.ended { state.day.saturating_sub(1) } else { state.day };
    let stats = RunStats {
        science_value: state.science_value,
        required_science_value,
        days_survived,
        min_lifetime_days: res.min_lifetime_days,
        total_cost_usd: res.cost.total_usd + state.extra_cost_usd,
        total_mass_kg: res.mass_kg,
        lost_data_fraction: if state.produced_mb > 0.0 { state.lost_mb / state.produced_mb } else { 0.0 },
        delivered_data_mb: state.delivered_mb,
        science_met: state.science_value >= required_science_value,
        lifetime_met: days_survived >= res.min_lifetime_days,
    };
    let success = r.scenario.is_some() && stats.science_met && stats.lifetime_met;
    let ending = if state.ended {
        format!(
            "The Mission ended on day {} of {} planned days of operations: a critical Component failed.",
            state.day, ctx.planned_days
        )
    } else if ctx.planned_days < res.min_lifetime_days {
        format!(
            "The Orbit gave out after {} days (drag or radiation), before the {} days of operations the Scenario needs.",
            ctx.planned_days, res.min_lifetime_days
        )
    } else {
        format!("Planned mission complete: all {} days of operations flown.", ctx.planned_days)
    };
    let debrief = debrief(&DebriefContext {
        catalog,
        design,
        scenario_id,
        timeline: &timeline,
        stats: &stats,
        success,
        ending,
        budget_usd: res.budget_usd,
        capacity_kg: res.capacity_kg,
    });
    let mut debrief = debrief;
    debrief.decisions = decision_notes(&state.taken, &timeline);
    RunResult { timeline, debrief }
}

/// The Debrief's notes on every question answered (or defaulted) during the Run.
fn decision_notes(taken: &[Taken], timeline: &[DaySnapshot]) -> Vec<String> {
    // Cumulative (delivered, lost) Data at the end of `day` (0 = launch), clamped to the timeline.
    let at = |day: u32| -> (f64, f64) {
        let day = (day as usize).min(timeline.len());
        if day == 0 { (0.0, 0.0) } else { (timeline[day - 1].delivered_data_mb, timeline[day - 1].lost_data_mb) }
    };
    taken
        .iter()
        .map(|t| {
            let (delivered_before, lost_before) = at(t.day - 1);
            let (delivered_after, lost_after) = at(t.day + t.days.max(1) - 1);
            decisions::note(t, delivered_after - delivered_before, lost_after - lost_before)
        })
        .collect()
}

/// A Component on board that can fail.
struct Part {
    id: String,
    name: String,
    daily_failure_chance: f64,
    critical: bool,
}

struct InstrumentOutput {
    data_mb_per_day: f64,
    needs_fine_pointing: bool,
    /// Science Value per MB delivered (fit x coverage x Scenario rate).
    science_per_mb: f64,
}

/// Everything about the Run that does not change from day to day.
struct RunContext<'a> {
    rules: &'a RunRules,
    /// The player's answers to interactive Random Events.
    decisions: &'a [Decision],
    tuning: EventTuning,
    planned_days: u32,
    /// Indexed like `ComponentKind::DRAW_ORDER`.
    parts: [Option<Part>; 5],
    instruments: Vec<InstrumentOutput>,
    generation_w: f64,
    consumption_w: f64,
    eclipse_fraction: f64,
    eclipse_energy_wh: f64,
    eclipse_minutes: f64,
    /// Eclipse cycles (orbits) per day.
    cycles_per_day: f64,
    battery_capacity_wh: f64,
    battery_max_depth: f64,
    downlink_mb_per_day: f64,
    storage_mb: f64,
}

const NO_EVENTS: EventTuning = EventTuning {
    solar_flare_daily_chance: 0.0,
    flare_safe_mode_days: 0,
    flare_failure_multiplier: 1.0,
    outage_daily_chance: 0.0,
    outage_days: 0,
};

impl<'a> RunContext<'a> {
    fn new(
        catalog: &'a Catalog,
        design: &Design,
        scenario_id: &str,
        r: &super::resources::Resolved,
        res: &Resources,
        decisions: &'a [Decision],
    ) -> Self {
        let chance = |t: ReliabilityTier| catalog.tier(t).map_or(0.0, |s| s.daily_failure_chance);
        let part = |id: &str, name: &str, tier: ReliabilityTier, critical: bool| Part {
            id: id.to_string(),
            name: name.to_string(),
            daily_failure_chance: chance(tier),
            critical,
        };
        let computer = &catalog.computer;
        let parts = [
            r.solar.map(|(o, t)| part(&o.id, &o.name, t, o.critical)),
            r.battery.map(|(o, t)| part(&o.id, &o.name, t, o.critical)),
            r.attitude.map(|(o, t)| part(&o.id, &o.name, t, o.critical)),
            r.comms.map(|(o, t)| part(&o.id, &o.name, t, o.critical)),
            Some(part(&computer.id, &computer.name, design.computer_tier, computer.critical)),
        ];
        let coverage = r.orbit.and_then(|o| o.coverage.get(scenario_id).copied()).unwrap_or(0.0);
        let per_gb = r.scenario.map_or(0.0, |s| s.science_value_per_gb);
        let instruments = r
            .instruments
            .iter()
            .map(|i| InstrumentOutput {
                data_mb_per_day: i.data_mb_per_day,
                needs_fine_pointing: i.needs_fine_pointing,
                science_per_mb: i.fit.get(scenario_id).copied().unwrap_or(0.0) * coverage * per_gb / 1000.0,
            })
            .collect();
        let min_days = r.scenario.map_or(0, |s| s.min_lifetime_days);
        let planned_days = match r.orbit.and_then(|o| o.lifetime_cap_days) {
            Some(cap) => min_days.min(cap.max(0.0).floor() as u32),
            None => min_days,
        };
        RunContext {
            rules: &catalog.run_rules,
            decisions,
            tuning: r.scenario.map_or(NO_EVENTS, |s| s.events.clone()),
            planned_days,
            parts,
            instruments,
            generation_w: res.generation_w,
            consumption_w: res.consumption_w,
            eclipse_fraction: r.orbit.map_or(0.0, |o| o.eclipse_fraction),
            eclipse_energy_wh: res.eclipse_energy_wh,
            eclipse_minutes: res.eclipse_minutes,
            cycles_per_day: r.orbit.filter(|o| o.period_min > 0.0).map_or(0.0, |o| 24.0 * 60.0 / o.period_min),
            battery_capacity_wh: r.battery.map_or(0.0, |(b, _)| b.capacity_wh),
            battery_max_depth: catalog.review.battery_max_depth_of_discharge,
            downlink_mb_per_day: res.downlink_mb_per_day,
            storage_mb: computer.storage_mb,
        }
    }
}

const SOLAR: usize = 0;
const BATTERY: usize = 1;
const ATTITUDE: usize = 2;
const COMMS: usize = 3;
const COMPUTER: usize = 4;

/// Everything that changes from day to day.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct RunState {
    day: u32,
    rng: Rng,
    /// Remaining battery capacity after wear.
    battery_capacity_wh: f64,
    stored_mb: f64,
    /// Science Value the stored Data will earn once delivered.
    stored_value: f64,
    produced_mb: f64,
    delivered_mb: f64,
    lost_mb: f64,
    science_value: f64,
    /// Indexed like `ComponentKind::DRAW_ORDER`.
    failed: [bool; 5],
    safe_mode_days_left: u32,
    outage_days_left: u32,
    /// Time on another ground station was bought for the current outage, so Data still comes down.
    station_bought: bool,
    /// Money spent on decisions during the Run, on top of the Design's cost.
    extra_cost_usd: f64,
    /// Questions answered so far, for the Debrief.
    taken: Vec<Taken>,
    /// A fatal failure ended the Mission.
    ended: bool,
    /// The battery has stopped covering the eclipse (reported once).
    battery_short: bool,
}

impl RunState {
    fn new(seed: u64, ctx: &RunContext) -> Self {
        RunState {
            day: 0,
            rng: Rng::new(seed),
            battery_capacity_wh: ctx.battery_capacity_wh,
            stored_mb: 0.0,
            stored_value: 0.0,
            produced_mb: 0.0,
            delivered_mb: 0.0,
            lost_mb: 0.0,
            science_value: 0.0,
            failed: [false; 5],
            safe_mode_days_left: 0,
            outage_days_left: 0,
            station_bought: false,
            extra_cost_usd: 0.0,
            taken: Vec::new(),
            ended: false,
            battery_short: false,
        }
    }
}

/// Simulates one day and returns its snapshot.
fn step(ctx: &RunContext, s: &mut RunState) -> DaySnapshot {
    s.day += 1;
    let tuning = &ctx.tuning;
    let draws = events::draw_day(&mut s.rng, tuning);
    let mut day_events = Vec::new();

    // ---- Random Events. Flares and outages ask the player first; the answer (or the default) applies today.
    let choices = &ctx.rules.decisions;
    let mut multiplier = 1.0;
    let mut flare_taken = None;
    if draws.flare {
        let prompt = decisions::flare_prompt(s.day, tuning, choices);
        let (choice, by_player) = decisions::chosen(&prompt, ctx.decisions);
        let mut event = if choice == decisions::RIDE_OUT {
            multiplier = tuning.flare_failure_multiplier * choices.flare_ride_out_failure_multiplier;
            events::solar_flare_ridden_out()
        } else {
            multiplier = tuning.flare_failure_multiplier;
            s.safe_mode_days_left = s.safe_mode_days_left.max(tuning.flare_safe_mode_days);
            events::solar_flare(tuning)
        };
        flare_taken = Some(Taken {
            day: s.day,
            choice: choice.to_string(),
            by_player,
            failed_names: Vec::new(),
            days: tuning.flare_safe_mode_days,
            cost_usd: 0.0,
        });
        event.prompt = Some(prompt);
        event.choice = Some(choice.to_string());
        day_events.push(event);
    }
    if draws.outage && s.outage_days_left == 0 {
        let prompt = decisions::outage_prompt(s.day, tuning, choices);
        let (choice, by_player) = decisions::chosen(&prompt, ctx.decisions);
        let buy = choice == decisions::BUY_STATION_TIME;
        let cost_usd = if buy { choices.outage_station_time_cost_usd } else { 0.0 };
        let mut event =
            if buy { events::ground_station_outage_covered(tuning) } else { events::ground_station_outage(tuning) };
        s.taken.push(Taken {
            day: s.day,
            choice: choice.to_string(),
            by_player,
            failed_names: Vec::new(),
            days: tuning.outage_days,
            cost_usd,
        });
        event.prompt = Some(prompt);
        event.choice = Some(choice.to_string());
        day_events.push(event);
        s.outage_days_left = tuning.outage_days;
        s.station_bought = buy;
        s.extra_cost_usd += cost_usd;
    }
    for (i, kind) in ComponentKind::DRAW_ORDER.into_iter().enumerate() {
        let Some(part) = &ctx.parts[i] else { continue };
        if s.failed[i] || draws.component_rolls[i] >= part.daily_failure_chance * multiplier {
            continue;
        }
        s.failed[i] = true;
        s.ended |= part.critical;
        if let Some(t) = &mut flare_taken {
            t.failed_names.push(part.name.clone());
        }
        day_events.push(events::component_failure(kind, &part.id, &part.name, part.critical, draws.flare));
    }

    if let Some(t) = flare_taken {
        // Keep the notes in day order, the flare before an outage on the same day.
        let at = s.taken.iter().position(|o| o.day == s.day).unwrap_or(s.taken.len());
        s.taken.insert(at, t);
    }

    // ---- battery wear: deeper discharges than the healthy depth wear the battery out fast. Wear only
    // shrinks capacity (so operating time is lost); it never ends the Mission by itself.
    if ctx.parts[BATTERY].is_some()
        && !s.failed[BATTERY]
        && ctx.eclipse_energy_wh > 0.0
        && s.battery_capacity_wh > 0.0
        && ctx.battery_max_depth > 0.0
    {
        let relative_depth = (ctx.eclipse_energy_wh / s.battery_capacity_wh / ctx.battery_max_depth)
            .min(1.0 / ctx.battery_max_depth);
        let wear = ctx.rules.battery_fade_per_cycle_at_max_depth
            * ctx.cycles_per_day
            * pow(relative_depth, ctx.rules.battery_fade_exponent);
        s.battery_capacity_wh = (s.battery_capacity_wh - wear * ctx.battery_capacity_wh).max(0.0);
    }

    // ---- Power: generation against use, and the battery through the eclipse
    let degraded = ctx.rules.degraded_output_fraction;
    let failed = s.failed;
    let output = |i: usize| if failed[i] { degraded } else { 1.0 };
    let generation_w = ctx.generation_w * output(SOLAR);
    let power_ratio = if ctx.consumption_w > 0.0 { (generation_w / ctx.consumption_w).min(1.0) } else { 1.0 };
    let capacity_wh = s.battery_capacity_wh * output(BATTERY);
    // Share of each eclipse the battery carries; the Instruments are off for the rest of it.
    let (eclipse_covered, battery_charge, battery_state) = if ctx.eclipse_energy_wh <= 0.0 {
        (1.0, 1.0, GaugeState::Ok)
    } else {
        let depth = if capacity_wh > 0.0 { ctx.eclipse_energy_wh / capacity_wh } else { f64::INFINITY };
        let state = if depth > 1.0 {
            GaugeState::Violated
        } else if depth > ctx.battery_max_depth {
            GaugeState::Thin
        } else {
            GaugeState::Ok
        };
        ((1.0 / depth).min(1.0), (1.0 - depth).clamp(0.0, 1.0), state)
    };
    // Uncovered part of each eclipse, plus the reboot and re-pointing after every brown-out.
    let battery_ratio = if battery_state == GaugeState::Violated {
        let recovery = ctx.rules.brownout_recovery_minutes * ctx.cycles_per_day / (24.0 * 60.0);
        (1.0 - ctx.eclipse_fraction * (1.0 - eclipse_covered) - recovery).max(0.0)
    } else {
        1.0
    };
    if battery_state == GaugeState::Violated && !s.battery_short && !s.ended {
        s.battery_short = true;
        if let Some(battery) = &ctx.parts[BATTERY] {
            let worn = s.battery_capacity_wh < ctx.battery_capacity_wh;
            day_events.push(events::battery_shortfall(&battery.id, &battery.name, ctx.eclipse_minutes, worn));
        }
    }
    let safe_mode = s.safe_mode_days_left > 0;
    let operating_fraction = if safe_mode || s.ended { 0.0 } else { power_ratio * battery_ratio };

    // ---- Data: produce, store, send down, lose what does not fit
    let pointing = output(ATTITUDE);
    let (produced_mb, produced_value) = ctx.instruments.iter().fold((0.0, 0.0), |(mb, value), i| {
        let made = i.data_mb_per_day * operating_fraction * if i.needs_fine_pointing { pointing } else { 1.0 };
        (mb + made, value + made * i.science_per_mb)
    });
    s.produced_mb += produced_mb;
    s.stored_mb += produced_mb;
    s.stored_value += produced_value;

    let ground_up = (s.outage_days_left == 0 || s.station_bought) && !s.ended;
    let downlink_mb = if ground_up { ctx.downlink_mb_per_day * output(COMMS) } else { 0.0 };
    let sent = take(s, downlink_mb.min(s.stored_mb));
    s.delivered_mb += sent.0;
    s.science_value += sent.1;

    let storage_mb = ctx.storage_mb * output(COMPUTER);
    if s.stored_mb > storage_mb {
        let lost = take(s, s.stored_mb - storage_mb);
        s.lost_mb += lost.0;
    }

    s.safe_mode_days_left = s.safe_mode_days_left.saturating_sub(1);
    s.outage_days_left = s.outage_days_left.saturating_sub(1);
    s.station_bought &= s.outage_days_left > 0;

    DaySnapshot {
        day: s.day,
        battery_charge: battery_charge * power_ratio,
        battery_state,
        in_eclipse_fraction: ctx.eclipse_fraction,
        operating_fraction,
        stored_data_mb: s.stored_mb,
        delivered_data_mb: s.delivered_mb,
        lost_data_mb: s.lost_mb,
        science_value: s.science_value,
        failed_components: (0..5)
            .filter(|&i| s.failed[i])
            .filter_map(|i| ctx.parts[i].as_ref().map(|p| p.id.clone()))
            .collect(),
        events: day_events,
    }
}

/// Removes `mb` of stored Data with its share of the stored Science Value; returns both.
fn take(s: &mut RunState, mb: f64) -> (f64, f64) {
    if mb <= 0.0 || s.stored_mb <= 0.0 {
        return (0.0, 0.0);
    }
    let value = s.stored_value * mb / s.stored_mb;
    s.stored_mb -= mb;
    s.stored_value -= value;
    (mb, value)
}

/// Integer power by repeated multiplication (bit-identical on every platform, unlike `powf`).
fn pow(x: f64, n: u32) -> f64 {
    (0..n).fold(1.0, |acc, _| acc * x)
}
