//! Personal Objectives: each Role, its player, and whether the objective was met and the bonus earned.
use leptos::prelude::*;

use crate::ui::state::AppState;

#[component]
pub fn ObjectivesSection() -> impl IntoView {
    let state = expect_context::<AppState>();
    let results = move || state.run_result.with(|r| r.as_ref().map(|r| r.debrief.objectives.clone()).unwrap_or_default());
    view! {
        <div class="panel objectives-section">
            <h3>"Personal Objectives"</h3>
            <ul class="objective-results">
                {move || results()
                    .into_iter()
                    .map(|o| {
                        let owner = state.team.with(|t| t.owner_name(o.role));
                        view! {
                            <li class="objective-result" class:met=o.met class:missed=!o.met>
                                <span class="objective-mark">{if o.met { "\u{2714}" } else { "\u{2716}" }}</span>
                                <div>
                                    <div>
                                        <span class="role-name">{o.role.name()}</span>
                                        <span class="muted">" \u{00B7} " {owner}</span>
                                    </div>
                                    <div>{o.description}</div>
                                    <div class="muted">{o.detail}</div>
                                </div>
                                <span class="objective-points num">
                                    {if o.met { format!("+{:.0}", o.bonus) } else { format!("missed ({:.0})", o.bonus) }}
                                </span>
                            </li>
                        }
                    })
                    .collect_view()}
            </ul>
        </div>
    }
}
