//! The Score breakdown: every line `sim` produced, then the total.
use leptos::prelude::*;

use crate::ui::state::AppState;

#[component]
pub fn ScoreSection() -> impl IntoView {
    let state = expect_context::<AppState>();
    let score = move || state.run_result.with(|r| r.as_ref().map(|r| r.debrief.score.clone()));
    view! {
        <div class="panel">
            <h3>"Score"</h3>
            {move || score().map(|score| view! {
                <table class="score-table">
                    <tbody>
                        {score
                            .lines
                            .into_iter()
                            .map(|line| view! {
                                <tr class:negative=line.points < 0.0>
                                    <td>{line.label}</td>
                                    <td class="num">{format!("{:+.1}", line.points)}</td>
                                </tr>
                            })
                            .collect_view()}
                        <tr class="total">
                            <td>"Score"</td>
                            <td class="num">{format!("{:.1}", score.total)}</td>
                        </tr>
                    </tbody>
                </table>
            })}
        </div>
    }
}
