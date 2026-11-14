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
