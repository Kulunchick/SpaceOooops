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
    let s = Settings { music: 0.25, effects: 0.8, muted: true };
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
    let s = Settings::from_text("quality=low\nmusic=0.5");
    assert_eq!(s.music, 0.5);
}
