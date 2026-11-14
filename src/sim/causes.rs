//! Debrief causes: why the Mission went the way it did, in plain language with this Run's numbers, the choice
//! that led to it and, where there is one, what would fix it. Ordered by what they cost; thresholds and the cap
//! live in `data/causes.toml`.
use serde::{Deserialize, Serialize};

use super::catalog::*;
use super::debrief::DebriefContext;
use super::design::ReliabilityTier;
use super::events::EventKind;
use super::resources::{Resolved, Resources, downlink_mb, resolve, resources};
use super::review::{Finding, GaugeState};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CauseKind {
    /// The battery could not carry the spacecraft through Earth's shadow (too small, or worn out).
    BatteryTooSmall,
    /// Data never reached the ground: more was made than the comms band could send, or the store overflowed.
    DataLost,
    /// A Component failed, ending the Run or degrading it.
    ComponentFailure,
    /// Even a perfect Run of these Instruments could not reach the required Science Value.
    ScienceShortfall,
    /// The Orbit cut the Run short (air drag in low Orbit, or radiation).
    LifetimeShortfall,
    /// The Orbit covers the Scenario's target poorly.
    PoorCoverage,
    /// An Instrument suits the Scenario poorly.
    PoorInstrumentFit,
    /// Random Events (solar flares, ground station outages) cost Science Value.
    RandomEvents,
    /// The Design never flew: a Launch Review violation.
    NotLaunched,
    /// On a Success: what went right.
    WentWell,
    /// On a Success: what held the Score down (a risk taken or a small margin).
    ScoreLimit,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Cause {
    pub kind: CauseKind,
    /// The full plain-language sentence shown to the player.
    pub text: String,
    /// The day of the Run the cause shows up, if it has one.
    pub day: Option<u32>,
}

/// A cause with what it cost, in Science Value (plus Score penalty points), for ordering.
struct Weighed {
    cause: Cause,
    impact: f64,
}

fn weighed(kind: CauseKind, text: String, day: Option<u32>, impact: f64) -> Weighed {
    Weighed { cause: Cause { kind, text, day }, impact }
}

/// The causes of a Design that never flew: its Launch Review violations.
pub(crate) fn not_launched_causes(rules: &CauseRules, violations: &[Finding]) -> Vec<Cause> {
    violations
        .iter()
        .take(rules.max_causes)
        .map(|f| Cause { kind: CauseKind::NotLaunched, text: format!("Launch Review stopped it: {}", f.reason), day: None })
        .collect()
}

/// Everything about the flown Design the causes are worked out from.
struct Facts<'a> {
    catalog: &'a Catalog,
    ctx: &'a DebriefContext<'a>,
    r: Resolved<'a>,
    res: Resources,
    scenario: &'a Scenario,
    /// Days the Orbit allows, capped at the Scenario's minimum Lifetime.
    planned_days: u32,
    coverage: f64,
    /// Science Value per day if every MB the Instruments make came down.
    value_per_day_all: f64,
    /// Science Value per day the comms band can actually bring down on a normal day.
    value_per_day: f64,
    /// Average Science Value of one MB of this Design's Data.
    value_per_mb: f64,
    /// Days spent in safe mode after solar flares.
    safe_mode_days: Vec<u32>,
}

impl<'a> Facts<'a> {
    fn new(ctx: &'a DebriefContext<'a>, scenario: &'a Scenario) -> Self {
        let catalog = ctx.catalog;
        let r = resolve(catalog, ctx.design, ctx.scenario_id);
        let res = resources(catalog, ctx.design, &r);
        let coverage = r.orbit.and_then(|o| o.coverage.get(&scenario.id).copied()).unwrap_or(0.0);
        let value_per_day_all: f64 = r
            .instruments
            .iter()
            .map(|i| i.data_mb_per_day * fit(i, &scenario.id) * coverage * scenario.science_value_per_gb / 1000.0)
            .sum();
        let deliverable = if res.data_mb_per_day > 0.0 { (res.downlink_mb_per_day / res.data_mb_per_day).min(1.0) } else { 1.0 };
        let planned_days = match r.orbit.and_then(|o| o.lifetime_cap_days) {
            Some(cap) => scenario.min_lifetime_days.min(cap.max(0.0).floor() as u32),
            None => scenario.min_lifetime_days,
        };
        let mut safe_mode_days = Vec::new();
        for day in ctx.timeline {
            if day.events.iter().any(|e| e.kind == EventKind::SolarFlare) {
                safe_mode_days.extend(day.day..day.day + scenario.events.flare_safe_mode_days);
            }
        }
        Facts {
            catalog,
            ctx,
            value_per_mb: if res.data_mb_per_day > 0.0 { value_per_day_all / res.data_mb_per_day } else { 0.0 },
            value_per_day: value_per_day_all * deliverable,
            value_per_day_all,
            r,
            res,
            scenario,
            planned_days,
            coverage,
            safe_mode_days,
        }
    }

