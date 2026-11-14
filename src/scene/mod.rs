//! Scene Spec: a plain-data description of what the 3D scene (and later the audio) should show right now.
//! Pure Rust, no web dependencies: it is the only place that decides *what* to show; the JS bridge in
//! `js/bridge/` decides only *how* to draw it. The UI builds `SceneInputs` from its state, calls `build`,
//! serialises the result to JSON and hands it to the bridge.
//!
//! Each concern lives in its own file (see the low-poly overhaul architecture contract). Every field added
//! to `SceneSpec` after the first version is optional or has a default, so an older renderer ignores it.
mod hangar;
mod shot;
mod spacecraft;

use serde::Serialize;

use crate::sim::{Catalog, Design};

pub use hangar::{HangarPreview, OrbitPreview, RocketPreview};
pub use shot::Shot;
pub use spacecraft::{Finish, Part, PartKind, SpacecraftModel};

/// The screen the player is on. The UI's screen state is this type, so every screen has a Shot.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Screen {
    Title,
    Sources,
    TeamSetup,
    ScenarioSelect,
    Brief,
    Design,
    LaunchReview,
    Run,
    Debrief,
    History,
}

impl Screen {
    /// Screens whose scene shows the spacecraft assembled from the Design. Later tickets add their screens
    /// (Launch Review on the pad, the Run in orbit, the Debrief shots) here.
    pub fn shows_spacecraft(self) -> bool {
        matches!(self, Screen::Design)
    }
}

/// Everything the Scene Spec is built from. Construct with `SceneInputs::on(screen)` and override fields
/// with struct-update syntax, so new inputs added by later tickets do not break existing callers.
#[derive(Clone, Debug)]
pub struct SceneInputs<'a> {
    pub screen: Screen,
    /// The catalog the Design's ids refer to. Needed for anything built from the Design.
    pub catalog: Option<&'a Catalog>,
    /// The current Design; the spacecraft model is assembled from it.
    pub design: Option<&'a Design>,
}

impl<'a> SceneInputs<'a> {
    pub fn on(screen: Screen) -> Self {
        SceneInputs { screen, catalog: None, design: None }
    }
}

/// What the scene should show. Serialised as camelCase JSON for the bridge.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SceneSpec {
    pub shot: Shot,
    /// The scene is dimmed behind the screen's panels (the UI draws the veil; the renderer may ignore it).
    pub dimmed: bool,
    /// The spacecraft assembled from the Design, on screens that show it (absent elsewhere).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spacecraft: Option<SpacecraftModel>,
    /// Design-screen previews next to the spacecraft: the chosen Orbit and Launch Vehicle.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hangar: Option<HangarPreview>,
}

/// Builds the Scene Spec for the given inputs.
pub fn build(inputs: &SceneInputs) -> SceneSpec {
    SceneSpec {
        shot: shot::for_screen(inputs.screen),
        dimmed: shot::dims(inputs.screen),
        spacecraft: spacecraft::for_inputs(inputs),
        hangar: hangar::for_inputs(inputs),
    }
}

/// The JSON the bridge's `apply(sceneSpec)` receives.
pub fn to_json(spec: &SceneSpec) -> String {
    serde_json::to_string(spec).expect("a Scene Spec is always serialisable")
}
