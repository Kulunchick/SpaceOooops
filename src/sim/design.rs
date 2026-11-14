//! The player's Design: every choice made before launch. Options are referenced by catalog id.
use serde::{Deserialize, Serialize};

/// How proven a Component is. Cost multipliers and failure chances live in `data/components.toml`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReliabilityTier {
    #[default]
    Basic,
    FlightProven,
}

/// A responsibility area; in solo play one player holds every Role.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    Science,
    Spacecraft,
    LaunchOrbit,
    BudgetComms,
}

impl Role {
    pub const ALL: [Role; 4] = [Role::Science, Role::Spacecraft, Role::LaunchOrbit, Role::BudgetComms];

    /// The Role name as written in the glossary.
    pub fn name(self) -> &'static str {
        match self {
            Role::Science => "Science",
            Role::Spacecraft => "Spacecraft",
            Role::LaunchOrbit => "Launch & Orbit",
            Role::BudgetComms => "Budget & Comms",
        }
    }
}

/// A chosen Component option together with its Reliability Tier.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pick {
    pub id: String,
    pub tier: ReliabilityTier,
}

impl Pick {
    pub fn new(id: &str, tier: ReliabilityTier) -> Self {
        Pick { id: id.to_string(), tier }
    }
}

/// Every choice the players make before launch. `Design::default()` is the empty Design.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Design {
    pub instruments: Vec<String>,
    pub structure: Option<String>,
    pub solar: Option<Pick>,
    pub battery: Option<Pick>,
    pub attitude: Option<Pick>,
    /// `None` means the spacecraft has no comms link (a hard violation).
    pub comms: Option<Pick>,
    /// The single computer option is always on board; only its tier is chosen.
    pub computer_tier: ReliabilityTier,
    pub orbit: Option<String>,
    pub launch_vehicle: Option<String>,
}
