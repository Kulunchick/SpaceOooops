//! What the Debrief says about the new Random Events that happened: the notes on their questions, the
//! real-life line of every real event, and which Design choices caused or softened one.
use super::event_effects::EventState;

/// Ticket 03 (04 for gambles): one note per question answered or defaulted, with its day:
/// `Day N, {title}: you chose "{label}". {note}.`
pub(crate) fn question_notes(pool: &EventState) -> Vec<(u32, String)> {
    pool.fired
        .iter()
        .filter_map(|f| {
            let q = f.spec.question.as_ref()?;
            let o = q.options.get(f.option?)?;
            let note = match (&o.bad, f.good) {
                (Some(bad), false) => format!("The gamble failed: {}", bad.note.trim()),
                (Some(_), true) => format!("The gamble paid off: {}", o.note.trim()),
                _ => sentence(&o.note),
            };
            let who = if f.by_player { "you chose" } else { "no answer was given, so by default the spacecraft chose" };
            Some((f.day, format!("Day {}, {}: {who} \"{}\". {}.", f.day, f.spec.title, o.label, note.trim_end_matches('.'))))
        })
        .collect()
}

/// The note as a sentence: no full stop of its own, a capital first letter.
fn sentence(note: &str) -> String {
    let note = note.trim().trim_end_matches('.');
    let mut chars = note.chars();
    chars.next().map_or(String::new(), |c| c.to_uppercase().chain(chars).collect())
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
