//! Go/No-Go at Launch Review. Team play: every Role votes and a No-Go shows its objection (the reason
//! `sim` gives for that Role's Personal Objective). Solo play: no vote, objectives are optional bonuses.
use leptos::prelude::*;
use spaceooops::sim::{self, ObjectiveOutlook, Role};

use super::state::AppState;
use super::team::role_index;

#[component]
pub fn GoNoGo() -> impl IntoView {
    let state = expect_context::<AppState>();
    let solo = move || state.team.with(|t| t.solo);
    // A trial Run: computed once per visit, and again if the Design changes.
    let outlook = Memo::new(move |_| {
        let id = state.scenario_id.get();
        state.design.with(|d| state.catalog.with_value(|c| sim::objective_outlook(c, d, &id)))
    });
    view! {
        <div class="panel go-no-go">
            <h3>{move || if solo() { "Personal Objectives (optional bonuses)" } else { "Go / No-Go" }}</h3>
            <p class="muted">
                {move || if solo() {
                    "Only counted on a successful Run. These are estimates from a trial Run of this Design."
                } else {
                    "Every Role must vote Go before Launch. Changing the Design clears the votes."
                }}
            </p>
            <ul class="vote-list">
                {Role::ALL.into_iter().map(|role| role_row(state, role, outlook)).collect_view()}
            </ul>
        </div>
    }
}

fn role_row(state: AppState, role: Role, outlook: Memo<Vec<ObjectiveOutlook>>) -> impl IntoView {
    let idx = role_index(role);
    let solo = move || state.team.with(|t| t.solo);
    let mine = move || outlook.with(|l| l.iter().find(|o| o.role == role).cloned());
    let vote = move || state.votes.with(|v| v[idx]);
    let cast = move |go: bool| state.votes.update(|v| v[idx] = Some(go));
    view! {
        <li class="vote-row" class:no-go=move || vote() == Some(false) class:go=move || vote() == Some(true)>
            <div class="vote-who">
                <span class="role-name">{role.name()}</span>
                <span class="muted">{move || state.team.with(|t| t.owner_name(role))}</span>
            </div>
            <div class="vote-objective">
                {move || mine().map(|o| view! {
                    <div>
                        {o.description.clone()} {format!(" (+{:.0})", o.bonus)}
                        <span class="outlook" class:at-risk=o.at_risk class:on-track=!o.at_risk>
                            {if o.at_risk { " At risk" } else { " On track" }}
                        </span>
                    </div>
                    {move || (solo() || vote() == Some(false)).then(|| view! {
                        <div class="muted objection">{o.reason.clone()}</div>
                    })}
                })}
            </div>
            <Show when=move || !solo()>
                <div class="vote-buttons" role="group" aria-label=format!("{} vote", role.name())>
                    <button class="btn vote-go" class:active=move || vote() == Some(true) on:click=move |_| cast(true)>
                        "Go"
                    </button>
                    <button class="btn vote-nogo" class:active=move || vote() == Some(false) on:click=move |_| cast(false)>
                        "No-Go"
                    </button>
                </div>
            </Show>
        </li>
    }
}
