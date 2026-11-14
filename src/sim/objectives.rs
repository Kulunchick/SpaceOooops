//! Personal Objectives: each Role's bonus goal, defined in `data/objectives.toml` as a rule kind plus a
//! threshold. Evaluated in the Debrief, and previewed before launch for Go/No-Go.
//! Like the Budget and Mass margin bonuses, a bonus only counts on a successful Run.
use serde::{Deserialize, Serialize};

use super::catalog::Catalog;
use super::debrief::DebriefContext;
use super::design::{Design, Role};
use super::run::run;

/// The fixed set of rule kinds; thresholds live in data.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ObjectiveRule {
    /// Total cost as a share of the Budget stays at or below the threshold.
    CostShareOfBudgetAtMost { threshold: f64 },
    /// Unused share of the Launch Vehicle's capacity is at least the threshold.
    MassMarginAtLeast { threshold: f64 },
    /// Science Value reaches the required value plus this extra share.
    ScienceAboveRequiredAtLeast { threshold: f64 },
    /// Share of Data produced that never reached the ground stays at or below the threshold.
    LostDataAtMost { threshold: f64 },
    /// The lowest end-of-day battery charge over the Run stays at or above the threshold.
    BatteryChargeNeverBelow { threshold: f64 },
}

/// One Personal Objective as defined in data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObjectiveSpec {
    pub id: String,
    pub role: Role,
    /// `None` = the Role's default; `Some` replaces the default in that Scenario.
    #[serde(default)]
    pub scenario: Option<String>,
    pub description: String,
    /// Score points on offer.
    pub bonus: f64,
    pub rule: ObjectiveRule,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObjectiveResult {
    pub role: Role,
    pub description: String,
    /// Rule held AND the Run succeeded.
    pub met: bool,
    /// Score points on offer; added to the Score (as a `PersonalObjective` line) only when `met`.
    pub bonus: f64,
    /// Plain-language account with the actual numbers.
    #[serde(default)]
    pub detail: String,
}

/// Pre-launch estimate of one Role's Personal Objective, from a trial Run of the current Design.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObjectiveOutlook {
    pub role: Role,
    pub description: String,
    pub bonus: f64,
    /// The trial Run missed the objective: the Role may reasonably vote No-Go over it.
    pub at_risk: bool,
    /// Plain-language reason, shown as the objection when the Role votes No-Go.
    pub reason: String,
}

/// The Personal Objectives that apply in a Scenario: one per Role that has one (Role order), with a
/// Scenario-specific entry replacing the Role's default.
pub fn scenario_objectives(catalog: &Catalog, scenario_id: &str) -> Vec<ObjectiveSpec> {
    Role::ALL
        .into_iter()
        .filter_map(|role| {
            let of_role = || catalog.objectives.iter().filter(move |o| o.role == role);
            of_role()
                .find(|o| o.scenario.as_deref() == Some(scenario_id))
                .or_else(|| of_role().find(|o| o.scenario.is_none()))
                .cloned()
        })
        .collect()
}

/// Estimates before launch whether each Role's Personal Objective looks at risk for this Design, by
/// flying it once with the data-defined preview seed. A Design that cannot succeed earns no bonus.
/// An estimate only: the real Run uses a fresh seed.
pub fn objective_outlook(catalog: &Catalog, design: &Design, scenario_id: &str) -> Vec<ObjectiveOutlook> {
    let trial = run(catalog, design, scenario_id, catalog.objectives_preview_seed);
    let results = trial.debrief.objectives;
    scenario_objectives(catalog, scenario_id)
        .into_iter()
        .map(|spec| {
            let result = results.iter().find(|r| r.role == spec.role);
            let met = result.is_some_and(|r| r.met);
            let detail = result.map_or_else(
                || "This Design cannot be flown, so no Personal Objective can be earned.".to_string(),
                |r| r.detail.clone(),
            );
            let reason = if met {
                format!("On track in a trial Run. {detail}")
            } else {
                format!("At risk in a trial Run. {detail}")
            };
            ObjectiveOutlook {
                role: spec.role,
                description: spec.description,
                bonus: spec.bonus,
                at_risk: !met,
                reason,
            }
        })
        .collect()
}

/// Personal Objective results for a Run.
pub(crate) fn objectives(ctx: &DebriefContext) -> Vec<ObjectiveResult> {
    let s = ctx.stats;
    let min_battery = ctx.timeline.iter().map(|d| d.battery_charge).fold(f64::INFINITY, f64::min);
    scenario_objectives(ctx.catalog, ctx.scenario_id)
        .into_iter()
        .map(|spec| {
            let (holds, detail) = match spec.rule {
                ObjectiveRule::CostShareOfBudgetAtMost { threshold } => {
                    let share = if ctx.budget_usd > 0.0 { s.total_cost_usd / ctx.budget_usd } else { f64::INFINITY };
                    (
                        share <= threshold,
                        format!(
                            "The Design uses {:.0}% of the Budget; the goal is {:.0}% or less.",
                            share * 100.0,
                            threshold * 100.0
                        ),
                    )
                }
                ObjectiveRule::MassMarginAtLeast { threshold } => match ctx.capacity_kg {
                    Some(cap) if cap > 0.0 => {
                        let margin = (cap - s.total_mass_kg) / cap;
                        (
                            margin >= threshold,
                            format!(
                                "{:.0}% of the Launch Vehicle's capacity is unused; the goal is {:.0}% or more.",
                                margin.max(0.0) * 100.0,
                                threshold * 100.0
                            ),
                        )
                    }
                    _ => (false, "No Launch Vehicle capacity to the chosen Orbit, so there is no Mass margin.".to_string()),
                },
                ObjectiveRule::ScienceAboveRequiredAtLeast { threshold } => {
                    let target = s.required_science_value * (1.0 + threshold);
                    (
                        s.science_value >= target,
                        format!("Science Value reached {:.0}; the goal is {:.0} or more.", s.science_value, target),
                    )
                }
                ObjectiveRule::LostDataAtMost { threshold } => (
                    s.lost_data_fraction <= threshold,
                    format!(
                        "{:.0}% of the Data produced never reached the ground; the goal is {:.0}% or less.",
                        s.lost_data_fraction * 100.0,
                        threshold * 100.0
                    ),
                ),
                ObjectiveRule::BatteryChargeNeverBelow { threshold } => {
                    if min_battery.is_finite() {
                        (
                            min_battery >= threshold,
                            format!(
                                "The battery's lowest charge was {:.0}%; the goal is to stay at {:.0}% or more.",
                                min_battery * 100.0,
                                threshold * 100.0
                            ),
                        )
                    } else {
                        (false, "The spacecraft never flew, so the battery cannot be judged.".to_string())
                    }
                }
            };
            let met = holds && ctx.success;
            let detail = if holds && !ctx.success {
                format!("{detail} But bonuses only count when the Mission succeeds.")
            } else {
                detail
            };
            ObjectiveResult { role: spec.role, description: spec.description, met, bonus: spec.bonus, detail }
        })
        .collect()
}
