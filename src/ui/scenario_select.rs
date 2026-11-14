//! Scenario select: every Scenario in the catalog with difficulty, objective and key constraints.
use leptos::prelude::*;
use spaceooops::sim::Design;

use super::format::money;
use super::state::{AppState, Screen};

#[component]
pub fn ScenarioSelectScreen() -> impl IntoView {
    let state = expect_context::<AppState>();
    let scenarios = state.catalog.with_value(|c| c.scenarios.clone());
    view! {
        <section class="screen">
            <h2>"Choose a Scenario"</h2>
            <div class="scenario-list">
                {scenarios
                    .into_iter()
                    .map(|s| {
                        let id = s.id.clone();
                        view! {
                            <article class="panel scenario-card">
                                <header class="scenario-head">
                                    <h3>{s.name.clone()}</h3>
                                    <span class=format!("difficulty difficulty-{}", s.difficulty.to_lowercase())>
                                        {s.difficulty.clone()}
                                    </span>
                                </header>
                                <p>{s.objective.clone()}</p>
                                <p class="muted">"Budget: " {money(s.budget_usd)}</p>
                                <ul class="constraints">
                                    {s.key_constraints.iter().map(|k| view! { <li>{k.clone()}</li> }).collect_view()}
                                </ul>
                                <button
                                    class="btn btn-primary"
                                    on:click=move |_| {
                                        if state.scenario_id.get_untracked() != id {
                                            state.design.set(Design::default());
                                            state.scenario_id.set(id.clone());
                                        }
                                        state.go(Screen::Brief);
                                    }
                                >
                                    "Select"
                                </button>
                            </article>
                        }
                    })
                    .collect_view()}
            </div>
            <button class="btn" on:click=move |_| state.go(Screen::Title)>"Back"</button>
        </section>
    }
}
