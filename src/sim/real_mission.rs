//! The Real-Mission Match shown in the Debrief: the real mission in `data/real_missions.toml` most like the
//! player's, by objective first, then Orbit class, then spacecraft size, with a line saying why.
use serde::{Deserialize, Serialize};

use super::catalog::{Catalog, MatchTerms, RealMission};
use super::debrief::DebriefContext;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RealMissionMatch {
    pub name: String,
    pub operator: String,
    pub year: u32,
    /// One factual sentence about the mission.
    pub blurb: String,
    /// Public page the facts come from.
    pub url: String,
    /// Short description of its size, e.g. "6U CubeSat".
    pub size_note: String,
    /// One line on why this mission was picked, comparing it with the player's Design.
    pub why: String,
}

/// What the player's Design is, in the terms real missions are described in.
struct Profile<'a> {
    purposes: &'a [String],
    orbit_class: &'a str,
    spacecraft_class: &'a str,
    size_u: f64,
    mass_kg: f64,
}

/// The real mission most like this one, or `None` when the catalog has none or the Scenario is unknown.
pub(crate) fn real_mission_match(ctx: &DebriefContext) -> Option<RealMissionMatch> {
    let c = ctx.catalog;
    let terms = &c.match_terms;
    let structure = ctx.design.structure.as_deref().and_then(|id| c.structure(id));
    let profile = Profile {
        purposes: terms.scenario_purposes.get(ctx.scenario_id).map_or(&[], |p| p.as_slice()),
        orbit_class: ctx.design.orbit.as_deref().and_then(|id| c.orbit(id)).map_or("", |o| o.orbit_class.as_str()),
        spacecraft_class: structure.map_or("", |s| s.spacecraft_class.as_str()),
        size_u: structure.map_or(0.0, |s| s.size_u),
        mass_kg: ctx.stats.total_mass_kg,
    };
    if profile.purposes.is_empty() {
        return None;
    }
    let best = c.real_missions.iter().fold(None::<(&RealMission, Rank)>, |best, m| {
        let r = rank(c, &profile, m);
        match best {
            Some((_, b)) if !r.better_than(&b) => best,
            _ => Some((m, r)),
        }
    })?;
    let m = best.0;
    Some(RealMissionMatch {
        name: m.name.clone(),
        operator: m.operator.clone(),
        year: m.year,
        blurb: m.blurb.clone(),
        url: m.url.clone(),
        size_note: m.size_note.clone(),
        why: why(terms, &profile, m),
    })
}

/// How close a mission is; compared field by field, in this order.
#[derive(Clone, Copy)]
struct Rank {
    shared_purposes: usize,
    same_orbit: bool,
    /// Steps apart on the spacecraft-class ladder (smaller is closer).
    class_steps: u32,
    /// How far apart in size, as |ln(ratio)| of CubeSat units or of Mass (smaller is closer).
    size_gap: f64,
}

impl Rank {
    fn better_than(&self, o: &Rank) -> bool {
        (self.shared_purposes, self.same_orbit, std::cmp::Reverse(self.class_steps))
            .cmp(&(o.shared_purposes, o.same_orbit, std::cmp::Reverse(o.class_steps)))
            .then(o.size_gap.total_cmp(&self.size_gap))
            .is_gt()
    }
}

fn rank(c: &Catalog, p: &Profile, m: &RealMission) -> Rank {
    let class_rank = |id: &str| c.match_terms.spacecraft_classes.get(id).map(|s| s.rank);
    let class_steps = match (class_rank(p.spacecraft_class), class_rank(&m.spacecraft_class)) {
        (Some(a), Some(b)) => a.abs_diff(b),
        _ => u32::MAX,
    };
    let gap = |a: f64, b: f64| if a > 0.0 && b > 0.0 { (a / b).ln().abs() } else { f64::INFINITY };
    let size_gap = match m.size_u {
        Some(u) if p.spacecraft_class == m.spacecraft_class => gap(p.size_u, u),
        _ => m.mass_kg.map_or(f64::INFINITY, |kg| gap(p.mass_kg, kg)),
    };
    Rank {
        shared_purposes: m.purposes.iter().filter(|t| p.purposes.contains(t)).count(),
        same_orbit: !p.orbit_class.is_empty() && p.orbit_class == m.orbit_class,
        class_steps,
        size_gap,
    }
}

/// "Same kind of objective (farm monitoring), the same kind of Orbit (sun-synchronous orbit), and it is also a CubeSat."
fn why(terms: &MatchTerms, p: &Profile, m: &RealMission) -> String {
    let word = |t: &String| terms.purpose_words.get(t).cloned().unwrap_or_else(|| t.replace('-', " "));
    let shared: Vec<String> = m.purposes.iter().filter(|t| p.purposes.contains(t)).map(word).collect();
    let objective = if shared.is_empty() {
        "The closest real mission we know of, though its objective differs".to_string()
    } else {
        format!("The same kind of objective as yours ({})", shared.join(" and "))
    };
    let orbit_name = |id: &str| terms.orbit_classes.get(id).cloned().unwrap_or_else(|| id.to_string());
    let orbit = if p.orbit_class == m.orbit_class {
        format!("from the same kind of Orbit ({})", orbit_name(&m.orbit_class))
    } else {
        format!("but from {} rather than {}", orbit_name(&m.orbit_class), orbit_name(p.orbit_class))
    };
    let class_name = |id: &str| terms.spacecraft_classes.get(id).map_or_else(|| id.to_string(), |s| s.name.clone());
    let size = if p.spacecraft_class == m.spacecraft_class {
        format!("and it is also {} ({})", class_name(&m.spacecraft_class), m.size_note)
    } else {
        format!("and it is {} ({}), not {}", class_name(&m.spacecraft_class), m.size_note, class_name(p.spacecraft_class))
    };
    format!("{objective}, {orbit}, {size}.")
}
