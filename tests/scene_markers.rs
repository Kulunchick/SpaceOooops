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

/// The Target Country outlines the globe draws, read from `js/bridge/land.js` (as other tests read data files):
/// country id -> its polygons of [lon, lat] points.
fn outlines() -> std::collections::BTreeMap<String, Vec<Vec<(f64, f64)>>> {
    let js = include_str!("../js/bridge/land.js");
    let line = js.lines().find(|l| l.starts_with("export const COUNTRIES = {")).expect("COUNTRIES in land.js");
    let body = line.split('{').nth(1).unwrap().split('}').next().unwrap();
    let mut out = std::collections::BTreeMap::new();
    for entry in body.split(',').map(str::trim).filter(|e| !e.is_empty()) {
        let (id, constant) = entry.split_once(':').unwrap();
        let start = js.find(&format!("export const {} = [", constant.trim())).expect("the outline constant");
        let text = &js[start..];
        let text = &text[text.find('[').unwrap()..text.find("\n];").unwrap() + 2];
        // JS to JSON: drop comments and trailing commas.
        let mut json: String = text.lines().map(|l| l.split("//").next().unwrap()).collect::<Vec<_>>().join("\n");
        while let Some(i) = json.find(",\n]").or_else(|| json.find(", ]")).or_else(|| json.find(",]")) {
            json.remove(i);
        }
        let json = json.replace(",\n  ]", "\n  ]");
        let v: serde_json::Value = serde_json::from_str(&json).unwrap_or_else(|e| panic!("{id}: {e}\n{json}"));
        let pair = |p: &serde_json::Value| (p[0].as_f64().unwrap(), p[1].as_f64().unwrap());
        let parts: Vec<Vec<(f64, f64)>> = if v[0][0].is_array() {
            v.as_array().unwrap().iter().map(|part| part.as_array().unwrap().iter().map(pair).collect()).collect()
        } else {
            vec![v.as_array().unwrap().iter().map(pair).collect()]
        };
        out.insert(id.trim().to_string(), parts);
    }
    out
}

/// Whether (lat, lon) is inside one of the polygons (even-odd rule, as land.js does).
fn inside(parts: &[Vec<(f64, f64)>], lat: f64, lon: f64) -> bool {
    parts.iter().any(|poly| {
        let mut hit = false;
        let mut j = poly.len() - 1;
        for i in 0..poly.len() {
            let ((xi, yi), (xj, yj)) = (poly[i], poly[j]);
            if (yi > lat) != (yj > lat) && lon < (xj - xi) * (lat - yi) / (yj - yi) + xi {
                hit = !hit;
            }
            j = i;
        }
        hit
    })
}

#[test]
fn every_target_country_and_the_home_country_have_an_outline_on_the_globe() {
    let catalog = load_catalog().unwrap();
    let outlines = outlines();
    assert!(outlines.contains_key(spaceooops::sim::HOME_COUNTRY));
    for s in &catalog.scenarios {
        assert!(outlines.contains_key(&s.target_country.id), "{}: no outline for '{}' in land.js", s.id, s.target_country.id);
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
fn markers_and_stations_sit_inside_their_target_country_and_markers_stay_apart() {
    let catalog = load_catalog().unwrap();
    let outlines = outlines();
    for s in &catalog.scenarios {
        let outline = &outlines[&s.target_country.id];
        assert!(inside(outline, s.marker_lat, s.marker_lon), "{} marker is outside {}", s.id, s.target_country.name);
        for g in &s.ground_stations {
            // The one station abroad: the Angolan Scenarios' backup in South Africa.
            if s.target_country.id == "angola" && g.name == "Johannesburg" {
                assert!(!inside(outline, g.lat, g.lon));
                continue;
            }
            assert!(inside(outline, g.lat, g.lon), "{} station {} is outside {}", s.id, g.name, s.target_country.name);
        }
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

/// Moves an Angolan Scenario's pin and Target Country elsewhere, so a follow cannot pass by resting on Angola.
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

/// The globe's highlight must show the facets under the country's pins and ground stations, so the Scene Spec
/// names them: every pin and station of every Scenario in the highlighted country.
#[test]
fn the_highlight_names_the_pins_and_stations_it_must_show() {
    let catalog = load_catalog().unwrap();
    let points = |spec: &scene::SceneSpec| spec.highlight_points.iter().map(|p| (p.lat, p.lon)).collect::<Vec<_>>();
    let japan = points(&select_spec(&catalog, Some("japan-disaster-response")));
    for p in [(38.3, 140.7), (36.07, 140.13), (26.5, 127.9)] {
        assert!(japan.contains(&p), "{p:?} in {japan:?}");
    }
    assert_eq!(japan.len(), 3);
    let home = points(&select_spec(&catalog, None));
    for s in catalog.scenarios.iter().filter(|s| s.target_country.id == "angola") {
        assert!(home.contains(&(s.marker_lat, s.marker_lon)), "{}", s.id);
        assert!(home.contains(&(s.ground_stations[0].lat, s.ground_stations[0].lon)), "{}", s.id);
    }
    let json = scene::to_json(&select_spec(&catalog, Some("japan-disaster-response")));
    let v: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(v["highlightPoints"][0]["lat"], 38.3);
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
