//! Playback of the Run timeline: the driver advances `AppState::playback`, the controls change speed,
//! skip to the end and open the Debrief. The whole timeline takes about 30 s at 1x, whatever its length.
use std::time::Duration;

use leptos::prelude::*;
use spaceooops::scene;

use crate::ui::state::{AppState, Screen};

/// Seconds the whole timeline takes at 1x.
const PLAYBACK_SECONDS: f64 = 30.0;
const TICK_MS: u64 = 50;
const SPEEDS: [f64; 2] = [1.0, 4.0];
/// Seconds a day with a Random Event takes at 1x when the 3D scene shows its effect (see `scene::effects::advance`).
const EVENT_DAY_SECONDS: f64 = 1.5;
/// Pause on the last day before moving to the Debrief by itself.
const END_HOLD_MS: u64 = 1500;

/// Invisible component: ticks the playback position forward while the Run screen is open.
#[component]
pub fn PlaybackDriver(speed: RwSignal<f64>) -> impl IntoView {
    let state = expect_context::<AppState>();
    let playback = state.playback;
    let run_result = state.run_result;
    let ticks_per_run = PLAYBACK_SECONDS * 1000.0 / TICK_MS as f64;
    // Milliseconds spent at the end of the timeline; past the hold, open the Debrief once.
    let held_ms = StoredValue::new(0u64);
    let handle = set_interval_with_handle(
        move || {
            let days = run_result.with_untracked(|r| r.as_ref().map_or(0, |r| r.timeline.len())) as f64;
            let step = days / ticks_per_run * speed.get_untracked();
            let now = playback.get_untracked();
            // Stop before a day that asks the player a question until it is answered (ticket 10).
            let limit = super::decision_prompt::pause_at(&state).map_or(days, |at| at.min(days));
            if now < limit {
                // With the 3D scene, a day with a Random Event plays slowly enough for its effect to be seen.
                let next = if state.scene_live.get_untracked() {
                    let event_step = speed.get_untracked() * TICK_MS as f64 / 1000.0 / EVENT_DAY_SECONDS;
                    run_result.with_untracked(|r| {
                        r.as_ref().map_or(now + step, |r| scene::effects::advance(r, now, step, event_step))
                    })
                } else {
                    now + step
                };
                playback.set(next.min(limit));
                return;
            }
            if now < days {
                return;
            }
            let held = held_ms.get_value() + TICK_MS;
            held_ms.set_value(held);
            if held == END_HOLD_MS {
                state.go(Screen::Debrief);
            }
        },
        Duration::from_millis(TICK_MS),
    );
    if let Ok(handle) = handle {
        on_cleanup(move || handle.clear());
    }
}

#[component]
pub fn PlaybackControls(speed: RwSignal<f64>) -> impl IntoView {
    let state = expect_context::<AppState>();
    let finished = move || state.playback_finished();
    let skip = move |_| {
        state.playback.set(state.run_days() as f64);
        state.go(Screen::Debrief);
    };
    view! {
        <div class="playback-controls">
            {SPEEDS
                .into_iter()
                .map(|s| view! {
                    <button
                        class="btn btn-small"
                        class:selected=move || speed.get() == s
                        aria-pressed=move || (speed.get() == s).to_string()
                        on:click=move |_| speed.set(s)
                    >
                        {format!("{s:.0}\u{00D7}")}
                    </button>
                })
                .collect_view()}
            <button class="btn btn-small" on:click=skip>"Skip \u{23ED}"</button>
            <button
                class="btn btn-primary"
                class:pulse=finished
                disabled=move || !finished()
                on:click=move |_| state.go(Screen::Debrief)
            >
                "Debrief"
            </button>
        </div>
    }
}
