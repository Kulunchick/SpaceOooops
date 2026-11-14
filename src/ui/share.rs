//! Share links: build a `#share=...` URL from the current Design, and open one at start-up.
//! A link never carries a Run result; the Design simply loads into the Design screen where Launch Review applies.
use leptos::prelude::*;
use spaceooops::sim;

use super::state::{AppState, Screen};
use super::storage;

const PREFIX: &str = "share=";

/// If the page's URL carries a share link, loads its Scenario and Design into the Design screen. Called at
/// start-up and on every `hashchange`, so a link pasted into an already-open tab loads too; leaving the Run
/// screen stops any playback in progress. A broken or outdated link goes to the Title screen with a friendly message.
pub fn open_link_from_url(state: AppState) {
    let fragment = storage::url_fragment();
    let Some(code) = fragment.strip_prefix(PREFIX) else { return };
    match state.catalog.with_value(|c| sim::decode_design(code, c)) {
        Ok((design, scenario_id)) => {
            state.notice.set(None);
            state.scenario_id.set(scenario_id);
            state.design.set(design);
            state.screen.set(Screen::Design);
        }
        Err(e) => {
            state.notice.set(Some(format!(
                "That share link could not be opened: {e}. Press Start to choose a Scenario and begin a new Design."
            )));
            state.screen.set(Screen::Title);
        }
    }
    storage::clear_url_fragment();
}

/// Opens share links pasted into the address bar of the already-open tab (the browser only fires `hashchange`).
pub fn listen_for_links(state: AppState) {
    let handle = window_event_listener(leptos::ev::hashchange, move |_| open_link_from_url(state));
    on_cleanup(move || handle.remove());
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
