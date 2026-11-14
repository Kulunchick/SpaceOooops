//! Live dashboards for the day the playback is showing: battery charge, stored Data, Science Value, days.
use leptos::prelude::*;
use spaceooops::sim::DaySnapshot;

use crate::ui::format::{num, percent};
use crate::ui::state::AppState;

#[component]
pub fn Dashboards() -> impl IntoView {
    let state = expect_context::<AppState>();
    let snapshot = Memo::new(move |_| {
        let index = state.playback_index()?;
        state.run_result.with(|r| r.as_ref().and_then(|r| r.timeline.get(index).cloned()))
    });
    let read = move |f: fn(&DaySnapshot) -> f64| snapshot.with(|s| s.as_ref().map_or(0.0, f));
    let required = move || state.run_result.with(|r| r.as_ref().map_or(0.0, |r| r.debrief.stats.required_science_value));
    let min_days = move || state.run_result.with(|r| r.as_ref().map_or(0, |r| r.debrief.stats.min_lifetime_days));
    let storage_mb = state.catalog.with_value(|c| c.computer.storage_mb);
    let days_elapsed = move || state.playback.get().floor();

    view! {
        <div class="dashboards" aria-live="off">
            <Dial
                label="Battery charge"
                value=Signal::derive(move || percent(read(|s| s.battery_charge)))
                fill=Signal::derive(move || read(|s| s.battery_charge))
                warn=Signal::derive(move || read(|s| s.battery_charge) < 0.5)
            />
            <Dial
                label="Stored Data"
                value=Signal::derive(move || format!("{} MB", num(read(|s| s.stored_data_mb).round())))
                fill=Signal::derive(move || ratio(read(|s| s.stored_data_mb), storage_mb))
                warn=Signal::derive(move || read(|s| s.lost_data_mb) > 0.0)
            />
            <Dial
                label="Science Value"
                value=Signal::derive(move || format!("{:.1} / {}", read(|s| s.science_value), num(required())))
                fill=Signal::derive(move || ratio(read(|s| s.science_value), required()))
                warn=Signal::derive(|| false)
            />
            <Dial
                label="Days elapsed"
                value=Signal::derive(move || format!("{:.0} / {}", days_elapsed(), min_days()))
                fill=Signal::derive(move || ratio(days_elapsed(), f64::from(min_days())))
                warn=Signal::derive(|| false)
            />
        </div>
    }
}

#[component]
fn Dial(label: &'static str, value: Signal<String>, fill: Signal<f64>, warn: Signal<bool>) -> impl IntoView {
    view! {
        <div class="dial" class:dial-warn=move || warn.get()>
            <div class="dial-label">{label}</div>
            <div class="dial-value">{move || value.get()}</div>
            <div class="dial-bar">
                <div class="dial-fill" style=move || format!("width: {:.1}%", fill.get().clamp(0.0, 1.0) * 100.0)></div>
            </div>
        </div>
    }
}

fn ratio(value: f64, limit: f64) -> f64 {
    if limit > 0.0 { value / limit } else { 0.0 }
}
