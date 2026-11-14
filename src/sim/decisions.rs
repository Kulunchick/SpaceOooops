//! Interactive Random Events: the questions a Run asks the player, the choices passed back into
//! `run_with_decisions`, and the Debrief's plain-language notes on what each choice did.
//! Every question, option and note text is produced here; the UI only renders it.
use serde::{Deserialize, Serialize};

use super::catalog::{DecisionRules, EventTuning};
use super::events::days;
use super::review::money;

/// The player's answer to one `DecisionPrompt`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Decision {
    /// `DecisionPrompt::id` of the question answered.
    pub prompt_id: String,
    /// `DecisionOption::id` of the option chosen.
    pub choice: String,
}

/// A question a Random Event asks the player before its day plays out.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DecisionPrompt {
    /// Unique within a Run; the same for every choice made, so a `Decision` can name it.
    pub id: String,
    /// The day the question is about (1-based); days before it do not depend on the answer.
    pub day: u32,
    pub question: String,
    pub options: Vec<DecisionOption>,
    /// The option applied when the player does not answer (e.g. the playback is skipped).
    pub default: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DecisionOption {
    pub id: String,
    /// Short button text.
    pub label: String,
    /// What this choice costs and what it buys, in one sentence.
    pub consequence: String,
}

pub(crate) const SAFE_MODE: &str = "safe-mode";
pub(crate) const RIDE_OUT: &str = "ride-out";
pub(crate) const BUY_STATION_TIME: &str = "buy-station-time";
pub(crate) const WAIT: &str = "wait";

pub(crate) fn flare_prompt(day: u32, tuning: &EventTuning, rules: &DecisionRules) -> DecisionPrompt {
    DecisionPrompt {
        id: format!("day-{day}-solar-flare"),
        day,
        question: "Solar-flare warning: a strong flare is about to hit the spacecraft. Enter safe mode?".to_string(),
        options: vec![
            DecisionOption {
                id: SAFE_MODE.to_string(),
                label: "Enter safe mode".to_string(),
                consequence: format!(
                    "The Instruments switch off for {}, so that science is lost, but the electronics are protected.",
                    days(tuning.flare_safe_mode_days)
                ),
            },
            DecisionOption {
                id: RIDE_OUT.to_string(),
                label: "Ride it out".to_string(),
                consequence: format!(
                    "The Instruments keep working, but today every Component is {:.0} times more likely to fail than in safe mode.",
                    rules.flare_ride_out_failure_multiplier
                ),
            },
        ],
        default: SAFE_MODE.to_string(),
    }
}

pub(crate) fn outage_prompt(day: u32, tuning: &EventTuning, rules: &DecisionRules) -> DecisionPrompt {
    DecisionPrompt {
        id: format!("day-{day}-ground-station-outage"),
        day,
        question: format!(
            "The ground station is going offline for {}. Buy extra time on another station?",
            days(tuning.outage_days)
        ),
        options: vec![
            DecisionOption {
                id: BUY_STATION_TIME.to_string(),
                label: "Buy station time".to_string(),
                consequence: format!(
                    "Costs {} of Budget, and Data keeps coming down as usual.",
                    money(rules.outage_station_time_cost_usd)
                ),
            },
            DecisionOption {
                id: WAIT.to_string(),
                label: "Wait it out".to_string(),
                consequence: format!(
                    "Free, but Data piles up on board for {}; whatever does not fit in storage is lost.",
                    days(tuning.outage_days)
                ),
            },
        ],
        default: WAIT.to_string(),
    }
}

/// The option that applies to `prompt`: the player's valid choice, else the default.
/// Returns the option id and whether the player chose it.
pub(crate) fn chosen<'a>(prompt: &'a DecisionPrompt, decisions: &'a [Decision]) -> (&'a str, bool) {
    decisions
        .iter()
        .rev()
        .find(|d| d.prompt_id == prompt.id && prompt.options.iter().any(|o| o.id == d.choice))
        .map_or((prompt.default.as_str(), false), |d| (d.choice.as_str(), true))
}

/// One question answered during the Run, kept for the Debrief.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Taken {
    pub day: u32,
    pub choice: String,
    pub by_player: bool,
    /// Names of Components that failed on the flare day.
    pub failed_names: Vec<String>,
    /// Safe-mode days (flare) or outage length (ground station).
    pub days: u32,
    pub cost_usd: f64,
}

/// Plain-language note on one decision and its effect. `delivered_mb` and `lost_mb` are the Data sent down and
/// lost over the outage window (only used for outage choices).
pub(crate) fn note(t: &Taken, delivered_mb: f64, lost_mb: f64) -> String {
    let who = if t.by_player { "you chose to" } else { "no answer was given, so by default the spacecraft chose to" };
    let failed = match t.failed_names.as_slice() {
        [] => None,
        names => Some(names.join(" and ")),
    };
    match t.choice.as_str() {
        SAFE_MODE => {
            let effect = match failed {
                None => "the electronics came through unharmed".to_string(),
                Some(names) => format!("even so, the {names} failed in the flare"),
            };
            format!(
                "Day {}, solar-flare warning: {who} enter safe mode. The Instruments were off for {}, and {effect}.",
                t.day,
                days(t.days)
            )
        }
        RIDE_OUT => {
            let effect = match failed {
                None => "the gamble paid off: nothing broke".to_string(),
                Some(names) => format!("the gamble failed: the {names} broke in the flare"),
            };
            format!("Day {}, solar-flare warning: {who} ride it out. The Instruments kept working, and {effect}.", t.day)
        }
        BUY_STATION_TIME => format!(
            "Day {}, ground-station outage: {who} buy station time for {}. {:.0} MB of Data came down during the {} outage.",
            t.day,
            money(t.cost_usd),
            delivered_mb,
            day_adjective(t.days)
        ),
        _ => {
            let effect = if lost_mb > 0.5 {
                format!("{lost_mb:.0} MB of Data did not fit in storage and was lost")
            } else {
                "everything fitted in storage until contact returned, so no Data was lost".to_string()
            };
            format!(
                "Day {}, ground-station outage: {who} wait it out for free. Data piled up on board, and {effect}.",
                t.day
            )
        }
    }
}

/// "3-day".
fn day_adjective(n: u32) -> String {
    format!("{n}-day")
}
