//! Scene Spec: the spacecraft model assembled from the Design on the Design screen (low-poly ticket 02).
mod common;

use common::*;
use spaceooops::scene::{self, Finish, HangarPreview, Part, PartKind, SceneInputs, SceneSpec, Screen, Shot};
use spaceooops::sim::{load_catalog, Catalog, Design};

fn spec_for(c: &Catalog, d: &Design) -> SceneSpec {
    scene::build(&SceneInputs { catalog: Some(c), design: Some(d), ..SceneInputs::on(Screen::Design) })
}

fn parts(c: &Catalog, d: &Design) -> Vec<Part> {
    spec_for(c, d).spacecraft.expect("the Design screen shows a spacecraft").parts
}

fn part<'a>(parts: &'a [Part], id: &str) -> Option<&'a Part> {
    parts.iter().find(|p| p.id == id)
}

#[test]
fn the_design_screen_assembles_the_bus_from_the_chosen_structure() {
    let c = load_catalog().unwrap();
    let parts = parts(&c, &crop_watch_known_good());
    let bus = part(&parts, "cubesat-6u").expect("a part for the chosen structure");
    assert_eq!(bus.kind, PartKind::Bus);
}

type Choose = fn(&mut Design, &str);

/// Every option a Design can pick, each with the one-choice change it makes to a Design.
fn every_choice(c: &Catalog) -> Vec<(String, Choose)> {
    use spaceooops::sim::{Pick, ReliabilityTier::Basic};
    let structure: Choose = |d, id| d.structure = Some(id.into());
    let solar: Choose = |d, id| d.solar = Some(Pick::new(id, Basic));
    let battery: Choose = |d, id| d.battery = Some(Pick::new(id, Basic));
    let attitude: Choose = |d, id| d.attitude = Some(Pick::new(id, Basic));
    let comms: Choose = |d, id| d.comms = Some(Pick::new(id, Basic));
    let instrument: Choose = |d, id| d.instruments.push(id.into());
    let mut out: Vec<(String, Choose)> = Vec::new();
    out.extend(c.structures.iter().map(|o| (o.id.clone(), structure)));
    out.extend(c.solar_panels.iter().map(|o| (o.id.clone(), solar)));
    out.extend(c.batteries.iter().map(|o| (o.id.clone(), battery)));
    out.extend(c.attitude_controls.iter().map(|o| (o.id.clone(), attitude)));
    out.extend(c.comms.iter().map(|o| (o.id.clone(), comms)));
    out.extend(c.instruments.iter().map(|o| (o.id.clone(), instrument)));
    out
}

#[test]
fn every_component_and_instrument_choice_shows_as_a_part_and_removing_it_removes_the_part() {
    let c = load_catalog().unwrap();
    for (id, choose) in every_choice(&c) {
        let without = Design::default();
        let mut chosen = without.clone();
        choose(&mut chosen, &id);
        assert!(part(&parts(&c, &chosen), &id).is_some(), "choosing {id} shows no part");
        assert!(part(&parts(&c, &without), &id).is_none(), "{id} shows although it is not chosen");
    }
}

#[test]
fn the_computer_is_always_on_board_so_it_always_shows() {
    let c = load_catalog().unwrap();
    let parts = parts(&c, &Design::default());
    let computer = part(&parts, &c.computer.id).expect("the computer shows on an empty Design");
    assert_eq!(computer.kind, PartKind::ComputerMarker);
}

fn volume(size: [f64; 3]) -> f64 {
    size[0] * size[1] * size[2]
}

#[test]
fn the_bus_grows_with_the_structure_size_3u_6u_12u_smallsat() {
    let c = load_catalog().unwrap();
    let bus_of = |id: &str| {
        let d = Design { structure: Some(id.into()), ..Design::default() };
        part(&parts(&c, &d), id).unwrap().size
    };
    let sizes: Vec<[f64; 3]> = ["cubesat-3u", "cubesat-6u", "cubesat-12u", "smallsat"].map(bus_of).into();
    for pair in sizes.windows(2) {
        let (small, big) = (pair[0], pair[1]);
        assert!(volume(big) > volume(small), "{big:?} is not bigger than {small:?}");
        for axis in 0..3 {
            assert!(big[axis] >= small[axis], "{big:?} shrinks along axis {axis} against {small:?}");
        }
    }
    // A 3U CubeSat is 10 x 10 x 30 cm.
    let three_u = [0.1, 0.3, 0.1];
    assert!((0..3).all(|a| (sizes[0][a] - three_u[a]).abs() < 1e-9), "3U bus is {:?}", sizes[0]);
}

