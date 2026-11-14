//! The Run's orbit view: where the spacecraft is on its Orbit, how far Earth has turned and whether the
//! spacecraft is in Earth's shadow, all from the playback position alone. The SVG orbit view (the no-WebGL
//! fallback) draws from the same functions, so both views and the dashboards always agree.
//!
//! Not to scale: altitude is drawn on a log-ish scale so LEO and GEO both fit in one shot, and the spacecraft
//! makes a fixed number of visual revolutions per Run (scaled by its period, so GEO is visibly slower).
use std::f64::consts::{PI, TAU};

use serde::Serialize;

use crate::sim::{Catalog, Design, Orbit, RunResult};

use super::{SceneInputs, Screen};

/// Visual revolutions of a reference orbit over the whole Run. Angular speed of every Orbit is scaled from
/// this by the inverse of its period, so GEO is visibly slower than LEO.
pub const VISUAL_REVS: f64 = 14.0;
pub const REFERENCE_PERIOD_MIN: f64 = 90.0;
/// Earth's rotation period (sidereal day), minutes. Turns the globe so a GEO spacecraft stays over one place.
pub const EARTH_ROTATION_MIN: f64 = 1436.07;

/// What the Run scene draws for the Orbit at the current playback position.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrbitView {
    /// Catalog id of the Orbit.
    pub id: String,
    /// "leo", "sso", "geo".
    pub class: String,
    /// Display radius of the orbit in Earth radii, on the same log-ish scale as the Design's Orbit preview.
    pub radius: f64,
    /// Tilt of the orbit plane against the equator, degrees.
    pub inclination_deg: f64,
    /// The spacecraft's angle on its orbit, radians in `0..TAU`. 0 is the middle of Earth's shadow (the point
    /// straight behind Earth, away from the Sun); the angle grows in the direction of flight.
    pub angle: f64,
    /// How far Earth has turned on its axis since launch, radians in `0..TAU`, same sense as `angle`.
    pub earth_turn: f64,
    /// Share of each orbit spent in Earth's shadow on the shown day: the timeline's value, else the Orbit's.
    pub shadow_fraction: f64,
    /// The spacecraft is in Earth's shadow right now (within `shadow_fraction * PI` of angle 0).
    pub in_shadow: bool,
    /// The day the playback shows, 1-based (0 before launch); the dashboards show the same day.
    pub day: usize,
    /// The playback position in days, as given.
    pub playback: f64,
}

/// Altitude share on the log-ish display scale: 0 at the surface, 1 at GEO.
pub fn altitude_share(altitude_km: f64) -> f64 {
    ((1.0 + altitude_km.max(0.0) / 500.0).ln() / (1.0 + 36_000.0_f64 / 500.0).ln()).clamp(0.0, 1.0)
}

/// Display radius of an orbit in Earth radii (Design preview and Run scene).
pub fn display_radius(altitude_km: f64) -> f64 {
    1.15 + 1.6 * altitude_share(altitude_km)
}

/// Share of the Run played, 0..1, for a playback position in days.
pub fn progress(playback: f64, days: usize) -> f64 {
    if days > 0 { (playback / days as f64).clamp(0.0, 1.0) } else { 0.0 }
}

/// The spacecraft's angle on an orbit of the given period, radians in `0..TAU`; 0 is the middle of the shadow.
pub fn angle(progress: f64, period_min: f64) -> f64 {
    (TAU * VISUAL_REVS * REFERENCE_PERIOD_MIN / period_min.max(1.0) * progress).rem_euclid(TAU)
}

/// How far Earth has turned since launch, radians in `0..TAU`.
pub fn earth_turn(progress: f64) -> f64 {
    (TAU * VISUAL_REVS * REFERENCE_PERIOD_MIN / EARTH_ROTATION_MIN * progress).rem_euclid(TAU)
}

/// Whether a spacecraft at `angle` is in Earth's shadow when `shadow_fraction` of each orbit is in shadow.
pub fn in_shadow(angle: f64, shadow_fraction: f64) -> bool {
    let half = PI * shadow_fraction.clamp(0.0, 1.0);
    let t = angle.rem_euclid(TAU);
    half > 0.0 && (t <= half || t >= TAU - half)
}

/// The day the playback shows, 1-based: 0 at launch, then day k while the playback is within it.
pub fn shown_day(playback: f64, days: usize) -> usize {
    if playback <= 0.0 { 0 } else { (playback.ceil() as usize).min(days) }
}

/// Today's shadow fraction: the timeline's own value for the shown day, else the Orbit's.
pub fn shadow_fraction(orbit: &Orbit, run: Option<&RunResult>, playback: f64) -> f64 {
    let days = run.map_or(0, |r| r.timeline.len());
    let index = shown_day(playback, days).max(1) - 1;
    run.and_then(|r| r.timeline.get(index)).map_or(orbit.eclipse_fraction, |d| d.in_eclipse_fraction).clamp(0.0, 1.0)
}

pub(super) fn for_inputs(inputs: &SceneInputs) -> Option<OrbitView> {
    if inputs.screen != Screen::Run {
        return None;
    }
    view(inputs.catalog?, inputs.design?, inputs.run, inputs.playback)
}

fn view(c: &Catalog, d: &Design, run: Option<&RunResult>, playback: f64) -> Option<OrbitView> {
    let o = c.orbit(d.orbit.as_deref()?)?;
    let days = run.map_or(0, |r| r.timeline.len());
    let p = progress(playback, days);
    let angle = angle(p, o.period_min);
    let shadow_fraction = shadow_fraction(o, run, playback);
    Some(OrbitView {
        id: o.id.clone(),
        class: o.orbit_class.clone(),
        radius: display_radius(o.altitude_km),
        inclination_deg: o.inclination_deg,
        angle,
        earth_turn: earth_turn(p),
        shadow_fraction,
        in_shadow: in_shadow(angle, shadow_fraction),
        day: shown_day(playback, days),
        playback,
    })
}
