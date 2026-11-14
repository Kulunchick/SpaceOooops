//! UI root. One file per screen/component; shared app state lives in `state.rs`.
//! The UI holds no game rules: it renders what `sim` returns.
mod brief;
mod budget_comms_tab;
mod cost_table;
mod design;
mod format;
mod gauges;
mod launch_orbit_tab;
mod launch_review;
mod option_card;
mod placeholder;
mod scenario_select;
mod science_tab;
mod spacecraft_tab;
mod state;
mod tier_select;
mod title;

use leptos::prelude::*;
use spaceooops::sim;

pub use state::{AppState, Screen};

#[component]
pub fn App() -> impl IntoView {
    let catalog = match sim::load_catalog() {
        Ok(c) => c,
        Err(e) => {
            return view! {
                <main class="app">
                    <section class="screen panel">
                        <h2>"The game data could not be loaded"</h2>
                        <p class="muted">{e.to_string()}</p>
                    </section>
                </main>
            }
            .into_any();
        }
    };
    let state = AppState::new(catalog);
    provide_context(state);
    let screen = state.screen;
    view! {
        <main class="app">
            {move || match screen.get() {
                Screen::Title => view! { <title::TitleScreen on_start=move || state.go(Screen::ScenarioSelect) /> }.into_any(),
                Screen::ScenarioSelect => view! { <scenario_select::ScenarioSelectScreen /> }.into_any(),
                Screen::Brief => view! { <brief::BriefScreen /> }.into_any(),
                Screen::Design => view! { <design::DesignScreen /> }.into_any(),
                Screen::LaunchReview => view! { <launch_review::LaunchReviewScreen /> }.into_any(),
                Screen::Run => view! { <placeholder::PlaceholderScreen on_back=move || state.go(Screen::Design) /> }.into_any(),
            }}
        </main>
    }
    .into_any()
}
