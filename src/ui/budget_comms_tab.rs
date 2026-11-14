//! Budget & Comms Role tab: comms band and the cost breakdown.
use leptos::prelude::*;

use super::cost_table::CostTable;
use super::format::{money, num};
use super::spacecraft_tab::slot_section;
use super::state::{AppState, Slot};

#[component]
pub fn BudgetCommsTab() -> impl IntoView {
    let state = expect_context::<AppState>();
    let comms = state.catalog.with_value(|c| c.comms.clone());
    let comms = comms
        .into_iter()
        .map(|o| {
            let stats = vec![
                ("Mass", format!("{} kg", num(o.mass_kg))),
                ("Power", format!("{} W", num(o.power_w))),
                ("Cost", money(o.cost_usd)),
                ("Data", format!("{} Mbps downlink", num(o.downlink_mbps))),
                ("Ground stations", format!("{} per day", money(o.ops_cost_usd_per_day))),
                ("Volume", format!("{}U", num(o.volume_u))),
            ];
            (o.id, o.name, o.why, stats)
        })
        .collect();
    view! {
        <section class="tab-panel">
            {slot_section(state, "Comms band", Slot::Comms, comms)}
            <h3>"Cost breakdown" <super::info_tip::InfoTip term="Cost breakdown" /></h3>
            <CostTable />
        </section>
    }
}
