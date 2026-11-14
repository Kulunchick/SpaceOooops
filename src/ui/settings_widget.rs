//! The settings control in the corner of every screen: music volume, effects volume, mute and the Graphics
//! level (Low/High), remembered between visits in local storage (same style as the run history store).
//! Further settings add a row inside `.settings-panel`.
use leptos::prelude::*;
use spaceooops::settings::{Graphics, Motion, Settings};

use super::{audio_bridge, scene_bridge, storage};

const KEY: &str = "spaceooops.settings.v1";

fn load() -> Settings {
    storage::read(KEY).map(|t| Settings::from_text(&t)).unwrap_or_default()
}

fn save(settings: &Settings) {
    storage::write(KEY, &settings.to_text());
}

/// The graphics level in force at start-up: the saved choice, else the device default (see `Graphics::choose`).
/// Computed once per page load and shared: reading the device hints creates a throwaway WebGL context.
pub fn initial_graphics() -> Graphics {
    thread_local! {
        static INITIAL: std::cell::OnceCell<Graphics> = const { std::cell::OnceCell::new() };
    }
    INITIAL.with(|cell| *cell.get_or_init(|| Graphics::choose(load().graphics, &scene_bridge::device_hints())))
}

/// The Motion choice at start-up: the saved one, else Full (the OS preference is not consulted).
pub fn initial_motion() -> Motion {
    load().motion
}

fn percent(v: f64) -> u32 {
    (v * 100.0).round() as u32
}

#[component]
pub fn SettingsWidget() -> impl IntoView {
    let settings = RwSignal::new(load());
    let open = RwSignal::new(false);
    // The level shown and drawn: the saved choice or the device default. Choosing one saves it.
    let graphics = RwSignal::new(initial_graphics());

    Effect::new(move |_| {
        let s = settings.get();
        audio_bridge::audio_set_volumes(s.music, s.effects, s.muted);
        scene_bridge::set_motion(s.motion);
        save(&s);
    });
    Effect::new(move |_| scene_bridge::set_graphics(graphics.get()));

    let graphics_button = move |label: &'static str, id: &'static str, level: Graphics| {
        view! {
            <button
                id=id
                type="button"
                class="lp-btn settings-quality-option"
                class:selected=move || graphics.get() == level
                aria-pressed=move || (graphics.get() == level).to_string()
                on:click=move |_| {
                    graphics.set(level);
                    settings.update(|s| s.graphics = Some(level));
                }
            >
                {label}
            </button>
        }
    };

    let motion_button = move |label: &'static str, id: &'static str, choice: Motion| {
        view! {
            <button
                id=id
                type="button"
                class="lp-btn settings-quality-option"
                class:selected=move || settings.get().motion == choice
                aria-pressed=move || (settings.get().motion == choice).to_string()
                on:click=move |_| settings.update(|s| s.motion = choice)
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
                aria-label="Sound and graphics settings"
                aria-expanded=move || open.get().to_string()
                aria-controls="settings-panel"
                on:click=move |_| open.update(|o| *o = !*o)
            >
                {move || if settings.get().muted { "Settings (muted)" } else { "Settings" }}
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
                <div class="settings-row settings-quality" role="group" aria-label="Graphics">
                    <span>"Graphics"</span>
                    <span class="settings-quality-options">
                        {graphics_button("Low", "settings-quality-low", Graphics::Low)}
                        {graphics_button("High", "settings-quality-high", Graphics::High)}
                    </span>
                </div>
                <div class="settings-row settings-quality" role="group" aria-label="Motion">
                    <span>"Motion"</span>
                    <span class="settings-quality-options">
                        {motion_button("Full", "settings-motion-full", Motion::Full)}
                        {motion_button("Reduced", "settings-motion-reduced", Motion::Reduced)}
                    </span>
                </div>
            </div>
        </div>
    }
}
