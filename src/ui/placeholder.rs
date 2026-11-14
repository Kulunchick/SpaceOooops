use leptos::prelude::*;

#[component]
pub fn PlaceholderScreen(on_back: impl Fn() + 'static) -> impl IntoView {
    view! {
        <section class="screen placeholder-screen">
            <h2>"Mission design"</h2>
            <p class="muted">"Coming soon."</p>
            <button class="btn" on:click=move |_| on_back()>"Back"</button>
        </section>
    }
}
