//! Shot: the camera framing the scene uses on each screen.
use serde::Serialize;

use super::Screen;

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
}

/// Whether the scene is dimmed behind the screen's panels. Screens still laid out as full-page documents dim it so
/// their text stays readable; a screen designed as panels over the scene (and its own Shot) shows it clearly.
pub(super) fn dims(screen: Screen) -> bool {
    !matches!(
        screen,
        Screen::Title | Screen::ScenarioSelect | Screen::Brief | Screen::Design | Screen::LaunchReview | Screen::Sources | Screen::TeamSetup | Screen::History
    )
}

pub(super) fn for_screen(screen: Screen) -> Shot {
    use Screen::*;
    match screen {
        Title => Shot::TitleOrbit,
        Design => Shot::Hangar,
        LaunchReview => Shot::Pad,
        // Screens that have no Shot of their own yet keep the title framing; the UI dims the scene behind
        // their panels. Later tickets move each screen to its own Shot (globe, hangar, pad, orbit-follow, ...).
        // Scenario select and Brief are framed by `markers::shot`, which `build` tries first.
        Sources | TeamSetup | ScenarioSelect | Brief | Run | Debrief | History => {
            Shot::TitleOrbit
        }
    }
}
