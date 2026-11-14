//! Scene Spec: a plain-data description of what the 3D scene (and later the audio) should show right now.
//! Pure Rust, no web dependencies: it is the only place that decides *what* to show; the JS bridge in
//! `js/bridge/` decides only *how* to draw it. The UI builds `SceneInputs` from its state, calls `build`,
//! serialises the result to JSON and hands it to the bridge.
//!
//! Each concern lives in its own file (see the low-poly overhaul architecture contract). Every field added
//! to `SceneSpec` after the first version is optional or has a default, so an older renderer ignores it.
mod shot;

use serde::Serialize;

pub use shot::Shot;

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

/// Everything the Scene Spec is built from. Construct with `SceneInputs::on(screen)` and override fields
/// with struct-update syntax, so new inputs added by later tickets do not break existing callers.
#[derive(Clone, Debug)]
pub struct SceneInputs {
    pub screen: Screen,
}

impl SceneInputs {
    pub fn on(screen: Screen) -> Self {
        SceneInputs { screen }
    }
}

/// What the scene should show. Serialised as camelCase JSON for the bridge.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SceneSpec {
    pub shot: Shot,
    /// The scene is dimmed behind the screen's panels (the UI draws the veil; the renderer may ignore it).
    pub dimmed: bool,
}

/// Builds the Scene Spec for the given inputs.
pub fn build(inputs: &SceneInputs) -> SceneSpec {
    SceneSpec { shot: shot::for_screen(inputs.screen), dimmed: shot::dims(inputs.screen) }
}

/// The JSON the bridge's `apply(sceneSpec)` receives.
pub fn to_json(spec: &SceneSpec) -> String {
    serde_json::to_string(spec).expect("a Scene Spec is always serialisable")
}
