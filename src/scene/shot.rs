//! Shot: the camera framing the scene uses on each screen.
use serde::Serialize;

use super::{DebriefScene, Fate, Screen};

/// A named camera target. Serialised as `{"kind": "titleOrbit"}`; variants that need data (a globe target,
/// say) carry it as extra fields next to `kind`.
#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum Shot {
    /// Earth with a small spacecraft circling it, the camera drifting slowly around them.
    TitleOrbit,
    /// The Design hangar: the spacecraft assembled on a stand in the centre, the Orbit preview and the rocket
    /// beside it. The player may drag to turn the model.
    Hangar,
    /// Launch Review: the Launch Vehicle with the spacecraft on top, on its pad. The renderer drives the camera
    /// itself through the countdown and lift-off.
    Pad,
    /// Scenario select: Earth with Angola centred, every Scenario marker in view.
    Globe { lat: f64, lon: f64 },
    /// Brief: the camera holds close over one place on Earth.
    Region { lat: f64, lon: f64 },
    /// The Run: Earth, its shadow and the whole orbit in view, the spacecraft flying round it. `radius` is the
    /// orbit's display radius in Earth radii (as in the Scene Spec's orbit view), so the camera can fit it.
    Orbit { radius: f64 },
    /// Debrief after a Success: the spacecraft holds calmly against Earth.
    DebriefHero,
    /// Debrief after a failure: the spacecraft shows what went wrong (`fate`: tumbling, dark, reentry, adrift).
    DebriefFailure { fate: Fate },
}

/// Whether the scene is dimmed behind the screen's panels. Screens still laid out as full-page documents dim it so
/// their text stays readable; a screen designed as panels over the scene (and its own Shot) shows it clearly.
pub(super) fn dims(screen: Screen) -> bool {
    !matches!(
        screen,
        Screen::Title
            | Screen::ScenarioSelect
            | Screen::Brief
            | Screen::Design
            | Screen::LaunchReview
            | Screen::Run
            | Screen::Sources
            | Screen::TeamSetup
            | Screen::History
    )
}

pub(super) fn for_screen(screen: Screen) -> Shot {
    use Screen::*;
    match screen {
        Title => Shot::TitleOrbit,
        Design => Shot::Hangar,
        LaunchReview => Shot::Pad,
        // Screens with no Shot of their own keep the title framing (the Run and Debrief shots come from their own
        // modules); the UI dims the scene behind the panels that still need it.
        // Scenario select and Brief are framed by `markers::shot`, which `build` tries first.
        Sources | TeamSetup | ScenarioSelect | Brief | Run | Debrief | History => {
            Shot::TitleOrbit
        }
    }
}

/// The Debrief's Shot, from its scene: the hero shot on Success, else the failure shot of its visual.
pub(super) fn for_debrief(d: &DebriefScene) -> Shot {
    if d.success { Shot::DebriefHero } else { Shot::DebriefFailure { fate: d.fate } }
}
