//! Scene Spec: a plain-data description of what the 3D scene (and the audio) should play right now.
//! Pure Rust, no web dependencies: it is the only place that decides *what* to show; the JS bridge in
//! `js/bridge/` decides only *how* to draw it. The UI builds `SceneInputs` from its state, calls `build`,
//! serialises the result to JSON and hands it to the bridge.
//!
//! Each concern lives in its own file (see the low-poly overhaul architecture contract). Every field added
//! to `SceneSpec` after the first version is optional or has a default, so an older renderer ignores it.
mod cues;
mod debrief;
pub mod effects;
mod hangar;
mod markers;
pub mod orbit;
mod pad;
mod shot;
mod spacecraft;

use serde::Serialize;

use crate::sim::{Catalog, Design};

pub use cues::{one_shots, Cue, Track};
pub use debrief::{fate_for_cause, DebriefScene, Fate, ComponentClass};
pub use effects::{Effect, EffectKind, Station};
pub use hangar::{HangarPreview, OrbitPreview, LaunchVehiclePreview};
pub use markers::Marker;
pub use orbit::OrbitView;
pub use pad::{PadScene, PadVote};
pub use shot::Shot;
pub use spacecraft::{Finish, Part, PartKind, SpacecraftModel};

use crate::sim::{RunResult, Scenario};

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
    /// Screens whose scene shows the spacecraft assembled from the Design. Add a screen here when its scene
    /// should show the spacecraft.
    pub fn shows_spacecraft(self) -> bool {
        matches!(self, Screen::Design | Screen::LaunchReview | Screen::Run | Screen::Debrief)
    }
}

/// Everything the Scene Spec is built from. Construct with `SceneInputs::on(screen)` and override fields
/// with struct-update syntax, so a new input does not break existing callers.
#[derive(Clone, Debug)]
pub struct SceneInputs<'a> {
    pub screen: Screen,
    /// The catalog the Design's ids refer to. Needed for anything built from the Design.
    pub catalog: Option<&'a Catalog>,
    /// The current Design; the spacecraft model is assembled from it.
    pub design: Option<&'a Design>,
    /// The catalog's Scenarios, for the markers on Scenario select and Brief.
    pub scenarios: &'a [Scenario],
    /// The id of the chosen Scenario (empty or unknown means none).
    pub chosen: Option<&'a str>,
    /// The Run being played back (Run screen); the orbit view reads its timeline.
    pub run: Option<&'a RunResult>,
    /// The Run playback position in days elapsed, fractional (0 at launch). The scene's only clock for the Run.
    pub playback: f64,
    /// Go / No-Go votes per Role at Launch Review (empty when none; solo play never votes).
    pub votes: &'a [Option<bool>],
    /// True while the launch sequence (countdown, lift-off) is playing.
    pub launching: bool,
}

impl SceneInputs<'static> {
    pub fn on(screen: Screen) -> Self {
        SceneInputs {
            screen,
            catalog: None,
            design: None,
            scenarios: &[],
            chosen: None,
            votes: &[],
            launching: false,
            run: None,
            playback: 0.0,
        }
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
    /// Launch Review: the rocket on the pad, vote lamps and the launch sequence flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pad: Option<PadScene>,
    /// Scenario pins on the globe; empty except on Scenario select and Brief.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub markers: Vec<Marker>,
    /// The Run's orbit view: the Orbit, the spacecraft's place on it and Earth's shadow (Run screen only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orbit: Option<OrbitView>,
    /// Run-scene effects of Random Events and Decisions active at the playback position (Run screen only).
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub effects: Vec<Effect>,
    /// Ground stations on the globe during the Run (their beacons, outages and contact beams).
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub stations: Vec<Station>,
    /// The Debrief's verdict visual (Debrief screen with a finished Run only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debrief: Option<DebriefScene>,
    /// The music track that should be playing now (the bridge crossfades when it changes).
    pub music: Track,
}

/// Builds the Scene Spec for the given inputs.
pub fn build(inputs: &SceneInputs<'_>) -> SceneSpec {
    let orbit = orbit::for_inputs(inputs);
    let debrief = debrief::for_inputs(inputs);
    SceneSpec {
        shot: markers::shot(inputs.screen, inputs.scenarios, inputs.chosen)
            .or_else(|| debrief.as_ref().map(shot::for_debrief))
            .or_else(|| orbit.as_ref().map(|o| Shot::Orbit { radius: o.radius }))
            .unwrap_or_else(|| shot::for_screen(inputs.screen)),
        dimmed: shot::dims(inputs.screen),
        spacecraft: spacecraft::for_inputs(inputs),
        hangar: hangar::for_inputs(inputs),
        pad: pad::for_inputs(inputs),
        markers: markers::markers(inputs.screen, inputs.scenarios, inputs.chosen),
        orbit,
        effects: effects::for_inputs(inputs),
        stations: effects::stations_for(inputs),
        debrief,
        music: cues::track_for(inputs.screen),
    }
}

/// The JSON the bridge's `apply(sceneSpec)` receives.
pub fn to_json(spec: &SceneSpec) -> String {
    serde_json::to_string(spec).expect("a Scene Spec is always serialisable")
}
