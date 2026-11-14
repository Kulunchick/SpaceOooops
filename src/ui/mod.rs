//! UI root. One file per screen; later tickets add screens here and extend `Screen`.
mod placeholder;
mod title;

use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Screen {
    Title,
    Placeholder,
}

#[component]
pub fn App() -> impl IntoView {
    let (screen, set_screen) = signal(Screen::Title);
    view! {
        <main class="app">
            {move || match screen.get() {
                Screen::Title => view! { <title::TitleScreen on_start=move || set_screen.set(Screen::Placeholder) /> }.into_any(),
                Screen::Placeholder => view! { <placeholder::PlaceholderScreen on_back=move || set_screen.set(Screen::Title) /> }.into_any(),
            }}
        </main>
    }
}
