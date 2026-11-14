//! Team setup screen: solo or team (2-4 players), a name per player, and which player holds each Role.
use leptos::prelude::*;
use spaceooops::sim::Role;

use super::state::{AppState, Screen};
use super::team::{MAX_TEAM, MIN_TEAM, role_index};

#[component]
pub fn TeamSetupScreen() -> impl IntoView {
    let state = expect_context::<AppState>();
    let team = state.team;
    let solo = move || team.with(|t| t.solo);
    let count = move || team.with(|t| t.players.len());
    view! {
        <section class="screen team-setup lp-panel">
            <h2 class="lp-display">"Who is playing?"</h2>
            <div class="panel">
                <div class="mode-switch" role="group" aria-label="Play mode">
                    <button class="lp-btn mode-btn" class:active=solo on:click=move |_| team.update(|t| t.set_solo())>
                        "Solo"
                    </button>
                    <button
                        class="lp-btn mode-btn"
                        class:active=move || !solo()
                        on:click=move |_| if solo() { team.update(|t| t.set_team(MIN_TEAM)) }
                    >
                        "Team (2\u{2013}4 players)"
                    </button>
                </div>
                <p class="muted">
                    {move || if solo() {
                        "One player holds all four Roles. Personal Objectives are optional bonuses and there is no Go/No-Go vote."
                    } else {
                        "Share one screen. Each Role has an owner with a Personal Objective, and every Role must vote Go before Launch."
                    }}
                </p>
                <Show when=move || !solo()>
                    <label class="field">
                        "Players "
                        <select
                            class="select"
                            aria-label="Number of players"
                            on:change=move |ev| {
                                let n = event_target_value(&ev).parse().unwrap_or(MIN_TEAM);
                                team.update(|t| t.set_team(n));
                            }
                            prop:value=move || count().to_string()
                        >
                            {(MIN_TEAM..=MAX_TEAM)
                                .map(|n| view! { <option value=n.to_string()>{n.to_string()}</option> })
                                .collect_view()}
                        </select>
                    </label>
                </Show>
            </div>
            <div class="panel">
                <h3>"Players"</h3>
                <div class="player-names">
                    {move || (0..count())
                        .map(|i| view! {
                            <label class="field">
                                {format!("Player {} ", i + 1)}
                                <input
                                    class="text-input"
                                    type="text"
                                    maxlength="24"
                                    placeholder=format!("Player {}", i + 1)
                                    prop:value=move || team.with(|t| t.players.get(i).cloned().unwrap_or_default())
                                    on:input=move |ev| {
                                        let name = event_target_value(&ev);
                                        team.update(|t| if let Some(p) = t.players.get_mut(i) { *p = name });
                                    }
                                />
                            </label>
                        })
                        .collect_view()}
                </div>
            </div>
            <Show when=move || !solo()>
                <div class="panel">
                    <h3>"Roles"</h3>
                    <p class="muted">"With fewer than four players, some players hold several Roles."</p>
                    <ul class="role-owners">
                        {Role::ALL
                            .into_iter()
                            .map(|role| view! {
                                <li class="field">
                                    <span class="role-name">{role.name()}</span>
                                    <select
                                        class="select"
                                        aria-label=format!("Owner of {}", role.name())
                                        on:change=move |ev| {
                                            let p = event_target_value(&ev).parse().unwrap_or(0);
                                            team.update(|t| t.owner[role_index(role)] = p);
                                        }
                                        prop:value=move || team.with(|t| t.owner[role_index(role)]).to_string()
                                    >
                                        {move || (0..count())
                                            .map(|p| view! {
                                                <option value=p.to_string()>{team.with(|t| t.player_name(p))}</option>
                                            })
                                            .collect_view()}
                                    </select>
                                </li>
                            })
                            .collect_view()}
                    </ul>
                    <Show when=move || !team.with(|t| t.is_complete())>
                        <p class="finding violated">"Every player needs at least one Role."</p>
                    </Show>
                </div>
            </Show>
            <div class="actions">
                <button class="lp-btn" on:click=move |_| state.go(Screen::Title)>"Back"</button>
                <button
                    class="lp-btn lp-btn-primary"
                    disabled=move || !team.with(|t| t.is_complete())
                    on:click=move |_| state.go(Screen::ScenarioSelect)
                >
                    "Choose a Scenario"
                </button>
            </div>
        </section>
    }
}
