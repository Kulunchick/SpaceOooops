//! Launch Review: resource totals, margins, hard-constraint violations and soft warnings for a Design.
//! Never panics on a partial or nonsensical Design; the UI calls it on every click.
use serde::{Deserialize, Serialize};

use super::catalog::*;
use super::design::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GaugeState {
    Ok,
    Thin,
    Violated,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Gauge {
    pub value: f64,
    /// A ceiling for budget, mass, power and data; a floor for lifetime.
    pub limit: f64,
    pub unit: String,
    pub state: GaugeState,
    /// Short human string, e.g. "9.4 / 12.0 kg".
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Gauges {
    pub budget: Gauge,
    pub mass: Gauge,
    pub power: Gauge,
    pub data: Gauge,
    pub lifetime: Gauge,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CostBreakdown {
    pub spacecraft_usd: f64,
    pub instruments_usd: f64,
    pub launch_usd: f64,
    pub operations_usd: f64,
    pub total_usd: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FindingKind {
    // Hard constraints: block launch.
    OverMass,
    OverBudget,
    NegativePower,
    NeedsReactionWheels,
    VolumeExceeded,
    NoComms,
    Incomplete,
    OrbitUnreachable,
    // Soft warnings: never block launch.
    ThinMargin,
    BatteryBarelyCoversEclipse,
    BatteryTooSmallForEclipse,
    DataAboveDownlink,
    BasicTierOnCritical,
    ShortLifetime,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Finding {
    pub kind: FindingKind,
    pub role: Role,
    /// The full plain-language sentence shown to the player.
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Incompatibility {
    pub option_id: String,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LaunchReview {
    pub gauges: Gauges,
    pub cost: CostBreakdown,
    pub violations: Vec<Finding>,
    pub warnings: Vec<Finding>,
    pub incompatible: Vec<Incompatibility>,
    /// True exactly when `violations` is empty.
    pub can_launch: bool,
}

/// Every option the Design refers to, resolved against the catalog (`None` = not chosen or unknown id).
struct Resolved<'a> {
    scenario: Option<&'a Scenario>,
    structure: Option<&'a Structure>,
    solar: Option<(&'a SolarPanel, ReliabilityTier)>,
    battery: Option<(&'a Battery, ReliabilityTier)>,
    attitude: Option<(&'a AttitudeControl, ReliabilityTier)>,
    comms: Option<(&'a Comms, ReliabilityTier)>,
    instruments: Vec<&'a Instrument>,
    orbit: Option<&'a Orbit>,
    launch_vehicle: Option<&'a LaunchVehicle>,
}

fn resolve<'a>(c: &'a Catalog, d: &Design, scenario_id: &str) -> Resolved<'a> {
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

/// Reviews a Design against a Scenario.
pub fn launch_review(catalog: &Catalog, design: &Design, scenario_id: &str) -> LaunchReview {
    let r = resolve(catalog, design, scenario_id);
    let rules = &catalog.review;
    let thin = rules.thin_margin_fraction;
    let computer = &catalog.computer;
    let cost_mult = |t: ReliabilityTier| catalog.tier(t).map_or(1.0, |s| s.cost_multiplier);
    let fail_chance = |t: ReliabilityTier| catalog.tier(t).map_or(0.0, |s| s.daily_failure_chance);

    // ---- totals
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
    let downlink_mb_per_day = r.comms.map_or(0.0, |(o, _)| o.downlink_mbps * contact_minutes * 60.0 / 8.0);

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

    // ---- gauges
    let gauges = Gauges {
        budget: ceiling_gauge(cost.total_usd, budget_usd, "USD", thin, format!("{} / {}", money(cost.total_usd), money(budget_usd))),
        mass: ceiling_gauge(
            mass_kg,
            capacity_kg.unwrap_or(0.0),
            "kg",
            thin,
            match capacity_kg {
                Some(cap) => format!("{mass_kg:.1} / {cap:.1} kg"),
                None => format!("{mass_kg:.1} kg / no capacity"),
            },
        ),
        power: ceiling_gauge(
            consumption_w,
            generation_w,
            "W",
            thin,
            format!("{consumption_w:.1} W used / {generation_w:.1} W made (orbit average)"),
        ),
        data: ceiling_gauge(
            data_mb_per_day,
            downlink_mb_per_day,
            "MB/day",
            thin,
            format!("{} / {} MB per day", mb(data_mb_per_day), mb(downlink_mb_per_day)),
        ),
        lifetime: floor_gauge(
            lifetime_days,
            f64::from(min_lifetime_days),
            "days",
            thin,
            format!(
                "{lifetime_days:.0} days (about {:.0} months) / at least {min_lifetime_days} days",
                lifetime_days / 30.4
            ),
        ),
    };

    // ---- hard constraints
    let mut violations = Vec::new();
    let mut hard = |kind, role, reason: String| violations.push(Finding { kind, role, reason });

    if r.scenario.is_none() {
        hard(FindingKind::Incomplete, Role::BudgetComms, format!("The Scenario '{scenario_id}' is not in the catalog."));
    }
    if r.instruments.is_empty() {
        hard(FindingKind::Incomplete, Role::Science, "Pick at least one Instrument: without one the Mission returns no Science Value.".into());
    }
    for id in design.instruments.iter().filter(|id| catalog.instrument(id).is_none()) {
        hard(FindingKind::Incomplete, Role::Science, format!("The Instrument '{id}' is not in the catalog. Remove it."));
    }
    let missing = [
        (r.structure.is_none(), Role::Spacecraft, "Pick a Structure: it sets how much fits on board."),
        (r.solar.is_none(), Role::Spacecraft, "Pick solar panels: without them the spacecraft has no Power."),
        (r.battery.is_none(), Role::Spacecraft, "Pick a battery: it keeps the spacecraft alive in Earth's shadow."),
        (r.attitude.is_none(), Role::Spacecraft, "Pick an attitude control system: it stops the spacecraft tumbling."),
        (r.orbit.is_none(), Role::LaunchOrbit, "Pick an Orbit."),
        (r.launch_vehicle.is_none(), Role::LaunchOrbit, "Pick a Launch Vehicle to carry the spacecraft to Orbit."),
    ];
    for (is_missing, role, reason) in missing {
        if is_missing {
            hard(FindingKind::Incomplete, role, reason.to_string());
        }
    }

    if let (Some(lv), Some(orbit)) = (r.launch_vehicle, r.orbit) {
        match capacity_kg {
            None => hard(
                FindingKind::OrbitUnreachable,
                Role::LaunchOrbit,
                format!("The {} cannot reach {}. Choose another Launch Vehicle or Orbit.", lv.name, orbit.name),
            ),
            Some(cap) if mass_kg > cap => hard(
                FindingKind::OverMass,
                Role::LaunchOrbit,
                format!(
                    "The spacecraft's Mass is {mass_kg:.1} kg, but the {} can carry only {cap:.1} kg to {}. Remove {:.1} kg or choose a bigger Launch Vehicle.",
                    lv.name,
                    orbit.name,
                    mass_kg - cap
                ),
            ),
            Some(_) => {}
        }
    }
    if r.scenario.is_some() && cost.total_usd > budget_usd {
        hard(
            FindingKind::OverBudget,
            Role::BudgetComms,
            format!(
                "The Mission costs {}, which is {} over the Budget of {}. Choose cheaper options.",
                money(cost.total_usd),
                money(cost.total_usd - budget_usd),
                money(budget_usd)
            ),
        );
    }
    if let (Some((solar, _)), Some(_), Some(orbit)) = (r.solar, r.structure, r.orbit) {
        if consumption_w > generation_w {
            hard(
                FindingKind::NegativePower,
                Role::Spacecraft,
                format!(
                    "On average the spacecraft uses {consumption_w:.1} W but the {} make only {generation_w:.1} W, because it spends {:.0}% of each orbit in Earth's shadow in {}. The battery would drain to empty. Add Power or remove Power-hungry parts.",
                    solar.name.to_lowercase(),
                    orbit.eclipse_fraction * 100.0,
                    orbit.name
                ),
            );
        }
    }
    if let Some((attitude, _)) = r.attitude {
        if !attitude.fine_pointing {
            let pointers = fine_pointing_names(catalog);
            for instrument in r.instruments.iter().filter(|i| i.needs_fine_pointing) {
                hard(
                    FindingKind::NeedsReactionWheels,
                    Role::Science,
                    format!(
                        "The {} must point precisely at its target, but {} cannot hold the spacecraft steady enough. Choose {} or drop this Instrument.",
                        instrument.name, attitude.name, pointers
                    ),
                );
            }
        }
    }
    if let Some(structure) = r.structure {
        if volume_used_u > structure.usable_volume_u {
            hard(
                FindingKind::VolumeExceeded,
                Role::Spacecraft,
                format!(
                    "The parts on board take {volume_used_u:.1}U, but the {} has room for only {:.1}U. Choose a bigger Structure or fewer, smaller parts.",
                    structure.name, structure.usable_volume_u
                ),
            );
        }
    }
    if design.comms.is_none() {
        hard(
            FindingKind::NoComms,
            Role::BudgetComms,
            "The spacecraft has no comms link: it could never send its Data or health to the ground. Pick a comms band.".into(),
        );
    } else if r.comms.is_none() {
        hard(FindingKind::Incomplete, Role::BudgetComms, "The chosen comms band is not in the catalog. Pick another.".into());
    }

    // ---- soft warnings
    let mut warnings = Vec::new();
    let mut soft = |kind, role, reason: String| warnings.push(Finding { kind, role, reason });
    let thin_pct = thin * 100.0;
    if gauges.budget.state == GaugeState::Thin {
        soft(FindingKind::ThinMargin, Role::BudgetComms, format!("Less than {thin_pct:.0}% of the Budget is left unspent, so there is little money for surprises."));
    }
    if gauges.mass.state == GaugeState::Thin && capacity_kg.is_some() {
        soft(FindingKind::ThinMargin, Role::LaunchOrbit, format!("The spacecraft is within {thin_pct:.0}% of the Launch Vehicle's Mass limit; real spacecraft always grow heavier while being built."));
    }
    if gauges.power.state == GaugeState::Thin && r.solar.is_some() {
        soft(FindingKind::ThinMargin, Role::Spacecraft, format!("Power use is within {thin_pct:.0}% of what the solar panels make; ageing cells or a failed string would leave the spacecraft short."));
    }
    if gauges.data.state == GaugeState::Thin && r.comms.is_some() {
        soft(FindingKind::ThinMargin, Role::BudgetComms, format!("The Instruments fill more than {:.0}% of the daily downlink; one missed ground pass and Data starts piling up.", 100.0 - thin_pct));
    }
    if r.comms.is_some() && data_mb_per_day > downlink_mb_per_day {
        soft(
            FindingKind::DataAboveDownlink,
            Role::BudgetComms,
            format!(
                "The Instruments make {} MB of Data per day, but the comms link can send only {} MB per day. The rest piles up on board and is eventually lost. Choose a faster comms band or fewer Instruments.",
                mb(data_mb_per_day),
                mb(downlink_mb_per_day)
            ),
        );
    }
    if let (Some((battery, _)), Some(_)) = (r.battery, r.orbit) {
        let depth = eclipse_energy_wh / battery.capacity_wh;
        if depth > 1.0 {
            soft(
                FindingKind::BatteryTooSmallForEclipse,
                Role::Spacecraft,
                format!(
                    "Each pass through Earth's shadow ({eclipse_minutes:.0} min) needs {eclipse_energy_wh:.1} Wh, more than the {} holds. The spacecraft will shut down in every eclipse. Choose a bigger battery or use less Power.",
                    battery.name
                ),
            );
        } else if depth > rules.battery_max_depth_of_discharge {
            soft(
                FindingKind::BatteryBarelyCoversEclipse,
                Role::Spacecraft,
                format!(
                    "Each pass through Earth's shadow ({eclipse_minutes:.0} min) uses {:.0}% of the {}. Batteries drained deeper than {:.0}% every orbit wear out fast. A bigger battery would last longer.",
                    depth * 100.0,
                    battery.name,
                    rules.battery_max_depth_of_discharge * 100.0
                ),
            );
        }
    }
    let critical_basic = [
        r.solar.filter(|(o, _)| o.critical).map(|(o, t)| (o.name.as_str(), t)),
        r.battery.filter(|(o, _)| o.critical).map(|(o, t)| (o.name.as_str(), t)),
        r.attitude.filter(|(o, _)| o.critical).map(|(o, t)| (o.name.as_str(), t)),
        r.comms.filter(|(o, _)| o.critical).map(|(o, t)| (o.name.as_str(), t)),
        computer.critical.then_some((computer.name.as_str(), design.computer_tier)),
    ];
    for (name, _) in critical_basic.into_iter().flatten().filter(|(_, t)| *t == ReliabilityTier::Basic) {
        soft(
            FindingKind::BasicTierOnCritical,
            Role::Spacecraft,
            format!("The {name} is Basic tier, and the Mission ends if it fails. A Flight-proven one costs more but fails far less often."),
        );
    }
    if gauges.lifetime.state == GaugeState::Violated && r.scenario.is_some() && lifetime_days > 0.0 {
        soft(
            FindingKind::ShortLifetime,
            Role::Spacecraft,
            format!(
                "The expected Lifetime is about {lifetime_days:.0} days, shorter than the {min_lifetime_days} days the Scenario needs. Flight-proven Components fail less often and last longer."
            ),
        );
    } else if gauges.lifetime.state == GaugeState::Thin && r.scenario.is_some() {
        soft(
            FindingKind::ThinMargin,
            Role::Spacecraft,
            format!("The expected Lifetime is only just above the {min_lifetime_days} days the Scenario needs; one early failure would cut the Mission short."),
        );
    }

    let incompatible = incompatibilities(catalog, &r);
    let can_launch = violations.is_empty();
    LaunchReview { gauges, cost, violations, warnings, incompatible, can_launch }
}

/// Options that clash with the current choices, each with the reason.
fn incompatibilities(catalog: &Catalog, r: &Resolved) -> Vec<Incompatibility> {
    let mut out = Vec::new();
    if let Some((attitude, _)) = r.attitude.filter(|(a, _)| !a.fine_pointing) {
        for instrument in catalog.instruments.iter().filter(|i| i.needs_fine_pointing) {
            out.push(Incompatibility {
                option_id: instrument.id.clone(),
                reason: format!("Needs precise pointing, which {} cannot provide.", attitude.name),
            });
        }
    }
    if let Some(pointer) = r.instruments.iter().find(|i| i.needs_fine_pointing) {
        for attitude in catalog.attitude_controls.iter().filter(|a| !a.fine_pointing) {
            out.push(Incompatibility {
                option_id: attitude.id.clone(),
                reason: format!("Cannot point the {} precisely enough.", pointer.name),
            });
        }
    }
    if let Some(orbit) = r.orbit {
        for lv in catalog.launch_vehicles.iter().filter(|lv| !lv.capacity_kg.contains_key(&orbit.id)) {
            out.push(Incompatibility { option_id: lv.id.clone(), reason: format!("Cannot reach {}.", orbit.name) });
        }
    }
    if let Some(lv) = r.launch_vehicle {
        for orbit in catalog.orbits.iter().filter(|o| !lv.capacity_kg.contains_key(&o.id)) {
            out.push(Incompatibility { option_id: orbit.id.clone(), reason: format!("The {} cannot reach this Orbit.", lv.name) });
        }
    }
    out
}

fn fine_pointing_names(catalog: &Catalog) -> String {
    let names: Vec<&str> = catalog.attitude_controls.iter().filter(|a| a.fine_pointing).map(|a| a.name.as_str()).collect();
    if names.is_empty() { "a pointing system".to_string() } else { names.join(" or ") }
}

fn ceiling_gauge(value: f64, limit: f64, unit: &str, thin: f64, detail: String) -> Gauge {
    let state = if value > limit {
        GaugeState::Violated
    } else if limit - value < thin * limit {
        GaugeState::Thin
    } else {
        GaugeState::Ok
    };
    Gauge { value, limit, unit: unit.to_string(), state, detail }
}

fn floor_gauge(value: f64, limit: f64, unit: &str, thin: f64, detail: String) -> Gauge {
    let state = if value < limit {
        GaugeState::Violated
    } else if value < limit * (1.0 + thin) {
        GaugeState::Thin
    } else {
        GaugeState::Ok
    };
    Gauge { value, limit, unit: unit.to_string(), state, detail }
}

/// "$1.35M", "$950k", "$800".
pub(crate) fn money(usd: f64) -> String {
    if usd.abs() >= 1_000_000.0 {
        format!("${:.2}M", usd / 1_000_000.0)
    } else if usd.abs() >= 1_000.0 {
        format!("${:.0}k", usd / 1_000.0)
    } else {
        format!("${usd:.0}")
    }
}

fn mb(value: f64) -> String {
    if value < 10.0 { format!("{value:.1}") } else { format!("{value:.0}") }
}
