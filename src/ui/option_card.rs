//! One selectable catalog option: name, "why" text, its numbers (each with a "why it matters" tip), and an
//! incompatibility note. The name and "why" form the select button; a click anywhere on the card also selects,
//! except on a "?" tip.
use leptos::prelude::*;

use super::info_tip::InfoTip;

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
        <div
            class="option-card"
            class:selected=move || selected.get()
            class:incompatible=move || incompatible.get().is_some()
            on:click=move |_| on_select.run(())
        >
            <button type="button" class="option-main" title=tooltip aria-pressed=move || selected.get().to_string()>
                <span class="option-head">
                    <span class="option-check">{move || if selected.get() { "\u{2714}" } else { "" }}</span>
                    <span class="option-name">{name}</span>
                </span>
                <span class="option-why">{why}</span>
            </button>
            <span class="option-stats">
                {stats
                    .into_iter()
                    .map(|(label, value)| view! {
                        <span class="stat">
                            <span class="stat-label">{label}</span>
                            <InfoTip term=label />
                            " "
                            <span class="stat-value">{value}</span>
                        </span>
                    })
                    .collect_view()}
            </span>
            {move || incompatible.get().map(|reason| view! {
                <span class="option-clash">"\u{26A0} Incompatible: " {reason}</span>
            })}
        </div>
    }
}
