//! Reliability Tier chooser for one Component (Basic / Flight-proven), labelled from the catalog.
use leptos::prelude::*;
use spaceooops::sim::ReliabilityTier;

use super::state::AppState;

#[component]
pub fn TierSelect(
    /// Current tier; `None` hides the chooser (no Component picked yet).
    #[prop(into)]
    current: Signal<Option<ReliabilityTier>>,
    on_change: Callback<ReliabilityTier>,
) -> impl IntoView {
    let state = expect_context::<AppState>();
    let tiers = state.catalog.with_value(|c| c.reliability_tiers.clone());
    view! {
        <Show when=move || current.get().is_some()>
            <div class="tier-select" role="radiogroup" aria-label="Reliability Tier">
                <span class="tier-label">"Reliability Tier:"</span>
                {tiers
                    .iter()
                    .map(|spec| {
                        let tier = spec.tier;
                        let label = format!("{} (cost \u{00D7}{})", spec.name, spec.cost_multiplier);
                        view! {
                            <button
                                class="tier-btn"
                                class:selected=move || current.get() == Some(tier)
                                title=spec.why.clone()
                                on:click=move |_| on_change.run(tier)
                            >
                                {label}
                            </button>
                        }
                    })
                    .collect_view()}
            </div>
        </Show>
    }
}
