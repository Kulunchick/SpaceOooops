//! Run-scene effects: what the Random Events and the player's Decisions look like in the 3D scene.
//!
//! Every effect has a start and a length in Run days, read off the Run timeline (which already carries the
//! Decisions taken). The Scene Spec lists only the effects active at the playback position, and the renderer
//! draws each one purely from where the playback is inside it, so pausing freezes an effect, speeding up runs
//! it faster, and skipping past it leaves no trace.
//!
//! Day convention (as the dashboards): day `k` (1-based) is the playback interval `(k - 1, k]`, so an effect
//! starting on day `k` has `start = k - 1` and is active for `start < playback <= start + days`.
use serde::Serialize;

use crate::sim::{EventKind, EventTuning, RunResult};

use super::{SceneInputs, Screen};

/// One effect in the Run scene.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Effect {
    /// Stable for the whole Run (also across re-runs after a Decision), so a new id is a new effect.
    pub id: String,
    #[serde(flatten)]
    pub kind: EffectKind,
    /// Playback position (days elapsed) where the effect begins.
    pub start: f64,
    /// Length in Run days.
    pub days: f64,
}

/// What an effect shows. JSON: `{"kind": "flare", ...}`, extra fields per kind.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum EffectKind {
    /// A solar flare: a wave travels from the Sun and hits the spacecraft.
    Flare,
    /// The flare warning was answered with safe mode: the solar panels feather and the lights dim.
    SafeMode,
    /// A ground station is offline: its beacon on the ground goes dark.
    StationDark { station: String },
    /// Data comes down: a beam links the spacecraft to the station (drawn while the station is in view).
    Contact { station: String },
    /// A Component failed: its part sparks on the failure day and stays dark for the rest of the Run.
    Failure { part: String },
    /// The battery stopped covering the eclipse: its part flashes a warning on the day it starts.
    BatteryShortfall { part: String },
}

/// The sim's option ids for the flare and outage answers that change what the scene shows.
const RIDE_OUT: &str = "ride-out";
const BUY_STATION_TIME: &str = "buy-station-time";

/// The Mission's own ground station, near Luanda, Angola.
pub const PRIMARY: &str = "primary";
/// The station whose time can be bought during an outage: a commercial site near Johannesburg, South Africa.
pub const BACKUP: &str = "backup";

/// A ground station on the globe (degrees). Plain places, no agency names or marks.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Station {
    pub id: String,
    pub lat: f64,
    pub lon: f64,
}

/// The ground stations of the Run, main station first.
pub fn stations() -> Vec<Station> {
    [(PRIMARY, -8.92, 13.18), (BACKUP, -25.89, 27.69)]
        .into_iter()
        .map(|(id, lat, lon)| Station { id: id.to_string(), lat, lon })
        .collect()
}

impl Effect {
    /// Whether the effect is in the scene at this playback position.
    pub fn active_at(&self, playback: f64) -> bool {
        self.start < playback && playback <= self.start + self.days
    }
}

