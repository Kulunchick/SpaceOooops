//! The Debrief's scene: on Success the spacecraft holds in a calm hero shot against Earth; on failure the scene
//! shows what went wrong, picked from the Debrief's main cause (the first one the sim lists). The verdict also
//! picks the Cue (see `cues.rs`). Everything here is decided in Rust; `js/bridge/debrief.js` only draws it.
use serde::Serialize;

use crate::sim::{CauseKind, Catalog, EventKind, RunResult};

use super::{SceneInputs, Screen};

/// What the Debrief scene shows. `Hero` is the Success; the rest are failures.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub enum Fate {
    /// Success: the spacecraft holds steady against Earth.
    Hero,
    /// Attitude control failed: the spacecraft spins out of control.
    Tumbling,
    /// Power or the battery failed: the spacecraft goes dark.
    Dark,
    /// Lifetime was too short: the spacecraft falls into the atmosphere and burns.
    Reentry,
    /// Any other failure (lost Data, too little Science Value, a dead radio, a Design that never flew):
    /// the spacecraft drifts, dimmed and silent.
    Adrift,
}

/// The kind of Component that failed, as far as the visual cares.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ComponentClass {
    Solar,
    Battery,
    Attitude,
    Comms,
    Computer,
}

/// The Debrief scene, in the Scene Spec on the Debrief screen once a Run exists.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DebriefScene {
    pub success: bool,
    pub fate: Fate,
}

/// The failure visual for a main cause. `part` is the Component behind a `ComponentFailure`, when known.
/// A failure never maps to `Hero`, whatever the cause.
pub fn fate_for_cause(kind: CauseKind, part: Option<ComponentClass>) -> Fate {
    match kind {
        CauseKind::BatteryTooSmall => Fate::Dark,
        CauseKind::LifetimeShortfall => Fate::Reentry,
        CauseKind::ComponentFailure => match part {
            Some(ComponentClass::Attitude) => Fate::Tumbling,
            Some(ComponentClass::Solar | ComponentClass::Battery) => Fate::Dark,
            Some(ComponentClass::Comms | ComponentClass::Computer) | None => Fate::Adrift,
        },
        CauseKind::DataLost
        | CauseKind::ScienceShortfall
        | CauseKind::PoorCoverage
        | CauseKind::PoorInstrumentFit
        | CauseKind::RandomEvents
        | CauseKind::NotLaunched
        | CauseKind::WentWell
        | CauseKind::ScoreLimit => Fate::Adrift,
    }
}

fn class_of(catalog: &Catalog, id: &str) -> Option<ComponentClass> {
    if catalog.attitude_controls.iter().any(|o| o.id == id) {
        Some(ComponentClass::Attitude)
    } else if catalog.solar_panels.iter().any(|o| o.id == id) {
        Some(ComponentClass::Solar)
    } else if catalog.batteries.iter().any(|o| o.id == id) {
        Some(ComponentClass::Battery)
    } else if catalog.comms.iter().any(|o| o.id == id) {
        Some(ComponentClass::Comms)
    } else if catalog.computer.id == id {
        Some(ComponentClass::Computer)
    } else {
        None
    }
}

/// The scene's fate for a finished Run: the hero shot on Success, else by the main cause.
pub fn fate(catalog: &Catalog, run: &RunResult) -> Fate {
    if run.debrief.success {
        return Fate::Hero;
    }
    let Some(main) = run.debrief.causes.first() else { return Fate::Adrift };
    let part = (main.kind == CauseKind::ComponentFailure)
        .then(|| {
            run.timeline
                .iter()
                .filter(|d| main.day.is_none_or(|day| d.day == day))
                .flat_map(|d| &d.events)
                .filter(|e| e.kind == EventKind::ComponentFailure)
                .find_map(|e| e.component.as_deref().and_then(|id| class_of(catalog, id)))
        })
        .flatten();
    fate_for_cause(main.kind, part)
}

pub(super) fn for_inputs(inputs: &SceneInputs) -> Option<DebriefScene> {
    if inputs.screen != Screen::Debrief {
        return None;
    }
    let run = inputs.run?;
    let fate = fate(inputs.catalog?, run);
    Some(DebriefScene { success: run.debrief.success, fate })
}
