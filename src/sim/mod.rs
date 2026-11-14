//! Pure simulation core. Must never depend on web/UI code.
//! The module root only declares and re-exports; one file per concern.
mod catalog;
mod causes;
mod debrief;
mod decisions;
mod design;
mod events;
mod objectives;
mod real_mission;
mod resources;
mod review;
mod rng;
mod run;
mod score;
mod share;

pub use catalog::*;
pub use causes::{Cause, CauseKind};
pub use debrief::{Debrief, RunStats};
pub use decisions::{Decision, DecisionOption, DecisionPrompt};
pub use design::*;
pub use events::{EventKind, RandomEvent};
pub use objectives::{
    ObjectiveOutlook, ObjectiveResult, ObjectiveRule, ObjectiveSpec, objective_outlook, scenario_objectives,
};
pub use real_mission::RealMissionMatch;
pub use review::*;
pub use run::{DaySnapshot, RunResult, run, run_with_decisions};
pub use score::{Score, ScoreLine, ScoreLineKind};
pub use share::{ShareError, decode_design, encode_design};

/// One-line game pitch shown on the title screen.
pub fn tagline() -> &'static str {
    "Design the mission. Survive the physics."
}
