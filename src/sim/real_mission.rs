//! The Real-Mission Match shown in the Debrief. Ticket 04 fills `real_mission_match`.
use serde::{Deserialize, Serialize};

use super::debrief::DebriefContext;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RealMissionMatch {
    pub name: String,
    pub operator: String,
    pub year: u32,
    pub blurb: String,
    pub url: String,
}

/// The real mission most like this one. `None` until ticket 04.
pub(crate) fn real_mission_match(_ctx: &DebriefContext) -> Option<RealMissionMatch> {
    None
}
