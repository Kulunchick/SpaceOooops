//! Random Events: what can go wrong on a day of the Run, drawn in a fixed order, with plain-language text.
use serde::{Deserialize, Serialize};

use super::catalog::EventTuning;
use super::decisions::DecisionPrompt;
use super::event_catalog::{EventGroup, SceneEffect};
use super::rng::Rng;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventKind {
    ComponentFailure,
    SolarFlare,
    GroundStationOutage,
    /// The battery can no longer carry the spacecraft through a whole eclipse (too small, or worn by deep
    /// discharges), so the Instruments switch off for part of every orbit. Not fatal by itself.
    BatteryShortfall,
    /// One of the new Random Events from `data/events.toml`; `RandomEvent::catalog` says which.
    Catalog,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RandomEvent {
    pub kind: EventKind,
    /// The full plain-language sentence shown to the player.
    pub text: String,
    /// Catalog id of the Component involved, if any.
    pub component: Option<String>,
    /// Set when this event asked the player a question before its day played out (interactive events).
    #[serde(default)]
    pub prompt: Option<DecisionPrompt>,
    /// For an event with a prompt: the id of the option that applied (the player's, or the default).
    #[serde(default)]
    pub choice: Option<String>,
    /// For a new Random Event (`EventKind::Catalog`): which one, and what the scene shows.
    #[serde(default)]
    pub catalog: Option<CatalogEvent>,
}

/// What a new Random Event carries beyond its text: its catalog entry and what the 3D scene shows for it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CatalogEvent {
    /// `EventSpec::id` in `data/events.toml`.
    pub id: String,
    pub group: EventGroup,
    pub scene: SceneEffect,
    /// A star model for a fly-by or a visitor.
    #[serde(default)]
    pub model: Option<String>,
    /// How many days the scene shows the effect: its longest timed effect, 1..=5.
    pub scene_days: u32,
    /// A Design choice softened the event.
    #[serde(default)]
    pub softened: bool,
    /// What applied could cost Science Value (less Power, Data, downlink, pointing, ...), so the Debrief may
    /// blame it; an event that only gave or only cost money never is.
    #[serde(default)]
    pub can_cost_science: bool,
}

/// The Component kinds that carry a Reliability Tier and can fail, in their fixed draw order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ComponentKind {
    Solar,
    Battery,
    Attitude,
    Comms,
    Computer,
}

impl ComponentKind {
    pub const DRAW_ORDER: [ComponentKind; 5] =
        [ComponentKind::Solar, ComponentKind::Battery, ComponentKind::Attitude, ComponentKind::Comms, ComponentKind::Computer];

    /// What the spacecraft has less of after a non-critical failure.
    fn lost_capability(self) -> &'static str {
        match self {
            ComponentKind::Solar => "Power",
            ComponentKind::Battery => "battery capacity",
            ComponentKind::Attitude => "pointing accuracy, so pointing Instruments return less Data",
            ComponentKind::Comms => "downlink",
            ComponentKind::Computer => "Data storage",
        }
    }
}

/// Everything random about one day, drawn before any of it is applied.
pub(crate) struct DayDraws {
    pub flare: bool,
    pub outage: bool,
    /// One uniform number per Component kind, in `ComponentKind::DRAW_ORDER`.
    pub component_rolls: [f64; 5],
}

/// Draws one day's random numbers. Always the same count in the same order, whatever the Design or the
/// state, so changing one choice does not reshuffle unrelated events.
pub(crate) fn draw_day(rng: &mut Rng, tuning: &EventTuning) -> DayDraws {
    let flare = rng.next_f64() < tuning.solar_flare_daily_chance;
    let outage = rng.next_f64() < tuning.outage_daily_chance;
    let mut component_rolls = [0.0; 5];
    for roll in &mut component_rolls {
        *roll = rng.next_f64();
    }
    DayDraws { flare, outage, component_rolls }
}