    fn days_flown(&self) -> u32 {
        self.ctx.timeline.len() as u32
    }

    /// Score points lost to the early-end penalty for `days` missing days.
    fn penalty(&self, days: u32) -> f64 {
        let min = self.scenario.min_lifetime_days;
        if min == 0 { 0.0 } else { f64::from(days) / f64::from(min) * self.catalog.scoring.early_end_penalty_points }
    }
}

fn fit(i: &Instrument, scenario_id: &str) -> f64 {
    i.fit.get(scenario_id).copied().unwrap_or(0.0)
}

/// The causes behind a Run's outcome, the costliest first, capped by `data/causes.toml`.
pub(crate) fn causes(ctx: &DebriefContext) -> Vec<Cause> {
    let Some(scenario) = ctx.catalog.scenario(ctx.scenario_id) else { return Vec::new() };
    let rules = &ctx.catalog.cause_rules;
    let f = Facts::new(ctx, scenario);

    let mut problems: Vec<Weighed> = Vec::new();
    problems.extend(battery(&f));
    problems.extend(data_lost(&f));
    problems.extend(component_failures(&f));
    problems.extend(orbit_lifetime(&f));
    let coverage = poor_coverage(&f, rules);
    let fit = poor_fit(&f, rules);
    if coverage.is_none() && fit.is_empty() && !ctx.stats.science_met {
        problems.extend(science_shortfall(&f));
    }
    problems.extend(coverage);
    problems.extend(fit);
    problems.extend(flares(&f));
    // Most costly first; the sort is stable, so equal costs keep the order above.
    problems.sort_by(|a, b| b.impact.total_cmp(&a.impact));
    let floor = rules.min_impact_share * scenario.required_science_value;
    let mut problems: Vec<Cause> = problems.into_iter().filter(|w| w.impact > floor).map(|w| w.cause).collect();

    if !ctx.success {
        problems.truncate(rules.max_causes);
        return problems;
    }
    let mut out = vec![went_well(&f)];
    out.append(&mut problems);
    out.extend(risk_taken(&f));
    if out.len() == 1 {
        out.extend(margin_limit(&f));
    }
    out.truncate(rules.max_causes);
    out
}

// ---------------------------------------------------------------- the causes

fn battery(f: &Facts) -> Option<Weighed> {
    let (battery, _) = f.r.battery?;
    let short: Vec<_> = f
        .ctx
        .timeline
        .iter()
        .filter(|d| d.battery_state == GaugeState::Violated && !f.safe_mode_days.contains(&d.day))
        .collect();
    let first = short.first()?.day;
    let lost_days: f64 = short.iter().map(|d| 1.0 - d.operating_fraction).sum();
    let lost_share = lost_days / f64::from(f.days_flown().max(1));
    let minutes = f.res.eclipse_minutes;
    let need_wh = f.res.eclipse_energy_wh;
    let healthy = f.catalog.review.battery_max_depth_of_discharge;
    let fix = match f.catalog.batteries.iter().find(|b| b.capacity_wh * healthy >= need_wh) {
        Some(b) if b.id != battery.id => format!(
            "The {} would carry it through with a shallow, healthy discharge.",
            b.name
        ),
        _ => "Even the biggest battery would drain deep: use fewer power-hungry parts.".to_string(),
    };
    let text = if need_wh > battery.capacity_wh {
        format!(
            "Your {} was too small: each orbit spends {minutes:.0} min in Earth's shadow, and getting through it takes \
             {need_wh:.1} Wh. From day {first} the Instruments switched off for part of every orbit, and {} of the \
             Mission's operating time was lost. {fix}",
            battery.name,
            pct(lost_share)
        )
    } else {
        format!(
            "Your {} was drained {} deep in each of the {minutes:.0} min in Earth's shadow every orbit (a healthy \
             battery stays under {}), so it wore out. From day {first} it could no longer cover the eclipse, and {} of \
             the Mission's operating time was lost. {fix}",
            battery.name,
            pct(need_wh / battery.capacity_wh),
            pct(healthy),
            pct(lost_share)
        )
    };
    Some(weighed(CauseKind::BatteryTooSmall, text, Some(first), lost_days * f.value_per_day))
}

