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
pub use run::{DaySnapshot, RunResult, battery_note, run, run_with_decisions};
pub use score::{Score, ScoreLine, ScoreLineKind};
pub use share::{ShareError, decode_design, encode_design};

/// One-line game pitch shown on the title screen.
pub fn tagline() -> &'static str {
    "Design the mission. Survive the physics."
}

/// A short explanation of the game for the title screen, one paragraph per entry.
pub fn game_intro() -> [&'static str; 2] {
    [
        "Pick a Scenario, a real problem a small satellite could help with over Africa, and design a spacecraft for \
         it across four Roles: Science, Spacecraft, Launch & Orbit, and Budget & Comms. Pass Launch Review, then \
         fly a Run day by day through eclipses, solar flares and failures.",
        "The Debrief explains what went right or wrong, with the numbers, so you can change the Design and fly \
         again. Play solo, or with 2 to 4 players on one screen, each owning Roles.",
    ]
}
