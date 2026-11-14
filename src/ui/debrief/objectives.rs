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
                        // Held but not counted: the rule held, yet bonuses need a successful Mission.
                        let not_counted = o.held && !o.met;
                        let mark = if o.met { "\u{2714}" } else if not_counted { "\u{2013}" } else { "\u{2716}" };
                        let points = if o.met {
                            format!("+{:.0}", o.bonus)
                        } else if not_counted {
                            format!("held, not counted ({:.0})", o.bonus)
                        } else {
                            format!("missed ({:.0})", o.bonus)
                        };
                        view! {
                            <li class="objective-result" class:met=o.met class:missed=!o.met && !not_counted class:not-counted=not_counted>
                                <span class="objective-mark">{mark}</span>
                                <div>
                                    <div>
                                        <span class="role-name">{o.role.name()}</span>
                                        <span class="muted">" \u{00B7} " {owner}</span>
                                    </div>
                                    <div>{o.description}</div>
                                    <div class="muted">{o.detail}</div>
                                </div>
                                <span class="objective-points num">
                                    {points}
                                </span>
                            </li>
                        }
                    })
                    .collect_view()}
            </ul>
        </div>
    }
}