fn data_lost(f: &Facts) -> Option<Weighed> {
    let lost_mb = f.ctx.timeline.last()?.lost_data_mb;
    let share = f.ctx.stats.lost_data_fraction;
    if lost_mb <= 0.0 {
        return None;
    }
    let impact = lost_mb * f.value_per_mb;
    let (comms, _) = f.r.comms?;
    let made = f.res.data_mb_per_day;
    let down = f.res.downlink_mb_per_day;
    let contact = f.r.orbit.map_or(0.0, |o| o.contact_minutes_per_day);
    let text = if made > down {
        let fix = f
            .catalog
            .comms
            .iter()
            .filter(|c| downlink_mb(c.downlink_mbps, contact) >= made)
            .min_by(|a, b| a.cost_usd.total_cmp(&b.cost_usd))
            .map_or(
                "No comms band can keep up from this Orbit: carry fewer Instruments, or pick an Orbit with longer ground contacts."
                    .to_string(),
                |c| format!("The {} would send {} a day.", c.name, data(downlink_mb(c.downlink_mbps, contact))),
            );
        format!(
            "Your {} could send only {} a day in {contact:.0} min of ground contact, but your Instruments made {} a \
             day: {} of your Data never reached the ground. {fix}",
            comms.name,
            data(down),
            data(made),
            pct(share)
        )
    } else {
        format!(
            "{} of your Data was lost: it piled up past the {} the {} can store while the ground station was \
             offline or a Component was failing. A faster comms band empties the store sooner.",
            pct(share),
            data(f.catalog.computer.storage_mb),
            f.catalog.computer.name
        )
    };
    let day = f.ctx.timeline.iter().find(|d| d.lost_data_mb > 0.0).map(|d| d.day);
    Some(weighed(CauseKind::DataLost, text, day, impact))
}

/// Name, Reliability Tier, criticality and what is lost when it degrades, for a Component id on board.
fn part(f: &Facts, id: &str) -> Option<(String, ReliabilityTier, bool, &'static str)> {
    let r = &f.r;
    if let Some((o, t)) = r.solar.filter(|(o, _)| o.id == id) {
        return Some((o.name.clone(), t, o.critical, "the spacecraft made less Power"));
    }
    if let Some((o, t)) = r.battery.filter(|(o, _)| o.id == id) {
        return Some((o.name.clone(), t, o.critical, "the battery held less charge"));
    }
    if let Some((o, t)) = r.attitude.filter(|(o, _)| o.id == id) {
        return Some((o.name.clone(), t, o.critical, "pointing Instruments returned less Data"));
    }
    if let Some((o, t)) = r.comms.filter(|(o, _)| o.id == id) {
        return Some((o.name.clone(), t, o.critical, "less Data came down each day"));
    }
    let c = &f.catalog.computer;
    (c.id == id).then(|| (c.name.clone(), f.ctx.design.computer_tier, c.critical, "less Data could be stored"))
}

fn tier_name(f: &Facts, t: ReliabilityTier) -> String {
    f.catalog.tier(t).map_or_else(|| format!("{t:?}"), |s| s.name.clone())
}

