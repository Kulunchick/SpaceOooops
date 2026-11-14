//! Share links: build a `#share=...` URL from the current Design, and open one at start-up.
//! A link never carries a Run result; the Design simply loads into the Design screen where Launch Review applies.
use leptos::prelude::*;
use spaceooops::sim;

use super::state::{AppState, Screen};
use super::storage;

const PREFIX: &str = "share=";

/// If the page was opened with a share link, loads its Scenario and Design into the Design screen.
/// A broken or outdated link leaves the Title screen with a friendly message.
pub fn open_link_from_url(state: AppState) {
    let fragment = storage::url_fragment();
    let Some(code) = fragment.strip_prefix(PREFIX) else { return };
    match state.catalog.with_value(|c| sim::decode_design(code, c)) {
        Ok((design, scenario_id)) => {
            state.scenario_id.set(scenario_id);
            state.design.set(design);
            state.screen.set(Screen::Design);
        }
        Err(e) => state.notice.set(Some(format!(
            "That share link could not be opened: {e}. Pick a Scenario below to start a new Design."
        ))),
    }
    storage::clear_url_fragment();
}

#[component]
pub fn ShareButton() -> impl IntoView {
    let state = expect_context::<AppState>();
    let link = RwSignal::new(None::<String>);
    let status = RwSignal::new(String::new());
    let share = move |_| {
        let code = state.design.with_untracked(|d| sim::encode_design(d, &state.scenario_id.get_untracked()));
        let url = storage::page_url_with_fragment(&format!("{PREFIX}{code}"));
        link.set(Some(url.clone()));
        status.set(String::new());
        leptos::task::spawn_local(async move {
            let msg = if storage::copy_to_clipboard(&url).await {
                "Link copied to the clipboard."
            } else {
                "Could not copy automatically; select the link below and copy it."
            };
            status.set(msg.to_string());
        });
    };
    view! {
        <div class="share">
            <button class="btn" on:click=share>"Share design"</button>
            {move || link.get().map(|url| view! {
                <div class="share-box">
                    <input class="share-link" type="text" readonly aria-label="Share link" prop:value=url
                        on:focus=move |ev| {
                            use web_sys::HtmlInputElement;
                            use web_sys::wasm_bindgen::JsCast;
                            if let Some(i) = ev.target().and_then(|t| t.dyn_into::<HtmlInputElement>().ok()) { i.select(); }
                        } />
                    <p class="muted" role="status">{move || status.get()}</p>
                </div>
            })}
        </div>
    }
}
