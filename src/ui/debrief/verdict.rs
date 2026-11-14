//! Debrief headline: Success or failure.
use leptos::prelude::*;

use crate::ui::state::AppState;

#[component]
pub fn Verdict() -> impl IntoView {
    let state = expect_context::<AppState>();
    let success = move || state.run_result.with(|r| r.as_ref().is_some_and(|r| r.debrief.success));
    let total = move || state.run_result.with(|r| r.as_ref().map_or(0.0, |r| r.debrief.score.total));
    view! {
        <div class="panel verdict" class:verdict-success=success class:verdict-failure=move || !success()>
            <div class="verdict-icon">{move || if success() { "\u{2714}" } else { "\u{2716}" }}</div>
            <div>
                <div class="verdict-title">{move || if success() { "Success" } else { "Mission failed" }}</div>
                <div class="verdict-score">"Score " {move || format!("{:.0}", total())}</div>
            </div>
        </div>
    }
}
