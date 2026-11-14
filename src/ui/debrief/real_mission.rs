//! The Real-Mission Match: the real mission `sim` picked as most like this one, and why.
use leptos::prelude::*;

use crate::ui::state::AppState;

#[component]
pub fn RealMissionSection() -> impl IntoView {
    let state = expect_context::<AppState>();
    let matched = move || state.run_result.with(|r| r.as_ref().and_then(|r| r.debrief.real_mission_match.clone()));
    view! {
        {move || matched().map(|m| view! {
            <div class="panel real-mission">
                <h3>"Real-Mission Match"</h3>
                <p class="real-mission-name">
                    <strong>{m.name}</strong>
                    <span class="muted">{format!(" \u{00B7} {} \u{00B7} {} \u{00B7} {}", m.operator, m.year, m.size_note)}</span>
                </p>
                <p>{m.blurb}</p>
                <p class="muted">{m.why}</p>
                <a href=m.url target="_blank" rel="noopener noreferrer">"Read about it (opens in a new tab)"</a>
            </div>
        })}
    }
}
