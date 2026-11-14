//! Launch & Orbit Role tab: Orbit and Launch Vehicle.
use leptos::prelude::*;

use super::format::{money, num, percent};
use super::option_card::OptionCard;
use super::state::AppState;

#[component]
pub fn LaunchOrbitTab() -> impl IntoView {
    let state = expect_context::<AppState>();
    let scenario_id = state.scenario_id.get_untracked();
    let (orbits, vehicles) = state.catalog.with_value(|c| (c.orbits.clone(), c.launch_vehicles.clone()));
    let orbit_names: Vec<(String, String)> = orbits.iter().map(|o| (o.id.clone(), o.name.clone())).collect();

    view! {
        <section class="tab-panel">
            <h3>"Orbit"</h3>
            <div class="option-grid">
                {orbits
                    .into_iter()
                    .map(|o| {
                        let id = o.id.clone();
                        let id_sel = id.clone();
                        let id_clash = id.clone();
                        let coverage = o.coverage.get(&scenario_id).copied().unwrap_or(0.0);
                        let stats = vec![
                            ("Altitude", format!("{} km", num(o.altitude_km))),
                            ("Time in shadow", format!("{} of each orbit ({} min)", percent(o.eclipse_fraction), num(o.period_min * o.eclipse_fraction))),
                            ("Ground contact", format!("{} min/day", num(o.contact_minutes_per_day))),
                            ("Lifetime limit", o.lifetime_cap_days.map_or("none".to_string(), |d| format!("{} days", num(d)))),
                            ("Scenario coverage", percent(coverage)),
                        ];
                        view! {
                            <OptionCard
                                name=o.name.clone()
                                why=o.why.clone()
                                stats=stats
                                selected=Signal::derive(move || state.design.with(|d| d.orbit.as_deref() == Some(id_sel.as_str())))
                                incompatible=Signal::derive(move || state.incompatible_reason(&id_clash))
                                on_select=Callback::new(move |_| state.design.update(|d| d.orbit = Some(id.clone())))
                            />
                        }
                    })
                    .collect_view()}
            </div>
            <h3>"Launch Vehicle"</h3>
            <div class="option-grid">
                {vehicles
                    .into_iter()
                    .map(|lv| {
                        let id = lv.id.clone();
                        let id_sel = id.clone();
                        let id_clash = id.clone();
                        let capacity = orbit_names
                            .iter()
                            .filter_map(|(oid, oname)| lv.capacity_kg.get(oid).map(|kg| format!("{} kg to {}", num(*kg), oname)))
                            .collect::<Vec<_>>()
                            .join("; ");
                        let stats = vec![
                            ("Cost", money(lv.cost_usd)),
                            ("Capacity", capacity),
                            ("Real example", lv.real_example.clone()),
                        ];
                        let why = format!("{} ({})", lv.why, lv.real_example);
                        view! {
                            <OptionCard
                                name=lv.name.clone()
                                why=why
                                stats=stats
                                selected=Signal::derive(move || state.design.with(|d| d.launch_vehicle.as_deref() == Some(id_sel.as_str())))
                                incompatible=Signal::derive(move || state.incompatible_reason(&id_clash))
                                on_select=Callback::new(move |_| state.design.update(|d| d.launch_vehicle = Some(id.clone())))
                            />
                        }
                    })
                    .collect_view()}
            </div>
        </section>
    }
}
