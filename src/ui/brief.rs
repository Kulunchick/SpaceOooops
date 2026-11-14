//! Scenario brief: objective, required Science Value, minimum Lifetime, Budget and why it matters.
use leptos::prelude::*;

use super::format::money;
use super::state::{AppState, Screen};

#[component]
pub fn BriefScreen() -> impl IntoView {
    let state = expect_context::<AppState>();
    let id = state.scenario_id.get_untracked();
    let scenario = state.catalog.with_value(|c| c.scenario(&id).cloned());
    let Some(s) = scenario else {
        return view! {
            <section class="screen">
                <button class="btn" on:click=move |_| state.go(Screen::ScenarioSelect)>"Choose a Scenario"</button>
            </section>
        }
        .into_any();
    };
    view! {
        <section class="screen brief">
            <p class="muted">"Scenario brief \u{00B7} " {s.difficulty.clone()}</p>
            <h2>{s.name.clone()}</h2>
            <div class="panel">
                <h3>"Objective"</h3>
                <p>{s.objective.clone()}</p>
                <h3>"Why it matters"</h3>
                <p>{s.why_it_matters.clone()}</p>
            </div>
            <dl class="brief-facts">
                <div class="panel"><dt>"Required Science Value"</dt><dd>{format!("{:.0}", s.required_science_value)}</dd></div>
                <div class="panel">
                    <dt>"Minimum Lifetime"</dt>
                    <dd>{format!("{} days (about {:.0} months)", s.min_lifetime_days, f64::from(s.min_lifetime_days) / 30.4)}</dd>
                </div>
                <div class="panel"><dt>"Budget"</dt><dd>{money(s.budget_usd)}</dd></div>
            </dl>
            <div class="panel">
                <h3>"Key constraints"</h3>
                <ul>{s.key_constraints.iter().map(|k| view! { <li>{k.clone()}</li> }).collect_view()}</ul>
            </div>
            <div class="actions">
                <button class="btn" on:click=move |_| state.go(Screen::ScenarioSelect)>"Back"</button>
                <button class="btn btn-primary" on:click=move |_| state.go(Screen::Design)>"Start Design"</button>
            </div>
        </section>
    }
    .into_any()
}
