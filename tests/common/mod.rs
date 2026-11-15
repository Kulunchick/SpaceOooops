//! Shared known-good Designs for all integration tests. Later tickets add one function per Scenario.
//! Include from a test file with `mod common;`.
#![allow(dead_code)]

use spaceooops::sim::{
    Catalog, Design, EventKind, Pick, RandomEvent, RunResult,
    ReliabilityTier::{Basic, FlightProven},
    load_catalog, parse_events,
};

/// The new Random Events of a Run, with their days.
pub fn new_events(r: &RunResult) -> Vec<(u32, &RandomEvent)> {
    r.timeline.iter().flat_map(|t| t.events.iter().filter(|e| e.kind == EventKind::Catalog).map(move |e| (t.day, e))).collect()
}

/// Share of `day` (1-based) the Instruments ran.
pub fn operating_on(r: &RunResult, day: u32) -> f64 {
    r.timeline[day as usize - 1].operating_fraction
}

/// The message `parse_events` rejects a `data/events.toml`-shaped text with.
pub fn parse_error(toml: &str) -> String {
    parse_events(toml).expect_err("the entry should be rejected").to_string()
}

/// The shipped catalog with the new Random Events switched off in every Scenario: a Run is as before them.
/// Behaviour tests that pin numbers use this, so new events added later never move their thresholds.
pub fn legacy_catalog() -> Catalog {
    with_new_events(load_catalog().unwrap(), 0, 0)
}

/// The shipped catalog with only the events of `toml` (a `data/events.toml`-shaped text), and every Scenario
/// giving a Run between `min` and `max` of them.
pub fn catalog_with_events(toml: &str, min: u32, max: u32) -> Catalog {
    let mut c = load_catalog().unwrap();
    c.events = parse_events(toml).unwrap();
    with_new_events(c, min, max)
}

/// The shipped catalog, events included, with every Scenario giving a Run between `min` and `max` of them.
pub fn with_new_events(mut c: Catalog, min: u32, max: u32) -> Catalog {
    for s in &mut c.scenarios {
        s.events.new_events_min = min;
        s.events.new_events_max = max;
    }
    c
}

pub const CROP_WATCH: &str = "crop-watch";

/// A 6U CubeSat with a multispectral imager on reaction wheels, deployable panels, a 40 Wh battery
/// and an S-band radio in SSO, on a rideshare. Passes Launch Review for Crop Watch with no warnings.
pub fn crop_watch_known_good() -> Design {
    Design {
        instruments: vec!["multispectral-imager".to_string()],
        structure: Some("cubesat-6u".to_string()),
        solar: Some(Pick::new("solar-deployable", FlightProven)),
        battery: Some(Pick::new("battery-m", FlightProven)),
        attitude: Some(Pick::new("reaction-wheels", FlightProven)),
        comms: Some(Pick::new("comms-s-band", FlightProven)),
        computer_tier: FlightProven,
        orbit: Some("sso-600".to_string()),
        launch_vehicle: Some("cubesat-rideshare".to_string()),
    }
}

pub const CONNECT_THE_VILLAGES: &str = "connect-the-villages";
pub const SPACE_WEATHER_SENTINEL: &str = "space-weather-sentinel";

/// A 12U CubeSat carrying a Ka-band village relay to GEO on a GEO rideshare slot. Body-mounted panels are
/// enough in GEO's long sunlight; reaction wheels keep the narrow beam on Angola. Passes cleanly.
pub fn connect_the_villages_known_good() -> Design {
    Design {
        instruments: vec!["village-relay".to_string()],
        structure: Some("cubesat-12u".to_string()),
        solar: Some(Pick::new("solar-body", FlightProven)),
        battery: Some(Pick::new("battery-l", FlightProven)),
        attitude: Some(Pick::new("reaction-wheels", FlightProven)),
        comms: Some(Pick::new("comms-s-band", FlightProven)),
        computer_tier: FlightProven,
        orbit: Some("geo".to_string()),
        launch_vehicle: Some("geo-rideshare".to_string()),
    }
}

pub const AMAZON_WATCH: &str = "amazon-watch";

/// A 12U CubeSat with the radar on reaction wheels, deployable panels, an 80 Wh battery and an X-band radio for
/// the radar's heavy Data, in SSO on a SmallSat rideshare (too heavy for a 6U slot). Passes cleanly in Brazil.
pub fn amazon_watch_known_good() -> Design {
    Design {
        instruments: vec!["sar-radar".to_string()],
        structure: Some("cubesat-12u".to_string()),
        solar: Some(Pick::new("solar-deployable", FlightProven)),
        battery: Some(Pick::new("battery-l", FlightProven)),
        attitude: Some(Pick::new("reaction-wheels", FlightProven)),
        comms: Some(Pick::new("comms-x-band", FlightProven)),
        computer_tier: FlightProven,
        orbit: Some("sso-600".to_string()),
        launch_vehicle: Some("smallsat-rideshare".to_string()),
    }
}

pub const CYCLONE_WATCH: &str = "cyclone-watch";

/// A 12U CubeSat with the weather imager on reaction wheels, body-mounted panels (GEO's sunlight is long), an 80 Wh
/// battery and an S-band radio, in GEO on a GEO rideshare slot. Passes cleanly in India.
pub fn cyclone_watch_known_good() -> Design {
    Design {
        instruments: vec!["weather-imager".to_string()],
        structure: Some("cubesat-12u".to_string()),
        solar: Some(Pick::new("solar-body", FlightProven)),
        battery: Some(Pick::new("battery-l", FlightProven)),
        attitude: Some(Pick::new("reaction-wheels", FlightProven)),
        comms: Some(Pick::new("comms-s-band", FlightProven)),
        computer_tier: FlightProven,
        orbit: Some("geo".to_string()),
        launch_vehicle: Some("geo-rideshare".to_string()),
    }
}

pub const JAPAN_DISASTER_RESPONSE: &str = "japan-disaster-response";

/// The radar spacecraft of Amazon Watch (12U CubeSat, X-band radio, SSO rideshare) with one thing given up to fit
/// Japan's tight Budget: Basic reaction wheels (not critical: a failure only blurs the radar's aim). The X-band
/// radio cannot be given up. Launches with one warning, the thin Budget margin that is this Scenario's lesson:
/// a Design that leaves 15% of the Budget unspent cannot carry the radar here.
pub fn japan_disaster_response_known_good() -> Design {
    Design { attitude: Some(Pick::new("reaction-wheels", Basic)), ..amazon_watch_known_good() }
}

pub const GLACIER_WATCH: &str = "glacier-watch";

/// Crop Watch's spacecraft again: a 6U CubeSat with a multispectral imager, flown in SSO for the years a glacier
/// trend needs. Passes cleanly in Argentina.
pub fn glacier_watch_known_good() -> Design {
    crop_watch_known_good()
}

/// A 6U CubeSat with a particle detector and an ionosphere probe in LEO 400 km, deployed from the space
/// station, sending its heavy Data down over an X-band radio. Passes cleanly.
pub fn space_weather_sentinel_known_good() -> Design {
    Design {
        instruments: vec!["particle-detector".to_string(), "ionosphere-probe".to_string()],
        structure: Some("cubesat-6u".to_string()),
        solar: Some(Pick::new("solar-deployable", FlightProven)),
        battery: Some(Pick::new("battery-m", FlightProven)),
        attitude: Some(Pick::new("magnetorquers", FlightProven)),
        comms: Some(Pick::new("comms-x-band", FlightProven)),
        computer_tier: FlightProven,
        orbit: Some("leo-400".to_string()),
        launch_vehicle: Some("station-deploy".to_string()),
    }
}
