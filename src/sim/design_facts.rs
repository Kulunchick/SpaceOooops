//! Design facts: plain yes/no statements about a Design that new Random Events can depend on ("only with a
//! UHF radio", "more likely in a low Orbit", "softened by a Flight-proven computer").
use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use super::catalog::{AntennaKind, SolarLayout};
use super::design::{Design, ReliabilityTier};
use super::resources::Resolved;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DesignFact {
    /// Orbit altitude below 2000 km.
    LowOrbit,
    /// Orbit altitude of 2000 km or more.
    HighOrbit,
    /// Solar panels on hinged wings.
    DeployablePanels,
    /// At least one optical Instrument (an imager).
    Camera,
    /// The comms option has a whip antenna (UHF).
    UhfRadio,
    /// Attitude control that can hold a pointing Instrument steady.
    PreciseAttitude,
    /// Attitude control of the Basic Reliability Tier.
    AttitudeBasic,
    /// The computer is of the Flight-proven Reliability Tier.
    ComputerFlightProven,
}

impl DesignFact {
    /// The choice behind the fact, as it reads mid-sentence in the Debrief ("... because of a low Orbit").
    pub fn phrase(self) -> &'static str {
        match self {
            DesignFact::LowOrbit => "a low Orbit",
            DesignFact::HighOrbit => "a high Orbit",
            DesignFact::DeployablePanels => "deployable solar panels",
            DesignFact::Camera => "an optical Instrument",
            DesignFact::UhfRadio => "a UHF radio",
            DesignFact::PreciseAttitude => "precise attitude control",
            DesignFact::AttitudeBasic => "Basic attitude control",
            DesignFact::ComputerFlightProven => "a Flight-proven computer",
        }
    }
}

/// The facts that hold for one Design.
pub(crate) type Facts = BTreeSet<DesignFact>;

/// Which facts hold for the Design, one line per fact.
pub(crate) fn facts(r: &Resolved, d: &Design) -> Facts {
    let checks = [
        (DesignFact::LowOrbit, r.orbit.is_some_and(|o| o.altitude_km < 2000.0)),
        (DesignFact::HighOrbit, r.orbit.is_some_and(|o| o.altitude_km >= 2000.0)),
        (DesignFact::DeployablePanels, r.solar.is_some_and(|(s, _)| s.layout == SolarLayout::Wings)),
        (DesignFact::Camera, r.instruments.iter().any(|i| i.optical)),
        (DesignFact::UhfRadio, r.comms.is_some_and(|(c, _)| c.antenna == AntennaKind::Whip)),
        (DesignFact::PreciseAttitude, r.attitude.is_some_and(|(a, _)| a.fine_pointing)),
        (DesignFact::AttitudeBasic, r.attitude.is_some_and(|(_, t)| t == ReliabilityTier::Basic)),
        (DesignFact::ComputerFlightProven, d.computer_tier == ReliabilityTier::FlightProven),
    ];
    checks.into_iter().filter(|(_, holds)| *holds).map(|(fact, _)| fact).collect()
}
