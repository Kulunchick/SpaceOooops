//! Scene Spec: Launch Review on the launch pad (low-poly ticket 07): the Shot, the rocket with the spacecraft on
//! top, the Go / No-Go vote marks and their Cues, and the launch sequence flag.
mod common;

use common::*;
use spaceooops::scene::{self, Cue, PadVote, PartKind, SceneInputs, SceneSpec, Screen, Shot};
use spaceooops::sim::{load_catalog, Catalog, Design};

fn pad_spec(c: &Catalog, d: &Design, votes: &[Option<bool>], launching: bool) -> SceneSpec {
    scene::build(&SceneInputs { catalog: Some(c), design: Some(d), votes, launching, ..SceneInputs::on(Screen::LaunchReview) })
}

#[test]
fn launch_review_frames_the_pad_shot_and_shows_the_scene_clearly() {
    let c = load_catalog().unwrap();
    let spec = pad_spec(&c, &crop_watch_known_good(), &[], false);
    assert_eq!(spec.shot, Shot::Pad);
    assert!(!spec.dimmed);
    let json: serde_json::Value = serde_json::from_str(&scene::to_json(&spec)).unwrap();
    assert_eq!(json["shot"]["kind"], "pad");
}

#[test]
fn the_pad_carries_the_chosen_launch_vehicle_with_the_assembled_spacecraft_on_top() {
    let c = load_catalog().unwrap();
    let d = crop_watch_known_good();
    let spec = pad_spec(&c, &d, &[], false);
    let pad = spec.pad.expect("a pad on Launch Review");
    assert_eq!(Some(pad.launch_vehicle.id.as_str()), d.launch_vehicle.as_deref());
    assert!(!pad.launch_vehicle.class.is_empty());
    let craft = spec.spacecraft.expect("the spacecraft rides on top of the rocket");
    assert_eq!(craft.parts[0].kind, PartKind::Bus);
}

#[test]
fn a_heavier_launch_vehicle_stands_taller_on_the_pad() {
    let c = load_catalog().unwrap();
    let mut heights: Vec<(String, f64)> = Vec::new();
    for lv in &c.launch_vehicles {
        let mut d = crop_watch_known_good();
        d.launch_vehicle = Some(lv.id.clone());
        let pad = pad_spec(&c, &d, &[], false).pad.unwrap();
        heights.push((pad.launch_vehicle.class.clone(), pad.launch_vehicle.height));
    }
    let small = heights.iter().filter(|(k, _)| k == "small").map(|(_, h)| *h).fold(f64::MAX, f64::min);
    let heavy = heights.iter().filter(|(k, _)| k == "heavy").map(|(_, h)| *h).fold(0.0, f64::max);
    assert!(heavy > small, "{heights:?}");
}

#[test]
fn there_is_no_pad_on_the_other_screens() {
    let c = load_catalog().unwrap();
    let d = crop_watch_known_good();
    for screen in [Screen::Title, Screen::Brief, Screen::Design, Screen::Run, Screen::Debrief] {
        let spec = scene::build(&SceneInputs { catalog: Some(&c), design: Some(&d), ..SceneInputs::on(screen) });
        assert!(spec.pad.is_none(), "{screen:?}");
    }
}

#[test]
fn without_a_launch_vehicle_there_is_no_pad() {
    let c = load_catalog().unwrap();
    let spec = pad_spec(&c, &Design::default(), &[], false);
    assert!(spec.pad.is_none());
}

#[test]
fn votes_reach_the_bridge_as_pending_go_and_nogo() {
    let c = load_catalog().unwrap();
    let spec = pad_spec(&c, &crop_watch_known_good(), &[Some(true), Some(false), None, None], false);
    let json: serde_json::Value = serde_json::from_str(&scene::to_json(&spec)).unwrap();
    assert_eq!(json["pad"]["votes"], serde_json::json!(["go", "nogo", "pending", "pending"]));
    assert_eq!(json["pad"]["launching"], false);
    assert_eq!(spec.pad.unwrap().votes[0], PadVote::Go);
}

#[test]
fn a_go_vote_emits_a_go_cue_and_a_no_go_vote_a_no_go_cue() {
    let c = load_catalog().unwrap();
    let d = crop_watch_known_good();
    let none = pad_spec(&c, &d, &[None; 4], false);
    let go = pad_spec(&c, &d, &[Some(true), None, None, None], false);
    let nogo = pad_spec(&c, &d, &[Some(true), Some(false), None, None], false);
    assert_eq!(scene::one_shots(&none, &go), vec![Cue::Go]);
    assert_eq!(scene::one_shots(&go, &nogo), vec![Cue::NoGo]);
}

#[test]
fn changing_a_vote_from_go_to_no_go_cues_the_no_go() {
    let c = load_catalog().unwrap();
    let d = crop_watch_known_good();
    let go = pad_spec(&c, &d, &[Some(true), None, None, None], false);
    let nogo = pad_spec(&c, &d, &[Some(false), None, None, None], false);
    assert_eq!(scene::one_shots(&go, &nogo), vec![Cue::NoGo]);
}

#[test]
fn clearing_the_votes_and_an_unchanged_vote_are_silent() {
    let c = load_catalog().unwrap();
    let d = crop_watch_known_good();
    let voted = pad_spec(&c, &d, &[Some(true), Some(false), None, None], false);
    let cleared = pad_spec(&c, &d, &[None; 4], false);
    assert!(scene::one_shots(&voted, &cleared).is_empty());
    assert!(scene::one_shots(&voted, &voted).is_empty());
}

#[test]
fn arriving_at_launch_review_with_votes_already_cast_is_silent() {
    let c = load_catalog().unwrap();
    let d = crop_watch_known_good();
    let design_screen = scene::build(&SceneInputs { catalog: Some(&c), design: Some(&d), ..SceneInputs::on(Screen::Design) });
    let pad = pad_spec(&c, &d, &[Some(true), None, None, None], false);
    assert!(scene::one_shots(&design_screen, &pad).is_empty());
}

#[test]
fn starting_the_launch_emits_the_launch_cue_once() {
    let c = load_catalog().unwrap();
    let d = crop_watch_known_good();
    let waiting = pad_spec(&c, &d, &[], false);
    let launching = pad_spec(&c, &d, &[], true);
    assert_eq!(scene::one_shots(&waiting, &launching), vec![Cue::Launch]);
    assert!(scene::one_shots(&launching, &launching).is_empty());
    assert!(launching.pad.unwrap().launching);
}

#[test]
fn the_new_cue_names_are_the_strings_the_bridge_plays() {
    assert_eq!(Cue::Go.name(), "go");
    assert_eq!(Cue::NoGo.name(), "nogo");
    assert_eq!(Cue::Launch.name(), "launch");
}
