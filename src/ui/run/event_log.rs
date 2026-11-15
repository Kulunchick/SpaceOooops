//! The Run's event log: Random Events appear as the playback reaches their day, newest first.
use leptos::prelude::*;
use spaceooops::sim::{EventGroup, EventKind};

use crate::ui::state::AppState;

#[component]
pub fn EventLog() -> impl IntoView {
    let state = expect_context::<AppState>();
    let shown = Memo::new(move |_| {
        let shown = state.shown_day();
        if shown == 0 {
            return Vec::new();
        }
        state.run_result.with(|r| {
            let Some(r) = r else { return Vec::new() };
            r.timeline[..shown.min(r.timeline.len())]
                .iter()
                .flat_map(|d| {
                    d.events.iter().map(move |e| (d.day, e.kind, e.text.clone(), e.catalog.as_ref().map(|c| c.group)))
                })
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
                        key=|(day, kind, text, _)| (*day, *kind as u8, text.clone())
                        children=|(day, kind, text, group)| view! {
                            <li class=event_class(kind, group)>
                                <span class="event-day">{format!("Day {day}")}</span>
                                <span class="event-icon">{icon(kind, group)}</span>
                                {mark(group).map(|m| view! { <span class="event-mark">{m}</span> })}
                                {text}
                            </li>
                        }
                    />
                </ul>
            </Show>
        </div>
    }
}

fn icon(kind: EventKind, group: Option<EventGroup>) -> &'static str {
    match (kind, group) {
        (EventKind::Catalog, Some(EventGroup::Fantasy)) => "\u{2753}",
        (EventKind::Catalog, Some(EventGroup::Lucky)) => "\u{2605}",
        (EventKind::ComponentFailure, _) => "\u{2716}",
        (EventKind::SolarFlare, _) => "\u{2600}",
        (EventKind::GroundStationOutage, _) => "\u{1F4E1}",
        (EventKind::BatteryShortfall, _) => "\u{1F50B}",
        (EventKind::Catalog, _) => "\u{2604}",
    }
}

/// The text mark of a fantasy or lucky event, so the difference never rests on colour alone.
fn mark(group: Option<EventGroup>) -> Option<&'static str> {
    match group {
        Some(EventGroup::Fantasy) => Some("Unexplained"),
        Some(EventGroup::Lucky) => Some("Lucky"),
        _ => None,
    }
}

fn event_class(kind: EventKind, group: Option<EventGroup>) -> &'static str {
    match (kind, group) {
        (EventKind::Catalog, Some(EventGroup::Fantasy)) => "event event-fantasy",
        (EventKind::Catalog, Some(EventGroup::Lucky)) => "event event-lucky",
        (EventKind::ComponentFailure, _) => "event event-bad",
        _ => "event event-warn",
    }
}
