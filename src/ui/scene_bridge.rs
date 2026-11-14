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
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

use super::state::AppState;

#[wasm_bindgen(inline_js = r#"
let bridge = null;
let latest = null;
export async function scene_init(canvas, quality) {
    try {
        const url = new URL('js/bridge/index.js', document.baseURI).href;
        const mod = await import(url);
        const ok = await mod.init(canvas, quality);
        if (!ok) return false;
        bridge = mod;
        if (latest !== null) bridge.apply(JSON.parse(latest));
        return true;
    } catch (e) {
        console.warn('3D scene unavailable, using the static background:', e);
        return false;
    }
}
export function scene_apply(json) {
    latest = json;
    if (bridge) bridge.apply(JSON.parse(json));
}
"#)]
extern "C" {
    /// Loads the bridge and starts the renderer; resolves to `true` when the 3D scene is running.
    fn scene_init(canvas: &web_sys::HtmlCanvasElement, quality: &str) -> js_sys::Promise;
    /// Hands the bridge the latest Scene Spec as JSON (kept until the bridge is ready).
    fn scene_apply(json: &str);
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
            let live = JsFuture::from(scene_init(&el, "high")).await.ok().and_then(|v| v.as_bool()).unwrap_or(false);
            mode.set(if live { SceneMode::Live } else { SceneMode::Fallback });
        });
    });

    // The one place the Scene Spec is built from the app state. Later tickets add inputs to `SceneInputs` here.
    let spec = Memo::new(move |_| {
        let screen = state.screen.get();
        let chosen = state.scenario_id.get();
        state.design.with(|design| {
            state.catalog.with_value(|c| {
                scene::build(&SceneInputs {
                    catalog: Some(c),
                    design: Some(design),
                    scenarios: &c.scenarios,
                    chosen: Some(&chosen),
                    ..SceneInputs::on(screen)
                })
            })
        })
    });
    Effect::new(move |_| spec.with(|s| scene_apply(&scene::to_json(s))));

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
