//! The catalog: Scenarios and every option a Design can pick, parsed from the bundled TOML data files.
//! Nothing here is specific to one Scenario or Orbit; new entries are added in `data/` only.
use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use super::design::ReliabilityTier;
use super::event_catalog::{EventSpec, EVENTS_FILE, parse_events};
use super::objectives::ObjectiveSpec;

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
    /// Where the Scenario's marker sits on the globe (degrees). Visual only; the sim never reads it.
    pub marker_lat: f64,
    pub marker_lon: f64,
    /// Illustration shown on the Scenario card (a path under `assets/`). Visual only.
    pub illustration: String,
    #[serde(default)]
    pub key_constraints: Vec<String>,
    #[serde(default)]
    pub tutorial_hints: Vec<String>,
    /// Science Value per GB delivered by an Instrument with fit 1.0 over an Orbit with coverage 1.0.
    pub science_value_per_gb: f64,
    /// The country this Scenario serves: card label and the globe's highlight. Required.
    pub target_country: TargetCountry,
    /// Random Event chances and severity in this Scenario.
    pub events: EventTuning,
}

/// A Scenario's Target Country: `id` keys the outline in the JS land mask, `name` is shown to the player.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TargetCountry {
    pub id: String,
    pub name: String,
}

/// The Target Country highlighted where no Scenario is in play (title, Sources, Scenario select).
pub const HOME_COUNTRY: &str = "angola";

/// Scenarios in the order the cards list them: Angolan ones first, then by difficulty (Easy, Medium, Hard);
/// ties keep catalog order.
pub fn scenarios_in_card_order(scenarios: &[Scenario]) -> Vec<&Scenario> {
    let rank = |d: &str| ["Easy", "Medium", "Hard"].iter().position(|x| *x == d).unwrap_or(usize::MAX);
    let mut ordered: Vec<&Scenario> = scenarios.iter().collect();
    ordered.sort_by_key(|s| (s.target_country.id != HOME_COUNTRY, rank(&s.difficulty)));
    ordered
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
    /// How many new Random Events (`data/events.toml`) a Run gets, at least and at most. 0 and 0 = none.
    #[serde(default)]
    pub new_events_min: u32,
    #[serde(default)]
    pub new_events_max: u32,
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
    /// Size class for the Real-Mission Match ("cubesat", "smallsat", "large").
    #[serde(default)]
    pub spacecraft_class: String,
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
    /// Visual only: how the panels sit on the 3D model.
    #[serde(default)]
    pub layout: SolarLayout,
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
    /// Visual only: the hardware the 3D model shows.
    #[serde(default)]
    pub hardware: AttitudeHardware,
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
    /// Visual only: the antenna the 3D model shows.
    #[serde(default)]
    pub antenna: AntennaKind,
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
    /// Visual only: the shape the Instrument has on the 3D model.
    #[serde(default)]
    pub shape: InstrumentShape,
    /// An imager: makes the Design fact `camera` hold for new Random Events.
    #[serde(default)]
    pub optical: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Orbit {
    pub id: String,
    pub name: String,
    pub why: String,
    pub altitude_km: f64,
    /// Orbit class for the Real-Mission Match ("leo", "sso", "geo").
    #[serde(default)]
    pub orbit_class: String,
    /// Tilt of the orbit plane against the equator, in degrees. Only used to draw the Orbit (0 if unknown).
    #[serde(default)]
    pub inclination_deg: f64,
    pub period_min: f64,
    /// Share of each orbit spent in Earth's shadow.
    pub eclipse_fraction: f64,
    pub contact_minutes_per_day: f64,
    /// Longest Lifetime this Orbit allows (atmospheric drag, radiation); absent = no cap.
    #[serde(default)]
    pub lifetime_cap_days: Option<f64>,
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
    /// Visual only: the Launch Vehicle model the 3D scene shows (the data key stays `rocket`).
    #[serde(default)]
    pub rocket: LaunchVehicleLook,
}

// ---- Visual-only catalog fields: they change how the 3D scene draws an option, never a game number. ----

/// How solar panels sit on the spacecraft.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SolarLayout {
    /// Cells glued to the bus walls.
    #[default]
    Body,
    /// Hinged wings that unfold from the bus.
    Wings,
}

