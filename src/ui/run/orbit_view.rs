//! Orbit animation: a schematic SVG of the spacecraft circling Earth on the chosen Orbit.
//! A pure rendering of data: the spacecraft's place comes only from `AppState::playback`, the shadow
//! from the orbit's eclipse fraction (the day's own value when the timeline has one), the flashes from
//! the day's Random Events. It is not to scale: altitude is drawn on a log-ish scale so LEO and GEO both fit.
//! The angle, Earth's turn, the shadow and the altitude scale come from `spaceooops::scene::orbit`, the same
//! functions the 3D Run scene is built from, so both views always agree. With WebGL the 3D scene replaces the
//! drawing (CSS hides the SVG) and only the caption stays.
use std::f64::consts::PI;

use leptos::prelude::*;
use spaceooops::scene::orbit;

use super::event_log::icon;
use crate::ui::format::{num, percent};
use crate::ui::state::AppState;

const W: f64 = 960.0;
const H: f64 = 420.0;
const CX: f64 = W / 2.0;
const CY: f64 = H / 2.0;
const EARTH_R: f64 = 80.0;
/// Drawn radius growth from the lowest to the highest Orbit (before the ellipse squash).
const ORBIT_SPAN: f64 = 340.0;
/// Smallest drawn half-angle of the shadow arc so a tiny shadow (GEO) is still visible.
const MIN_ARC_HALF: f64 = 0.1;
/// How long a Random Event's flash and caption stay on screen, whatever the playback speed.
const FLASH_MS: u64 = 2000;

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
            let f = orbit::altitude_share(o.altitude_km);
            Geometry {
                name: o.name.clone(),
                altitude_km: o.altitude_km,
                period_min: o.period_min.max(1.0),
                eclipse_fraction: o.eclipse_fraction.clamp(0.0, 1.0),
                rx: EARTH_R + 14.0 + ORBIT_SPAN * f,
                squash: 0.28 + 0.72 * o.inclination_deg.to_radians().sin().abs(),
            }
        }
        None => Geometry {
            name: String::new(),
            altitude_km: 0.0,
            period_min: orbit::REFERENCE_PERIOD_MIN,
            eclipse_fraction: 0.0,
            rx: EARTH_R + 40.0,
            squash: 0.5,
        },
    })
}

