//! A small "?" button that explains why a parameter, gauge or Component category matters. The text comes from
//! `data/glossary.toml` through the catalog. Opens on hover, keyboard focus or tap; Escape or leaving closes it.
use leptos::prelude::*;

use super::state::AppState;

/// Renders nothing when the glossary has no text for `term`.
#[component]
pub fn InfoTip(#[prop(into)] term: String) -> impl IntoView {
    let state = expect_context::<AppState>();
    let text = state.catalog.with_value(|c| c.glossary_why(&term).map(str::to_string));
    text.map(|text| {
        let open = RwSignal::new(false);
        let label = format!("Why {term} matters");
        view! {
            <span class="infotip" class:open=move || open.get() on:mouseleave=move |_| open.set(false)>
                <button
                    type="button"
                    class="infotip-btn"
                    aria-label=label
                    aria-expanded=move || open.get().to_string()
                    on:click=move |ev| {
                        // Inside an option card: explain, do not select.
                        ev.stop_propagation();
                        open.update(|o| *o = !*o);
                    }
                    on:keydown=move |ev| {
                        if ev.key() == "Escape" {
                            open.set(false);
                        }
                    }
                    on:blur=move |_| open.set(false)
                >
                    "?"
                </button>
                <span class="infotip-text" role="tooltip">{text}</span>
            </span>
        }
    })
}
