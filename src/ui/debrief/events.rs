//! The new Random Events in the Debrief, as `sim` wrote them: how the real ones happen in real life, and
//! which Design choices caused or softened one. Hidden when there is nothing to say.
use leptos::prelude::*;

use crate::ui::state::AppState;

#[component]
pub fn EventsSection() -> impl IntoView {
    let state = expect_context::<AppState>();
    let lines = move |pick: fn(&spaceooops::sim::Debrief) -> &Vec<String>| {
        state.run_result.with(|r| r.as_ref().map(|r| pick(&r.debrief).clone()).unwrap_or_default())
    };
    let real_life = move || lines(|d| &d.real_life);
    let design_notes = move || lines(|d| &d.design_notes);
    view! {
        <Show when=move || !real_life().is_empty() || !design_notes().is_empty()>
            <div class="panel debrief-events">
                <h3>"Random Events"</h3>
                <Show when=move || !design_notes().is_empty()>
                    <ul class="event-design-notes">
                        {move || design_notes().into_iter().map(|n| view! { <li>{n}</li> }).collect_view()}
                    </ul>
                </Show>
                <Show when=move || !real_life().is_empty()>
                    <h4>"In real life"</h4>
                    <ul class="event-real-life">
                        {move || real_life().into_iter().map(|n| view! { <li>{n}</li> }).collect_view()}
                    </ul>
                </Show>
            </div>
        </Show>
    }
}