pub(crate) fn solar_flare(tuning: &EventTuning) -> RandomEvent {
    RandomEvent {
        kind: EventKind::SolarFlare,
        text: format!(
            "A solar flare hit. The spacecraft switched off its Instruments and waited in safe mode for {}.",
            days(tuning.flare_safe_mode_days)
        ),
        component: None,
        prompt: None,
        choice: None,
        catalog: None,
    }
}

pub(crate) fn solar_flare_ridden_out() -> RandomEvent {
    RandomEvent {
        kind: EventKind::SolarFlare,
        text: "A solar flare hit. The spacecraft rode it out with its Instruments running.".to_string(),
        component: None,
        prompt: None,
        choice: None,
        catalog: None,
    }
}

/// "The ground station" or "The Tsukuba ground station".
fn station_name(station: &str) -> String {
    if station.is_empty() { "The ground station".to_string() } else { format!("The {station} ground station") }
}

pub(crate) fn ground_station_outage_covered(tuning: &EventTuning, station: &str) -> RandomEvent {
    RandomEvent {
        kind: EventKind::GroundStationOutage,
        text: format!(
            "{} went offline for {}. Time bought on another station kept the Data coming down.",
            station_name(station),
            days(tuning.outage_days)
        ),
        component: None,
        prompt: None,
        choice: None,
        catalog: None,
    }
}

pub(crate) fn ground_station_outage(tuning: &EventTuning, station: &str) -> RandomEvent {
    RandomEvent {
        kind: EventKind::GroundStationOutage,
        text: format!(
            "{} went offline for {}. Data piles up on board until contact returns.",
            station_name(station),
            days(tuning.outage_days)
        ),
        component: None,
        prompt: None,
        choice: None,
        catalog: None,
    }
}

pub(crate) fn component_failure(kind: ComponentKind, id: &str, name: &str, fatal: bool, during_flare: bool) -> RandomEvent {
    let (cause, name) =
        if during_flare { ("Hit by the solar flare, the", mid_sentence(name)) } else { ("The", name.to_string()) };
    let text = if fatal {
        format!("{cause} {name} failed. The spacecraft cannot work without it: the Mission ends.")
    } else {
        format!("{cause} {name} failed. The spacecraft carries on with less {}.", kind.lost_capability())
    };
    RandomEvent { kind: EventKind::ComponentFailure, text, component: Some(id.to_string()), prompt: None, choice: None, catalog: None }
}

pub(crate) fn battery_shortfall(id: &str, name: &str, eclipse_minutes: f64, worn: bool) -> RandomEvent {
    let why = if worn {
        format!("Worn by draining too deep in every pass through Earth's shadow, the {name} can no longer")
    } else {
        format!("The {name} is too small to")
    };
    RandomEvent {
        kind: EventKind::BatteryShortfall,
        text: format!(
            "{why} carry the spacecraft through the {eclipse_minutes:.0} minutes in Earth's shadow each orbit. The Instruments now switch off for part of every eclipse, so less Data comes down."
        ),
        component: Some(id.to_string()),
        prompt: None,
        choice: None,
        catalog: None,
    }
}

/// A catalog name as it reads mid-sentence: "Deployable solar panels" becomes "deployable solar panels", but
/// names that start with an abbreviation or a code ("S-band radio", "UHF radio") keep their capitals.
fn mid_sentence(name: &str) -> String {
    let first_word = name.split_whitespace().next().unwrap_or("");
    let plain_word = first_word.chars().skip(1).all(|c| c.is_ascii_lowercase()) && first_word.len() > 1;
    let mut chars = name.chars();
    match chars.next() {
        Some(c) if plain_word => c.to_lowercase().chain(chars).collect(),
        _ => name.to_string(),
    }
}

pub(crate) fn days(n: u32) -> String {
    if n == 1 { "1 day".to_string() } else { format!("{n} days") }
}
