//! Debrief causes: why the Mission went the way it did. Ticket 04 fills `causes`.
use serde::{Deserialize, Serialize};

use super::debrief::DebriefContext;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CauseKind {
    BatteryTooSmall,
    DataLost,
    ComponentFailure,
    ScienceShortfall,
    LifetimeShortfall,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Cause {
    pub kind: CauseKind,
    /// The full plain-language sentence shown to the player.
    pub text: String,
    /// The day of the Run the cause shows up, if it has one.
    pub day: Option<u32>,
}

/// The causes behind a Run's outcome. Empty until ticket 04.
pub(crate) fn causes(_ctx: &DebriefContext) -> Vec<Cause> {
    Vec::new()
}
