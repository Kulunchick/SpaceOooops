//! The Run's event log: Random Events appear as the playback reaches their day, newest first.
use leptos::prelude::*;
use spaceooops::sim::EventKind;

use crate::ui::state::AppState;

#[component]
pub fn EventLog() -> impl IntoView {
    let state = expect_context::<AppState>();
    let shown = Memo::new(move |_| {
        let Some(index) = state.playback_index() else { return Vec::new() };
        if state.playback.get() <= 0.0 {
            return Vec::new();
        }
        state.run_result.with(|r| {
            let Some(r) = r else { return Vec::new() };
            r.timeline[..=index]
                .iter()
                .flat_map(|d| d.events.iter().map(move |e| (d.day, e.kind, e.text.clone())))
                .rev()
                .collect::<Vec<_>>()
        })
    });
    view! {
        <div class="panel event-log">
            <h3>"Event log"</h3>
            <Show
                when=move || shown.with(|s| !s.is_empty())
                fallback=|| view! { <p class="muted">"No Random Events yet."</p> }
            >
                <ul class="events" aria-live="polite">
                    <For
                        each=move || shown.get()
                        key=|(day, kind, text)| (*day, *kind as u8, text.clone())
                        children=|(day, kind, text)| view! {
                            <li class=event_class(kind)>
                                <span class="event-day">{format!("Day {day}")}</span>
                                <span class="event-icon">{icon(kind)}</span>
                                {text}
                            </li>
                        }
                    />
                </ul>
            </Show>
        </div>
    }
}

fn icon(kind: EventKind) -> &'static str {
    match kind {
        EventKind::ComponentFailure => "\u{2716}",
        EventKind::SolarFlare => "\u{2600}",
        EventKind::GroundStationOutage => "\u{1F4E1}",
        EventKind::BatteryShortfall => "\u{1F50B}",
    }
}

fn event_class(kind: EventKind) -> &'static str {
    match kind {
        EventKind::ComponentFailure => "event event-bad",
        EventKind::SolarFlare | EventKind::GroundStationOutage | EventKind::BatteryShortfall => "event event-warn",
    }
}
