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

/// Ticket 05: one line per fired event with an `only` condition, a `more_likely` whose fact held, or a
/// mitigation: which choice caused or softened it (`DesignFact::phrase`).
#[allow(unused_variables)]
pub(crate) fn design_notes(pool: &EventState) -> Vec<String> {
    Vec::new()
}
