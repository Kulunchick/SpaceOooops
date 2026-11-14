//! Why the Mission went the way it did: the causes `sim` found, rendered verbatim, costliest first.
use leptos::prelude::*;
use spaceooops::sim::CauseKind;

use crate::ui::state::AppState;

#[component]
pub fn CausesSection() -> impl IntoView {
    let state = expect_context::<AppState>();
    let causes = move || state.run_result.with(|r| r.as_ref().map(|r| r.debrief.causes.clone()).unwrap_or_default());
    view! {
        <Show when=move || !causes().is_empty()>
            <div class="panel causes">
                <h3>"Why it went this way"</h3>
                <ul class="cause-list">
                    {move || causes()
                        .into_iter()
                        .map(|c| {
                            let good = c.kind == CauseKind::WentWell;
                            view! {
                                <li class="cause" class:cause-good=good class:cause-bad=!good>
                                    <span class="cause-icon" aria-hidden="true">{if good { "\u{2714}" } else { "\u{2192}" }}</span>
                                    <span class="cause-text">
                                        {c.day.map(|d| view! { <span class="cause-day">{format!("Day {d}")}</span> " " })}
                                        {c.text}
                                    </span>
                                </li>
                            }
                        })
                        .collect_view()}
                </ul>
            </div>
        </Show>
    }
}