#[test]
fn a_flight_proven_component_has_a_different_finish_from_a_basic_one() {
    use spaceooops::sim::ReliabilityTier::Basic;
    let c = load_catalog().unwrap();
    let mut basic = crop_watch_known_good();
    for slot in [&mut basic.solar, &mut basic.battery, &mut basic.attitude, &mut basic.comms] {
        slot.as_mut().unwrap().tier = Basic;
    }
    basic.computer_tier = Basic;
    let proven = crop_watch_known_good();
    let ids = ["solar-deployable", "battery-m", "reaction-wheels", "comms-s-band", "computer-obc"];
    for id in ids {
        let b = part(&parts(&c, &basic), id).unwrap().finish;
        let p = part(&parts(&c, &proven), id).unwrap().finish;
        assert_eq!(b, Finish::Basic, "{id}");
        assert_eq!(p, Finish::FlightProven, "{id}");
    }
}

#[test]
fn body_mounted_and_deployable_panels_are_different_hardware_and_wings_reach_beyond_the_bus() {
    let c = load_catalog().unwrap();
    let mut d = crop_watch_known_good();
    let wings = part(&parts(&c, &d), "solar-deployable").unwrap().clone();
    d.solar.as_mut().unwrap().id = "solar-body".into();
    let body = part(&parts(&c, &d), "solar-body").unwrap().clone();
    assert_eq!(wings.kind, PartKind::SolarWings);
    assert_eq!(body.kind, PartKind::BodyPanels);
    let bus = part(&parts(&c, &d), "cubesat-6u").unwrap().size;
    assert!(wings.size[1] > bus[0], "a wing reaches out further than the bus is wide");
}

#[test]
fn uhf_s_band_and_x_band_show_three_different_antennas() {
    let c = load_catalog().unwrap();
    let mut shapes: Vec<String> = c
        .comms
        .iter()
        .map(|o| {
            let d = Design { comms: Some(spaceooops::sim::Pick::new(&o.id, Default::default())), ..Design::default() };
            let p = part(&parts(&c, &d), &o.id).unwrap().clone();
            assert_eq!(p.kind, PartKind::Antenna);
            p.shape.expect("an antenna has a shape")
        })
        .collect();
    shapes.sort();
    assert_eq!(shapes, ["dish", "patch", "whip"]);
}

#[test]
fn reaction_wheels_and_magnetorquers_are_distinct_hardware() {
    let c = load_catalog().unwrap();
    let kind_of = |id: &str| {
        let d = Design { attitude: Some(spaceooops::sim::Pick::new(id, Default::default())), ..Design::default() };
        part(&parts(&c, &d), id).unwrap().kind
    };
    assert_eq!(kind_of("reaction-wheels"), PartKind::ReactionWheels);
    assert_eq!(kind_of("magnetorquers"), PartKind::Magnetorquers);
}

#[test]
fn a_bigger_battery_leaves_a_bigger_marker() {
    let c = load_catalog().unwrap();
    let marker = |id: &str| {
        let d = Design { battery: Some(spaceooops::sim::Pick::new(id, Default::default())), ..Design::default() };
        let p = part(&parts(&c, &d), id).unwrap().clone();
        assert_eq!(p.kind, PartKind::BatteryMarker);
        volume(p.size)
    };
    assert!(marker("battery-s") < marker("battery-m"));
    assert!(marker("battery-m") < marker("battery-l"));
}