fn component_failures(f: &Facts) -> Vec<Weighed> {
    let mut out = Vec::new();
    for day in f.ctx.timeline {
        for e in day.events.iter().filter(|e| e.kind == EventKind::ComponentFailure) {
            let Some(id) = &e.component else { continue };
            let Some((name, tier, critical, lost)) = part(f, id) else { continue };
            let flare = if f.safe_mode_days.contains(&day.day) { ", hit by a solar flare," } else { "" };
            let tier_text = tier_name(f, tier);
            let advice = tier_advice(f, tier);
            let (text, impact) = if critical {
                let missing = f.planned_days.saturating_sub(day.day.saturating_sub(1));
                let short = f.scenario.min_lifetime_days.saturating_sub(day.day.saturating_sub(1));
                (
                    format!(
                        "Your {tier_text} {name} failed on day {}{flare} and the Mission ended there, {short} days short \
                         of the {} days needed. {advice}",
                        day.day, f.scenario.min_lifetime_days
                    ),
                    f64::from(missing) * f.value_per_day + f.penalty(short),
                )
            } else {
                let remaining = f.days_flown().saturating_sub(day.day);
                let kept = f.catalog.run_rules.degraded_output_fraction;
                (
                    format!(
                        "Your {tier_text} {name} failed on day {}{flare}. It kept working at {} of normal for the last \
                         {remaining} days, so {lost}. {advice}",
                        day.day,
                        pct(kept)
                    ),
                    f64::from(remaining) * f.value_per_day * (1.0 - kept),
                )
            };
            out.push(weighed(CauseKind::ComponentFailure, text, Some(day.day), impact));
        }
    }
    out
}

/// What the Reliability Tier had to do with a failure.
fn tier_advice(f: &Facts, tier: ReliabilityTier) -> String {
    let basic = f.catalog.tier(ReliabilityTier::Basic).map_or(0.0, |s| s.daily_failure_chance);
    let proven = f.catalog.tier(ReliabilityTier::FlightProven).map_or(0.0, |s| s.daily_failure_chance);
    match tier {
        ReliabilityTier::Basic if proven > 0.0 => format!(
            "Flight-proven parts fail about {:.0} times less often; a {} part has about a {} chance of failing within {} days.",
            basic / proven,
            tier_name(f, tier),
            pct(chance_within(basic, f.scenario.min_lifetime_days)),
            f.scenario.min_lifetime_days
        ),
        ReliabilityTier::Basic => String::new(),
        ReliabilityTier::FlightProven => "Even Flight-proven parts fail now and then: this was bad luck.".to_string(),
    }
}

/// Chance a part with this daily failure chance fails at least once in `days`.
fn chance_within(daily: f64, days: u32) -> f64 {
    1.0 - (0..days).fold(1.0, |acc, _| acc * (1.0 - daily))
}

fn orbit_lifetime(f: &Facts) -> Option<Weighed> {
    let orbit = f.r.orbit?;
    let min = f.scenario.min_lifetime_days;
    if f.planned_days >= min {
        return None;
    }
    let short = min - f.planned_days;
    let why = if orbit.altitude_km < f.catalog.cause_rules.drag_altitude_below_km {
        format!("Air drag at {:.0} km pulled the spacecraft down", orbit.altitude_km)
    } else {
        format!("Radiation in {} wore the spacecraft out", orbit.name)
    };
    let better = f
        .catalog
        .orbits
        .iter()
        .filter(|o| o.lifetime_cap_days.is_none_or(|cap| cap >= f64::from(min)))
        .max_by(|a, b| cov(a, &f.scenario.id).total_cmp(&cov(b, &f.scenario.id)))
        .map_or(String::new(), |o| {
            format!(" The {} lasts long enough for {}.", o.name, f.scenario.name)
        });
    let text = format!(
        "{why} after {} days, {short} days before the {min} days {} needs.{better}",
        f.planned_days, f.scenario.name
    );
    // Days lost to the Orbit are only counted if a failure did not end the Run before.
    let lost_days = if f.days_flown() >= f.planned_days { short } else { 0 };
    let impact = f64::from(lost_days) * f.value_per_day + f.penalty(lost_days);
    Some(weighed(CauseKind::LifetimeShortfall, text, Some(f.planned_days), impact))
}

fn cov(o: &Orbit, scenario_id: &str) -> f64 {
    o.coverage.get(scenario_id).copied().unwrap_or(0.0)
}

