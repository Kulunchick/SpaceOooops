//! Under the Role tabs: who owns the selected Role and what its Personal Objective is.
use leptos::prelude::*;
use spaceooops::sim;

use super::state::AppState;

#[component]
pub fn RoleBanner() -> impl IntoView {
    let state = expect_context::<AppState>();
    let objective = move || {
        let role = state.tab.get();
        let id = state.scenario_id.get();
        state.catalog.with_value(|c| sim::scenario_objectives(c, &id).into_iter().find(|o| o.role == role))
    };
    let owner = move || state.team.with(|t| t.owner_name(state.tab.get()));
    let solo = move || state.team.with(|t| t.solo);
    view! {
        <div class="panel role-banner">
            <div class="role-owner">
                {move || if solo() { "Player: " } else { "Owner: " }}
                <strong>{owner}</strong>
            </div>
            {move || objective().map(|o| view! {
                <div class="role-objective">
                    <span class="objective-tag">"Personal Objective"</span>
                    " " {o.description}
                    <span class="objective-bonus">{format!(" (+{:.0} bonus{})", o.bonus, if solo() { ", optional" } else { "" })}</span>
                </div>
            })}
        </div>
    }
}