#[component]
pub fn OrbitView() -> impl IntoView {
    let state = expect_context::<AppState>();
    let g = geometry(state);
    let geo = StoredValue::new(g.clone());

    // Share of the Run played, 0..1.
    let progress = move || orbit::progress(state.playback.get(), state.run_days());
    // Angle of the spacecraft, in radians; 0 is the point behind Earth, in the middle of the shadow.
    let period = g.period_min;
    let theta = move || orbit::angle(progress(), period);

    // Today's shadow fraction: the timeline's own value, else the Orbit's.
    let shadow_fraction = Memo::new(move |_| {
        let orbit_fraction = geo.with_value(|g| g.eclipse_fraction);
        state.run_result.with(|r| orbit::shadow_fraction_or(orbit_fraction, r.as_ref(), state.playback.get()))
    });
    let in_shadow = move || orbit::in_shadow(theta(), shadow_fraction.get());
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
        let half = half.max(MIN_ARC_HALF);
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
        let phi = orbit::earth_turn(progress());
        let squash = geo.with_value(|g| g.squash);
        let r = EARTH_R * 0.7;
        (CX + r * phi.cos(), CY + r * squash * phi.sin(), phi.sin() >= 0.0)
    };

    // Random Events: the newest event among the days passed since the last tick is latched for a fixed
    // wall-clock time, so the flash and its caption are seen at any speed and no event day is skipped over.
    let latched: RwSignal<Option<(u32, String)>> = RwSignal::new(None);
    let serial = StoredValue::new(0u32);
    let last_index: StoredValue<Option<usize>> = StoredValue::new(None);
    Effect::new(move |_| {
        let Some(cur) = state.playback_index() else { return };
        let from = match last_index.get_value() {
            Some(prev) if prev < cur => prev + 1,
            Some(prev) if prev == cur => return,
            Some(_) => cur,
            None => 0,
        };
        last_index.set_value(Some(cur));
        let newest = state.run_result.with_untracked(|r| {
            r.as_ref().and_then(|r| {
                r.timeline.get(from..=cur)?.iter().rev().find_map(|d| {
                    d.events.last().map(|e| format!("Day {}: {} {}", d.day, icon(e.kind, e.catalog.as_ref().map(|c| c.group)), e.text))
                })
            })
        });
        if let Some(text) = newest {
            let n = serial.get_value() + 1;
            serial.set_value(n);
            latched.set(Some((n, text)));
            set_timeout(
                move || {
                    if latched.try_with_untracked(|l| l.as_ref().map(|l| l.0)) == Some(Some(n)) {
                        latched.try_set(None);
                    }
                },
                std::time::Duration::from_millis(FLASH_MS),
            );
        }
    });
    let flash = move || latched.with(|l| l.as_ref().map(|l| l.0));
    let event_text = move || latched.with(|l| l.as_ref().map(|l| l.1.clone()).unwrap_or_default());

    let (name, altitude, period_min, shadow) = (g.name.clone(), g.altitude_km, g.period_min, g.eclipse_fraction);
    let orbit_rx = g.rx;
    let orbit_ry = g.ry();
    let view_box = format!("0 0 {W} {H}");
    let aria = format!("Orbit animation: the spacecraft circling Earth on the {name}");
    let sun_ray = format!("M 12 {CY} H 40 M 12 {} H 34 M 12 {} H 34", CY - 14.0, CY + 14.0);
    let sun_y = CY - 24.0;
    let cone = format!("M {CX} {} L {W} {} L {W} {} L {CX} {} Z", CY - EARTH_R, CY - EARTH_R * 0.55, CY + EARTH_R * 0.55, CY + EARTH_R);
    let night_y = CY - EARTH_R;
    let night_h = EARTH_R * 2.0;
    let night_label_x = CX + 8.0;
    let night_label_y = CY + 5.0;

    view! {
        <div class="run-orbit-slot panel">
            <svg class="orbit-svg" viewBox=view_box role="img" aria-label=aria>
                <defs>
                    <radialGradient id="earth-grad" cx="35%" cy="35%" r="75%">
                        <stop offset="0%" stop-color="#4aa3d9" />
                        <stop offset="100%" stop-color="#14456b" />
                    </radialGradient>
                    <linearGradient id="shadow-grad" x1="0" y1="0" x2="1" y2="0">
                        <stop offset="0%" stop-color="#000" stop-opacity="0.75" />
                        <stop offset="100%" stop-color="#000" stop-opacity="0.05" />
                    </linearGradient>
                    <clipPath id="earth-clip"><circle cx=CX cy=CY r=EARTH_R /></clipPath>
                </defs>
                <text class="orbit-sun" x="12" y=sun_y>"\u{2600} Sun"</text>
                <path class="orbit-sun-rays" d=sun_ray />
                <path class="orbit-shadow-band" d=cone fill="url(#shadow-grad)" />
                <ellipse class="orbit-line" cx=CX cy=CY rx=orbit_rx ry=orbit_ry />
                <path class="orbit-eclipse-arc" d=shadow_path />
                <circle class="earth" cx=CX cy=CY r=EARTH_R fill="url(#earth-grad)" />
                <g clip-path="url(#earth-clip)">
                    <rect class="earth-night" x=CX y=night_y width=EARTH_R height=night_h />
                </g>
                <line class="earth-terminator" x1=CX y1=night_y x2=CX y2=night_y + night_h />
                <text class="orbit-night-label" x=night_label_x y=night_label_y>"\u{263E} Night"</text>
                <circle class="ground-marker" r="5"
                    cx=move || format!("{:.1}", marker().0) cy=move || format!("{:.1}", marker().1)
                    opacity=move || if marker().2 { "1" } else { "0.25" } />
                <g class="spacecraft" class:in-shadow=in_shadow transform=position
                    opacity=move || if hidden_behind_earth() { "0" } else { "1" }>
                    <circle class="sc-halo" r="22" />
                    <g transform="scale(1.8)">
                    {move || flash().map(|n| view! { <circle class="event-flash" r="10" data-event=n.to_string() /> })}
                    <rect class="sc-panel" x="-13" y="-2.5" width="9" height="5" />
                    <rect class="sc-panel" x="4" y="-2.5" width="9" height="5" />
                    <rect class="sc-body" x="-4" y="-4" width="8" height="8" rx="1.5" />
                    </g>
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
                <span class="orbit-event" class:orbit-event-live=move || flash().is_some()>{event_text}</span>
            </div>
        </div>
    }
}