/// The attitude control hardware the spacecraft shows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AttitudeHardware {
    #[default]
    Magnetorquers,
    ReactionWheels,
}

/// The antenna a comms option shows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AntennaKind {
    /// Tape-measure dipole whips (UHF).
    #[default]
    Whip,
    /// A flat patch antenna (S-band).
    Patch,
    /// A small parabolic dish (X-band).
    Dish,
}

/// The shape an Instrument has on the spacecraft.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum InstrumentShape {
    /// A plain payload box (an Instrument without a shape of its own).
    #[default]
    Box,
    /// A long telescope barrel with a sunshade.
    Telescope,
    /// A squat camera with a large germanium lens and radiator fins.
    ThermalCamera,
    /// A small box camera with a wide lens.
    WideCamera,
    /// A folded reflector dish on an arm.
    FoldedDish,
    /// A helical antenna.
    Helix,
    /// A stack of detector discs behind a collimator.
    ParticleStack,
    /// A sphere probe on a long boom.
    ProbeBoom,
}

/// How a Launch Vehicle looks: its class and real height.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct LaunchVehicleLook {
    pub class: LaunchVehicleClass,
    pub height_m: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LaunchVehicleClass {
    /// A small single-core launcher.
    #[default]
    Small,
    /// A medium two-stage rocket.
    Medium,
    /// A rocket with strap-on boosters.
    Heavy,
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
    /// Numbers behind the choices of interactive Random Events.
    pub decisions: DecisionRules,
}

/// Tuning of the choices the player is offered during the Run (`[run.decisions]` in run.toml).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DecisionRules {
    /// Riding out a solar flare: each Component's failure chance on the flare day is multiplied by this
    /// (on top of the Scenario's flare multiplier).
    pub flare_ride_out_failure_multiplier: f64,
    /// Buying time on another ground station for one whole outage.
    pub outage_station_time_cost_usd: f64,
    /// Data lost over an outage below this many MB is reported as "no Data was lost" (rounding noise).
    pub negligible_lost_data_mb: f64,
}

/// A real mission the Debrief can compare the player's Design with (`data/real_missions.toml`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RealMission {
    pub name: String,
    pub operator: String,
    pub year: u32,
    pub orbit_class: String,
    pub spacecraft_class: String,
    /// CubeSat size in units, when it is a CubeSat.
    #[serde(default)]
    pub size_u: Option<f64>,
    #[serde(default)]
    pub mass_kg: Option<f64>,
    /// Short human description of its size, e.g. "6U CubeSat".
    pub size_note: String,
    pub purposes: Vec<String>,
    /// One factual sentence.
    pub blurb: String,
    /// Public page the facts come from.
    pub url: String,
    /// "primary" (operator or space agency page) or "secondary" (catalogue, encyclopedia or news).
    pub verified: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpacecraftClass {
    pub name: String,
    /// Smallest first.
    pub rank: u32,
}

/// The words and groupings the Real-Mission Match works with (`data/real_missions.toml`).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct MatchTerms {
    /// Purpose tags of each Scenario's objective, keyed by scenario id.
    pub scenario_purposes: BTreeMap<String, Vec<String>>,
    pub purpose_words: BTreeMap<String, String>,
    pub orbit_classes: BTreeMap<String, String>,
    pub spacecraft_classes: BTreeMap<String, SpacecraftClass>,
}

/// A "why it matters" explanation for a Design parameter, gauge or Component category (`data/glossary.toml`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GlossaryTerm {
    /// The label as shown on screen, e.g. "Mass".
    pub key: String,
    /// Other labels that mean the same, e.g. "Time in shadow" for the eclipse.
    #[serde(default)]
    pub aliases: Vec<String>,
    pub why: String,
}

