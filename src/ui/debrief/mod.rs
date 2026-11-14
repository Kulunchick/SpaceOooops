//! Debrief screen: one small component per section, all reading the last Run from `AppState`.
//! Later tickets add a section file here and one line in the list below:
//! causes and Real-Mission Match (04), Personal Objectives (06), comparison with the previous Run (09).
mod actions;
mod causes;
mod compare;
mod decisions;
mod real_mission;
mod objectives;
mod score;
mod stats;
mod verdict;

use leptos::prelude::*;

use super::state::AppState;
use actions::DebriefActions;
use compare::ComparisonSection;
use score::ScoreSection;
use stats::StatsSection;
use verdict::Verdict;

#[component]
pub fn DebriefScreen() -> impl IntoView {
    let state = expect_context::<AppState>();
    let has_run = move || state.run_result.with(|r| r.is_some());
    view! {
        <section class="screen debrief lp-panel">
            <h2 class="lp-display">"Debrief"</h2>
            <Show when=has_run fallback=|| view! { <p class="muted">"No Run yet."</p> }>
                <Verdict />
                <causes::CausesSection />
                <ScoreSection />
                <StatsSection />
                <decisions::DecisionsSection />
                <objectives::ObjectivesSection />
                <real_mission::RealMissionSection />
                <ComparisonSection />
            </Show>
            <DebriefActions />
        </section>
    }
}
