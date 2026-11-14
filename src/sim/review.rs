//! Launch Review: resource totals, margins, hard-constraint violations and soft warnings for a Design.
//! Never panics on a partial or nonsensical Design; the UI calls it on every click.
use serde::{Deserialize, Serialize};

use super::catalog::*;
use super::design::*;
use super::resources::{Resolved, Resources, resolve, resources};

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

/// Reviews a Design against a Scenario.
pub fn launch_review(catalog: &Catalog, design: &Design, scenario_id: &str) -> LaunchReview {
    let r = resolve(catalog, design, scenario_id);
    let rules = &catalog.review;
    let thin = rules.thin_margin_fraction;
    let computer = &catalog.computer;
    let Resources {
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
    } = resources(catalog, design, &r);

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
                None if design.launch_vehicle.is_none() || design.orbit.is_none() => {
                    format!("{mass_kg:.1} kg / Launch Vehicle or Orbit not chosen yet")
                }
                None => format!("{mass_kg:.1} kg / this Launch Vehicle cannot reach this Orbit"),
            },
        ),
        power: ceiling_gauge(
            consumption_w,
            generation_w,
            "W",
            thin,
            if design.solar.is_none() || design.orbit.is_none() {
                format!("{consumption_w:.1} W used / solar panels or Orbit not chosen yet")
            } else {
                format!("{consumption_w:.1} W used / {generation_w:.1} W made (orbit average)")
            },
        ),
        data: ceiling_gauge(
            data_mb_per_day,
            downlink_mb_per_day,
            "MB/day",
            thin,
            if design.comms.is_none() || design.orbit.is_none() {
                format!("{} MB per day / comms or Orbit not chosen yet", mb(data_mb_per_day))
            } else {
                format!("{} / {} MB per day", mb(data_mb_per_day), mb(downlink_mb_per_day))
            },
        ),
        lifetime: floor_gauge(
            lifetime_days,
            f64::from(min_lifetime_days),
            "days",
            thin,
            if [design.solar.is_none(), design.battery.is_none(), design.attitude.is_none(), design.comms.is_none()]
                .contains(&true)
                || design.orbit.is_none()
            {
                format!("Components or Orbit not chosen yet / at least {min_lifetime_days} days")
            } else {
                format!(
                    "{lifetime_days:.0} days (about {:.0} months) / at least {min_lifetime_days} days",
                    lifetime_days / 30.4
                )
            },
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
    if let (Some((solar, _)), Some(_), Some(orbit)) = (r.solar, r.structure, r.orbit)
        && consumption_w > generation_w {
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
    if let Some((attitude, _)) = r.attitude
        && !attitude.fine_pointing {
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
    if let Some(structure) = r.structure
        && volume_used_u > structure.usable_volume_u {
            hard(
                FindingKind::VolumeExceeded,
                Role::Spacecraft,
                format!(
                    "The parts on board take {volume_used_u:.1}U, but the {} has room for only {:.1}U. Choose a bigger Structure or fewer, smaller parts.",
                    structure.name, structure.usable_volume_u
                ),
            );
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
            format!(
                "The {name} {} Basic tier, and the Mission ends if {}. A Flight-proven choice costs more but fails far less often.",
                if name.ends_with('s') { "are" } else { "is" },
                if name.ends_with('s') { "they fail" } else { "it fails" }
            ),
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
    let value = value + 0.0; // an empty sum is -0.0; adding +0.0 makes it +0.0, so "-0.0" is never shown
    if value < 10.0 { format!("{value:.1}") } else { format!("{value:.0}") }
}
