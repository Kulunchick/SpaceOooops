//! Scene Spec: which Shot the 3D scene frames on each screen.
use spaceooops::scene::{self, SceneInputs, Screen, Shot};

fn shot_on(screen: Screen) -> Shot {
    scene::build(&SceneInputs::on(screen)).shot
}

#[test]
fn title_screen_frames_the_title_orbit_shot() {
    assert_eq!(shot_on(Screen::Title), Shot::TitleOrbit);
}

#[test]
fn bridge_receives_the_title_shot_as_camel_case_json() {
    let json = scene::to_json(&scene::build(&SceneInputs::on(Screen::Title)));
    let value: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(value["shot"]["kind"], "titleOrbit");
}

#[test]
fn the_title_shows_the_scene_clearly() {
    assert!(!scene::build(&SceneInputs::on(Screen::Title)).dimmed);
}

#[test]
fn screens_still_laid_out_as_full_pages_dim_the_scene_behind_them() {
    // Design is a hangar over a visible scene since ticket 02 (see tests/scene_spacecraft.rs).
    for screen in [Screen::LaunchReview, Screen::Run, Screen::Debrief] {
        assert!(scene::build(&SceneInputs::on(screen)).dimmed, "{screen:?} should dim the scene");
    }
}

#[test]
fn sources_team_setup_and_history_are_panels_over_a_visible_scene() {
    for screen in [Screen::Sources, Screen::TeamSetup, Screen::History] {
        assert!(!scene::build(&SceneInputs::on(screen)).dimmed, "{screen:?} should show the scene");
    }
}

#[test]
fn every_screen_has_a_shot_so_the_scene_never_goes_blank() {
    use Screen::*;
    for screen in [Title, Sources, TeamSetup, ScenarioSelect, Brief, Design, LaunchReview, Run, Debrief, History] {
        let json = scene::to_json(&scene::build(&SceneInputs::on(screen)));
        assert!(json.contains(r#""shot":{"kind":""#), "{screen:?} has no shot: {json}");
    }
}
