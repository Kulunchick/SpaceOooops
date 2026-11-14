//! Audio Cues: *what* should be heard. The JS bridge (`js/bridge/audio.js`) decides only *how*.
//!
//! Two kinds:
//! - the **music track** for the screen (`SceneSpec.music`), which the bridge crossfades to;
//! - **one-shot Cues** triggered by a state change, decided by `one_shots(previous, current)`.
//!
//! Adding a one-shot Cue: add a variant and its name below, push it from `one_shots`, and (optionally) map the
//! name to a sample in `js/bridge/audio.js` (`SAMPLES`). A name without a sample plays a short synthesised tone.
use std::collections::BTreeSet;

use serde::Serialize;

use super::{PadVote, SceneSpec, Screen, SpacecraftModel};

/// The looping music tracks.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Track {
    /// Calm: title, Scenario select, Brief, Design and the other non-Run screens.
    Menu,
    /// Tenser: during the Run.
    Run,
}

impl Track {
    /// The name the bridge's `setMusic(track)` receives (the same as in the Scene Spec JSON).
    pub fn name(self) -> &'static str {
        match self {
            Track::Menu => "menu",
            Track::Run => "run",
        }
    }
}

pub fn track_for(screen: Screen) -> Track {
    match screen {
        Screen::Run => Track::Run,
        _ => Track::Menu,
    }
}

/// A one-shot sound.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Cue {
    /// A part was added to or removed from the spacecraft.
    Snap,
    /// A Role voted Go at Launch Review.
    Go,
    /// A Role voted No-Go at Launch Review.
    NoGo,
    /// Launch was pressed: the launch sequence starts (the countdown ticks are timed by the renderer).
    Launch,
}

impl Cue {
    /// The name the bridge's `play(cue)` receives.
    pub fn name(self) -> &'static str {
        match self {
            Cue::Snap => "snap",
            Cue::Go => "go",
            Cue::NoGo => "nogo",
            Cue::Launch => "launch",
        }
    }
}

/// More part changes than this in one step is a whole spacecraft arriving (a share link, "Load this Design"),
/// not the player snapping something on. A swap of one option is a removal plus an addition, so 2 is allowed.
const MAX_SNAPPED_PARTS: usize = 2;

/// The one-shot Cues for the change from `previous` to `current`.
pub fn one_shots(previous: &SceneSpec, current: &SceneSpec) -> Vec<Cue> {
    let mut cues = Vec::new();
    if let (Some(before), Some(after)) = (&previous.spacecraft, &current.spacecraft) {
        let ids = |m: &SpacecraftModel| m.parts.iter().map(|p| p.id.clone()).collect::<BTreeSet<_>>();
        let changed = ids(before).symmetric_difference(&ids(after)).count();
        if changed > 0 && changed <= MAX_SNAPPED_PARTS {
            cues.push(Cue::Snap);
        }
    }
    if let (Some(before), Some(after)) = (&previous.pad, &current.pad) {
        let moved = |to: PadVote| before.votes.iter().zip(&after.votes).any(|(was, now)| was != now && *now == to);
        if moved(PadVote::Go) {
            cues.push(Cue::Go);
        }
        if moved(PadVote::NoGo) {
            cues.push(Cue::NoGo);
        }
        if after.launching && !before.launching {
            cues.push(Cue::Launch);
        }
    }
    cues
}
