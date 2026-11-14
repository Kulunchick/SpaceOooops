//! Share links: a compact, URL-safe text form of a Design plus its Scenario id.
//!
//! Format (version 1): `v1.<scenario>.<instruments>.<structure>.<solar>.<battery>.<attitude>.<comms>.<computer tier>.<orbit>.<launch vehicle>`
//! Fields are separated by `.`; instrument ids by `_`; a Pick is `<id>~<b|f>` (Basic / Flight-proven);
//! an empty field means "not chosen". Catalog ids are kebab-case, so none of these separators can clash.
//! Decoding validates every id against the catalog and never panics.
use super::catalog::Catalog;
use super::design::{Design, Pick, ReliabilityTier};

const VERSION: &str = "v1";
const FIELDS: usize = 11;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShareError {
    /// Not a share link at all, or cut short.
    Malformed,
    /// A link made by a different version of the game.
    UnsupportedVersion,
    /// The link names an option or Scenario this catalog does not have.
    UnknownId(String),
}

impl std::fmt::Display for ShareError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShareError::Malformed => write!(f, "the link is damaged or incomplete"),
            ShareError::UnsupportedVersion => write!(f, "the link was made by another version of the game"),
            ShareError::UnknownId(id) => write!(f, "the link mentions \"{id}\", which this version does not know"),
        }
    }
}

fn tier_char(t: ReliabilityTier) -> char {
    match t {
        ReliabilityTier::Basic => 'b',
        ReliabilityTier::FlightProven => 'f',
    }
}

fn pick_text(p: &Option<Pick>) -> String {
    p.as_ref().map_or(String::new(), |p| format!("{}~{}", p.id, tier_char(p.tier)))
}

/// URL-safe text for the Design and the Scenario it was made for.
pub fn encode_design(design: &Design, scenario_id: &str) -> String {
    let opt = |o: &Option<String>| o.clone().unwrap_or_default();
    [
        VERSION.to_string(),
        scenario_id.to_string(),
        design.instruments.join("_"),
        opt(&design.structure),
        pick_text(&design.solar),
        pick_text(&design.battery),
        pick_text(&design.attitude),
        pick_text(&design.comms),
        tier_char(design.computer_tier).to_string(),
        opt(&design.orbit),
        opt(&design.launch_vehicle),
    ]
    .join(".")
}

fn parse_tier(s: &str) -> Result<ReliabilityTier, ShareError> {
    match s {
        "b" => Ok(ReliabilityTier::Basic),
        "f" => Ok(ReliabilityTier::FlightProven),
        _ => Err(ShareError::Malformed),
    }
}

fn check(id: &str, known: impl Fn(&str) -> bool) -> Result<(), ShareError> {
    let shaped = !id.is_empty() && id.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-');
    if !shaped {
        return Err(ShareError::Malformed);
    }
    if known(id) { Ok(()) } else { Err(ShareError::UnknownId(id.to_string())) }
}

fn parse_pick(s: &str, known: impl Fn(&str) -> bool) -> Result<Option<Pick>, ShareError> {
    if s.is_empty() {
        return Ok(None);
    }
    let (id, tier) = s.split_once('~').ok_or(ShareError::Malformed)?;
    check(id, known)?;
    Ok(Some(Pick { id: id.to_string(), tier: parse_tier(tier)? }))
}

fn parse_opt(s: &str, known: impl Fn(&str) -> bool) -> Result<Option<String>, ShareError> {
    if s.is_empty() {
        return Ok(None);
    }
    check(s, known)?;
    Ok(Some(s.to_string()))
}

/// Reads text made by `encode_design`, checking every id against the catalog. Returns the Design and Scenario id.
pub fn decode_design(text: &str, catalog: &Catalog) -> Result<(Design, String), ShareError> {
    let parts: Vec<&str> = text.trim().split('.').collect();
    let version = parts[0];
    if version.len() < 2 || !version.starts_with('v') || !version[1..].chars().all(|c| c.is_ascii_digit()) {
        return Err(ShareError::Malformed);
    }
    if version != VERSION {
        return Err(ShareError::UnsupportedVersion);
    }
    if parts.len() != FIELDS {
        return Err(ShareError::Malformed);
    }
    check(parts[1], |id| catalog.scenario(id).is_some())?;
    let instruments = if parts[2].is_empty() {
        Vec::new()
    } else {
        parts[2]
            .split('_')
            .map(|id| check(id, |i| catalog.instrument(i).is_some()).map(|_| id.to_string()))
            .collect::<Result<_, _>>()?
    };
    let design = Design {
        instruments,
        structure: parse_opt(parts[3], |id| catalog.structure(id).is_some())?,
        solar: parse_pick(parts[4], |id| catalog.solar_panel(id).is_some())?,
        battery: parse_pick(parts[5], |id| catalog.battery(id).is_some())?,
        attitude: parse_pick(parts[6], |id| catalog.attitude_control(id).is_some())?,
        comms: parse_pick(parts[7], |id| catalog.comms_option(id).is_some())?,
        computer_tier: parse_tier(parts[8])?,
        orbit: parse_opt(parts[9], |id| catalog.orbit(id).is_some())?,
        launch_vehicle: parse_opt(parts[10], |id| catalog.launch_vehicle(id).is_some())?,
    };
    Ok((design, parts[1].to_string()))
}
