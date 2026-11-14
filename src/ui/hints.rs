//! First-time hints on the Design screen, from the Scenario's `tutorial_hints` data (only Crop Watch has any,
//! so it doubles as the tutorial). Dismissed once, they stay hidden in this browser.
use leptos::prelude::*;

use super::storage;

#[component]
pub fn FirstTimeHints(scenario_id: String, hints: Vec<String>) -> impl IntoView {
    let key = format!("spaceooops.hints-dismissed.{scenario_id}");
    let dismissed = RwSignal::new(hints.is_empty() || storage::read(&key).is_some());
    move || {
        (!dismissed.get()).then(|| {
            let key = key.clone();
            view! {
                <aside class="panel first-hints" aria-label="First-time hints">
                    <div class="first-hints-head">
                        <h3>"First time? Start here"</h3>
                        <button
                            class="btn"
                            on:click=move |_| {
                                storage::write(&key, "1");
                                dismissed.set(true);
                            }
                        >
                            "Got it, hide hints"
                        </button>
                    </div>
                    <ol>{hints.iter().map(|h| view! { <li>{h.clone()}</li> }).collect_view()}</ol>
                    <p class="muted">"Tap any \u{201C}?\u{201D} to learn why a number matters."</p>
                </aside>
            }
        })
    }
}
