use leptos::prelude::*;
use spaceooops::sim;

#[component]
pub fn TitleScreen(on_start: impl Fn() + 'static, on_sources: impl Fn() + 'static) -> impl IntoView {
    view! {
        <section class="screen title-screen">
            <h1 class="title">"SpaceOooops"</h1>
            <p class="tagline">{sim::tagline()}</p>
            <div class="title-intro">
                {sim::game_intro().into_iter().map(|p| view! { <p>{p}</p> }).collect_view()}
            </div>
            <button class="btn btn-primary" on:click=move |_| on_start()>"Start"</button>
            <button class="btn" on:click=move |_| on_sources()>"Sources"</button>
            <super::history::TitleExtras />
        </section>
    }
}
