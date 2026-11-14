use leptos::prelude::*;
use spaceooops::sim;

/// The title panel, drawn over the low-poly Earth (or the static backdrop without WebGL).
#[component]
pub fn TitleScreen(on_start: impl Fn() + 'static, on_sources: impl Fn() + 'static) -> impl IntoView {
    view! {
        <section class="screen title-screen">
            <div class="title-panel lp-panel">
                <h1 class="title lp-display">"SpaceOooops"</h1>
                <p class="tagline">{sim::tagline()}</p>
                <div class="title-intro">
                    {sim::game_intro().into_iter().map(|p| view! { <p>{p}</p> }).collect_view()}
                </div>
                <div class="title-actions">
                    <button class="lp-btn lp-btn-primary" on:click=move |_| on_start()>"Start"</button>
                    <button class="lp-btn" on:click=move |_| on_sources()>"Sources"</button>
                    <super::history::TitleExtras />
                </div>
            </div>
        </section>
    }
}
