//! The decisions taken during the Run and what they did, as `sim` wrote them (ticket 10).
use leptos::prelude::*;

use crate::ui::state::AppState;

#[component]
pub fn DecisionsSection() -> impl IntoView {
    let state = expect_context::<AppState>();
    let notes = move || state.run_result.with(|r| r.as_ref().map(|r| r.debrief.decisions.clone()).unwrap_or_default());
    view! {
        <Show when=move || !notes().is_empty()>
            <div class="panel debrief-decisions">
                <h3>"Your decisions"</h3>
                <ul class="decision-notes">
                    {move || notes().into_iter().map(|n| view! { <li>{n}</li> }).collect_view()}
                </ul>
            </div>
        </Show>
    }
}
