//! Live dashboards for the day the playback is showing: battery charge, stored Data, Science Value, days.
use leptos::prelude::*;
use spaceooops::sim::{DaySnapshot, GaugeState};

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
    // The day counter: the day shown, and once the playback ends, the days the Mission actually survived.
    let days_elapsed = move || {
        if state.playback_finished() {
            state.run_result.with(|r| r.as_ref().map_or(0, |r| r.debrief.stats.days_survived)) as f64
        } else {
            state.shown_day() as f64
        }
    };
    let battery_state = move || snapshot.with(|s| s.as_ref().map_or(GaugeState::Ok, |s| s.battery_state));

    view! {
        <div class="dashboards" aria-live="off">
            <Dial
                label="Battery charge"
                value=Signal::derive(move || percent(read(|s| s.battery_charge)))
                fill=Signal::derive(move || read(|s| s.battery_charge))
                warn=Signal::derive(move || battery_state() != GaugeState::Ok)
                note=Signal::derive(move || battery_note(battery_state()).to_string())
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
                label="Days of operations"
                value=Signal::derive(move || format!("{:.0} / {}", days_elapsed(), min_days()))
                fill=Signal::derive(move || ratio(days_elapsed(), f64::from(min_days())))
                warn=Signal::derive(|| false)
            />
        </div>
    }
}

#[component]
fn Dial(
    label: &'static str,
    value: Signal<String>,
    fill: Signal<f64>,
    warn: Signal<bool>,
    /// Text status shown under the bar, so a warning never relies on colour alone.
    #[prop(optional)]
    note: Option<Signal<String>>,
) -> impl IntoView {
    view! {
        <div class="dial" class:dial-warn=move || warn.get()>
            <div class="dial-label">{label}</div>
            <div class="dial-value">{move || value.get()}</div>
            <div class="dial-bar">
                <div class="dial-fill" style=move || format!("width: {:.1}%", fill.get().clamp(0.0, 1.0) * 100.0)></div>
            </div>
            {note.map(|n| view! { <div class="dial-label dial-note">{move || n.get()}</div> })}
        </div>
    }
}

/// Plain-language status for the battery state `sim` reports.
fn battery_note(state: GaugeState) -> &'static str {
    match state {
        GaugeState::Ok => "\u{2714} Healthy",
        GaugeState::Thin => "\u{26A0} Drained deep each eclipse: wearing fast",
        GaugeState::Violated => "\u{2716} Cannot cover the eclipse: Instruments off part of each orbit",
    }
}

fn ratio(value: f64, limit: f64) -> f64 {
    if limit > 0.0 { value / limit } else { 0.0 }
}
