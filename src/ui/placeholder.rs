use leptos::prelude::*;

/// Stand-in for the Run screen until ticket 03.
#[component]
pub fn PlaceholderScreen(on_back: impl Fn() + 'static) -> impl IntoView {
    view! {
        <section class="screen placeholder-screen">
            <h2>"Run"</h2>
            <p class="muted">"Coming soon."</p>
            <button class="btn" on:click=move |_| on_back()>"Modify design"</button>
        </section>
    }
}
