//! Resource maths shared by Launch Review and the Run: what a Design weighs, costs, makes and uses.
//! Both read these numbers, so a Design the gauges call healthy behaves the same way in flight.
use super::catalog::*;
use super::design::*;
use super::review::CostBreakdown;

/// Every option the Design refers to, resolved against the catalog (`None` = not chosen or unknown id).
pub(crate) struct Resolved<'a> {
    pub scenario: Option<&'a Scenario>,
    pub structure: Option<&'a Structure>,
    pub solar: Option<(&'a SolarPanel, ReliabilityTier)>,
    pub battery: Option<(&'a Battery, ReliabilityTier)>,
    pub attitude: Option<(&'a AttitudeControl, ReliabilityTier)>,
    pub comms: Option<(&'a Comms, ReliabilityTier)>,
    pub instruments: Vec<&'a Instrument>,
    pub orbit: Option<&'a Orbit>,
    pub launch_vehicle: Option<&'a LaunchVehicle>,
}

pub(crate) fn resolve<'a>(c: &'a Catalog, d: &Design, scenario_id: &str) -> Resolved<'a> {
    fn pick(p: &Option<Pick>) -> Option<(&str, ReliabilityTier)> {
        p.as_ref().map(|p| (p.id.as_str(), p.tier))
    }
    Resolved {
        scenario: c.scenario(scenario_id),
        structure: d.structure.as_deref().and_then(|id| c.structure(id)),
        solar: pick(&d.solar).and_then(|(id, t)| c.solar_panel(id).map(|o| (o, t))),
        battery: pick(&d.battery).and_then(|(id, t)| c.battery(id).map(|o| (o, t))),
        attitude: pick(&d.attitude).and_then(|(id, t)| c.attitude_control(id).map(|o| (o, t))),
        comms: pick(&d.comms).and_then(|(id, t)| c.comms_option(id).map(|o| (o, t))),
        instruments: d.instruments.iter().filter_map(|id| c.instrument(id)).collect(),
        orbit: d.orbit.as_deref().and_then(|id| c.orbit(id)),
        launch_vehicle: d.launch_vehicle.as_deref().and_then(|id| c.launch_vehicle(id)),
    }
}

/// Static totals and rates for one Design in one Scenario.
pub(crate) struct Resources {
    pub mass_kg: f64,
    pub volume_used_u: f64,
    pub cost: CostBreakdown,
    pub budget_usd: f64,
    pub min_lifetime_days: u32,
    /// Orbit-averaged generation, eclipse included.
    pub generation_w: f64,
    /// Orbit-averaged consumption.
    pub consumption_w: f64,
    pub eclipse_minutes: f64,
    /// Energy the battery must supply through one pass in Earth's shadow.
    pub eclipse_energy_wh: f64,
    pub data_mb_per_day: f64,
    pub downlink_mb_per_day: f64,
    /// Launch Vehicle capacity to the chosen Orbit; `None` = unreachable or not chosen.
    pub capacity_kg: Option<f64>,
    /// Expected Lifetime from the Orbit cap and the summed daily failure chances.
    pub lifetime_days: f64,
}

pub(crate) fn resources(catalog: &Catalog, design: &Design, r: &Resolved) -> Resources {
    let computer = &catalog.computer;
    let cost_mult = |t: ReliabilityTier| catalog.tier(t).map_or(1.0, |s| s.cost_multiplier);
    let fail_chance = |t: ReliabilityTier| catalog.tier(t).map_or(0.0, |s| s.daily_failure_chance);

    let mass_kg = r.structure.map_or(0.0, |o| o.mass_kg)
        + r.solar.map_or(0.0, |(o, _)| o.mass_kg)
        + r.battery.map_or(0.0, |(o, _)| o.mass_kg)
        + r.attitude.map_or(0.0, |(o, _)| o.mass_kg)
        + r.comms.map_or(0.0, |(o, _)| o.mass_kg)
        + computer.mass_kg
        + r.instruments.iter().map(|i| i.mass_kg).sum::<f64>();
    let volume_used_u = r.solar.map_or(0.0, |(o, _)| o.volume_u)
        + r.battery.map_or(0.0, |(o, _)| o.volume_u)
        + r.attitude.map_or(0.0, |(o, _)| o.volume_u)
        + r.comms.map_or(0.0, |(o, _)| o.volume_u)
        + computer.volume_u
        + r.instruments.iter().map(|i| i.volume_u).sum::<f64>();

    let spacecraft_usd = r.structure.map_or(0.0, |o| o.cost_usd)
        + r.solar.map_or(0.0, |(o, t)| o.cost_usd * cost_mult(t))
        + r.battery.map_or(0.0, |(o, t)| o.cost_usd * cost_mult(t))
        + r.attitude.map_or(0.0, |(o, t)| o.cost_usd * cost_mult(t))
        + r.comms.map_or(0.0, |(o, t)| o.cost_usd * cost_mult(t))
        + computer.cost_usd * cost_mult(design.computer_tier);
    let instruments_usd: f64 = r.instruments.iter().map(|i| i.cost_usd).sum();
    let launch_usd = r.launch_vehicle.map_or(0.0, |o| o.cost_usd);
    let min_lifetime_days = r.scenario.map_or(0, |s| s.min_lifetime_days);
    let operations_usd = r.comms.map_or(0.0, |(o, _)| o.ops_cost_usd_per_day) * f64::from(min_lifetime_days);
    let cost = CostBreakdown {
        spacecraft_usd,
        instruments_usd,
        launch_usd,
        operations_usd,
        total_usd: spacecraft_usd + instruments_usd + launch_usd + operations_usd,
    };
    let budget_usd = r.scenario.map_or(0.0, |s| s.budget_usd);

    let eclipse_fraction = r.orbit.map_or(0.0, |o| o.eclipse_fraction);
    let generation_w = match (r.solar, r.structure) {
        (Some((s, _)), Some(st)) => s.generation_w_per_u * st.size_u * (1.0 - eclipse_fraction),
        _ => 0.0,
    };
    let consumption_w = r.attitude.map_or(0.0, |(o, _)| o.power_w)
        + r.comms.map_or(0.0, |(o, _)| o.power_w)
        + computer.power_w
        + r.instruments.iter().map(|i| i.power_w).sum::<f64>();
    let eclipse_minutes = r.orbit.map_or(0.0, |o| o.period_min * o.eclipse_fraction);
    let eclipse_energy_wh = consumption_w * eclipse_minutes / 60.0;

    let data_mb_per_day: f64 = r.instruments.iter().map(|i| i.data_mb_per_day).sum();
    let contact_minutes = r.orbit.map_or(0.0, |o| o.contact_minutes_per_day);
    let downlink_mb_per_day = r.comms.map_or(0.0, |(o, _)| downlink_mb(o.downlink_mbps, contact_minutes));

    let capacity_kg = match (r.launch_vehicle, r.orbit) {
        (Some(lv), Some(o)) => lv.capacity_kg.get(&o.id).copied(),
        _ => None,
    };

    let daily_failure: f64 = r.solar.map_or(0.0, |(_, t)| fail_chance(t))
        + r.battery.map_or(0.0, |(_, t)| fail_chance(t))
        + r.attitude.map_or(0.0, |(_, t)| fail_chance(t))
        + r.comms.map_or(0.0, |(_, t)| fail_chance(t))
        + fail_chance(design.computer_tier);
    let failure_limited_days = if daily_failure > 0.0 { 1.0 / daily_failure } else { f64::INFINITY };
    let lifetime_days = r.orbit.map_or(failure_limited_days, |o| o.lifetime_cap_days.min(failure_limited_days));
    let lifetime_days = if lifetime_days.is_finite() { lifetime_days.floor() } else { 0.0 };

    Resources {
        mass_kg,
        volume_used_u,
        cost,
        budget_usd,
        min_lifetime_days,
        generation_w,
        consumption_w,
        eclipse_minutes,
        eclipse_energy_wh,
        data_mb_per_day,
        downlink_mb_per_day,
        capacity_kg,
        lifetime_days,
    }
}

/// Megabytes a link of `mbps` sends in `minutes` of ground contact.
pub(crate) fn downlink_mb(mbps: f64, minutes: f64) -> f64 {
    mbps * minutes * 60.0 / 8.0
}
