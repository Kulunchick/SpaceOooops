//! Orbit animation: a schematic SVG of the spacecraft circling Earth on the chosen Orbit.
//! A pure rendering of data: the spacecraft's place comes only from `AppState::playback`, the shadow
//! from the orbit's eclipse fraction (the day's own value when the timeline has one), the flashes from
//! the day's Random Events. It is not to scale: altitude is drawn on a log-ish scale so LEO and GEO both fit.
use std::f64::consts::{PI, TAU};

use leptos::prelude::*;
use spaceooops::sim::EventKind;

use crate::ui::format::{num, percent};
use crate::ui::state::AppState;

const W: f64 = 480.0;
const H: f64 = 340.0;
const CX: f64 = W / 2.0;
const CY: f64 = H / 2.0;
const EARTH_R: f64 = 40.0;
/// Visual revolutions of a reference orbit over the whole Run. Angular speed of every Orbit is
/// scaled from this by the inverse of its period, so GEO is visibly slower than LEO.
const VISUAL_REVS: f64 = 14.0;
const REFERENCE_PERIOD_MIN: f64 = 90.0;
/// Earth's rotation period (sidereal day), minutes. Drives the ground marker on the globe.
const EARTH_ROTATION_MIN: f64 = 1436.07;

/// Everything about the Orbit that is needed to draw it.
#[derive(Clone, PartialEq)]
struct Geometry {
    name: String,
    altitude_km: f64,
    period_min: f64,
    eclipse_fraction: f64,
    /// Horizontal radius of the drawn orbit ellipse.
    rx: f64,
    /// Vertical squash of the ellipse: flat for an equatorial orbit, round for a polar one.
    squash: f64,
}

impl Geometry {
    fn ry(&self) -> f64 {
        self.rx * self.squash
    }
    /// Point on the orbit ellipse at angle `theta`.
    fn at(&self, theta: f64) -> (f64, f64) {
        (CX + self.rx * theta.cos(), CY + self.ry() * theta.sin())
    }
}

fn geometry(state: AppState) -> Geometry {
    let id = state.design.with_untracked(|d| d.orbit.clone()).unwrap_or_default();
    state.catalog.with_value(|c| match c.orbit(&id) {
        Some(o) => {
            // Log scale on altitude: 0 at the surface, 1 at roughly GEO.
            let f = ((1.0 + o.altitude_km.max(0.0) / 500.0).ln() / (1.0 + 36_000.0_f64 / 500.0).ln()).clamp(0.0, 1.0);
            Geometry {
                name: o.name.clone(),
                altitude_km: o.altitude_km,
                period_min: o.period_min.max(1.0),
                eclipse_fraction: o.eclipse_fraction.clamp(0.0, 1.0),
                rx: EARTH_R + 16.0 + 105.0 * f,
                squash: 0.28 + 0.72 * o.inclination_deg.to_radians().sin().abs(),
            }
        }
        None => Geometry {
            name: String::new(),
            altitude_km: 0.0,
            period_min: REFERENCE_PERIOD_MIN,
            eclipse_fraction: 0.0,
            rx: EARTH_R + 30.0,
            squash: 0.5,
        },
    })
}

fn event_icon(kind: EventKind) -> &'static str {
    match kind {
        EventKind::ComponentFailure => "\u{2716}",
        EventKind::SolarFlare => "\u{2600}",
        EventKind::GroundStationOutage => "\u{1F4E1}",
        EventKind::BatteryWornOut => "\u{1F50B}",
    }
}

