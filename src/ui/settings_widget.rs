//! The settings control in the corner of every screen: music volume, effects volume and mute, remembered
//! between visits in local storage (same style as the run history store). Later settings (the quality
//! level) add a row inside `.settings-panel`.
use leptos::prelude::*;
use spaceooops::settings::{Quality, Settings};

use super::{audio_bridge, scene_bridge, storage};

const KEY: &str = "spaceooops.settings.v1";

fn load() -> Settings {
    storage::read(KEY).map(|t| Settings::from_text(&t)).unwrap_or_default()
}

fn save(settings: &Settings) {
    storage::write(KEY, &settings.to_text());
}

/// The quality level in force at start-up: the saved choice, else the device default (see `Quality::choose`).
pub fn initial_quality() -> Quality {
    Quality::choose(load().quality, &scene_bridge::device_hints())
}

fn percent(v: f64) -> u32 {
    (v * 100.0).round() as u32
}

#[component]
pub fn SettingsWidget() -> impl IntoView {
    let settings = RwSignal::new(load());
    let open = RwSignal::new(false);
    // The level shown and drawn: the saved choice or the device default. Choosing one saves it.
    let quality = RwSignal::new(initial_quality());

    Effect::new(move |_| {
        let s = settings.get();
        audio_bridge::audio_set_volumes(s.music, s.effects, s.muted);
        save(&s);
    });
    Effect::new(move |_| scene_bridge::set_quality(quality.get()));

    let quality_button = move |label: &'static str, id: &'static str, level: Quality| {
        view! {
            <button
                id=id
                type="button"
                class="lp-btn settings-quality-option"
                class:selected=move || quality.get() == level
                aria-pressed=move || (quality.get() == level).to_string()
                on:click=move |_| {
                    quality.set(level);
                    settings.update(|s| s.quality = Some(level));
                }
            >
                {label}
            </button>
        }
    };

    let slider = move |label: &'static str, id: &'static str, get: fn(&Settings) -> f64, set: fn(&mut Settings, f64)| {
        view! {
            <label class="settings-row" for=id>
                <span>{label}</span>
                <input
                    id=id
                    type="range"
                    min="0"
                    max="100"
                    step="1"
                    prop:value=move || percent(get(&settings.get())).to_string()
                    on:input=move |ev| {
                        if let Ok(v) = event_target_value(&ev).parse::<f64>() {
                            settings.update(|s| set(s, (v / 100.0).clamp(0.0, 1.0)));
                        }
                    }
                />
            </label>
        }
    };

    view! {
        <div class="settings-widget">
            <button
                class="lp-btn settings-toggle"
                type="button"
                aria-label="Sound settings"
                aria-expanded=move || open.get().to_string()
                aria-controls="settings-panel"
                on:click=move |_| open.update(|o| *o = !*o)
            >
                {move || if settings.get().muted { "Sound: off" } else { "Sound" }}
            </button>
            <div class="settings-panel lp-panel" id="settings-panel" hidden=move || !open.get()>
                {slider("Music", "settings-music", |s| s.music, |s, v| s.music = v)}
                {slider("Effects", "settings-effects", |s| s.effects, |s, v| s.effects = v)}
                <label class="settings-row settings-mute">
                    <span>"Mute"</span>
                    <input
                        id="settings-mute"
                        type="checkbox"
                        prop:checked=move || settings.get().muted
                        on:change=move |ev| settings.update(|s| s.muted = event_target_checked(&ev))
                    />
                </label>
                <div class="settings-row settings-quality" role="group" aria-label="Graphics quality">
                    <span>"Quality"</span>
                    <span class="settings-quality-options">
                        {quality_button("Low", "settings-quality-low", Quality::Low)}
                        {quality_button("High", "settings-quality-high", Quality::High)}
                    </span>
                </div>
            </div>
        </div>
    }
}
