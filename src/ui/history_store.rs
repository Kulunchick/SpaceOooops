//! Run history kept in browser local storage: one compact line per Run, newest first.
//! Every number comes from the Debrief; nothing is recomputed. Corrupt or old lines are skipped.
use spaceooops::sim::{self, Design, RunResult};

use super::storage;

const KEY: &str = "spaceooops.history.v1";
const LINE_TAG: &str = "h1";
/// Oldest Runs are dropped beyond this many.
pub const MAX_RECORDS: usize = 30;

#[derive(Clone, Debug, PartialEq)]
pub struct HistoryRecord {
    pub scenario_id: String,
    /// The Design as `sim::encode_design` text (carries its own version).
    pub design_code: String,
    pub seed: u64,
    pub timestamp_ms: f64,
    pub success: bool,
    pub score: f64,
    pub science_value: f64,
    pub cost_usd: f64,
    pub mass_kg: f64,
    pub days_survived: u32,
}

impl HistoryRecord {
    pub fn from_run(design: &Design, scenario_id: &str, seed: u64, result: &RunResult, timestamp_ms: f64) -> Self {
        let d = &result.debrief;
        HistoryRecord {
            scenario_id: scenario_id.to_string(),
            design_code: sim::encode_design(design, scenario_id),
            seed,
            timestamp_ms,
            success: d.success,
            score: d.score.total,
            science_value: d.stats.science_value,
            cost_usd: d.stats.total_cost_usd,
            mass_kg: d.stats.total_mass_kg,
            days_survived: d.stats.days_survived,
        }
    }

    fn to_line(&self) -> String {
        format!(
            "{LINE_TAG}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
            self.scenario_id,
            self.design_code,
            self.seed,
            self.timestamp_ms,
            u8::from(self.success),
            self.score,
            self.science_value,
            self.cost_usd,
            self.mass_kg,
            self.days_survived
        )
    }

    fn from_line(line: &str) -> Option<Self> {
        let p: Vec<&str> = line.trim().split('|').collect();
        if p.len() != 11 || p[0] != LINE_TAG {
            return None;
        }
        let finite = |s: &str| s.parse::<f64>().ok().filter(|v| v.is_finite());
        Some(HistoryRecord {
            scenario_id: p[1].to_string(),
            design_code: p[2].to_string(),
            seed: p[3].parse().ok()?,
            timestamp_ms: finite(p[4])?,
            success: match p[5] {
                "1" => true,
                "0" => false,
                _ => return None,
            },
            score: finite(p[6])?,
            science_value: finite(p[7])?,
            cost_usd: finite(p[8])?,
            mass_kg: finite(p[9])?,
            days_survived: p[10].parse().ok()?,
        })
    }
}

fn parse_all(text: &str) -> Vec<HistoryRecord> {
    text.lines().filter_map(HistoryRecord::from_line).take(MAX_RECORDS).collect()
}

fn to_text(records: &[HistoryRecord]) -> String {
    records.iter().map(HistoryRecord::to_line).collect::<Vec<_>>().join("\n")
}

/// Stored history, newest first; empty when storage is unavailable or unreadable.
pub fn load() -> Vec<HistoryRecord> {
    storage::read(KEY).map(|t| parse_all(&t)).unwrap_or_default()
}

/// Best-effort save of the whole list (newest first, capped).
pub fn save(records: &[HistoryRecord]) {
    let capped = &records[..records.len().min(MAX_RECORDS)];
    storage::write(KEY, &to_text(capped));
}

pub fn clear() {
    storage::remove(KEY);
}
