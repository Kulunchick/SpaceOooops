//! Debrief buttons: back to the Design with every choice intact, or fly the same Design with a new seed.
use leptos::prelude::*;

use crate::ui::state::{AppState, Screen};

#[component]
pub fn DebriefActions() -> impl IntoView {
    let state = expect_context::<AppState>();
    view! {
        <div class="actions">
            <button class="btn" on:click=move |_| state.go(Screen::Design)>"Modify design"</button>
            <button class="btn btn-primary" on:click=move |_| state.launch()>"Fly again"</button>
        </div>
    }
}
