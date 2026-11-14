//! Run screen: plays back the timeline `sim::run` returned. It never re-simulates.
//! Layout: playback controls, the orbit animation slot, live dashboards, the event log. With WebGL the panels sit
//! at the edges of the screen over the 3D orbit scene (style/run-scene.css); without it, as a page with the SVG.
mod dashboards;
mod decision_prompt;
mod event_log;
mod orbit_view;
mod playback;

use leptos::prelude::*;

use super::state::AppState;
use dashboards::Dashboards;
use event_log::EventLog;
use orbit_view::OrbitView;
use playback::{PlaybackControls, PlaybackDriver};

#[component]
pub fn RunScreen() -> impl IntoView {
    let state = expect_context::<AppState>();
    let speed = RwSignal::new(1.0);
    let scenario_name = move || {
        let id = state.scenario_id.get();
        state.catalog.with_value(|c| c.scenario(&id).map(|s| s.name.clone()).unwrap_or_default())
    };
    view! {
        <section class="screen run-screen">
            <PlaybackDriver speed=speed />
            <div class="run-head">
                <h2>"Run: " {scenario_name}</h2>
                <PlaybackControls speed=speed />
            </div>
            <decision_prompt::DecisionPromptPanel />
            // With WebGL the 3D scene frames the orbit inside this empty stage (style/run-scene.css lays the
            // panels round it); without WebGL it folds away and the SVG orbit view below is drawn instead.
            <div class="run-stage" data-scene-frame aria-hidden="true"></div>
            <OrbitView />
            <Show when=move || state.playback_finished()>
                <p class="run-ending" aria-live="polite">
                    {move || state.run_result.with(|r| r.as_ref().map(|r| r.debrief.ending.clone()).unwrap_or_default())}
                </p>
            </Show>
            <Dashboards />
            <EventLog />
        </section>
    }
}