#[test]
fn every_instrument_mounts_with_a_shape_of_its_own() {
    let c = load_catalog().unwrap();
    let d = Design { instruments: c.instruments.iter().map(|i| i.id.clone()).collect(), ..Design::default() };
    let parts = parts(&c, &d);
    let mut shapes: Vec<String> = c
        .instruments
        .iter()
        .map(|i| {
            let p = part(&parts, &i.id).unwrap();
            assert_eq!(p.kind, PartKind::Instrument);
            p.shape.clone().unwrap()
        })
        .collect();
    assert!(!shapes.iter().any(|s| s == "box"), "an Instrument has no shape of its own: {shapes:?}");
    shapes.sort();
    shapes.dedup();
    assert_eq!(shapes.len(), c.instruments.len(), "two Instruments share a shape");
}

#[test]
fn parts_mount_on_the_bus_surface_not_inside_it() {
    let c = load_catalog().unwrap();
    let parts = parts(&c, &space_weather_sentinel_known_good());
    let bus = parts[0].size;
    for p in parts.iter().filter(|p| !matches!(p.kind, PartKind::Bus | PartKind::BodyPanels)) {
        let on_face = (0..3).any(|a| (p.position[a].abs() - bus[a] / 2.0).abs() < 1e-9 && p.facing[a] != 0.0);
        assert!(on_face, "{} is not on a bus face: {:?}", p.id, p.position);
    }
}

#[test]
fn without_a_structure_the_parts_hang_on_an_empty_frame() {
    let c = load_catalog().unwrap();
    let parts = parts(&c, &Design::default());
    assert_eq!(parts[0].kind, PartKind::Bus);
    assert_eq!(parts[0].finish, Finish::Outline);
}

#[test]
fn only_the_design_screen_shows_the_assembled_spacecraft() {
    let c = load_catalog().unwrap();
    let d = crop_watch_known_good();
    let title = scene::build(&SceneInputs { catalog: Some(&c), design: Some(&d), ..SceneInputs::on(Screen::Title) });
    assert!(title.spacecraft.is_none());
    let json = scene::to_json(&spec_for(&c, &d));
    let value: serde_json::Value = serde_json::from_str(&json).unwrap();
    let first = &value["spacecraft"]["parts"][0];
    assert_eq!(first["kind"], "bus");
    assert_eq!(first["id"], "cubesat-6u");
    assert!(first["size"].is_array() && first["position"].is_array() && first["facing"].is_array());
}

#[test]
fn a_sweep_over_every_catalog_combination_builds_a_scene_spec() {
    use spaceooops::sim::{Pick, ReliabilityTier::{Basic, FlightProven}};
    let c = load_catalog().unwrap();
    let some = |v: Vec<String>| v.into_iter().map(Some).chain([None]).collect::<Vec<_>>();
    let structures = some(c.structures.iter().map(|o| o.id.clone()).collect());
    let solar = some(c.solar_panels.iter().map(|o| o.id.clone()).collect());
    let battery = some(c.batteries.iter().map(|o| o.id.clone()).collect());
    let attitude = some(c.attitude_controls.iter().map(|o| o.id.clone()).collect());
    let comms = some(c.comms.iter().map(|o| o.id.clone()).collect());
    let orbits = some(c.orbits.iter().map(|o| o.id.clone()).collect());
    let vehicles = some(c.launch_vehicles.iter().map(|o| o.id.clone()).collect());
    let n = c.instruments.len();
    let mut built = 0;
    for (i, s) in structures.iter().enumerate() {
        for so in &solar {
            for b in &battery {
                for a in &attitude {
                    for co in &comms {
                        for mask in 0..(1u32 << n) {
                            let tier = if (mask + i as u32) % 2 == 0 { Basic } else { FlightProven };
                            let pick = |id: &Option<String>| id.as_ref().map(|id| Pick::new(id, tier));
                            let d = Design {
                                instruments: (0..n).filter(|k| mask & (1 << k) != 0).map(|k| c.instruments[k].id.clone()).collect(),
                                structure: s.clone(),
                                solar: pick(so),
                                battery: pick(b),
                                attitude: pick(a),
                                comms: pick(co),
                                computer_tier: tier,
                                orbit: orbits[mask as usize % orbits.len()].clone(),
                                launch_vehicle: vehicles[mask as usize % vehicles.len()].clone(),
                            };
                            let spec = spec_for(&c, &d);
                            let parts = &spec.spacecraft.as_ref().unwrap().parts;
                            // bus + computer + one part per chosen option
                            let expected = 2
                                + d.instruments.len()
                                + [&d.solar, &d.battery, &d.attitude, &d.comms].iter().filter(|p| p.is_some()).count();
                            assert_eq!(parts.len(), expected, "{d:?}");
                            assert!(parts.iter().all(|p| p.size.iter().chain(&p.position).all(|x| x.is_finite())));
                            scene::to_json(&spec);
                            built += 1;
                        }
                    }
                }
            }
        }
    }
    assert!(built > 10_000);
}

