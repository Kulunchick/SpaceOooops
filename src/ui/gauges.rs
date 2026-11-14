//! The five always-visible resource gauges, driven by the live Launch Review.
use leptos::prelude::*;
use spaceooops::sim::{Gauge, GaugeState};

use super::state::AppState;

#[component]
pub fn Gauges() -> impl IntoView {
    let state = expect_context::<AppState>();
    let review = state.review;
    view! {
        <div class="gauges" aria-live="polite">
            <GaugeView label="Budget" floor=false gauge=Signal::derive(move || review.with(|r| r.gauges.budget.clone())) />
            <GaugeView label="Mass" floor=false gauge=Signal::derive(move || review.with(|r| r.gauges.mass.clone())) />
            <GaugeView label="Power" floor=false gauge=Signal::derive(move || review.with(|r| r.gauges.power.clone())) />
            <GaugeView label="Data" floor=false gauge=Signal::derive(move || review.with(|r| r.gauges.data.clone())) />
            <GaugeView label="Lifetime" floor=true gauge=Signal::derive(move || review.with(|r| r.gauges.lifetime.clone())) />
        </div>
    }
}

#[component]
fn GaugeView(label: &'static str, floor: bool, gauge: Signal<Gauge>) -> impl IntoView {
    let state_class = move || match gauge.with(|g| g.state) {
        GaugeState::Ok => "gauge gauge-ok",
        GaugeState::Thin => "gauge gauge-thin",
        GaugeState::Violated => "gauge gauge-violated",
    };
    let badge = move || match gauge.with(|g| g.state) {
        GaugeState::Ok => "\u{2714} OK",
        GaugeState::Thin => "\u{26A0} Thin margin",
        GaugeState::Violated => "\u{2716} Violated",
    };
    // Bar fill: share of the limit used (ceiling gauges) or reached (the Lifetime floor).
    let fill = move || {
        gauge.with(|g| {
            let share = if g.limit > 0.0 { g.value / g.limit } else if g.value > 0.0 { 1.0 } else { 0.0 };
            let share = if floor { share / 2.0 } else { share };
            format!("width: {:.0}%", (share.clamp(0.0, 1.0)) * 100.0)
        })
    };
    view! {
        <div class=state_class>
            <div class="gauge-top">
                <span class="gauge-label">{label}</span>
                <span class="gauge-badge">{badge}</span>
            </div>
            <div class="gauge-bar" class:gauge-floor=floor><div class="gauge-fill" style=fill></div></div>
            <div class="gauge-detail">{move || gauge.with(|g| g.detail.clone())}</div>
        </div>
    }
}
