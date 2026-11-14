//! Cost breakdown table from the live Launch Review.
use leptos::prelude::*;

use super::format::money;
use super::state::AppState;

#[component]
pub fn CostTable() -> impl IntoView {
    let state = expect_context::<AppState>();
    let review = state.review;
    let budget = move || {
        let id = state.scenario_id.get();
        state.catalog.with_value(|c| c.scenario(&id).map_or(0.0, |s| s.budget_usd))
    };
    let row = move |label: &'static str, f: fn(&spaceooops::sim::CostBreakdown) -> f64| {
        view! { <tr><th scope="row">{label}</th><td>{move || review.with(|r| money(f(&r.cost)))}</td></tr> }
    };
    view! {
        <table class="cost-table">
            <tbody>
                {row("Spacecraft", |c| c.spacecraft_usd)}
                {row("Instruments", |c| c.instruments_usd)}
                {row("Launch", |c| c.launch_usd)}
                {row("Operations", |c| c.operations_usd)}
                <tr class="total"><th scope="row">"Total"</th><td>{move || review.with(|r| money(r.cost.total_usd))}</td></tr>
                <tr><th scope="row">"Budget"</th><td>{move || money(budget())}</td></tr>
            </tbody>
        </table>
    }
}
