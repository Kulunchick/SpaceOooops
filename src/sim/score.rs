//! The Score: Science Value plus margin bonuses (and, later, Personal Objectives) minus penalties.
//! Every weight comes from `data/scoring.toml`; `total` is always the sum of the lines.
use serde::{Deserialize, Serialize};

use super::catalog::ScoringRules;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScoreLineKind {
    ScienceValue,
    BudgetMargin,
    MassMargin,
    PersonalObjective,
    Penalty,
    /// A bonus from a new Random Event (helping rescuers); never part of Science Value.
    EventBonus,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScoreLine {
    pub kind: ScoreLineKind,
    /// Plain-language label shown next to the points.
    pub label: String,
    /// Negative for penalties.
    pub points: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Score {
    /// Always the sum of `lines`.
    pub total: f64,
    pub lines: Vec<ScoreLine>,
}

impl Score {
    pub fn from_lines(lines: Vec<ScoreLine>) -> Self {
        let total = lines.iter().map(|l| l.points).sum();
        Score { total, lines }
    }
}

/// What the Score is computed from.
pub(crate) struct ScoreInput {
    pub science_value: f64,
    pub total_cost_usd: f64,
    pub budget_usd: f64,
    pub total_mass_kg: f64,
    /// Launch Vehicle capacity to the Orbit; `None` gives no Mass bonus.
    pub capacity_kg: Option<f64>,
    pub days_survived: u32,
    pub min_lifetime_days: u32,
    pub success: bool,
}

pub(crate) fn score(rules: &ScoringRules, s: &ScoreInput) -> Vec<ScoreLine> {
    let mut lines = vec![ScoreLine {
        kind: ScoreLineKind::ScienceValue,
        label: format!("Science Value ({:.1})", s.science_value),
        points: s.science_value * rules.points_per_science_value,
    }];
    let paid = s.success || !rules.margin_bonuses_need_success;
    let unpaid = if paid { "" } else { "; paid only on a Success" };
    let budget_margin = margin(s.budget_usd - s.total_cost_usd, s.budget_usd);
    lines.push(ScoreLine {
        kind: ScoreLineKind::BudgetMargin,
        label: format!("Unspent Budget ({:.0}% left{unpaid})", budget_margin * 100.0),
        points: if paid { budget_margin * rules.budget_margin_points } else { 0.0 },
    });
    let mass_margin = s.capacity_kg.map_or(0.0, |cap| margin(cap - s.total_mass_kg, cap));
    lines.push(ScoreLine {
        kind: ScoreLineKind::MassMargin,
        label: format!("Mass margin ({:.0}% of the Launch Vehicle's capacity unused{unpaid})", mass_margin * 100.0),
        points: if paid { mass_margin * rules.mass_margin_points } else { 0.0 },
    });
    if s.days_survived < s.min_lifetime_days && s.min_lifetime_days > 0 {
        let missed = f64::from(s.min_lifetime_days - s.days_survived) / f64::from(s.min_lifetime_days);
        lines.push(ScoreLine {
            kind: ScoreLineKind::Penalty,
            label: format!(
                "Mission ended early: {} of the {} days required were flown",
                s.days_survived, s.min_lifetime_days
            ),
            points: -missed * rules.early_end_penalty_points,
        });
    }
    lines
}

/// Share of `limit` left over, clamped to `0..=1`.
fn margin(left: f64, limit: f64) -> f64 {
    if limit > 0.0 { (left / limit).clamp(0.0, 1.0) } else { 0.0 }
}
