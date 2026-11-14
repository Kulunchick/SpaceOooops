//! One selectable catalog option: name, "why" tooltip, its numbers, and an incompatibility note.
use leptos::prelude::*;

#[component]
pub fn OptionCard(
    name: String,
    why: String,
    /// (label, value) pairs, e.g. ("Mass", "1.5 kg").
    stats: Vec<(&'static str, String)>,
    #[prop(into)] selected: Signal<bool>,
    #[prop(into)] incompatible: Signal<Option<String>>,
    on_select: Callback<()>,
) -> impl IntoView {
    let tooltip = why.clone();
    view! {
        <button
            class="option-card"
            class:selected=move || selected.get()
            class:incompatible=move || incompatible.get().is_some()
            title=tooltip
            aria-pressed=move || selected.get().to_string()
            on:click=move |_| on_select.run(())
        >
            <span class="option-head">
                <span class="option-check">{move || if selected.get() { "\u{2714}" } else { "" }}</span>
                <span class="option-name">{name}</span>
            </span>
            <span class="option-why">{why}</span>
            <span class="option-stats">
                {stats
                    .into_iter()
                    .map(|(label, value)| view! {
                        <span class="stat"><span class="stat-label">{label}</span>" "<span class="stat-value">{value}</span></span>
                    })
                    .collect_view()}
            </span>
            {move || incompatible.get().map(|reason| view! {
                <span class="option-clash">"\u{26A0} Incompatible: " {reason}</span>
            })}
        </button>
    }
}
