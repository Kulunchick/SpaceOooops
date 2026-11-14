//! Design screen, laid out around the hangar: a header with the Role tabs and actions, the open Role as a
//! translucent side panel (Science and Spacecraft on the left, Launch & Orbit and Budget & Comms on the right),
//! an empty stage in the middle where the 3D spacecraft is assembled (drag on it to turn the model), and the
//! gauge strip along the bottom edge. Without WebGL the stage folds away and the panel takes the width.
use leptos::prelude::*;
use spaceooops::sim::Role;

use super::budget_comms_tab::BudgetCommsTab;
use super::gauges::Gauges;
use super::launch_orbit_tab::LaunchOrbitTab;
use super::role_banner::RoleBanner;
use super::science_tab::ScienceTab;
use super::spacecraft_tab::SpacecraftTab;
use super::state::{AppState, Screen};

/// Which side of the model a Role's panel docks on, so the four Roles sit around the shared spacecraft.
fn panel_side(role: Role) -> &'static str {
    match role {
        Role::Science | Role::Spacecraft => "left",
        Role::LaunchOrbit | Role::BudgetComms => "right",
    }
}

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
        <section class="screen design-screen hangar-layout" data-panel-side=move || panel_side(tab.get())>
            <header class="design-head">
                <h2 class="lp-display">"Design \u{00B7} " {name}</h2>
                <nav class="tabs" role="tablist" aria-label="Roles">
                    {Role::ALL
                        .into_iter()
                        .map(|role| view! {
                            <button
                                class="tab"
                                role="tab"
                                data-side=panel_side(role)
                                class:active=move || tab.get() == role
                                aria-selected=move || (tab.get() == role).to_string()
                                on:click=move |_| tab.set(role)
                            >
                                {role.name()}
                            </button>
                        })
                        .collect_view()}
                </nav>
                <div class="actions">
                    <button class="btn" on:click=move |_| state.go(Screen::Brief)>"Scenario brief"</button>
                    <button class="btn btn-primary" on:click=move |_| state.go(Screen::LaunchReview)>"Launch Review"</button>
                    <super::share::ShareButton />
                </div>
            </header>
            <div class="design-main">
                <div class="role-panel lp-panel" role="tabpanel">
                    <RoleBanner />
                    <super::hints::FirstTimeHints scenario_id=id.clone() hints=hints />
                    {move || match tab.get() {
                        Role::Science => view! { <ScienceTab /> }.into_any(),
                        Role::Spacecraft => view! { <SpacecraftTab /> }.into_any(),
                        Role::LaunchOrbit => view! { <LaunchOrbitTab /> }.into_any(),
                        Role::BudgetComms => view! { <BudgetCommsTab /> }.into_any(),
                    }}
                </div>
                <div class="hangar-stage" data-scene-drag aria-hidden="true">
                    <span class="hangar-hint">"Drag to turn the spacecraft"</span>
                </div>
            </div>
            <div class="gauge-strip"><Gauges /></div>
        </section>
    }
}
