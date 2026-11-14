//! The Run: a day-by-day flight of a Design, driven by a seed. Each day is one pure `step` over explicit
//! `RunState`, so the loop can later take player decisions between days (ticket 10).
use serde::{Deserialize, Serialize};

use super::catalog::*;
use super::debrief::{Debrief, DebriefContext, RunStats, debrief};
use super::design::*;
use super::events::{self, ComponentKind, RandomEvent};
use super::resources::{Resources, resolve, resources};
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
/// Never panics: an incomplete or nonsensical Design simply flies badly.
pub fn run(catalog: &Catalog, design: &Design, scenario_id: &str, seed: u64) -> RunResult {
    let r = resolve(catalog, design, scenario_id);
    let res = resources(catalog, design, &r);
    let ctx = RunContext::new(catalog, design, scenario_id, &r, &res);
    let mut state = RunState::new(seed, &ctx);
    let mut timeline = Vec::with_capacity(ctx.planned_days as usize);
    while state.day < ctx.planned_days && !state.ended {
        timeline.push(step(&ctx, &mut state));
    }

    let days_survived = if state.ended { state.day.saturating_sub(1) } else { state.day };
    let required_science_value = r.scenario.map_or(0.0, |s| s.required_science_value);
    let stats = RunStats {
        science_value: state.science_value,
        required_science_value,
        days_survived,
        min_lifetime_days: res.min_lifetime_days,
        total_cost_usd: res.cost.total_usd,
        total_mass_kg: res.mass_kg,
        lost_data_fraction: if state.produced_mb > 0.0 { state.lost_mb / state.produced_mb } else { 0.0 },
        delivered_data_mb: state.delivered_mb,
    };
    let success = r.scenario.is_some()
        && stats.science_value >= required_science_value
        && days_survived >= res.min_lifetime_days;
    let debrief = debrief(&DebriefContext {
        catalog,
        design,
        scenario_id,
        timeline: &timeline,
        stats: &stats,
        success,
        budget_usd: res.budget_usd,
        capacity_kg: res.capacity_kg,
    });
    RunResult { timeline, debrief }
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
    tuning: EventTuning,
    planned_days: u32,
    /// Indexed like `ComponentKind::DRAW_ORDER`.
    parts: [Option<Part>; 5],
    instruments: Vec<InstrumentOutput>,
    generation_w: f64,
    consumption_w: f64,
    eclipse_fraction: f64,
    eclipse_energy_wh: f64,
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
        let planned_days = match r.orbit {
            Some(o) if o.lifetime_cap_days >= 0.0 => min_days.min(o.lifetime_cap_days.floor() as u32),
            _ => min_days,
        };
        RunContext {
            rules: &catalog.run_rules,
            tuning: r.scenario.map_or(NO_EVENTS, |s| s.events.clone()),
            planned_days,
            parts,
            instruments,
            generation_w: res.generation_w,
            consumption_w: res.consumption_w,
            eclipse_fraction: r.orbit.map_or(0.0, |o| o.eclipse_fraction),
            eclipse_energy_wh: res.eclipse_energy_wh,
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
    /// A fatal failure ended the Mission.
    ended: bool,
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
            ended: false,
        }
    }
}

/// Simulates one day and returns its snapshot.
fn step(ctx: &RunContext, s: &mut RunState) -> DaySnapshot {
    s.day += 1;
    let tuning = &ctx.tuning;
    let draws = events::draw_day(&mut s.rng, tuning);
    let mut day_events = Vec::new();

    // ---- Random Events
    if draws.flare {
        day_events.push(events::solar_flare(tuning));
        s.safe_mode_days_left = s.safe_mode_days_left.max(tuning.flare_safe_mode_days);
    }
    if draws.outage && s.outage_days_left == 0 {
        day_events.push(events::ground_station_outage(tuning));
        s.outage_days_left = tuning.outage_days;
    }
    let multiplier = if draws.flare { tuning.flare_failure_multiplier } else { 1.0 };
    for (i, kind) in ComponentKind::DRAW_ORDER.into_iter().enumerate() {
        let Some(part) = &ctx.parts[i] else { continue };
        if s.failed[i] || draws.component_rolls[i] >= part.daily_failure_chance * multiplier {
            continue;
        }
        s.failed[i] = true;
        s.ended |= part.critical;
        day_events.push(events::component_failure(kind, &part.id, &part.name, part.critical, draws.flare));
    }

    // ---- battery wear: deeper discharges than the healthy depth wear the battery out fast
    if let Some(battery) = &ctx.parts[BATTERY]
        && !s.failed[BATTERY]
        && ctx.eclipse_energy_wh > 0.0
    {
        if s.battery_capacity_wh > 0.0 && ctx.battery_max_depth > 0.0 {
            let relative_depth = ctx.eclipse_energy_wh / s.battery_capacity_wh / ctx.battery_max_depth;
            let wear = ctx.rules.battery_fade_per_cycle_at_max_depth
                * ctx.cycles_per_day
                * pow(relative_depth, ctx.rules.battery_fade_exponent);
            s.battery_capacity_wh = (s.battery_capacity_wh - wear * ctx.battery_capacity_wh).max(0.0);
        }
        if s.battery_capacity_wh < ctx.eclipse_energy_wh {
            s.failed[BATTERY] = true;
            s.ended |= battery.critical;
            day_events.push(events::battery_worn_out(&battery.id, &battery.name, battery.critical));
        }
    }

    // ---- Power: generation against use, and the battery through the eclipse
    let degraded = ctx.rules.degraded_output_fraction;
    let failed = s.failed;
    let output = |i: usize| if failed[i] { degraded } else { 1.0 };
    let generation_w = ctx.generation_w * output(SOLAR);
    let power_ratio = if ctx.consumption_w > 0.0 { (generation_w / ctx.consumption_w).min(1.0) } else { 1.0 };
    let capacity_wh = s.battery_capacity_wh * output(BATTERY);
    let (battery_ratio, battery_charge) = if ctx.eclipse_energy_wh <= 0.0 {
        (1.0, 1.0)
    } else if capacity_wh <= 0.0 {
        (0.0, 0.0)
    } else {
        let depth = ctx.eclipse_energy_wh / capacity_wh;
        ((1.0 / depth).min(1.0), (1.0 - depth).clamp(0.0, 1.0))
    };
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

    let ground_up = s.outage_days_left == 0 && !s.ended;
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

    DaySnapshot {
        day: s.day,
        battery_charge: battery_charge * power_ratio,
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
