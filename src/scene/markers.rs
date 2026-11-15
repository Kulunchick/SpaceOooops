//! Scenario markers: the pins on the globe on Scenario select and Brief, and the Shots that frame them.
//! Positions come from each Scenario's data (`marker_lat`, `marker_lon`); nothing here is hard-coded per Scenario.
use serde::Serialize;

use super::{Screen, Shot};
use crate::sim::Scenario;

/// Where the select Shot looks: the middle of Angola, so all markers are in view.
pub const ANGOLA_CENTRE: (f64, f64) = (-12.5, 17.5);

/// One pin on the globe. `selected` is the Scenario the Brief is open on.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Marker {
    pub id: String,
    pub lat: f64,
    pub lon: f64,
    pub selected: bool,
}

fn shows_markers(screen: Screen) -> bool {
    matches!(screen, Screen::ScenarioSelect | Screen::Brief)
}

fn chosen_scenario<'a>(screen: Screen, scenarios: &'a [Scenario], chosen: Option<&str>) -> Option<&'a Scenario> {
    if screen != Screen::Brief {
        return None;
    }
    scenarios.iter().find(|s| Some(s.id.as_str()) == chosen)
}

pub(super) fn markers(screen: Screen, scenarios: &[Scenario], chosen: Option<&str>) -> Vec<Marker> {
    if !shows_markers(screen) {
        return Vec::new();
    }
    let selected = chosen_scenario(screen, scenarios, chosen).map(|s| s.id.as_str());
    scenarios
        .iter()
        .map(|s| Marker {
            id: s.id.clone(),
            lat: s.marker_lat,
            lon: s.marker_lon,
            selected: Some(s.id.as_str()) == selected,
        })
        .collect()
}

/// The one Target Country the globe highlights: on Scenario select the hovered or focused card's country (Angola
/// while none is), Angola on the title and Sources screens (and when no Scenario is chosen), and the chosen
/// Scenario's country on every screen after the choice.
pub(super) fn highlighted_country(screen: Screen, scenarios: &[Scenario], chosen: Option<&str>, previewed: Option<&str>) -> String {
    let home = || crate::sim::HOME_COUNTRY.to_string();
    if screen == Screen::ScenarioSelect {
        return scenarios.iter().find(|s| Some(s.id.as_str()) == previewed).map_or_else(home, |s| s.target_country.id.clone());
    }
    if matches!(screen, Screen::Title | Screen::Sources) {
        return home();
    }
    scenarios
        .iter()
        .find(|s| Some(s.id.as_str()) == chosen)
        .map_or_else(home, |s| s.target_country.id.clone())
}

/// A place on the globe (degrees).
#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
pub struct LatLon {
    pub lat: f64,
    pub lon: f64,
}

/// The places the highlight of `country` must show: the pin and the ground stations of every Scenario set there,
/// each once. The bridge keeps the facets under those inside the country's outline lit even where the coarse globe
/// puts a facet's centre outside it (a pin on a narrow coast or island).
pub(super) fn highlight_points(scenarios: &[Scenario], country: &str) -> Vec<LatLon> {
    let mut out: Vec<LatLon> = Vec::new();
    for s in scenarios.iter().filter(|s| s.target_country.id == country) {
        let places = std::iter::once((s.marker_lat, s.marker_lon)).chain(s.ground_stations.iter().map(|g| (g.lat, g.lon)));
        for (lat, lon) in places {
            let p = LatLon { lat, lon };
            if !out.contains(&p) {
                out.push(p);
            }
        }
    }
    out
}

/// The Shot for Scenario select (the whole region) or the Brief (holding on the chosen marker).
/// `None` for every other screen.
pub(super) fn shot(screen: Screen, scenarios: &[Scenario], chosen: Option<&str>, previewed: Option<&str>) -> Option<Shot> {
    match screen {
        Screen::ScenarioSelect => Some(match scenarios.iter().find(|s| Some(s.id.as_str()) == previewed) {
            Some(s) => Shot::Globe { lat: s.marker_lat, lon: s.marker_lon },
            None => Shot::Globe { lat: ANGOLA_CENTRE.0, lon: ANGOLA_CENTRE.1 },
        }),
        Screen::Brief => Some(match chosen_scenario(screen, scenarios, chosen) {
            Some(s) => Shot::Region { lat: s.marker_lat, lon: s.marker_lon },
            None => Shot::Globe { lat: ANGOLA_CENTRE.0, lon: ANGOLA_CENTRE.1 },
        }),
        _ => None,
    }
}
