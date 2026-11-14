//! Launch Review on the launch pad: the chosen Launch Vehicle standing on the pad with the assembled spacecraft
//! on top, a mark per Role's Go / No-Go vote, and the flag that starts the countdown and lift-off.
use serde::Serialize;

use super::hangar::{rocket_preview, RocketPreview};
use super::{SceneInputs, Screen};

/// One Role's vote as the pad shows it (a lamp on the tower).
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PadVote {
    Pending,
    Go,
    NoGo,
}

impl PadVote {
    fn of(vote: Option<bool>) -> Self {
        match vote {
            Some(true) => PadVote::Go,
            Some(false) => PadVote::NoGo,
            None => PadVote::Pending,
        }
    }
}

/// What the pad Shot shows besides the spacecraft (which is `SceneSpec.spacecraft`).
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PadScene {
    pub launch_vehicle: RocketPreview,
    /// One entry per Role in `Role::ALL` order; empty when no vote has been cast or none applies (solo play).
    pub votes: Vec<PadVote>,
    /// True from the moment Launch is pressed until the Run starts: the renderer plays the countdown, ignition,
    /// lift-off and ascent, then tells the UI it is done.
    pub launching: bool,
}

pub(super) fn for_inputs(inputs: &SceneInputs) -> Option<PadScene> {
    if inputs.screen != Screen::LaunchReview {
        return None;
    }
    let launch_vehicle = rocket_preview(inputs.catalog?, inputs.design?)?;
    Some(PadScene { launch_vehicle, votes: inputs.votes.iter().copied().map(PadVote::of).collect(), launching: inputs.launching })
}
