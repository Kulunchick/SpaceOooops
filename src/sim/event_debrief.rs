//! What the Debrief says about the new Random Events that happened: the notes on their questions, the
//! real-life line of every real event, and which Design choices caused or softened one.
use super::event_effects::EventState;

/// Ticket 03 (04 for gambles): one note per question answered or defaulted, with its day:
/// `Day N, {title}: you chose "{label}". {note}.`
#[allow(unused_variables)]
pub(crate) fn question_notes(pool: &EventState) -> Vec<(u32, String)> {
    Vec::new()
}

/// Ticket 06: `{title}: {real_life}` for each real event that fired, once, in day order.
#[allow(unused_variables)]
pub(crate) fn real_life_lines(pool: &EventState) -> Vec<String> {
    Vec::new()
}

/// One line per fired event with an `only` condition, a `more_likely` whose fact held, or a mitigation that
/// applied: which Design choice caused or softened it.
pub(crate) fn design_notes(pool: &EventState) -> Vec<String> {
    pool.fired
        .iter()
        .filter_map(|f| {
            let mut parts = Vec::new();
            if let Some(fact) = f.spec.only {
                parts.push(format!("it could only happen with {}", fact.phrase()));
            }
            if let Some(m) = f.spec.more_likely.as_ref().filter(|m| pool.facts.contains(&m.fact)) {
                parts.push(format!("it was more likely with {}", m.fact.phrase()));
            }
            if let Some(m) = f.spec.mitigation.as_ref().filter(|_| f.softened) {
                parts.push(format!("it was softened by {}", m.fact.phrase()));
            }
            (!parts.is_empty()).then(|| format!("Day {}, {}: {}.", f.day, f.spec.title, parts.join(", and ")))
        })
        .collect()
}
