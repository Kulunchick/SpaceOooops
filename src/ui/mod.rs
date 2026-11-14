//! UI root. One file per screen; later tickets add screens here and extend `Screen`.
mod placeholder;
mod sources;
mod title;

use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Screen {
    Title,
    Placeholder,
    Sources,
}

#[component]
pub fn App() -> impl IntoView {
    let (screen, set_screen) = signal(Screen::Title);
    view! {
        <main class="app">
            {move || match screen.get() {
                Screen::Title => view! { <title::TitleScreen on_start=move || set_screen.set(Screen::Placeholder) on_sources=move || set_screen.set(Screen::Sources) /> }.into_any(),
                Screen::Sources => view! { <sources::SourcesScreen on_back=move || set_screen.set(Screen::Title) /> }.into_any(),
                Screen::Placeholder => view! { <placeholder::PlaceholderScreen on_back=move || set_screen.set(Screen::Title) /> }.into_any(),
            }}
        </main>
    }
}
