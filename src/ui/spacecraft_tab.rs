//! Spacecraft Role tab: Structure, solar panels, battery, attitude control, computer Reliability Tier.
use leptos::prelude::*;
use spaceooops::sim::ReliabilityTier;

use super::format::{money, num};
use super::option_card::OptionCard;
use super::state::{AppState, Slot};
use super::tier_select::TierSelect;

/// (id, name, why, stats) of one option card.
pub type CardData = (String, String, String, Vec<(&'static str, String)>);

/// A row of option cards for one tiered Component slot, followed by its tier chooser.
pub fn slot_section(state: AppState, title: &'static str, slot: Slot, options: Vec<CardData>) -> impl IntoView {
    view! {
        <h3>{title} <super::info_tip::InfoTip term=title /></h3>
        <div class="option-grid">
            {options
                .into_iter()
                .map(|(id, name, why, stats)| {
                    let id_sel = id.clone();
                    let id_clash = id.clone();
                    view! {
                        <OptionCard
                            name=name
                            why=why
                            stats=stats
                            selected=Signal::derive(move || {
                                state.design.with(|d| slot.get(d).as_ref().is_some_and(|p| p.id == id_sel))
                            })
                            incompatible=Signal::derive(move || state.incompatible_reason(&id_clash))
                            on_select=Callback::new(move |_| {
                                state.design.update(|d| {
                                    if slot.get(d).as_ref().is_some_and(|p| p.id == id) {
                                        *slot.get_mut(d) = None;
                                    } else {
                                        slot.choose(d, &id);
                                    }
                                })
                            })
                        />
                    }
                })
                .collect_view()}
        </div>
        <TierSelect
            current=Signal::derive(move || state.design.with(|d| slot.get(d).as_ref().map(|p| p.tier)))
            on_change=Callback::new(move |tier: ReliabilityTier| {
                state.design.update(|d| {
                    if let Some(p) = slot.get_mut(d) {
                        p.tier = tier;
                    }
                })
            })
        />
    }
}

#[component]
pub fn SpacecraftTab() -> impl IntoView {
    let state = expect_context::<AppState>();
    let (structures, solar, batteries, attitude, computer) = state.catalog.with_value(|c| {
        (
            c.structures.clone(),
            c.solar_panels.clone(),
            c.batteries.clone(),
            c.attitude_controls.clone(),
            c.computer.clone(),
        )
    });

    let solar = solar
        .into_iter()
        .map(|o| {
            let stats = vec![
                ("Mass", format!("{} kg", num(o.mass_kg))),
                ("Power", format!("+{} W per U in sunlight", num(o.generation_w_per_u))),
                ("Cost", money(o.cost_usd)),
                ("Data", "none".to_string()),
                ("Volume", format!("{}U", num(o.volume_u))),
            ];
            (o.id, o.name, o.why, stats)
        })
        .collect();
    let batteries = batteries
        .into_iter()
        .map(|o| {
            let stats = vec![
                ("Mass", format!("{} kg", num(o.mass_kg))),
                ("Power", format!("stores {} Wh", num(o.capacity_wh))),
                ("Cost", money(o.cost_usd)),
                ("Data", "none".to_string()),
                ("Volume", format!("{}U", num(o.volume_u))),
            ];
            (o.id, o.name, o.why, stats)
        })
        .collect();
    let attitude = attitude
        .into_iter()
        .map(|o| {
            let stats = vec![
                ("Mass", format!("{} kg", num(o.mass_kg))),
                ("Power", format!("{} W", num(o.power_w))),
                ("Cost", money(o.cost_usd)),
                ("Data", "none".to_string()),
                ("Volume", format!("{}U", num(o.volume_u))),
                ("Pointing", if o.fine_pointing { "Precise".to_string() } else { "Rough".to_string() }),
            ];
            (o.id, o.name, o.why, stats)
        })
        .collect();

    view! {
        <section class="tab-panel">
            <h3>"Structure" <super::info_tip::InfoTip term="Structure" /></h3>
            <div class="option-grid">
                {structures
                    .into_iter()
                    .map(|o| {
                        let id = o.id.clone();
                        let id_sel = id.clone();
                        let id_clash = id.clone();
                        let stats = vec![
                            ("Mass", format!("{} kg", num(o.mass_kg))),
                            ("Power", "none".to_string()),
                            ("Cost", money(o.cost_usd)),
                            ("Data", "none".to_string()),
                            ("Room inside", format!("{}U", num(o.usable_volume_u))),
                        ];
                        view! {
                            <OptionCard
                                name=o.name.clone()
                                why=o.why.clone()
                                stats=stats
                                selected=Signal::derive(move || state.design.with(|d| d.structure.as_deref() == Some(id_sel.as_str())))
                                incompatible=Signal::derive(move || state.incompatible_reason(&id_clash))
                                on_select=Callback::new(move |_| state.design.update(|d| d.structure = Some(id.clone())))
                            />
                        }
                    })
                    .collect_view()}
            </div>
            {slot_section(state, "Solar panels", Slot::Solar, solar)}
            {slot_section(state, "Battery", Slot::Battery, batteries)}
            {slot_section(state, "Attitude control", Slot::Attitude, attitude)}
            <h3>{computer.name.clone()} <super::info_tip::InfoTip term=computer.name.clone() /></h3>
            <p class="muted" title=computer.why.clone()>
                {computer.why.clone()} " "
                {format!("Mass {} kg \u{00B7} Power {} W \u{00B7} Cost {} \u{00B7} Storage {} MB", num(computer.mass_kg), num(computer.power_w), money(computer.cost_usd), num(computer.storage_mb))}
            </p>
            <TierSelect
                current=Signal::derive(move || Some(state.design.with(|d| d.computer_tier)))
                on_change=Callback::new(move |tier: ReliabilityTier| state.design.update(|d| d.computer_tier = tier))
            />
        </section>
    }
}
