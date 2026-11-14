use leptos::prelude::*;
use spaceooops::sim;

#[component]
pub fn TitleScreen(on_start: impl Fn() + 'static) -> impl IntoView {
    view! {
        <section class="screen title-screen">
            <h1 class="title">"SpaceOooops"</h1>
            <p class="tagline">{sim::tagline()}</p>
            <button class="btn btn-primary" on:click=move |_| on_start()>"Start"</button>
        </section>
    }
}
