//! Shared known-good Designs for all integration tests. Later tickets add one function per Scenario.
//! Include from a test file with `mod common;`.
#![allow(dead_code)]

use spaceooops::sim::{Design, Pick, ReliabilityTier::FlightProven};

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
