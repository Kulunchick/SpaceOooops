//! Change since the previous Run of the same Scenario, from the stored history (newest record = this Run).
//! Each row carries an arrow and a word, so the change never relies on colour alone.
use leptos::prelude::*;

use crate::ui::format::{money, num};
use crate::ui::history_store::HistoryRecord;
use crate::ui::state::AppState;

struct Row {
    label: &'static str,
    now: String,
    before: String,
    delta: f64,
    /// Whether a higher value is an improvement (cost and Mass: lower is better).
    higher_is_better: bool,
}

fn rows(now: &HistoryRecord, prev: &HistoryRecord) -> Vec<Row> {
    let row = |label, now_v: f64, prev_v: f64, fmt: &dyn Fn(f64) -> String, higher_is_better| Row {
        label,
        now: fmt(now_v),
        before: fmt(prev_v),
        delta: now_v - prev_v,
        higher_is_better,
    };
    vec![
        row("Score", now.score, prev.score, &|v| num(v.round()), true),
        row("Science Value", now.science_value, prev.science_value, &|v| format!("{v:.1}"), true),
        row("Mission cost", now.cost_usd, prev.cost_usd, &money, false),
        row("Mass", now.mass_kg, prev.mass_kg, &|v| format!("{v:.1} kg"), false),
        row("Lifetime", f64::from(now.days_survived), f64::from(prev.days_survived), &|v| format!("{v:.0} days"), true),
    ]
}

fn view_row(r: Row) -> impl IntoView {
    let same = r.delta.abs() < 1e-6;
    let (arrow, word) = if same {
        ("\u{2192}", "same")
    } else if r.delta > 0.0 {
        ("\u{2191}", "up")
    } else {
        ("\u{2193}", "down")
    };
    let better = !same && (r.delta > 0.0) == r.higher_is_better;
    view! {
        <tr>
            <th scope="row">{r.label}</th>
            <td class="num">{r.before}</td>
            <td class="num">{r.now}</td>
            <td class="num" class:ok=better class:bad=!same && !better>
                <span aria-hidden="true">{arrow}</span>
                " " {word}
            </td>
        </tr>
    }
}

#[component]
pub fn ComparisonSection() -> impl IntoView {
    let state = expect_context::<AppState>();
    let pair = move || {
        state.history.with(|h| {
            let now = h.first()?;
            let prev = h[1..].iter().find(|r| r.scenario_id == now.scenario_id)?;
            Some((now.clone(), prev.clone()))
        })
    };
    view! {
        <div class="panel compare">
            <h3>"Compared with your previous Run"</h3>
            {move || match pair() {
                None => view! { <p class="muted">"No earlier Run of this Scenario to compare with."</p> }.into_any(),
                Some((now, prev)) => view! {
                    <table class="compare-table">
                        <thead><tr><th></th><th>"Previous"</th><th>"This Run"</th><th>"Change"</th></tr></thead>
                        <tbody>{rows(&now, &prev).into_iter().map(view_row).collect_view()}</tbody>
                    </table>
                }.into_any(),
            }}
        </div>
    }
}
