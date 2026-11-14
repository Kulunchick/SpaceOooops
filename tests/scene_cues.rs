//! Scene Spec: the audio Cues (low-poly ticket 04): the music track for the screen, and one-shot sounds
//! triggered by the difference between the previous and the current state.
mod common;

use common::*;
use spaceooops::scene::{self, Cue, SceneInputs, SceneSpec, Screen, Track};
use spaceooops::sim::{load_catalog, Catalog, Design, Pick, ReliabilityTier::Basic};

fn spec(c: &Catalog, d: &Design, screen: Screen) -> SceneSpec {
    scene::build(&SceneInputs { catalog: Some(c), design: Some(d), ..SceneInputs::on(screen) })
}

fn music_on(screen: Screen) -> Track {
    scene::build(&SceneInputs::on(screen)).music
}

#[test]
fn the_menu_track_plays_everywhere_outside_the_run() {
    for screen in [
        Screen::Title,
        Screen::Sources,
        Screen::TeamSetup,
        Screen::ScenarioSelect,
        Screen::Brief,
        Screen::Design,
        Screen::LaunchReview,
        Screen::Debrief,
        Screen::History,
    ] {
        assert_eq!(music_on(screen), Track::Menu, "{screen:?}");
    }
}

#[test]
fn the_run_track_plays_during_the_run() {
    assert_eq!(music_on(Screen::Run), Track::Run);
}

#[test]
fn the_bridge_receives_the_track_as_a_lowercase_name() {
    let json = scene::to_json(&scene::build(&SceneInputs::on(Screen::Run)));
    let value: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(value["music"], "run");
}

/// A Design with one Instrument more or less than `crop_watch_known_good`, to add and remove one part.
fn with_one_more_instrument(c: &Catalog) -> Design {
    let mut d = crop_watch_known_good();
    let extra = c.instruments.iter().find(|i| !d.instruments.contains(&i.id)).expect("an unused Instrument");
    d.instruments.push(extra.id.clone());
    d
}

#[test]
fn adding_a_part_emits_a_snap() {
    let c = load_catalog().unwrap();
    let before = crop_watch_known_good();
    let after = with_one_more_instrument(&c);
    let cues = scene::one_shots(&spec(&c, &before, Screen::Design), &spec(&c, &after, Screen::Design));
    assert_eq!(cues, vec![Cue::Snap]);
}

#[test]
fn removing_a_part_emits_a_snap() {
    let c = load_catalog().unwrap();
    let before = with_one_more_instrument(&c);
    let after = crop_watch_known_good();
    let cues = scene::one_shots(&spec(&c, &before, Screen::Design), &spec(&c, &after, Screen::Design));
    assert_eq!(cues, vec![Cue::Snap]);
}

#[test]
fn swapping_one_option_snaps_once() {
    let c = load_catalog().unwrap();
    let before = crop_watch_known_good();
    let mut after = before.clone();
    let current = before.battery.as_ref().map(|p| p.id.clone());
    let other = c.batteries.iter().find(|b| Some(&b.id) != current.as_ref()).unwrap();
    after.battery = Some(Pick::new(&other.id, Basic));
    let cues = scene::one_shots(&spec(&c, &before, Screen::Design), &spec(&c, &after, Screen::Design));
    assert_eq!(cues, vec![Cue::Snap]);
}

#[test]
fn changing_nothing_emits_nothing() {
    let c = load_catalog().unwrap();
    let d = crop_watch_known_good();
    assert!(scene::one_shots(&spec(&c, &d, Screen::Design), &spec(&c, &d, Screen::Design)).is_empty());
}

#[test]
fn loading_a_whole_design_at_once_does_not_burst_into_snaps() {
    let c = load_catalog().unwrap();
    let empty = Design::default();
    let loaded = crop_watch_known_good();
    let cues = scene::one_shots(&spec(&c, &empty, Screen::Design), &spec(&c, &loaded, Screen::Design));
    assert!(cues.is_empty(), "{cues:?}");
}

#[test]
fn arriving_on_the_design_screen_with_a_built_design_is_silent() {
    let c = load_catalog().unwrap();
    let d = crop_watch_known_good();
    let cues = scene::one_shots(&spec(&c, &d, Screen::Brief), &spec(&c, &d, Screen::Design));
    assert!(cues.is_empty(), "{cues:?}");
}

#[test]
fn cue_names_are_the_strings_the_bridge_plays() {
    assert_eq!(Cue::Snap.name(), "snap");
}
