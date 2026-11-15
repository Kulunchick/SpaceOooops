//! The new Random Events as data (`data/events.toml`): what each one says, which mechanics it uses, what the
//! scene shows, and for an interactive one its question. Parsed and checked here, so a typo fails the catalog
//! tests with the file, the event and the field, and never reaches a player.
use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use super::catalog::CatalogError;
use super::design_facts::DesignFact;

pub(crate) const EVENTS_FILE: &str = "data/events.toml";

/// One new Random Event.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EventSpec {
    /// Kebab-case, unique.
    pub id: String,
    pub group: EventGroup,
    pub title: String,
    /// The event-log sentence of a passive event (unused when the event asks a question).
    #[serde(default)]
    pub log: String,
    /// What the 3D scene shows.
    pub scene: SceneEffect,
    /// A star model for a fly-by or a visitor (ticket 09).
    #[serde(default)]
    pub model: Option<String>,
    /// How this happens in real life, for the Debrief. Required for real events, forbidden for fantasy ones.
    #[serde(default)]
    pub real_life: Option<String>,
    /// The event is possible only when this Design fact holds.
    #[serde(default)]
    pub only: Option<DesignFact>,
    #[serde(default)]
    pub more_likely: Option<MoreLikely>,
    #[serde(default)]
    pub mitigation: Option<Mitigation>,
    /// What a passive event does.
    #[serde(default)]
    pub effects: Vec<EffectSpec>,
    #[serde(default)]
    pub question: Option<QuestionSpec>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum EventGroup {
    Real,
    /// Shown with the "Unexplained" mark.
    Fantasy,
    Lucky,
}

/// The reusable 3D effect an event shows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SceneEffect {
    FlyBy,
    Hit,
    LinkInterference,
    PanelShading,
    BonusGlow,
    Visitor,
}

/// The closed set of things a new event can do to a Run.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Mechanic {
    // ---- timed: for `days` days or to the end of the Run; `strength` is a signed share
    Power,
    InstrumentsOff,
    Downlink,
    Data,
    Storage,
    Pointing,
    BatteryCapacity,
    FailureChance,
    // ---- instant
    /// US$: positive gained, negative spent.
    Budget,
    /// A share of the Scenario's required Science Value.
    ScienceBonus,
    /// One more day's Data (Data +100% for one day).
    ExtraDataDay,
    /// Score points on a line of their own.
    ScoreBonus,
}

impl Mechanic {
    /// A timed mechanic lasts `days` days or to the end of the Run; an instant one acts once.
    pub fn is_timed(self) -> bool {
        !matches!(self, Mechanic::Budget | Mechanic::ScienceBonus | Mechanic::ExtraDataDay | Mechanic::ScoreBonus)
    }

    /// Its `strength` is a share, so it must lie in -1.0..=1.0.
    fn is_share(self) -> bool {
        self.is_timed() || self == Mechanic::ScienceBonus
    }

    /// The name as written in the data file.
    pub fn name(self) -> &'static str {
        match self {
            Mechanic::Power => "power",
            Mechanic::InstrumentsOff => "instruments-off",
            Mechanic::Downlink => "downlink",
            Mechanic::Data => "data",
            Mechanic::Storage => "storage",
            Mechanic::Pointing => "pointing",
            Mechanic::BatteryCapacity => "battery-capacity",
            Mechanic::FailureChance => "failure-chance",
            Mechanic::Budget => "budget",
            Mechanic::ScienceBonus => "science-bonus",
            Mechanic::ExtraDataDay => "extra-data-day",
            Mechanic::ScoreBonus => "score-bonus",
        }
    }
}

