//! Interactive Random Events on the Run screen: the playback pauses before a day that asks the player a
//! question, this panel shows the question and its two options, and a choice re-flies the Run through
//! `AppState::decide`. All texts come from `sim`.
use leptos::prelude::*;
use spaceooops::sim::{Decision, DecisionPrompt, RunResult};

use crate::ui::state::AppState;

/// The first unanswered question whose day has not been played yet. The playback stops at the end of the
/// day before it (`day - 1`) until it is answered.
pub fn next_prompt(result: &RunResult, decisions: &[Decision], playback: f64) -> Option<DecisionPrompt> {
    result
        .timeline
        .iter()
        .flat_map(|d| d.events.iter())
        .filter_map(|e| e.prompt.as_ref())
        .find(|p| f64::from(p.day) - 1.0 >= playback - 1e-9 && !decisions.iter().any(|d| d.prompt_id == p.id))
        .cloned()
}

/// Untracked: for the playback driver's timer.
pub fn pause_at(state: &AppState) -> Option<f64> {
    let playback = state.playback.get_untracked();
    state.decisions.with_untracked(|decisions| {
        state.run_result.with_untracked(|r| {
            r.as_ref().and_then(|r| next_prompt(r, decisions, playback)).map(|p| f64::from(p.day) - 1.0)
        })
    })
}

#[component]
pub fn DecisionPromptPanel() -> impl IntoView {
    let state = expect_context::<AppState>();
    // The question the playback is waiting on, if it has reached it.
    let waiting = Memo::new(move |_| {
        let playback = state.playback.get();
        state.decisions.with(|decisions| {
            state.run_result.with(|r| {
                r.as_ref()
                    .and_then(|r| next_prompt(r, decisions, playback))
                    .filter(|p| playback >= f64::from(p.day) - 1.0 - 1e-9)
            })
        })
    });
    move || {
        waiting.get().map(|prompt| {
            let id = prompt.id.clone();
            let default = prompt.default.clone();
            view! {
                <div class="panel decision-prompt" role="alertdialog" aria-live="assertive" aria-label="Decision">
                    <p class="decision-day">{format!("Day {}", prompt.day)}</p>
                    <p class="decision-question">{prompt.question.clone()}</p>
                    <div class="decision-options">
                        {prompt
                            .options
                            .into_iter()
                            .map(|o| {
                                let decision = Decision { prompt_id: id.clone(), choice: o.id.clone() };
                                let is_default = o.id == default;
                                view! {
                                    <button
                                        class="decision-option"
                                        class:decision-default=is_default
                                        on:click=move |_| state.decide(decision.clone())
                                    >
                                        <span class="decision-label">{o.label}</span>
                                        <span class="decision-consequence">{o.consequence}</span>
                                    </button>
                                }
                            })
                            .collect_view()}
                    </div>
                </div>
            }
        })
    }
}
