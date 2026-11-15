//! Scenario select: every Scenario in the catalog with difficulty, objective and key constraints, as cards beside
//! the globe. Each card has an illustration slot (placeholder art from the Scenario data, easy to swap).
use leptos::prelude::*;
use spaceooops::sim::{scenarios_in_card_order, Design};

use super::format::money;
use super::state::{AppState, Screen};

#[component]
pub fn ScenarioSelectScreen() -> impl IntoView {
    let state = expect_context::<AppState>();
    let scenarios = state.catalog.with_value(|c| scenarios_in_card_order(&c.scenarios).into_iter().cloned().collect::<Vec<_>>());
    view! {
        <section class="screen scenario-screen">
            <h2 class="lp-display scenario-title">"Choose a Scenario"</h2>
            <p class="scenario-hint">"Each pin on the globe is one Scenario, somewhere around the world."</p>
            <div class="scenario-list">
                {scenarios
                    .into_iter()
                    .map(|s| {
                        let id = s.id.clone();
                        view! {
                            <article class="lp-panel scenario-card">
                                <div class="illustration-slot">
                                    <img src=s.illustration.clone() alt="" width="320" height="180" />
                                </div>
                                <div class="scenario-body">
                                    <header class="scenario-head">
                                        <h3 class="lp-display">{s.name.clone()}</h3>
                                        <span class=format!("difficulty difficulty-{}", s.difficulty.to_lowercase())>
                                            {s.difficulty.clone()}
                                        </span>
                                    </header>
                                    <p class="scenario-country">"Target Country: " {s.target_country.name.clone()}</p>
                                    <p>{s.objective.clone()}</p>
                                    <p class="muted">"Budget: " {money(s.budget_usd)}</p>
                                    <ul class="constraints">
                                        {s.key_constraints.iter().map(|k| view! { <li>{k.clone()}</li> }).collect_view()}
                                    </ul>
                                    <button
                                        class="lp-btn lp-btn-primary"
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
                                </div>
                            </article>
                        }
                    })
                    .collect_view()}
            </div>
            <button class="lp-btn" on:click=move |_| state.go(Screen::Title)>"Back"</button>
        </section>
    }
}
