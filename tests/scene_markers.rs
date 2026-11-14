//! Scene Spec: the Scenario markers on the globe (Scenario select) and the region the Brief holds on.
use spaceooops::scene::{self, SceneInputs, Screen, Shot};
use spaceooops::sim::{load_catalog, Catalog};

fn spec_on(catalog: &Catalog, screen: Screen, chosen: Option<&str>) -> scene::SceneSpec {
    scene::build(&SceneInputs { scenarios: &catalog.scenarios, chosen, ..SceneInputs::on(screen) })
}

fn angle_between(a: (f64, f64), b: (f64, f64)) -> f64 {
    let (la, oa, lb, ob) = (a.0.to_radians(), a.1.to_radians(), b.0.to_radians(), b.1.to_radians());
    (la.sin() * lb.sin() + la.cos() * lb.cos() * (oa - ob).cos()).clamp(-1.0, 1.0).acos().to_degrees()
}

#[test]
fn each_scenarios_marker_position_comes_from_its_data() {
    let catalog = load_catalog().unwrap();
    let spec = spec_on(&catalog, Screen::ScenarioSelect, None);
    assert_eq!(spec.markers.len(), catalog.scenarios.len());
    for s in &catalog.scenarios {
        let marker = spec.markers.iter().find(|m| m.id == s.id).unwrap_or_else(|| panic!("no marker for {}", s.id));
        assert_eq!((marker.lat, marker.lon), (s.marker_lat, s.marker_lon));
    }
}

#[test]
fn markers_sit_in_angola_and_stay_apart_at_the_select_shot() {
    let catalog = load_catalog().unwrap();
    for s in &catalog.scenarios {
        assert!((-18.1..=-4.3).contains(&s.marker_lat), "{} latitude {}", s.id, s.marker_lat);
        assert!((11.6..=24.1).contains(&s.marker_lon), "{} longitude {}", s.id, s.marker_lon);
    }
    for (i, a) in catalog.scenarios.iter().enumerate() {
        for b in &catalog.scenarios[i + 1..] {
            let apart = angle_between((a.marker_lat, a.marker_lon), (b.marker_lat, b.marker_lon));
            assert!(apart >= 5.0, "{} and {} are only {apart:.1} degrees apart", a.id, b.id);
        }
    }
}

#[test]
fn only_scenario_select_and_brief_show_markers() {
    let catalog = load_catalog().unwrap();
    use Screen::*;
    for screen in [Title, Sources, TeamSetup, Design, LaunchReview, Run, Debrief, History] {
        assert!(spec_on(&catalog, screen, Some("crop-watch")).markers.is_empty(), "{screen:?} shows markers");
    }
    assert!(!spec_on(&catalog, Brief, Some("crop-watch")).markers.is_empty());
}

#[test]
fn scenario_select_frames_the_globe_over_angola() {
    let catalog = load_catalog().unwrap();
    match spec_on(&catalog, Screen::ScenarioSelect, None).shot {
        Shot::Globe { lat, lon } => {
            assert!((-18.0..=-5.0).contains(&lat) && (12.0..=24.0).contains(&lon), "({lat}, {lon})");
        }
        other => panic!("expected the globe shot, got {other:?}"),
    }
}

#[test]
fn the_brief_holds_on_the_chosen_scenarios_marker() {
    let catalog = load_catalog().unwrap();
    for s in &catalog.scenarios {
        let spec = spec_on(&catalog, Screen::Brief, Some(&s.id));
        assert_eq!(spec.shot, Shot::Region { lat: s.marker_lat, lon: s.marker_lon });
        let selected: Vec<_> = spec.markers.iter().filter(|m| m.selected).map(|m| m.id.as_str()).collect();
        assert_eq!(selected, [s.id.as_str()]);
    }
}

#[test]
fn nothing_is_selected_while_choosing() {
    let catalog = load_catalog().unwrap();
    let spec = spec_on(&catalog, Screen::ScenarioSelect, Some("crop-watch"));
    assert!(spec.markers.iter().all(|m| !m.selected));
}

#[test]
fn the_brief_without_a_known_scenario_keeps_the_globe() {
    let catalog = load_catalog().unwrap();
    assert!(matches!(spec_on(&catalog, Screen::Brief, Some("nope")).shot, Shot::Globe { .. }));
}

#[test]
fn bridge_receives_markers_and_shots_as_camel_case_json() {
    let catalog = load_catalog().unwrap();
    let json = scene::to_json(&spec_on(&catalog, Screen::Brief, Some("crop-watch")));
    let v: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(v["shot"]["kind"], "region");
    assert!(v["shot"]["lat"].is_number());
    assert_eq!(v["markers"][0]["id"], "crop-watch");
    assert!(v["markers"][0]["selected"].is_boolean());
}
