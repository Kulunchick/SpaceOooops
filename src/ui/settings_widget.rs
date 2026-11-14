//! The settings control in the corner of every screen: music volume, effects volume and mute, remembered
//! between visits in local storage (same style as the run history store). Later settings (the quality
//! level) add a row inside `.settings-panel`.
use leptos::prelude::*;
use spaceooops::settings::Settings;

use super::{audio_bridge, storage};

const KEY: &str = "spaceooops.settings.v1";

fn load() -> Settings {
    storage::read(KEY).map(|t| Settings::from_text(&t)).unwrap_or_default()
}

fn save(settings: &Settings) {
    storage::write(KEY, &settings.to_text());
}

fn percent(v: f64) -> u32 {
    (v * 100.0).round() as u32
}

#[component]
pub fn SettingsWidget() -> impl IntoView {
    let settings = RwSignal::new(load());
    let open = RwSignal::new(false);

    Effect::new(move |_| {
        let s = settings.get();
        audio_bridge::audio_set_volumes(s.music, s.effects, s.muted);
        save(&s);
    });

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
            </div>
        </div>
    }
}
