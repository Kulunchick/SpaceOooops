//! History screen (past Runs from local storage) and the title-screen extras: History button and notices.
use leptos::prelude::*;
use spaceooops::sim;

use super::format::{money, num};
use super::history_store::{self, HistoryRecord};
use super::state::{AppState, Screen};

/// Shown on the title screen: any notice (e.g. a broken share link) and the way to the History screen.
#[component]
pub fn TitleExtras() -> impl IntoView {
    let state = expect_context::<AppState>();
    view! {
        {move || state.notice.get().map(|m| view! { <p class="notice" role="alert">{m}</p> })}
        <button class="btn" on:click=move |_| { state.notice.set(None); state.go(Screen::History) }>"History"</button>
    }
}

fn when_text(ms: f64) -> String {
    let d = leptos::web_sys::js_sys::Date::new(&leptos::wasm_bindgen::JsValue::from_f64(ms));
    String::from(d.to_locale_string("en-GB", &leptos::wasm_bindgen::JsValue::UNDEFINED))
}

#[component]
pub fn HistoryScreen() -> impl IntoView {
    let state = expect_context::<AppState>();
    let message = RwSignal::new(None::<String>);
    let load = move |r: HistoryRecord| match state.catalog.with_value(|c| sim::decode_design(&r.design_code, c)) {
        Ok((design, scenario_id)) => {
            state.scenario_id.set(scenario_id);
            state.design.set(design);
            state.go(Screen::Design);
        }
        Err(e) => message.set(Some(format!("That Design can no longer be loaded: {e}."))),
    };
    view! {
        <section class="screen history-screen lp-panel">
            <h2 class="lp-display">"History"</h2>
            {move || message.get().map(|m| view! { <p class="notice" role="alert">{m}</p> })}
            {move || {
                let records = state.history.get();
                if records.is_empty() {
                    return view! { <p class="muted">"No Runs yet. Fly a Design and it will appear here."</p> }.into_any();
                }
                view! {
                    <ul class="history-list">
                        {records.into_iter().map(|r| {
                            let name = state.catalog.with_value(|c| c.scenario(&r.scenario_id).map(|s| s.name.clone()))
                                .unwrap_or_else(|| r.scenario_id.clone());
                            let verdict = if r.success { "Success" } else { "Failed" };
                            let rec = r.clone();
                            view! {
                                <li class="history-item">
                                    <div>
                                        <strong>{name}</strong>
                                        " \u{00B7} " <span class:ok=r.success class:bad=!r.success>{verdict}</span>
                                        <p class="muted">{when_text(r.timestamp_ms)}</p>
                                        <p class="num">
                                            {format!("Score {} \u{00B7} Science {:.1} \u{00B7} {} \u{00B7} {:.1} kg \u{00B7} {} days",
                                                num(r.score.round()), r.science_value, money(r.cost_usd), r.mass_kg, r.days_survived)}
                                        </p>
                                    </div>
                                    <button class="lp-btn" on:click=move |_| load(rec.clone())>"Load this Design"</button>
                                </li>
                            }
                        }).collect_view()}
                    </ul>
                }.into_any()
            }}
            <div class="actions">
                <button class="lp-btn" on:click=move |_| state.go(Screen::Title)>"Back"</button>
                <button class="lp-btn" on:click=move |_| { history_store::clear(); state.history.set(Vec::new()); }
                    disabled=move || state.history.with(|h| h.is_empty())>"Clear history"</button>
            </div>
        </section>
    }
}
