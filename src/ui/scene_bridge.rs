//! The 3D scene behind every screen: mounts the canvas once, loads the JS bridge (`js/bridge/index.js`),
//! and pushes a fresh Scene Spec to it whenever the app state it is built from changes.
//! This is the only file with `wasm-bindgen` externs for the bridge.
//!
//! The bridge is loaded with a dynamic `import()` relative to the page's base URL, so it works under the
//! GitHub Pages sub-path. If the import fails or the browser has no WebGL, the scene layer switches to the
//! fallback: the canvas is hidden and a static background shows instead; every screen still works.
use leptos::html::Canvas;
use leptos::prelude::*;
use spaceooops::scene::{self, SceneInputs};
use spaceooops::settings::{DeviceHints, Quality};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

use super::audio_bridge;
use super::state::AppState;

#[wasm_bindgen(inline_js = r#"
let bridge = null;
let latest = null;
let wantedQuality = null;
export async function scene_init(canvas, quality) {
    try {
        const url = new URL('js/bridge/index.js', document.baseURI).href;
        const mod = await import(url);
        const ok = await mod.init(canvas, wantedQuality ?? quality);
        if (!ok) return false;
        bridge = mod;
        if (latest !== null) bridge.apply(JSON.parse(latest));
        return true;
    } catch (e) {
        console.warn('3D scene unavailable, using the static background:', e);
        return false;
    }
}
export function scene_reduced_motion() {
    return Boolean(window.matchMedia && window.matchMedia('(prefers-reduced-motion: reduce)').matches);
}
export function scene_apply(json) {
    latest = json;
    if (bridge) bridge.apply(JSON.parse(json));
}
export function scene_set_quality(q) {
    wantedQuality = q;
    if (bridge) bridge.setQuality(q);
}
export function scene_device_cores() { return Number(navigator.hardwareConcurrency) || 0; }
export function scene_device_memory() { return Number(navigator.deviceMemory) || 0; }
export function scene_device_renderer() {
    try {
        const probe = document.createElement('canvas');
        const gl = probe.getContext('webgl2') || probe.getContext('webgl');
        if (!gl) return '';
        const info = gl.getExtension('WEBGL_debug_renderer_info');
        const name = String(gl.getParameter(info ? info.UNMASKED_RENDERER_WEBGL : gl.RENDERER) || '');
        gl.getExtension('WEBGL_lose_context')?.loseContext();
        return name;
    } catch (e) {
        return '';
    }
}
"#)]
extern "C" {
    /// Changes the quality of a running scene (or the one about to start).
    fn scene_set_quality(quality: &str);
    fn scene_device_cores() -> f64;
    fn scene_device_memory() -> f64;
    fn scene_device_renderer() -> String;
    /// Loads the bridge and starts the renderer; resolves to `true` when the 3D scene is running.
    fn scene_init(canvas: &web_sys::HtmlCanvasElement, quality: &str) -> js_sys::Promise;
    /// Hands the bridge the latest Scene Spec as JSON (kept until the bridge is ready).
    fn scene_apply(json: &str);
    /// The player asked the browser for reduced motion.
    fn scene_reduced_motion() -> bool;
}

/// What this browser says about the machine, for the Low-quality default.
pub fn device_hints() -> DeviceHints {
    let cores = scene_device_cores();
    let memory = scene_device_memory();
    let renderer = scene_device_renderer();
    DeviceHints {
        cores: (cores > 0.0).then_some(cores as u32),
        memory_gb: (memory > 0.0).then_some(memory),
        renderer: (!renderer.is_empty()).then_some(renderer),
    }
}

/// Switches the 3D scene's quality (safe before the scene has started).
pub fn set_quality(quality: Quality) {
    scene_set_quality(quality.name());
}

/// Reduced motion skips the launch sequence, as it cuts every camera move.
pub fn reduced_motion() -> bool {
    scene_reduced_motion()
}

/// Whether the 3D scene is running. Exposed on the scene layer as `data-scene` for CSS and browser tests.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum SceneMode {
    Loading,
    Live,
    Fallback,
}

impl SceneMode {
    fn as_str(self) -> &'static str {
        match self {
            SceneMode::Loading => "loading",
            SceneMode::Live => "live",
            SceneMode::Fallback => "fallback",
        }
    }
}

/// The full-viewport scene layer. Mount once, outside the per-screen views.
#[component]
pub fn SceneLayer() -> impl IntoView {
    let state = expect_context::<AppState>();
    let canvas = NodeRef::<Canvas>::new();
    let mode = RwSignal::new(SceneMode::Loading);

    canvas.on_load(move |el| {
        leptos::task::spawn_local(async move {
            let live = JsFuture::from(scene_init(&el, super::settings_widget::initial_quality().name())).await.ok().and_then(|v| v.as_bool()).unwrap_or(false);
            state.scene_live.set(live);
            mode.set(if live { SceneMode::Live } else { SceneMode::Fallback });
        });
    });

    // The one place the Scene Spec is built from the app state. Later tickets add inputs to `SceneInputs` here.
    let spec = Memo::new(move |_| {
        let screen = state.screen.get();
        let chosen = state.scenario_id.get();
        // Solo play never votes, so only team play puts vote lamps on the tower.
        let votes: Vec<Option<bool>> = if state.team.with(|t| t.solo) { Vec::new() } else { state.votes.get().to_vec() };
        let launching = state.launching.get();
        // The Run playback position is the Run scene's only clock; read it only on the Run so other screens
        // do not rebuild the spec on every playback tick.
        let playback = if screen == scene::Screen::Run { state.playback.get() } else { 0.0 };
        state.run_result.with(|run| {
            state.design.with(|design| {
                state.catalog.with_value(|c| {
                    scene::build(&SceneInputs {
                        votes: &votes,
                        launching,
                        catalog: Some(c),
                        design: Some(design),
                        scenarios: &c.scenarios,
                        chosen: Some(&chosen),
                        run: run.as_ref(),
                        playback,
                        ..SceneInputs::on(screen)
                    })
                })
            })
        })
    });
    // The renderer reports the end of the launch sequence (finished or skipped by a click).
    let _ = window_event_listener_untyped("spaceooops-launch-done", move |_| state.finish_launch());
    Effect::new(move |_| spec.with(|s| scene_apply(&scene::to_json(s))));
    // Audio: the music track from the spec, and one-shot Cues from the change since the previous spec.
    // Audio is independent of the renderer, so this runs with or without WebGL.
    let previous = StoredValue::new(None::<scene::SceneSpec>);
    Effect::new(move |_| {
        spec.with(|s| {
            audio_bridge::audio_set_music(s.music.name());
            previous.with_value(|prev| {
                if let Some(prev) = prev {
                    for cue in scene::one_shots(prev, s) {
                        audio_bridge::audio_play(cue.name());
                    }
                }
            });
            previous.set_value(Some(s.clone()));
        })
    });

    view! {
        <div
            class="scene-layer"
            class:dimmed=move || spec.with(|s| s.dimmed)
            aria-hidden="true"
            data-scene=move || mode.get().as_str()
        >
            <div class="scene-fallback"></div>
            <canvas class="scene-canvas" node_ref=canvas></canvas>
        </div>
    }
}
