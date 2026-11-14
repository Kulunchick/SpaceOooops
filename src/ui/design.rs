//! Design screen: gauges always visible, four Role tabs, and the way on to Launch Review.
use leptos::prelude::*;
use spaceooops::sim::Role;

use super::budget_comms_tab::BudgetCommsTab;
use super::gauges::Gauges;
use super::launch_orbit_tab::LaunchOrbitTab;
use super::role_banner::RoleBanner;
use super::science_tab::ScienceTab;
use super::spacecraft_tab::SpacecraftTab;
use super::state::{AppState, Screen};

#[component]
pub fn DesignScreen() -> impl IntoView {
    let state = expect_context::<AppState>();
    let id = state.scenario_id.get_untracked();
    let (name, hints) = state
        .catalog
        .with_value(|c| c.scenario(&id).map(|s| (s.name.clone(), s.tutorial_hints.clone())))
        .unwrap_or_default();
    let tab = state.tab;
    view! {
        <section class="screen design-screen">
            <header class="design-head">
                <h2>"Design \u{00B7} " {name}</h2>
                <div class="actions">
                    <button class="btn" on:click=move |_| state.go(Screen::Brief)>"Scenario brief"</button>
                    <button class="btn btn-primary" on:click=move |_| state.go(Screen::LaunchReview)>"Launch Review"</button>
                </div>
            </header>
            <div class="gauges-sticky"><Gauges /></div>
            <nav class="tabs" role="tablist">
                {Role::ALL
                    .into_iter()
                    .map(|role| view! {
                        <button
                            class="tab"
                            role="tab"
                            class:active=move || tab.get() == role
                            aria-selected=move || (tab.get() == role).to_string()
                            on:click=move |_| tab.set(role)
                        >
                            {role.name()}
                        </button>
                    })
                    .collect_view()}
            </nav>
            <RoleBanner />
            {(!hints.is_empty()).then(|| view! {
                <details class="panel hints">
                    <summary>"Hints"</summary>
                    <ul>{hints.into_iter().map(|h| view! { <li>{h}</li> }).collect_view()}</ul>
                </details>
            })}
            {move || match tab.get() {
                Role::Science => view! { <ScienceTab /> }.into_any(),
                Role::Spacecraft => view! { <SpacecraftTab /> }.into_any(),
                Role::LaunchOrbit => view! { <LaunchOrbitTab /> }.into_any(),
                Role::BudgetComms => view! { <BudgetCommsTab /> }.into_any(),
            }}
        </section>
    }
}