#[test]
fn a_stale_design_with_unknown_ids_still_builds() {
    let c = load_catalog().unwrap();
    let d = Design {
        structure: Some("warp-core".into()),
        instruments: vec!["tricorder".into()],
        orbit: Some("mars".into()),
        launch_vehicle: Some("catapult".into()),
        ..Design::default()
    };
    let spec = spec_for(&c, &d);
    assert_eq!(spec.spacecraft.unwrap().parts.len(), 2);
}

#[test]
fn the_design_screen_frames_the_hangar_and_shows_the_scene_clearly() {
    let c = load_catalog().unwrap();
    let spec = spec_for(&c, &crop_watch_known_good());
    assert_eq!(spec.shot, Shot::Hangar);
    assert!(!spec.dimmed);
    let json = scene::to_json(&spec);
    assert!(json.contains(r#""shot":{"kind":"hangar"}"#), "{json}");
}

fn hangar(c: &Catalog, d: &Design) -> HangarPreview {
    spec_for(c, d).hangar.expect("the Design screen has hangar previews")
}

#[test]
fn choosing_an_orbit_previews_it_and_leo_sso_and_geo_look_different() {
    let c = load_catalog().unwrap();
    let preview = |id: &str| {
        let d = Design { orbit: Some(id.into()), ..Design::default() };
        hangar(&c, &d).orbit.expect("an Orbit preview")
    };
    let (leo, sso, geo) = (preview("leo-400"), preview("sso-600"), preview("geo"));
    assert_eq!(leo.id, "leo-400");
    assert!(1.0 < leo.radius && leo.radius < sso.radius && sso.radius < geo.radius, "higher orbits ring wider");
    assert!((sso.inclination_deg - 97.8).abs() < 1e-9 && geo.inclination_deg == 0.0);
    assert!(hangar(&c, &Design::default()).orbit.is_none(), "no Orbit chosen, no preview");
}

#[test]
fn choosing_a_launch_vehicle_shows_its_rocket_and_a_taller_rocket_stands_taller() {
    let c = load_catalog().unwrap();
    let rocket = |id: &str| {
        let d = Design { launch_vehicle: Some(id.into()), ..Design::default() };
        hangar(&c, &d).launch_vehicle.expect("a rocket preview")
    };
    let small = rocket("small-launcher");
    let medium = rocket("medium-launcher");
    assert_eq!(small.id, "small-launcher");
    assert_eq!(small.class, "small");
    assert_eq!(medium.class, "medium");
    assert!(small.height < medium.height);
    let bus = part(&parts(&c, &crop_watch_known_good()), "cubesat-6u").unwrap().size;
    assert!(small.height > bus[1] * 3.0, "even the small rocket dwarfs the spacecraft");
    assert!(hangar(&c, &Design::default()).launch_vehicle.is_none());
    for lv in &c.launch_vehicles {
        assert!(rocket(&lv.id).height > 0.0, "{} has no rocket look", lv.id);
    }
}

#[test]
fn every_part_id_is_unique_so_the_renderer_can_diff_parts_by_id() {
    let c = load_catalog().unwrap();
    let mut d = space_weather_sentinel_known_good();
    d.instruments.push("wide-camera".into());
    let parts = parts(&c, &d);
    let mut ids: Vec<&str> = parts.iter().map(|p| p.id.as_str()).collect();
    ids.sort();
    ids.dedup();
    assert_eq!(ids.len(), parts.len());
}
