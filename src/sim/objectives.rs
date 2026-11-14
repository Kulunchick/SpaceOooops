//! Personal Objectives evaluated in the Debrief. Ticket 06 fills `objectives`.
use serde::{Deserialize, Serialize};

use super::debrief::DebriefContext;
use super::design::Role;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObjectiveResult {
    pub role: Role,
    pub description: String,
    pub met: bool,
    /// Score points earned (added as a `PersonalObjective` Score line when met).
    pub bonus: f64,
}

/// Personal Objective results for a Run. Empty until ticket 06.
pub(crate) fn objectives(_ctx: &DebriefContext) -> Vec<ObjectiveResult> {
    Vec::new()
}