fn poor_coverage(f: &Facts, rules: &CauseRules) -> Option<Weighed> {
    let orbit = f.r.orbit?;
    if f.coverage >= rules.poor_coverage_below {
        return None;
    }
    let best = f.catalog.orbits.iter().max_by(|a, b| cov(a, &f.scenario.id).total_cmp(&cov(b, &f.scenario.id)))?;
    let best_cov = cov(best, &f.scenario.id);
    if best_cov <= f.coverage {
        return None;
    }
    let per_cov = if f.coverage > 0.0 { f.value_per_day / f.coverage } else { 0.0 };
    let per_cov = if per_cov > 0.0 { per_cov } else { f.scenario.required_science_value / f64::from(f.planned_days.max(1)) };
    let impact = per_cov * (best_cov - f.coverage) * f64::from(f.planned_days);
    let text = format!(
        "The {} covers only {} of what {} needs, so even the Data that came down earned little Science Value. \
         The {} covers {}.",
        orbit.name,
        pct(f.coverage),
        f.scenario.name,
        best.name,
        pct(best_cov)
    );
    Some(weighed(CauseKind::PoorCoverage, text, None, impact))
}

fn poor_fit(f: &Facts, rules: &CauseRules) -> Vec<Weighed> {
    let id = &f.scenario.id;
    let Some(best) = f.catalog.instruments.iter().max_by(|a, b| fit(a, id).total_cmp(&fit(b, id))) else {
        return Vec::new();
    };
    let per_gb = f.scenario.science_value_per_gb / 1000.0;
    f.r.instruments
        .iter()
        .filter(|i| fit(i, id) < rules.poor_fit_below && fit(best, id) > fit(i, id))
        .map(|i| {
            let suits = if fit(i, id) > 0.0 {
                format!("suits {} only {}: its Data earns little Science Value here.", f.scenario.name, pct(fit(i, id)))
            } else {
                format!("does nothing for {}: its Data earns no Science Value here.", f.scenario.name)
            };
            let text = format!("Your {} {suits} The {} suits it {}.", i.name, best.name, pct(fit(best, id)));
            let impact = i.data_mb_per_day * (fit(best, id) - fit(i, id)) * f.coverage.max(0.1) * per_gb * f64::from(f.planned_days);
            weighed(CauseKind::PoorInstrumentFit, text, None, impact)
        })
        .collect()
}

fn science_shortfall(f: &Facts) -> Option<Weighed> {
    let days = f.scenario.min_lifetime_days;
    let best_case = f.value_per_day_all * f64::from(days);
    let required = f.scenario.required_science_value;
    if best_case >= required {
        return None;
    }
    let text = format!(
        "Even with every day perfect and all Data delivered, your Instruments could earn at most {} Science Value in \
         {days} days, short of the {} required. Carry more Instruments that suit {}.",
        value(best_case),
        value(required),
        f.scenario.name
    );
    Some(weighed(CauseKind::ScienceShortfall, text, None, required - best_case))
}

fn flares(f: &Facts) -> Option<Weighed> {
    let days: Vec<u32> = f
        .ctx
        .timeline
        .iter()
        .filter(|d| d.events.iter().any(|e| e.kind == EventKind::SolarFlare))
        .map(|d| d.day)
        .collect();
    if days.is_empty() {
        return None;
    }
    let flown = f.days_flown();
    let safe: Vec<&u32> = f.safe_mode_days.iter().filter(|&&d| d <= flown).collect();
    let impact = safe.len() as f64 * f.value_per_day;
    let count = if days.len() == 1 { "A solar flare".to_string() } else { format!("{} solar flares", days.len()) };
    let text = format!(
        "{count} (day {}) put the spacecraft in safe mode for {} days in all, costing about {} Science Value. No \
         Design can stop a flare: a margin above the required Science Value absorbs it.",
        list(&days),
        safe.len(),
        value(impact)
    );
    Some(weighed(CauseKind::RandomEvents, text, days.first().copied(), impact))
}

fn went_well(f: &Facts) -> Cause {
    let s = f.ctx.stats;
    let best = f.r.instruments.iter().max_by(|a, b| fit(a, &f.scenario.id).total_cmp(&fit(b, &f.scenario.id)));
    let instrument = best.map_or(String::new(), |i| {
        format!(" Your {} suits {} ({} fit)", i.name, f.scenario.name, pct(fit(i, &f.scenario.id)))
    });
    let orbit = f.r.orbit.map_or(String::new(), |o| format!(", the {} covers it ({})", o.name, pct(f.coverage)));
    let comms = f.r.comms.map_or(String::new(), |(c, _)| format!(", and the {} brought {} down", c.name, data(s.delivered_data_mb)));
    let text = format!(
        "Science Value {} of {} required, over all {} days.{instrument}{orbit}{comms}.",
        value(s.science_value),
        value(s.required_science_value),
        s.days_survived
    );
    Cause { kind: CauseKind::WentWell, text, day: None }
}

