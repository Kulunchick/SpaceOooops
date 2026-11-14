//! Science Role tab: pick one or more Instruments.
use leptos::prelude::*;

use super::format::{money, num, percent};
use super::option_card::OptionCard;
use super::state::AppState;

#[component]
pub fn ScienceTab() -> impl IntoView {
    let state = expect_context::<AppState>();
    let scenario_id = state.scenario_id.get_untracked();
    let instruments = state.catalog.with_value(|c| c.instruments.clone());
    view! {
        <section class="tab-panel">
            <h3>"Instruments" <super::info_tip::InfoTip term="Instruments" /></h3>
            <p class="muted">"Pick one or more. Each one adds Mass, Power, cost and Data."</p>
            <div class="option-grid">
                {instruments
                    .into_iter()
                    .map(|i| {
                        let id = i.id.clone();
                        let id_sel = id.clone();
                        let id_clash = id.clone();
                        let fit = i.fit.get(&scenario_id).copied().unwrap_or(0.0);
                        let stats = vec![
                            ("Mass", format!("{} kg", num(i.mass_kg))),
                            ("Power", format!("{} W", num(i.power_w))),
                            ("Cost", money(i.cost_usd)),
                            ("Data", format!("{} MB/day", num(i.data_mb_per_day))),
                            ("Volume", format!("{}U", num(i.volume_u))),
                            ("Scenario fit", percent(fit)),
                            ("Pointing", if i.needs_fine_pointing { "Precise".to_string() } else { "Any".to_string() }),
                        ];
                        view! {
                            <OptionCard
                                name=i.name.clone()
                                why=i.why.clone()
                                stats=stats
                                selected=Signal::derive(move || state.design.with(|d| d.instruments.contains(&id_sel)))
                                incompatible=Signal::derive(move || state.incompatible_reason(&id_clash))
                                on_select=Callback::new(move |_| {
                                    state.design.update(|d| {
                                        if let Some(pos) = d.instruments.iter().position(|x| *x == id) {
                                            d.instruments.remove(pos);
                                        } else {
                                            d.instruments.push(id.clone());
                                        }
                                    })
                                })
                            />
                        }
                    })
                    .collect_view()}
            </div>
        </section>
    }
}