#[component]
pub fn OrbitView() -> impl IntoView {
    let state = expect_context::<AppState>();
    let g = geometry(state);
    let geo = StoredValue::new(g.clone());

    // Share of the Run played, 0..1.
    let progress = move || {
        let days = state.run_days() as f64;
        if days > 0.0 { (state.playback.get() / days).clamp(0.0, 1.0) } else { 0.0 }
    };
    // Angle of the spacecraft, in radians; 0 is the point behind Earth, in the middle of the shadow.
    let period = g.period_min;
    let theta = move || (TAU * VISUAL_REVS * REFERENCE_PERIOD_MIN / period * progress()).rem_euclid(TAU);

    // The shown day's shadow fraction and Random Events (changes once per day, not per tick).
    let day_snapshot = Memo::new(move |_| {
        let i = state.playback_index()?;
        state.run_result.with(|r| r.as_ref().and_then(|r| r.timeline.get(i).map(|d| (i, d.in_eclipse_fraction, d.events.clone()))))
    });
    // Today's shadow fraction: the timeline's own value, else the Orbit's.
    let shadow_fraction = Memo::new(move |_| {
        day_snapshot.with(|d| d.as_ref().map_or(geo.with_value(|g| g.eclipse_fraction), |d| d.1)).clamp(0.0, 1.0)
    });
    let in_shadow = move || {
        let half = PI * shadow_fraction.get();
        let t = theta();
        half > 0.0 && (t <= half || t >= TAU - half)
    };
    let hidden_behind_earth = move || {
        let t = theta();
        let (x, y) = geo.with_value(|g| g.at(t));
        t > PI && (x - CX).powi(2) + (y - CY).powi(2) < EARTH_R * EARTH_R
    };
    let position = move || {
        let (x, y) = geo.with_value(|g| g.at(theta()));
        format!("translate({x:.1} {y:.1})")
    };

    // Shadow arc drawn on the orbit.
    let shadow_path = move || {
        let half = PI * shadow_fraction.get();
        if half <= 0.0 {
            return String::new();
        }
        let steps = ((half / PI * 40.0).ceil() as usize).max(2);
        geo.with_value(|g| {
            (0..=steps)
                .map(|i| {
                    let (x, y) = g.at(-half + 2.0 * half * i as f64 / steps as f64);
                    format!("{}{x:.1} {y:.1}", if i == 0 { "M" } else { "L" })
                })
                .collect::<String>()
        })
    };

    // Ground marker spinning with Earth: over GEO it stays under the spacecraft.
    let marker = move || {
        let phi = TAU * VISUAL_REVS * REFERENCE_PERIOD_MIN / EARTH_ROTATION_MIN * progress();
        let squash = geo.with_value(|g| g.squash);
        let r = EARTH_R * 0.7;
        (CX + r * phi.cos(), CY + r * squash * phi.sin(), phi.sin() >= 0.0)
    };

    // Random Events of the day being shown: a ring flashes once on the spacecraft per such day.
    let flash = Memo::new(move |_| {
        day_snapshot.with(|d| {
            let (day, _, events) = d.as_ref()?;
            events.first().map(|e| (*day, e.kind))
        })
    });
    let event_text = move || {
        day_snapshot.with(|d| {
            d.as_ref()
                .and_then(|d| d.2.first())
                .map(|e| format!("{} {}", event_icon(e.kind), e.text))
                .unwrap_or_default()
        })
    };

    let (name, altitude, period_min, shadow) = (g.name.clone(), g.altitude_km, g.period_min, g.eclipse_fraction);
    let orbit_rx = g.rx;
    let orbit_ry = g.ry();
    let view_box = format!("0 0 {W} {H}");
    let aria = format!("Orbit animation: the spacecraft circling Earth on the {name}");
    let sun_ray = format!("M 70 {CY} H {}", CX - orbit_rx - 12.0);
    let night_y = CY - EARTH_R;
    let night_h = EARTH_R * 2.0;
    let night_label_x = CX + EARTH_R + 6.0;
    let night_label_y = night_y - 6.0;
    let band_w = W - CX;

    view! {
        <div class="run-orbit-slot panel">
            <svg class="orbit-svg" viewBox=view_box role="img" aria-label=aria>
                <defs>
                    <radialGradient id="earth-grad" cx="35%" cy="35%" r="75%">
                        <stop offset="0%" stop-color="#4aa3d9" />
                        <stop offset="100%" stop-color="#14456b" />
                    </radialGradient>
                    <clipPath id="earth-clip"><circle cx=CX cy=CY r=EARTH_R /></clipPath>
                </defs>
                <text class="orbit-sun" x="14" y=CY>"\u{2600} Sun"</text>
                <path class="orbit-sun-rays" d=sun_ray />
                <rect class="orbit-shadow-band" x=CX y=night_y width=band_w height=night_h />
                <ellipse class="orbit-line" cx=CX cy=CY rx=orbit_rx ry=orbit_ry />
                <path class="orbit-eclipse-arc" d=shadow_path />
                <circle class="earth" cx=CX cy=CY r=EARTH_R fill="url(#earth-grad)" />
                <g clip-path="url(#earth-clip)">
                    <rect class="earth-night" x=CX y=night_y width=EARTH_R height=night_h />
                </g>
                <line class="earth-terminator" x1=CX y1=night_y x2=CX y2=night_y + night_h />
                <text class="orbit-night-label" x=night_label_x y=night_label_y>"\u{263E} Night"</text>
                <circle class="ground-marker" r="3.5"
                    cx=move || format!("{:.1}", marker().0) cy=move || format!("{:.1}", marker().1)
                    opacity=move || if marker().2 { "1" } else { "0.25" } />
                <g class="spacecraft" class:in-shadow=in_shadow transform=position
                    opacity=move || if hidden_behind_earth() { "0" } else { "1" }>
                    {move || flash.get().map(|_| view! { <circle class="event-flash" r="10" /> })}
                    <rect class="sc-panel" x="-13" y="-2.5" width="9" height="5" />
                    <rect class="sc-panel" x="4" y="-2.5" width="9" height="5" />
                    <rect class="sc-body" x="-4" y="-4" width="8" height="8" rx="1.5" />
                </g>
            </svg>
            <div class="orbit-caption" aria-live="polite">
                <span class="orbit-name">{name}</span>
                <span class="orbit-facts">
                    {format!("Altitude {} km \u{00B7} Period {} min \u{00B7} {} of each orbit in shadow", num(altitude), num(period_min), percent(shadow))}
                </span>
                <span class="orbit-state" class:orbit-state-shadow=in_shadow>
                    {move || if in_shadow() { "\u{263E} In Earth's shadow" } else { "\u{2600} In sunlight" }}
                </span>
                <span class="orbit-event">{event_text}</span>
            </div>
        </div>
    }
}