/// On a Success: critical Components flown at the Basic Reliability Tier, a risk that paid off this time.
fn risk_taken(f: &Facts) -> Option<Cause> {
    let r = &f.r;
    let mut names: Vec<String> = Vec::new();
    let mut add = |name: &str, t: ReliabilityTier, critical: bool| {
        if critical && t == ReliabilityTier::Basic {
            names.push(name.to_string());
        }
    };
    if let Some((o, t)) = r.solar { add(&o.name, t, o.critical) }
    if let Some((o, t)) = r.battery { add(&o.name, t, o.critical) }
    if let Some((o, t)) = r.attitude { add(&o.name, t, o.critical) }
    if let Some((o, t)) = r.comms { add(&o.name, t, o.critical) }
    add(&f.catalog.computer.name, f.ctx.design.computer_tier, f.catalog.computer.critical);
    if names.is_empty() {
        return None;
    }
    let daily = f.catalog.tier(ReliabilityTier::Basic).map_or(0.0, |s| s.daily_failure_chance);
    let any = chance_within(daily * names.len() as f64, f.scenario.min_lifetime_days);
    let one = if names.len() == 1 { "it" } else { "one of them" };
    let text = format!(
        "Risk taken: your {} flew at the {} Reliability Tier. That saved money, but there was about a {} chance \
         {one} would fail and end the Mission. This time it held.",
        list(&names),
        tier_name(f, ReliabilityTier::Basic),
        pct(any)
    );
    Some(Cause { kind: CauseKind::ScoreLimit, text, day: None })
}

/// On a clean Success: which margin bonus fell furthest short of its maximum.
fn margin_limit(f: &Facts) -> Option<Cause> {
    let s = f.ctx.stats;
    let rules = &f.catalog.scoring;
    let budget_left = share(f.ctx.budget_usd - s.total_cost_usd, f.ctx.budget_usd);
    let mass_left = f.ctx.capacity_kg.map_or(0.0, |cap| share(cap - s.total_mass_kg, cap));
    let budget_missed = (1.0 - budget_left) * rules.budget_margin_points;
    let mass_missed = (1.0 - mass_left) * rules.mass_margin_points;
    let text = if budget_missed >= mass_missed {
        format!(
            "What held the Score down: only {} of the Budget was left unspent, so the Budget bonus added {:.0} of a \
             possible {:.0} points. Cheaper Components or a cheaper launch would raise it.",
            pct(budget_left),
            budget_left * rules.budget_margin_points,
            rules.budget_margin_points
        )
    } else {
        format!(
            "What held the Score down: the spacecraft used {} of the Launch Vehicle's capacity, so the Mass bonus \
             added {:.0} of a possible {:.0} points. A lighter spacecraft would raise it.",
            pct(1.0 - mass_left),
            mass_left * rules.mass_margin_points,
            rules.mass_margin_points
        )
    };
    Some(Cause { kind: CauseKind::ScoreLimit, text, day: None })
}

// ---------------------------------------------------------------- wording helpers

fn share(left: f64, limit: f64) -> f64 {
    if limit > 0.0 { (left / limit).clamp(0.0, 1.0) } else { 0.0 }
}

fn pct(fraction: f64) -> String {
    format!("{:.0}%", fraction * 100.0)
}

fn value(v: f64) -> String {
    if v.abs() < 10.0 { format!("{v:.1}") } else { format!("{v:.0}") }
}

/// A Data volume in MB or GB.
fn data(mb: f64) -> String {
    if mb >= 1000.0 {
        format!("{:.1} GB", mb / 1000.0)
    } else if mb >= 10.0 {
        format!("{mb:.0} MB")
    } else {
        format!("{mb:.1} MB")
    }
}

/// "3", "3 and 40", "3, 40 and 200".
fn list<T: ToString>(items: &[T]) -> String {
    let words: Vec<String> = items.iter().map(T::to_string).collect();
    match words.split_last() {
        Some((last, rest)) if !rest.is_empty() => format!("{} and {last}", rest.join(", ")),
        Some((last, _)) => last.clone(),
        None => String::new(),
    }
}
