//! The bundled data files load into a usable catalog.
use spaceooops::sim::load_catalog;

#[test]
fn bundled_data_files_load() {
    let catalog = load_catalog().expect("bundled data files must parse");
    let crop_watch = catalog
        .scenarios
        .iter()
        .find(|s| s.id == "crop-watch")
        .expect("Crop Watch scenario is bundled");
    assert!(crop_watch.budget_usd > 0.0);
    assert!(crop_watch.min_lifetime_days > 0);
    assert!(crop_watch.required_science_value > 0.0);
    assert!(!crop_watch.why_it_matters.is_empty());
    assert!(catalog.structures.len() >= 2);
    assert!(catalog.solar_panels.len() >= 2);
    assert!(catalog.batteries.len() >= 2);
    assert!(catalog.attitude_controls.len() >= 2);
    assert!(catalog.comms.len() >= 2);
    assert!(catalog.instruments.len() >= 2);
    assert!(!catalog.orbits.is_empty());
    assert!(!catalog.launch_vehicles.is_empty());
    assert!(catalog.launch_vehicles.iter().all(|lv| !lv.real_example.is_empty()));
    assert_eq!(catalog.reliability_tiers.len(), 2);
}

#[test]
fn every_option_explains_itself() {
    let catalog = load_catalog().unwrap();
    let whys: Vec<&str> = catalog
        .structures.iter().map(|o| o.why.as_str())
        .chain(catalog.solar_panels.iter().map(|o| o.why.as_str()))
        .chain(catalog.batteries.iter().map(|o| o.why.as_str()))
        .chain(catalog.attitude_controls.iter().map(|o| o.why.as_str()))
        .chain(catalog.comms.iter().map(|o| o.why.as_str()))
        .chain(catalog.instruments.iter().map(|o| o.why.as_str()))
        .chain(catalog.orbits.iter().map(|o| o.why.as_str()))
        .chain(catalog.launch_vehicles.iter().map(|o| o.why.as_str()))
        .chain(std::iter::once(catalog.computer.why.as_str()))
        .collect();
    assert!(whys.iter().all(|w| !w.trim().is_empty()));
}
