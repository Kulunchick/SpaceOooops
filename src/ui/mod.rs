//! UI root. One file per screen/component; shared app state lives in `state.rs`.
//! The UI holds no game rules: it renders what `sim` returns.
mod brief;
mod budget_comms_tab;
mod cost_table;
mod debrief;
mod design;
mod format;
mod gauges;
mod history;
mod history_store;
mod go_no_go;
mod launch_orbit_tab;
mod launch_review;
mod option_card;
mod role_banner;
mod run;
mod scenario_select;
mod science_tab;
mod seed;
mod share;
mod sources;
mod spacecraft_tab;
mod state;
mod storage;
mod team;
mod team_setup;
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
    share::open_link_from_url(state);
    let screen = state.screen;
    view! {
        <main class="app">
            {move || match screen.get() {
                Screen::Title => view! {
                    <title::TitleScreen on_start=move || state.go(Screen::TeamSetup) on_sources=move || state.go(Screen::Sources) />
                }.into_any(),
                Screen::Sources => view! { <sources::SourcesScreen on_back=move || state.go(Screen::Title) /> }.into_any(),
                Screen::TeamSetup => view! { <team_setup::TeamSetupScreen /> }.into_any(),
                Screen::ScenarioSelect => view! { <scenario_select::ScenarioSelectScreen /> }.into_any(),
                Screen::Brief => view! { <brief::BriefScreen /> }.into_any(),
                Screen::Design => view! { <design::DesignScreen /> }.into_any(),
                Screen::LaunchReview => view! { <launch_review::LaunchReviewScreen /> }.into_any(),
                Screen::Run => view! { <run::RunScreen /> }.into_any(),
                Screen::History => view! { <history::HistoryScreen /> }.into_any(),
                Screen::Debrief => view! { <debrief::DebriefScreen /> }.into_any(),
            }}
        </main>
    }
    .into_any()
}
