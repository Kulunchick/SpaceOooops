//! Player settings: the text kept in local storage (low-poly ticket 04).
use spaceooops::settings::Settings;

#[test]
fn defaults_keep_music_well_below_effects_and_are_not_muted() {
    let s = Settings::default();
    assert!(s.music < s.effects);
    assert!(!s.muted);
}

#[test]
fn settings_survive_a_round_trip_through_text() {
    let s = Settings { music: 0.25, effects: 0.8, muted: true, graphics: None };
    assert_eq!(Settings::from_text(&s.to_text()), s);
}

#[test]
fn empty_or_corrupt_text_gives_the_defaults() {
    assert_eq!(Settings::from_text(""), Settings::default());
    assert_eq!(Settings::from_text("garbage\n=\nmusic=abc"), Settings::default());
}

#[test]
fn volumes_are_clamped_to_zero_to_one_and_nan_is_ignored() {
    let s = Settings::from_text("music=7\neffects=-2\nmuted=1");
    assert_eq!((s.music, s.effects, s.muted), (1.0, 0.0, true));
    assert_eq!(Settings::from_text("music=NaN").music, Settings::default().music);
}

#[test]
fn unknown_keys_are_ignored_so_later_settings_can_share_the_text() {
    let s = Settings::from_text("shiny=yes\nmusic=0.5");
    assert_eq!(s.music, 0.5);
}

// ---- Graphics (low-poly ticket 10) ----
use spaceooops::settings::{DeviceHints, Graphics};

fn strong() -> DeviceHints {
    DeviceHints { cores: Some(8), memory_gb: Some(8.0), renderer: Some("ANGLE (NVIDIA GeForce RTX 3060)".into()) }
}

#[test]
fn quality_is_remembered_and_unset_by_default() {
    assert_eq!(Settings::default().graphics, None);
    let s = Settings { graphics: Some(Graphics::Low), ..Settings::default() };
    assert_eq!(Settings::from_text(&s.to_text()).graphics, Some(Graphics::Low));
    let s = Settings { graphics: Some(Graphics::High), ..Settings::default() };
    assert_eq!(Settings::from_text(&s.to_text()).graphics, Some(Graphics::High));
    // No choice made: nothing is written, so a later change of the device rule still applies.
    assert!(!Settings::default().to_text().contains("quality"));
}

#[test]
fn an_unreadable_quality_means_no_choice() {
    assert_eq!(Settings::from_text("quality=ultra").graphics, None);
    assert_eq!(Settings::from_text("quality=").graphics, None);
}

#[test]
fn text_stored_before_the_quality_setting_still_loads() {
    let s = Settings::from_text("music=0.5\neffects=0.6\nmuted=1");
    assert_eq!((s.music, s.effects, s.muted, s.graphics), (0.5, 0.6, true, None));
}

#[test]
fn a_capable_device_defaults_to_high() {
    assert_eq!(Graphics::default_for(&strong()), Graphics::High);
    assert_eq!(Graphics::default_for(&DeviceHints::default()), Graphics::High, "unknown hints are not a reason for Low");
}

#[test]
fn weak_devices_default_to_low() {
    assert_eq!(Graphics::default_for(&DeviceHints { cores: Some(2), ..strong() }), Graphics::Low);
    assert_eq!(Graphics::default_for(&DeviceHints { memory_gb: Some(2.0), ..strong() }), Graphics::Low);
    for r in ["ANGLE (Google, Vulkan 1.3.0 (SwiftShader Device (Subzero)))", "llvmpipe (LLVM 15)", "Microsoft Basic Render Driver"] {
        let hints = DeviceHints { renderer: Some(r.into()), ..strong() };
        assert_eq!(Graphics::default_for(&hints), Graphics::Low, "{r}");
    }
}

#[test]
fn a_saved_choice_always_beats_the_device_rule() {
    let weak = DeviceHints { cores: Some(1), memory_gb: Some(1.0), renderer: Some("SwiftShader".into()) };
    assert_eq!(Graphics::choose(Some(Graphics::High), &weak), Graphics::High);
    assert_eq!(Graphics::choose(Some(Graphics::Low), &strong()), Graphics::Low);
    assert_eq!(Graphics::choose(None, &weak), Graphics::Low);
    assert_eq!(Graphics::choose(None, &strong()), Graphics::High);
}
