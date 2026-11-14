//! Hangar previews on the Design screen: the chosen Orbit around a small Earth, and the chosen Launch
//! Vehicle's rocket standing next to the spacecraft. Neither is to scale (see each field).
use serde::Serialize;

use crate::sim::{Catalog, Design};

use super::{SceneInputs, Screen};

/// What the hangar shows besides the spacecraft.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HangarPreview {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orbit: Option<OrbitPreview>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_vehicle: Option<RocketPreview>,
}

/// The chosen Orbit drawn as a ring around a small Earth of radius 1.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrbitPreview {
    pub id: String,
    /// Ring radius in Earth radii, on the same log-ish altitude scale as the 2D orbit view, so LEO and GEO
    /// both fit in the shot.
    pub radius: f64,
    pub inclination_deg: f64,
    /// "leo", "sso", "geo".
    pub class: String,
}

/// The chosen Launch Vehicle's rocket.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RocketPreview {
    pub id: String,
    /// "small", "medium", "heavy".
    pub class: String,
    /// Real height in metres.
    pub height_m: f64,
    /// Display height: log-scaled from the real height so an 18 m and a 70 m rocket both fit in one shot next
    /// to a 30 cm CubeSat, yet the bigger rocket stands visibly taller. Not to scale: about 1 for the smallest
    /// rocket and 2 for the tallest; the renderer scales it to its frame.
    pub height: f64,
}

pub(super) fn for_inputs(inputs: &SceneInputs) -> Option<HangarPreview> {
    if inputs.screen != Screen::Design {
        return None;
    }
    Some(preview(inputs.catalog?, inputs.design?))
}

/// Altitude share on the 2D orbit view's log-ish scale: 0 at the surface, 1 at GEO.
fn altitude_share(altitude_km: f64) -> f64 {
    ((1.0 + altitude_km.max(0.0) / 500.0).ln() / (1.0 + 36_000.0_f64 / 500.0).ln()).clamp(0.0, 1.0)
}

fn preview(c: &Catalog, d: &Design) -> HangarPreview {
    let orbit = d.orbit.as_deref().and_then(|id| c.orbit(id)).map(|o| OrbitPreview {
        id: o.id.clone(),
        radius: 1.15 + 1.6 * altitude_share(o.altitude_km),
        inclination_deg: o.inclination_deg,
        class: o.orbit_class.clone(),
    });
    let launch_vehicle = d.launch_vehicle.as_deref().and_then(|id| c.launch_vehicle(id)).map(|lv| {
        let height_m = if lv.rocket.height_m > 0.0 { lv.rocket.height_m } else { 20.0 };
        RocketPreview {
            id: lv.id.clone(),
            class: serde_json::to_value(lv.rocket.class).ok().and_then(|v| v.as_str().map(String::from)).unwrap_or_default(),
            height_m,
            height: 0.7 * (height_m / 4.0).ln().max(0.5),
        }
    });
    HangarPreview { orbit, launch_vehicle }
}