/// One thing an event does.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EffectSpec {
    pub mechanic: Mechanic,
    /// Signed share (-0.05 = 5% less), US$ for `budget`, points for `score-bonus`.
    #[serde(default)]
    pub strength: f64,
    #[serde(default)]
    pub days: Option<u32>,
    #[serde(default)]
    pub to_end: bool,
    /// Starts this many days after the event's day.
    #[serde(default)]
    pub delay: u32,
    /// Applies only when this Design fact holds.
    #[serde(default)]
    pub when: Option<DesignFact>,
    /// The Score line's label of a `score-bonus`.
    #[serde(default)]
    pub label: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MoreLikely {
    pub fact: DesignFact,
    /// 2..=4 times as likely to be picked when the fact holds.
    pub weight: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Mitigation {
    pub fact: DesignFact,
    /// Strengths are multiplied by this when the fact holds (0 = nothing happens).
    pub scale: f64,
    /// The event-log sentence when the mitigation applied.
    pub log: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QuestionSpec {
    pub text: String,
    /// The option id taken when the player does not answer; must be free.
    pub default: String,
    pub options: Vec<OptionSpec>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OptionSpec {
    pub id: String,
    pub label: String,
    /// What the option costs and buys; the sim appends the price and, for a gamble, the odds.
    pub consequence: String,
    #[serde(default)]
    pub cost_usd: f64,
    /// The event-log sentence when this option (its good outcome) applied.
    pub log: String,
    /// The Debrief note: what it led to.
    pub note: String,
    #[serde(default)]
    pub effects: Vec<EffectSpec>,
    /// A gamble: probability of the good outcome (the fields above); `bad` is the other one.
    #[serde(default)]
    pub chance: Option<f64>,
    #[serde(default)]
    pub bad: Option<Outcome>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Outcome {
    pub log: String,
    pub note: String,
    #[serde(default)]
    pub effects: Vec<EffectSpec>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct EventsFile {
    #[serde(default)]
    events: Vec<EventSpec>,
}

/// Parses and checks an events file (the shipped one, or a test's own).
pub fn parse_events(text: &str) -> Result<Vec<EventSpec>, CatalogError> {
    let file: EventsFile = toml::from_str(text)
        .map_err(|e| CatalogError::Parse { file: EVENTS_FILE.to_string(), message: e.to_string() })?;
    validate(&file.events).map_err(|m| CatalogError::Invalid(format!("{EVENTS_FILE}: {m}")))?;
    Ok(file.events)
}

/// The ids of the four built-in Random Events, which a new event may not reuse.
const RESERVED_IDS: [&str; 4] = ["solar-flare", "ground-station-outage", "component-failure", "battery-shortfall"];

fn validate(events: &[EventSpec]) -> Result<(), String> {
    let mut seen = BTreeSet::new();
    for e in events {
        let id = &e.id;
        if !is_kebab_case(id) {
            return Err(format!("event id '{id}' must be kebab-case (lower-case words joined by '-')"));
        }
        if RESERVED_IDS.contains(&id.as_str()) {
            return Err(format!("event id '{id}' is taken by a built-in Random Event"));
        }
        if !seen.insert(id.as_str()) {
            return Err(format!("event id '{id}' is used twice"));
        }
        validate_event(e).map_err(|m| format!("event '{id}': {m}"))?;
    }
    validate_pool(events)
}

fn validate_event(e: &EventSpec) -> Result<(), String> {
    if e.title.trim().is_empty() {
        return Err("needs a title".to_string());
    }
    match (e.group, &e.real_life) {
        (EventGroup::Real, None) => return Err("a real event needs a real_life line".to_string()),
        (EventGroup::Fantasy, Some(_)) => return Err("a fantasy event has no real_life line".to_string()),
        _ => {}
    }
    if e.question.is_none() {
        if e.log.trim().is_empty() {
            return Err("a passive event needs log text".to_string());
        }
        if e.effects.is_empty() {
            return Err("a passive event needs at least one effect".to_string());
        }
    }
    for effect in &e.effects {
        validate_effect(effect)?;
    }
    validate_question(e)?;
    validate_gamble(e)?;
    validate_design(e)?;
    validate_model(e)
}

/// Checks one effect (also used for the effects of options and outcomes).
pub(crate) fn validate_effect(effect: &EffectSpec) -> Result<(), String> {
    let name = effect.mechanic.name();
    if effect.mechanic.is_timed() {
        let days = effect.days.is_some_and(|d| d >= 1);
        if effect.days == Some(0) || days == effect.to_end {
            return Err(format!("'{name}' lasts either days >= 1 or to_end = true, exactly one of them"));
        }
    } else if effect.days.is_some() || effect.to_end {
        return Err(format!("'{name}' acts once, so it takes neither days nor to_end"));
    }
    if effect.mechanic.is_share() && !(-1.0..=1.0).contains(&effect.strength) {
        return Err(format!("'{name}' strength {} is a share and must lie in -1.0..=1.0", effect.strength));
    }
    if effect.mechanic == Mechanic::Power && effect.strength > 0.0 {
        return Err(format!(
            "'{name}' may only lower the Power: a Design with spare Power could not use a gain (give more Data instead)"
        ));
    }
    if effect.mechanic == Mechanic::FailureChance && effect.strength > 0.0 {
        return Err(format!("'{name}' may only lower the failure chance, so no new event ends the Mission"));
    }
    if effect.mechanic == Mechanic::ScoreBonus && effect.label.as_ref().is_none_or(|l| l.trim().is_empty()) {
        return Err(format!("'{name}' needs a label for its Score line"));
    }
    Ok(())
}

/// Ticket 03: exactly two options, distinct ids, the default exists and is free, a question XOR effects + log.
fn validate_question(e: &EventSpec) -> Result<(), String> {
    let Some(q) = &e.question else { return Ok(()) };
    if !e.effects.is_empty() {
        return Err("an event with a question has its effects on its options, not on the event".to_string());
    }
    if q.text.trim().is_empty() {
        return Err("the question needs text".to_string());
    }
    if q.options.len() != 2 {
        return Err(format!("a question has exactly two options, not {}", q.options.len()));
    }
    if q.options[0].id == q.options[1].id {
        return Err("the two options need distinct ids".to_string());
    }
    for o in &q.options {
        let id = &o.id;
        if !is_kebab_case(id) {
            return Err(format!("option id '{id}' must be kebab-case"));
        }
        if o.label.trim().is_empty() || o.consequence.trim().is_empty() || o.note.trim().is_empty() {
            return Err(format!("option '{id}' needs a label, a consequence and a note"));
        }
        if o.log.trim().is_empty() {
            return Err(format!("option '{id}' needs log text"));
        }
        if !o.cost_usd.is_finite() || o.cost_usd < 0.0 {
            return Err(format!("option '{id}' must cost a non-negative amount of US$"));
        }
        for effect in &o.effects {
            validate_effect(effect).map_err(|m| format!("option '{id}': {m}"))?;
        }
    }
    match q.options.iter().find(|o| o.id == q.default) {
        None => Err(format!("the default '{}' is not one of the options", q.default)),
        Some(o) if o.cost_usd > 0.0 => Err(format!("the default option '{}' must be free (cost_usd = 0)", o.id)),
        Some(_) => Ok(()),
    }
}

/// Ticket 04: `chance` and `bad` both or neither, chance in 0..=1.
fn validate_gamble(e: &EventSpec) -> Result<(), String> {
    let Some(q) = &e.question else { return Ok(()) };
    for o in &q.options {
        let id = &o.id;
        match (o.chance, &o.bad) {
            (None, None) => {}
            (Some(_), None) => return Err(format!("option '{id}' has a chance but no bad outcome (bad)")),
            (None, Some(_)) => return Err(format!("option '{id}' has a bad outcome but no chance")),
            (Some(chance), Some(bad)) => {
                if !(0.0..=1.0).contains(&chance) {
                    return Err(format!("option '{id}': chance must be between 0 and 1, not {chance}"));
                }
                if bad.log.trim().is_empty() || bad.note.trim().is_empty() {
                    return Err(format!("option '{id}': the bad outcome (bad) needs a log and a note"));
                }
                for effect in &bad.effects {
                    validate_effect(effect).map_err(|m| format!("option '{id}', bad outcome: {m}"))?;
                }
            }
        }
    }
    Ok(())
}

/// Weight 2..=4, scale in 0..1, scale 0 when the event has `instruments-off` (it cannot be scaled).
fn validate_design(e: &EventSpec) -> Result<(), String> {
    if let Some(m) = &e.more_likely
        && !(2..=4).contains(&m.weight)
    {
        return Err(format!("more_likely weight {} must be 2 to 4", m.weight));
    }
    if let Some(m) = &e.mitigation {
        if !(0.0..1.0).contains(&m.scale) {
            return Err(format!("mitigation scale {} must lie in 0 up to (not including) 1", m.scale));
        }
        if m.log.trim().is_empty() {
            return Err("a mitigation needs log text".to_string());
        }
        let options = e.question.iter().flat_map(|q| &q.options);
        let all = e.effects.iter().chain(options.clone().flat_map(|o| &o.effects)).chain(
            options.filter_map(|o| o.bad.as_ref()).flat_map(|b| &b.effects),
        );
        if m.scale != 0.0 && all.into_iter().any(|f| f.mechanic == Mechanic::InstrumentsOff) {
            return Err("a mitigation of an event with instruments-off must have scale = 0.0, as Instruments off cannot be scaled".to_string());
        }
    }
    Ok(())
}

/// Ticket 09: the model is a known name, and only with a fly-by or a visitor.
fn validate_model(e: &EventSpec) -> Result<(), String> {
    let Some(model) = &e.model else { return Ok(()) };
    if !STAR_MODELS.contains(&model.as_str()) {
        return Err(format!("unknown model '{model}' (known models: {})", STAR_MODELS.join(", ")));
    }
    if !matches!(e.scene, SceneEffect::FlyBy | SceneEffect::Visitor) {
        return Err("a model goes only with a fly-by or a visitor scene".to_string());
    }
    Ok(())
}

/// The star models `js/bridge/star_models.js` can draw.
pub const STAR_MODELS: [&str; 4] = ["ufo", "whale", "dragon", "inspector"];

/// Space agencies, matched as whole words.
const DENIED_AGENCIES: [&str; 15] =
    ["NASA", "ESA", "JAXA", "Roscosmos", "ISRO", "CNSA", "CNES", "DLR", "INPE", "CONAE", "KARI", "CSA", "UKSA", "SpaceX", "USSR"];
/// Countries and their adjectives, matched as the start of a word ("Russia" also catches "Russian").
const DENIED_COUNTRIES: [&str; 20] = [
    "Russia", "Soviet", "China", "Chinese", "America", "USA", "Japan", "India", "Brazil", "Angola", "Argentin",
    "France", "French", "German", "Israel", "Iran", "Korea", "Britain", "British", "Ukrain",
];

/// Ticket 07: no event text names a real country or agency (the game jokes about "a rival agency" only). The
/// pool's size and split are pinned by the catalog tests, as a test catalog may hold a single event.
fn validate_pool(events: &[EventSpec]) -> Result<(), String> {
    for e in events {
        for text in texts(e) {
            for word in text.split(|c: char| !c.is_alphanumeric()).filter(|w| !w.is_empty()) {
                let denied = DENIED_AGENCIES.contains(&word) || DENIED_COUNTRIES.iter().any(|c| word.starts_with(c));
                if denied {
                    return Err(format!(
                        "event '{}' names '{word}': event texts name no real country or agency (say \"a rival agency\")",
                        e.id
                    ));
                }
            }
        }
    }
    Ok(())
}

/// Every player-facing text of an event.
fn texts(e: &EventSpec) -> Vec<&str> {
    let mut out = vec![e.title.as_str(), e.log.as_str()];
    out.extend(e.real_life.as_deref());
    out.extend(e.mitigation.as_ref().map(|m| m.log.as_str()));
    let mut effects: Vec<&EffectSpec> = e.effects.iter().collect();
    if let Some(q) = &e.question {
        out.push(q.text.as_str());
        for o in &q.options {
            out.extend([o.label.as_str(), o.consequence.as_str(), o.log.as_str(), o.note.as_str()]);
            effects.extend(&o.effects);
            if let Some(b) = &o.bad {
                out.extend([b.log.as_str(), b.note.as_str()]);
                effects.extend(&b.effects);
            }
        }
    }
    out.extend(effects.into_iter().filter_map(|f| f.label.as_deref()));
    out
}

fn is_kebab_case(id: &str) -> bool {
    !id.is_empty()
        && id.split('-').all(|w| !w.is_empty() && w.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit()))
}
