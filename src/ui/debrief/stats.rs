//! Key numbers of the Run, straight from `RunStats`.
use leptos::prelude::*;

use crate::ui::format::{data_volume, money, num, percent};
use crate::ui::info_tip::InfoTip;
use crate::ui::state::AppState;

#[component]
pub fn StatsSection() -> impl IntoView {
    let state = expect_context::<AppState>();
    let stats = move || state.run_result.with(|r| r.as_ref().map(|r| r.debrief.stats.clone()));
    view! {
        <div class="panel">
            <h3>"Key numbers"</h3>
            {move || stats().map(|s| {
                let science_ok = s.science_met;
                let lifetime_ok = s.lifetime_met;
                let lost_any = s.lost_data_fraction > 0.0;
                view! {
                    <dl class="stats">
                        <dt>"Science Value" <InfoTip term="Science Value" /></dt>
                        <dd class:ok=science_ok class:bad=!science_ok>
                            {format!("{:.1} of {} required", s.science_value, num(s.required_science_value))}
                        </dd>
                        <dt>"Lifetime" <InfoTip term="Lifetime" /></dt>
                        <dd class:ok=lifetime_ok class:bad=!lifetime_ok>
                            {format!("{} of {} days", s.days_survived, s.min_lifetime_days)}
                        </dd>
                        <dt>"Data delivered" <InfoTip term="Data delivered" /></dt>
                        <dd>{data_volume(s.delivered_data_mb)}</dd>
                        <dt>"Data lost" <InfoTip term="Data lost" /></dt>
                        <dd class:bad=lost_any>{percent(s.lost_data_fraction)}</dd>
                        <dt>"Mission cost" <InfoTip term="Mission cost" /></dt>
                        <dd>{money(s.total_cost_usd)}</dd>
                        <dt>"Mass" <InfoTip term="Mass" /></dt>
                        <dd>{format!("{:.1} kg", s.total_mass_kg)}</dd>
                    </dl>
                }
            })}
        </div>
    }
}
