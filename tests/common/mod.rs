//! Shared known-good Designs for all integration tests. Later tickets add one function per Scenario.
//! Include from a test file with `mod common;`.
#![allow(dead_code)]

use spaceooops::sim::{Catalog, Design, Pick, ReliabilityTier::FlightProven, load_catalog, parse_events};

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
