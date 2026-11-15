//! What the Debrief says about the new Random Events that happened: the notes on their questions, the
//! real-life line of every real event, and which Design choices caused or softened one.
use super::event_catalog::EventGroup;
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
pub(crate) fn real_life_lines(pool: &EventState) -> Vec<String> {
    let mut seen: Vec<&str> = Vec::new();
    let mut lines = Vec::new();
    for f in &pool.fired {
        let Some(text) = f.spec.real_life.as_ref().filter(|_| f.spec.group == EventGroup::Real) else { continue };
        if seen.contains(&f.spec.id.as_str()) {
            continue;
        }
        seen.push(&f.spec.id);
        lines.push(format!("{}: {}", f.spec.title, text.trim()));
    }
    lines
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
