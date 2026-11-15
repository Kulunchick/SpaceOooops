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

/// Rough centre of each Target Country (degrees) and the radius within which its Scenarios' markers must sit.
fn country_centre(id: &str) -> ((f64, f64), f64) {
    match id {
        "angola" => ((-12.5, 17.5), 9.0),
        "brazil" => ((-10.0, -53.0), 20.0),
        "india" => ((22.0, 79.0), 15.0),
        other => panic!("no known centre for Target Country '{other}'"),
    }
}

#[test]
fn a_scenario_without_a_target_country_fails_to_load() {
    let missing = include_str!("../data/scenarios.toml").replacen("target_country = ", "target_county = ", 1);
    let parsed: toml::Table = toml::from_str(&missing).unwrap();
    let first = parsed["scenarios"].as_array().unwrap()[0].clone();
    assert!(first.try_into::<spaceooops::sim::Scenario>().is_err());
}

#[test]
fn cards_list_angola_first_then_by_difficulty() {
    let catalog = load_catalog().unwrap();
    let ordered = spaceooops::sim::scenarios_in_card_order(&catalog.scenarios);
    assert_eq!(ordered.len(), catalog.scenarios.len());
    let rank = |d: &str| ["Easy", "Medium", "Hard"].iter().position(|x| *x == d).unwrap_or(99);
    let keys: Vec<(bool, usize)> =
        ordered.iter().map(|s| (s.target_country.id != "angola", rank(&s.difficulty))).collect();
    assert!(keys.windows(2).all(|w| w[0] <= w[1]), "{keys:?}");
}

#[test]
fn the_highlighted_country_follows_the_screen() {
    let catalog = load_catalog().unwrap();
    use Screen::*;
    for screen in [Title, Sources, ScenarioSelect] {
        let spec = spec_on(&catalog, screen, Some("space-weather-sentinel"));
        assert_eq!(spec.highlighted_country, "angola", "{screen:?}");
    }
    for s in &catalog.scenarios {
        for screen in [Brief, TeamSetup, Design, LaunchReview, Run, Debrief] {
            let spec = spec_on(&catalog, screen, Some(&s.id));
            assert_eq!(spec.highlighted_country, s.target_country.id, "{screen:?}");
        }
    }
    assert_eq!(spec_on(&catalog, Design, None).highlighted_country, "angola");
    let json = scene::to_json(&spec_on(&catalog, Brief, Some("crop-watch")));
    let v: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(v["highlightedCountry"], "angola");
}

#[test]
fn markers_sit_near_their_target_country_and_stay_apart() {
    let catalog = load_catalog().unwrap();
    for s in &catalog.scenarios {
        let (centre, radius) = country_centre(&s.target_country.id);
        let away = angle_between(centre, (s.marker_lat, s.marker_lon));
        assert!(away <= radius, "{} marker is {away:.1} degrees from {}", s.id, s.target_country.name);
        // The main station is in the Target Country (Angola's backup is in South Africa, so only the main one).
        let main = &s.ground_stations[0];
        let away = angle_between(centre, (main.lat, main.lon));
        assert!(away <= radius, "{} station {} is {away:.1} degrees from {}", s.id, main.name, s.target_country.name);
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

fn select_spec(catalog: &Catalog, previewed: Option<&str>) -> scene::SceneSpec {
    scene::build(&SceneInputs {
        scenarios: &catalog.scenarios,
        chosen: Some("crop-watch"),
        previewed,
        ..SceneInputs::on(Screen::ScenarioSelect)
    })
}

/// All Scenarios are in Angola today, so move one pin and its Target Country to prove the follow.
fn catalog_with_space_weather_in(country: &str, lat: f64, lon: f64) -> Catalog {
    let mut catalog = load_catalog().unwrap();
    let s = catalog.scenarios.iter_mut().find(|s| s.id == "space-weather-sentinel").unwrap();
    s.marker_lat = lat;
    s.marker_lon = lon;
    s.target_country.id = country.to_string();
    catalog
}

#[test]
fn select_shot_frames_the_previewed_scenarios_pin() {
    let catalog = catalog_with_space_weather_in("brazil", -10.0, -55.0);
    let spec = select_spec(&catalog, Some("space-weather-sentinel"));
    assert_eq!(spec.shot, Shot::Globe { lat: -10.0, lon: -55.0 });
}

#[test]
fn hovering_the_brazil_card_turns_the_globe_to_its_pin_and_outlines_brazil() {
    let catalog = load_catalog().unwrap();
    let amazon = catalog.scenario("amazon-watch").expect("the Brazil Scenario is bundled");
    let spec = select_spec(&catalog, Some("amazon-watch"));
    assert_eq!(spec.shot, Shot::Globe { lat: amazon.marker_lat, lon: amazon.marker_lon });
    assert_eq!(spec.highlighted_country, "brazil");
}

#[test]
fn hovering_the_india_card_turns_the_globe_to_its_pin_and_outlines_india() {
    let catalog = load_catalog().unwrap();
    let cyclone = catalog.scenario("cyclone-watch").expect("the India Scenario is bundled");
    let spec = select_spec(&catalog, Some("cyclone-watch"));
    assert_eq!(spec.shot, Shot::Globe { lat: cyclone.marker_lat, lon: cyclone.marker_lon });
    assert_eq!(spec.highlighted_country, "india");
}

#[test]
fn select_shot_rests_on_angola_with_nothing_previewed_or_an_unknown_id() {
    let catalog = catalog_with_space_weather_in("brazil", -10.0, -55.0);
    let home = Shot::Globe { lat: -12.5, lon: 17.5 };
    assert_eq!(select_spec(&catalog, None).shot, home);
    assert_eq!(select_spec(&catalog, Some("nope")).shot, home);
}

#[test]
fn select_highlight_follows_the_previewed_scenario_and_rests_on_angola() {
    let catalog = catalog_with_space_weather_in("brazil", -10.0, -55.0);
    assert_eq!(select_spec(&catalog, Some("space-weather-sentinel")).highlighted_country, "brazil");
    assert_eq!(select_spec(&catalog, None).highlighted_country, "angola");
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
