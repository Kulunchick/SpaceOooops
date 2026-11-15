//! The Debrief: assembled once the Run is over from the timeline and the Design's static numbers.
//! Each part (Score, causes, Personal Objectives, Real-Mission Match) lives in its own file.
use serde::{Deserialize, Serialize};

use super::catalog::Catalog;
use super::causes::{Cause, causes};
use super::design::Design;
use super::objectives::{ObjectiveResult, objectives};
use super::real_mission::{RealMissionMatch, real_mission_match};
use super::run::DaySnapshot;
use super::score::{Score, ScoreInput, ScoreLine, ScoreLineKind, score};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RunStats {
    pub science_value: f64,
    pub required_science_value: f64,
    pub days_survived: u32,
    pub min_lifetime_days: u32,
    pub total_cost_usd: f64,
    pub total_mass_kg: f64,
    /// Share of all Data produced that never reached the ground (0..=1).
    pub lost_data_fraction: f64,
    pub delivered_data_mb: f64,
    /// The required Science Value was reached.
    pub science_met: bool,
    /// The Run lasted the minimum Lifetime.
    pub lifetime_met: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Debrief {
    /// Required Science Value reached AND the minimum Lifetime survived.
    pub success: bool,
    /// One plain-language sentence on how the Run ended ("Planned mission complete: ...").
    pub ending: String,
    pub score: Score,
    pub causes: Vec<Cause>,
    pub objectives: Vec<ObjectiveResult>,
    pub real_mission_match: Option<RealMissionMatch>,
    /// Plain-language notes on decisions taken during the Run (ticket 10).
    pub decisions: Vec<String>,
    pub stats: RunStats,
    /// How each real new Random Event that happened happens in real life.
    #[serde(default)]
    pub real_life: Vec<String>,
    /// Which Design choices caused or softened a new Random Event.
    #[serde(default)]
    pub design_notes: Vec<String>,
}

/// Everything the Debrief parts may look at.
#[allow(dead_code)] // design, scenario_id and timeline are for causes, objectives and the match (tickets 04, 06)
pub(crate) struct DebriefContext<'a> {
    pub catalog: &'a Catalog,
    pub design: &'a Design,
    pub scenario_id: &'a str,
    pub timeline: &'a [DaySnapshot],
    pub stats: &'a RunStats,
    pub success: bool,
    pub ending: String,
    /// Budget of the Scenario and Launch Vehicle capacity to the Orbit, for margin bonuses.
    pub budget_usd: f64,
    pub capacity_kg: Option<f64>,
    /// Score lines from new Random Events, after the Personal Objectives.
    pub event_score_lines: Vec<ScoreLine>,
}

/// The Debrief of a Design that did not pass Launch Review: it never flew, so it scores nothing.
pub(crate) fn not_launched(stats: RunStats, causes: Vec<Cause>) -> Debrief {
    Debrief {
        success: false,
        ending: "Not launched: the Design does not pass Launch Review, so it never flew.".to_string(),
        score: Score::from_lines(vec![ScoreLine {
            kind: ScoreLineKind::Penalty,
            label: "Not launched: no Science Value and no bonuses".to_string(),
            points: 0.0,
        }]),
        causes,
        objectives: Vec::new(),
        real_mission_match: None,
        decisions: Vec::new(),
        stats,
        real_life: Vec::new(),
        design_notes: Vec::new(),
    }
}

pub(crate) fn debrief(ctx: &DebriefContext) -> Debrief {
    let s = ctx.stats;
    let objectives = objectives(ctx);
    let mut lines = score(
        &ctx.catalog.scoring,
        &ScoreInput {
            science_value: s.science_value,
            total_cost_usd: s.total_cost_usd,
            budget_usd: ctx.budget_usd,
            total_mass_kg: s.total_mass_kg,
            capacity_kg: ctx.capacity_kg,
            days_survived: s.days_survived,
            min_lifetime_days: s.min_lifetime_days,
            success: ctx.success,
        },
    );
    lines.extend(objectives.iter().filter(|o| o.met).map(|o| ScoreLine {
        kind: ScoreLineKind::PersonalObjective,
        label: o.description.clone(),
        points: o.bonus,
    }));
    lines.extend(ctx.event_score_lines.iter().cloned());
    Debrief {
        success: ctx.success,
        ending: ctx.ending.clone(),
        score: Score::from_lines(lines),
        causes: causes(ctx),
        objectives,
        real_mission_match: real_mission_match(ctx),
        decisions: Vec::new(),
        stats: s.clone(),
        real_life: Vec::new(),
        design_notes: Vec::new(),
    }
}