/// Which Debrief causes are worth telling, and how many (`data/causes.toml`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CauseRules {
    pub max_causes: usize,
    /// Causes costing less than this share of the required Science Value are left out.
    pub min_impact_share: f64,
    pub poor_coverage_below: f64,
    pub poor_fit_below: f64,
    pub drag_altitude_below_km: f64,
    /// When weighing a poorly suited Instrument, the Orbit's coverage counts as at least this, so a poor
    /// Instrument still shows its cost in an Orbit that covers almost nothing.
    pub min_coverage_for_fit_cost: f64,
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
    pub cause_rules: CauseRules,
    pub real_missions: Vec<RealMission>,
    pub match_terms: MatchTerms,
    pub glossary: Vec<GlossaryTerm>,
    /// Personal Objectives: one default per Role plus optional Scenario-specific ones.
    pub objectives: Vec<ObjectiveSpec>,
    /// Seed of the trial Run behind the pre-launch objective outlook.
    pub objectives_preview_seed: u64,
    /// The new Random Events (`data/events.toml`).
    #[serde(default)]
    pub events: Vec<EventSpec>,
}

impl Catalog {
    /// The "why it matters" text for an on-screen label (case-insensitive, aliases included).
    pub fn glossary_why(&self, label: &str) -> Option<&str> {
        let label = label.trim();
        self.glossary
            .iter()
            .find(|t| t.key.eq_ignore_ascii_case(label) || t.aliases.iter().any(|a| a.eq_ignore_ascii_case(label)))
            .map(|t| t.why.as_str())
    }
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
const OBJECTIVES: (&str, &str) = ("data/objectives.toml", include_str!("../../data/objectives.toml"));

#[derive(Deserialize)]
struct ObjectivesFile {
    preview_seed: u64,
    objectives: Vec<ObjectiveSpec>,
}

const EVENTS: (&str, &str) = (EVENTS_FILE, include_str!("../../data/events.toml"));
const CAUSES: (&str, &str) = ("data/causes.toml", include_str!("../../data/causes.toml"));

#[derive(Deserialize)]
struct CausesFile {
    causes: CauseRules,
}

const REAL_MISSIONS: (&str, &str) = ("data/real_missions.toml", include_str!("../../data/real_missions.toml"));
const GLOSSARY: (&str, &str) = ("data/glossary.toml", include_str!("../../data/glossary.toml"));

#[derive(Deserialize)]
struct RealMissionsFile {
    real_missions: Vec<RealMission>,
    #[serde(flatten)]
    terms: MatchTerms,
}

#[derive(Deserialize)]
struct GlossaryFile {
    terms: Vec<GlossaryTerm>,
}

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
    let causes: CausesFile = parse(CAUSES)?;
    let real_missions: RealMissionsFile = parse(REAL_MISSIONS)?;
    let glossary: GlossaryFile = parse(GLOSSARY)?;
    let objectives: ObjectivesFile = parse(OBJECTIVES)?;
    let events = parse_events(EVENTS.1)?;
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
        cause_rules: causes.causes,
        real_missions: real_missions.real_missions,
        match_terms: real_missions.terms,
        glossary: glossary.terms,
        objectives: objectives.objectives,
        objectives_preview_seed: objectives.preview_seed,
        events,
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
    for o in &c.objectives {
        if let Some(key) = o.scenario.as_ref().filter(|k| !scenario_ids.contains(k.as_str())) {
            return Err(CatalogError::Invalid(format!("objective '{}' is for unknown scenario '{key}'", o.id)));
        }
    }
    for s in &c.scenarios {
        if s.events.new_events_min > s.events.new_events_max {
            return Err(CatalogError::Invalid(format!(
                "scenario '{}': new_events_min is more than new_events_max",
                s.id
            )));
        }
    }
    for tier in [ReliabilityTier::Basic, ReliabilityTier::FlightProven] {
        if c.tier(tier).is_none() {
            return Err(CatalogError::Invalid(format!("reliability tier {tier:?} is missing")));
        }
    }
    Ok(())
}
