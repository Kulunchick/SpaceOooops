//! The catalog: Scenarios and every option a Design can pick, parsed from the bundled TOML data files.
//! Nothing here is specific to one Scenario or Orbit; new entries are added in `data/` only.
use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use super::design::ReliabilityTier;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Scenario {
    pub id: String,
    pub name: String,
    pub difficulty: String,
    pub objective: String,
    pub why_it_matters: String,
    pub required_science_value: f64,
    pub min_lifetime_days: u32,
    pub budget_usd: f64,
    #[serde(default)]
    pub key_constraints: Vec<String>,
    #[serde(default)]
    pub tutorial_hints: Vec<String>,
    /// Science Value per GB delivered by an Instrument with fit 1.0 over an Orbit with coverage 1.0.
    pub science_value_per_gb: f64,
    /// Random Event chances and severity in this Scenario.
    pub events: EventTuning,
}

/// Per-Scenario Random Event chances (per day) and severity.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EventTuning {
    pub solar_flare_daily_chance: f64,
    pub flare_safe_mode_days: u32,
    /// On a flare day every Component's failure chance is multiplied by this.
    pub flare_failure_multiplier: f64,
    pub outage_daily_chance: f64,
    pub outage_days: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Structure {
    pub id: String,
    pub name: String,
    pub why: String,
    pub mass_kg: f64,
    pub cost_usd: f64,
    /// Outer size in CubeSat units; body-mounted solar output scales with it.
    pub size_u: f64,
    /// Volume left for Components and Instruments.
    pub usable_volume_u: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SolarPanel {
    pub id: String,
    pub name: String,
    pub why: String,
    pub mass_kg: f64,
    pub cost_usd: f64,
    pub volume_u: f64,
    /// Average output while in sunlight, per CubeSat unit of Structure size.
    pub generation_w_per_u: f64,
    #[serde(default)]
    pub critical: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Battery {
    pub id: String,
    pub name: String,
    pub why: String,
    pub mass_kg: f64,
    pub cost_usd: f64,
    pub volume_u: f64,
    pub capacity_wh: f64,
    #[serde(default)]
    pub critical: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AttitudeControl {
    pub id: String,
    pub name: String,
    pub why: String,
    pub mass_kg: f64,
    pub power_w: f64,
    pub cost_usd: f64,
    pub volume_u: f64,
    /// Can hold a pointing Instrument steady on its target.
    #[serde(default)]
    pub fine_pointing: bool,
    #[serde(default)]
    pub critical: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Comms {
    pub id: String,
    pub name: String,
    pub why: String,
    pub mass_kg: f64,
    /// Orbit-averaged consumption (the transmitter only runs during ground contacts).
    pub power_w: f64,
    pub cost_usd: f64,
    pub volume_u: f64,
    pub downlink_mbps: f64,
    /// Ground-station fees for this band, paid every day of the planned Lifetime.
    pub ops_cost_usd_per_day: f64,
    #[serde(default)]
    pub critical: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Computer {
    pub id: String,
    pub name: String,
    pub why: String,
    pub mass_kg: f64,
    pub power_w: f64,
    pub cost_usd: f64,
    pub volume_u: f64,
    pub storage_mb: f64,
    #[serde(default)]
    pub critical: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Instrument {
    pub id: String,
    pub name: String,
    pub why: String,
    pub mass_kg: f64,
    /// Orbit-averaged consumption.
    pub power_w: f64,
    pub cost_usd: f64,
    pub volume_u: f64,
    pub data_mb_per_day: f64,
    #[serde(default)]
    pub needs_fine_pointing: bool,
    /// How well the Instrument suits each Scenario objective (0..=1), keyed by scenario id.
    #[serde(default)]
    pub fit: BTreeMap<String, f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Orbit {
    pub id: String,
    pub name: String,
    pub why: String,
    pub altitude_km: f64,
    pub period_min: f64,
    /// Share of each orbit spent in Earth's shadow.
    pub eclipse_fraction: f64,
    pub contact_minutes_per_day: f64,
    /// Longest Lifetime this Orbit allows (atmospheric drag, radiation).
    pub lifetime_cap_days: f64,
    /// How well the Orbit covers each Scenario's target (0..=1), keyed by scenario id.
    #[serde(default)]
    pub coverage: BTreeMap<String, f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LaunchVehicle {
    pub id: String,
    pub name: String,
    pub why: String,
    pub real_example: String,
    pub cost_usd: f64,
    /// Payload capacity per orbit id; a missing entry means it cannot reach that Orbit.
    pub capacity_kg: BTreeMap<String, f64>,
}

/// Cost multiplier and failure chance of one Reliability Tier.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TierSpec {
    pub tier: ReliabilityTier,
    pub name: String,
    pub why: String,
    pub cost_multiplier: f64,
    /// Chance that one Component of this tier fails on any given day.
    pub daily_failure_chance: f64,
}

/// Thresholds the Launch Review uses.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReviewRules {
    /// A margin below this share of the limit counts as thin.
    pub thin_margin_fraction: f64,
    /// Deepest battery discharge per eclipse that keeps the battery healthy.
    pub battery_max_depth_of_discharge: f64,
}

/// Rules of the Run that hold in every Scenario.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RunRules {
    /// Share of normal output a failed non-critical Component keeps.
    pub degraded_output_fraction: f64,
    /// Share of the original battery capacity lost per eclipse cycle at the healthy depth of discharge.
    pub battery_fade_per_cycle_at_max_depth: f64,
    /// How steeply wear grows with deeper discharge.
    pub battery_fade_exponent: u32,
    /// Minutes of each orbit lost to rebooting and re-pointing after the battery browns out in an eclipse.
    #[serde(default)]
    pub brownout_recovery_minutes: f64,
}

fn yes() -> bool {
    true
}

/// Score weights.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScoringRules {
    pub points_per_science_value: f64,
    pub budget_margin_points: f64,
    pub mass_margin_points: f64,
    /// Budget and Mass margin bonuses are paid only when the Mission is a Success.
    #[serde(default = "yes")]
    pub margin_bonuses_need_success: bool,
    pub early_end_penalty_points: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Catalog {
    pub scenarios: Vec<Scenario>,
    pub structures: Vec<Structure>,
    pub solar_panels: Vec<SolarPanel>,
    pub batteries: Vec<Battery>,
    pub attitude_controls: Vec<AttitudeControl>,
    pub comms: Vec<Comms>,
    pub computer: Computer,
    pub reliability_tiers: Vec<TierSpec>,
    pub instruments: Vec<Instrument>,
    pub orbits: Vec<Orbit>,
    pub launch_vehicles: Vec<LaunchVehicle>,
    pub review: ReviewRules,
    pub run_rules: RunRules,
    pub scoring: ScoringRules,
}

impl Catalog {
    pub fn scenario(&self, id: &str) -> Option<&Scenario> {
        self.scenarios.iter().find(|s| s.id == id)
    }
    pub fn structure(&self, id: &str) -> Option<&Structure> {
        self.structures.iter().find(|o| o.id == id)
    }
    pub fn solar_panel(&self, id: &str) -> Option<&SolarPanel> {
        self.solar_panels.iter().find(|o| o.id == id)
    }
    pub fn battery(&self, id: &str) -> Option<&Battery> {
        self.batteries.iter().find(|o| o.id == id)
    }
    pub fn attitude_control(&self, id: &str) -> Option<&AttitudeControl> {
        self.attitude_controls.iter().find(|o| o.id == id)
    }
    pub fn comms_option(&self, id: &str) -> Option<&Comms> {
        self.comms.iter().find(|o| o.id == id)
    }
    pub fn instrument(&self, id: &str) -> Option<&Instrument> {
        self.instruments.iter().find(|o| o.id == id)
    }
    pub fn orbit(&self, id: &str) -> Option<&Orbit> {
        self.orbits.iter().find(|o| o.id == id)
    }
    pub fn launch_vehicle(&self, id: &str) -> Option<&LaunchVehicle> {
        self.launch_vehicles.iter().find(|o| o.id == id)
    }
    pub fn tier(&self, tier: ReliabilityTier) -> Option<&TierSpec> {
        self.reliability_tiers.iter().find(|t| t.tier == tier)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CatalogError {
    /// A data file is not valid TOML or does not match the expected shape.
    Parse { file: String, message: String },
    /// The data parsed but is inconsistent (duplicate ids, dangling references, missing tiers).
    Invalid(String),
}

impl std::fmt::Display for CatalogError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CatalogError::Parse { file, message } => write!(f, "{file}: {message}"),
            CatalogError::Invalid(message) => write!(f, "{message}"),
        }
    }
}

const SCENARIOS: (&str, &str) = ("data/scenarios.toml", include_str!("../../data/scenarios.toml"));
const COMPONENTS: (&str, &str) = ("data/components.toml", include_str!("../../data/components.toml"));
const INSTRUMENTS: (&str, &str) = ("data/instruments.toml", include_str!("../../data/instruments.toml"));
const ORBITS: (&str, &str) = ("data/orbits.toml", include_str!("../../data/orbits.toml"));
const LAUNCH_VEHICLES: (&str, &str) =
    ("data/launch_vehicles.toml", include_str!("../../data/launch_vehicles.toml"));
const REVIEW: (&str, &str) = ("data/review.toml", include_str!("../../data/review.toml"));
const RUN: (&str, &str) = ("data/run.toml", include_str!("../../data/run.toml"));
const SCORING: (&str, &str) = ("data/scoring.toml", include_str!("../../data/scoring.toml"));

#[derive(Deserialize)]
struct ScenariosFile {
    scenarios: Vec<Scenario>,
}

#[derive(Deserialize)]
struct ComponentsFile {
    structures: Vec<Structure>,
    solar_panels: Vec<SolarPanel>,
    batteries: Vec<Battery>,
    attitude_controls: Vec<AttitudeControl>,
    comms: Vec<Comms>,
    computer: Computer,
    reliability_tiers: Vec<TierSpec>,
}

#[derive(Deserialize)]
struct InstrumentsFile {
    instruments: Vec<Instrument>,
}

#[derive(Deserialize)]
struct OrbitsFile {
    orbits: Vec<Orbit>,
}

#[derive(Deserialize)]
struct LaunchVehiclesFile {
    launch_vehicles: Vec<LaunchVehicle>,
}

#[derive(Deserialize)]
struct ReviewFile {
    review: ReviewRules,
}

#[derive(Deserialize)]
struct RunFile {
    run: RunRules,
}

#[derive(Deserialize)]
struct ScoringFile {
    scoring: ScoringRules,
}

fn parse<T: serde::de::DeserializeOwned>((file, text): (&str, &str)) -> Result<T, CatalogError> {
    toml::from_str(text).map_err(|e| CatalogError::Parse { file: file.to_string(), message: e.to_string() })
}

/// Parses the data files bundled into the build.
pub fn load_catalog() -> Result<Catalog, CatalogError> {
    let scenarios: ScenariosFile = parse(SCENARIOS)?;
    let components: ComponentsFile = parse(COMPONENTS)?;
    let instruments: InstrumentsFile = parse(INSTRUMENTS)?;
    let orbits: OrbitsFile = parse(ORBITS)?;
    let launch_vehicles: LaunchVehiclesFile = parse(LAUNCH_VEHICLES)?;
    let review: ReviewFile = parse(REVIEW)?;
    let run: RunFile = parse(RUN)?;
    let scoring: ScoringFile = parse(SCORING)?;
    let catalog = Catalog {
        scenarios: scenarios.scenarios,
        structures: components.structures,
        solar_panels: components.solar_panels,
        batteries: components.batteries,
        attitude_controls: components.attitude_controls,
        comms: components.comms,
        computer: components.computer,
        reliability_tiers: components.reliability_tiers,
        instruments: instruments.instruments,
        orbits: orbits.orbits,
        launch_vehicles: launch_vehicles.launch_vehicles,
        review: review.review,
        run_rules: run.run,
        scoring: scoring.scoring,
    };
    validate(&catalog)?;
    Ok(catalog)
}

fn validate(c: &Catalog) -> Result<(), CatalogError> {
    let option_ids = c
        .structures.iter().map(|o| &o.id)
        .chain(c.solar_panels.iter().map(|o| &o.id))
        .chain(c.batteries.iter().map(|o| &o.id))
        .chain(c.attitude_controls.iter().map(|o| &o.id))
        .chain(c.comms.iter().map(|o| &o.id))
        .chain(std::iter::once(&c.computer.id))
        .chain(c.instruments.iter().map(|o| &o.id))
        .chain(c.orbits.iter().map(|o| &o.id))
        .chain(c.launch_vehicles.iter().map(|o| &o.id));
    let mut seen = BTreeSet::new();
    for id in option_ids {
        if !seen.insert(id.as_str()) {
            return Err(CatalogError::Invalid(format!("option id '{id}' is used twice")));
        }
    }
    let scenario_ids: BTreeSet<&str> = c.scenarios.iter().map(|s| s.id.as_str()).collect();
    let orbit_ids: BTreeSet<&str> = c.orbits.iter().map(|o| o.id.as_str()).collect();
    for instrument in &c.instruments {
        if let Some(key) = instrument.fit.keys().find(|k| !scenario_ids.contains(k.as_str())) {
            return Err(CatalogError::Invalid(format!("instrument '{}' has fit for unknown scenario '{key}'", instrument.id)));
        }
    }
    for orbit in &c.orbits {
        if let Some(key) = orbit.coverage.keys().find(|k| !scenario_ids.contains(k.as_str())) {
            return Err(CatalogError::Invalid(format!("orbit '{}' has coverage for unknown scenario '{key}'", orbit.id)));
        }
    }
    for lv in &c.launch_vehicles {
        if let Some(key) = lv.capacity_kg.keys().find(|k| !orbit_ids.contains(k.as_str())) {
            return Err(CatalogError::Invalid(format!("launch vehicle '{}' has capacity to unknown orbit '{key}'", lv.id)));
        }
    }
    for tier in [ReliabilityTier::Basic, ReliabilityTier::FlightProven] {
        if c.tier(tier).is_none() {
            return Err(CatalogError::Invalid(format!("reliability tier {tier:?} is missing")));
        }
    }
    Ok(())
}
