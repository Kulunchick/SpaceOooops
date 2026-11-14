//! Run screen: plays back the timeline `sim::run` returned. It never re-simulates.
//! Layout: playback controls, the orbit animation slot, live dashboards, the event log.
mod dashboards;
mod event_log;
mod playback;

use leptos::prelude::*;

use super::state::AppState;
use dashboards::Dashboards;
use event_log::EventLog;
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
            // ---- ORBIT ANIMATION SLOT (ticket 07) ----
            // Put the SVG orbit animation component here. It must read `AppState::playback`
            // (days elapsed, fractional) so it stays in sync with the dashboards and the event log.
            <div class="run-orbit-slot"></div>
            <Dashboards />
            <EventLog />
        </section>
    }
}