/// Every effect of a Run, in timeline order. `tuning` is the Scenario's Random Event tuning (how long safe mode
/// and an outage last); without it they last one day.
pub fn all(run: &RunResult, tuning: Option<&EventTuning>) -> Vec<Effect> {
    let run_days = run.timeline.len() as f64;
    let safe_days = tuning.map_or(1, |t| t.flare_safe_mode_days);
    let mut out = Vec::new();
    // Cut at the end of the Run, so an effect never outlasts the timeline.
    let mut push = |id: String, kind: EffectKind, start: f64, days: f64| {
        let days = days.min(run_days - start);
        if days > 0.0 {
            out.push(Effect { id, kind, start, days });
        }
    };
    let outage_days = tuning.map_or(1, |t| t.outage_days);
    // Days (1-based, inclusive) on which time bought on the backup station carries the Data.
    let mut backup_until = 0;
    // The contact pass being built: (first day, station, last day).
    let mut pass: Option<(u32, &'static str, u32)> = None;
    let mut passes = Vec::new();
    let mut delivered_before = 0.0;
    for t in &run.timeline {
        let start = f64::from(t.day) - 1.0;
        for e in &t.events {
            match e.kind {
                EventKind::SolarFlare => {
                    push(format!("day-{}-flare", t.day), EffectKind::Flare, start, 1.0);
                    if e.choice.as_deref() != Some(RIDE_OUT) && safe_days > 0 {
                        push(format!("day-{}-safe-mode", t.day), EffectKind::SafeMode, start, f64::from(safe_days));
                    }
                }
                EventKind::GroundStationOutage => {
                    let station = EffectKind::StationDark { station: PRIMARY.to_string() };
                    push(format!("day-{}-outage", t.day), station, start, f64::from(outage_days.max(1)));
                    if e.choice.as_deref() == Some(BUY_STATION_TIME) {
                        backup_until = t.day + outage_days.max(1) - 1;
                    }
                }
                EventKind::ComponentFailure | EventKind::BatteryShortfall => {
                    let Some(part) = e.component.clone() else { continue };
                    let (id, kind, days) = if e.kind == EventKind::ComponentFailure {
                        (format!("day-{}-failure-{part}", t.day), EffectKind::Failure { part }, run_days - start)
                    } else {
                        (format!("day-{}-battery-{part}", t.day), EffectKind::BatteryShortfall { part }, 1.0)
                    };
                    push(id, kind, start, days);
                }
            }
        }
        // Ground contact: a beam on every day Data came down, to whichever station carried it.
        let downlinked = t.delivered_data_mb > delivered_before;
        delivered_before = t.delivered_data_mb;
        let station = if t.day <= backup_until { BACKUP } else { PRIMARY };
        pass = match (pass, downlinked) {
            (Some((first, s, last)), true) if s == station && last + 1 == t.day => Some((first, s, t.day)),
            (open, now) => {
                passes.extend(open);
                now.then_some((t.day, station, t.day))
            }
        };
    }
    passes.extend(pass);
    for (first, station, last) in passes {
        let kind = EffectKind::Contact { station: station.to_string() };
        push(format!("day-{first}-contact-{station}"), kind, f64::from(first) - 1.0, f64::from(last - first + 1));
    }
    out.sort_by(|a, b| a.start.total_cmp(&b.start));
    out
}

/// The effects active at `playback`.
pub fn at(run: &RunResult, tuning: Option<&EventTuning>, playback: f64) -> Vec<Effect> {
    all(run, tuning).into_iter().filter(|e| e.active_at(playback)).collect()
}

/// Whether the playback should cross Random Event days slowly (`advance`): only while the 3D scene shows their
/// effects, and not for a player who asked for reduced motion (no animated effects to wait for; the Run keeps its
/// ordinary pace, and the event log and any question still hold the playback as before).
pub fn slows_event_days(scene_live: bool, reduced_motion: bool) -> bool {
    scene_live && !reduced_motion
}

/// The next playback position for the playback driver: `step` days on ordinary days, but a day with a Random
/// Event is entered exactly at its start and crossed in steps of `event_step`, so its effects are seen (and its
/// Cue heard) at any speed. Never goes past the end of the Run.
pub fn advance(run: &RunResult, now: f64, step: f64, event_step: f64) -> f64 {
    let end = run.timeline.len() as f64;
    let has_events = |day: f64| run.timeline.get(day as usize - 1).is_some_and(|t| !t.events.is_empty());
    // The day the next step moves through: day k covers (k - 1, k].
    let day = now.floor() + 1.0;
    let next = if has_events(day) {
        (now + event_step.min(step)).min(day)
    } else {
        // Stop at the start of the next event day, if the step would cross into it.
        let reach = now + step;
        let mut d = day + 1.0;
        let mut next = reach;
        while d - 1.0 < reach && d <= end {
            if has_events(d) {
                next = d - 1.0;
                break;
            }
            d += 1.0;
        }
        next
    };
    next.min(end)
}

/// The Scenario's Random Event tuning, from the inputs' catalog and chosen Scenario.
fn tuning<'a>(inputs: &SceneInputs<'a>) -> Option<&'a EventTuning> {
    Some(&inputs.catalog?.scenario(inputs.chosen?)?.events)
}

pub(super) fn stations_for(inputs: &SceneInputs) -> Vec<Station> {
    if inputs.screen == Screen::Run && inputs.run.is_some() { stations() } else { Vec::new() }
}

pub(super) fn for_inputs(inputs: &SceneInputs) -> Vec<Effect> {
    match (inputs.screen, inputs.run) {
        (Screen::Run, Some(run)) => at(run, tuning(inputs), inputs.playback),
        _ => Vec::new(),
    }
}
