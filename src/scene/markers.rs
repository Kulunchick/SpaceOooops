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

/// The one Target Country the globe highlights: Angola on the title, Sources and Scenario select screens (and when
/// no Scenario is chosen), the chosen Scenario's country on every screen after the choice.
pub(super) fn highlighted_country(screen: Screen, scenarios: &[Scenario], chosen: Option<&str>) -> String {
    let home = || crate::sim::HOME_COUNTRY.to_string();
    if matches!(screen, Screen::Title | Screen::Sources | Screen::ScenarioSelect) {
        return home();
    }
    scenarios
        .iter()
        .find(|s| Some(s.id.as_str()) == chosen)
        .map_or_else(home, |s| s.target_country.id.clone())
}

/// The Shot for Scenario select (the whole region) or the Brief (holding on the chosen marker).
/// `None` for every other screen.
pub(super) fn shot(screen: Screen, scenarios: &[Scenario], chosen: Option<&str>) -> Option<Shot> {
    match screen {
        Screen::ScenarioSelect => Some(Shot::Globe { lat: ANGOLA_CENTRE.0, lon: ANGOLA_CENTRE.1 }),
        Screen::Brief => Some(match chosen_scenario(screen, scenarios, chosen) {
            Some(s) => Shot::Region { lat: s.marker_lat, lon: s.marker_lon },
            None => Shot::Globe { lat: ANGOLA_CENTRE.0, lon: ANGOLA_CENTRE.1 },
        }),
        _ => None,
    }
}
