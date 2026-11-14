//! Share links: a Design and its Scenario survive an encode/decode round trip; bad input is an error.
mod common;

use common::*;
use spaceooops::sim::*;

fn catalog() -> Catalog {
    load_catalog().unwrap()
}

fn known_good() -> Vec<(&'static str, Design)> {
    vec![
        (CROP_WATCH, crop_watch_known_good()),
        (CONNECT_THE_VILLAGES, connect_the_villages_known_good()),
        (SPACE_WEATHER_SENTINEL, space_weather_sentinel_known_good()),
    ]
}

#[test]
fn known_good_designs_round_trip() {
    let c = catalog();
    for (id, design) in known_good() {
        let text = encode_design(&design, id);
        let (back, scenario) = decode_design(&text, &c).unwrap();
        assert_eq!(back, design);
        assert_eq!(scenario, id);
    }
}

#[test]
fn partial_and_empty_designs_round_trip() {
    let c = catalog();
    let mut partial = crop_watch_known_good();
    partial.solar = None;
    partial.comms = None;
    partial.launch_vehicle = None;
    partial.instruments.clear();
    partial.computer_tier = ReliabilityTier::Basic;
    for d in [partial, Design::default()] {
        let (back, scenario) = decode_design(&encode_design(&d, CROP_WATCH), &c).unwrap();
        assert_eq!(back, d);
        assert_eq!(scenario, CROP_WATCH);
    }
}

#[test]
fn encoding_is_url_safe_and_versioned() {
    let text = encode_design(&crop_watch_known_good(), CROP_WATCH);
    assert!(text.starts_with("v1."), "{text}");
    assert!(text.chars().all(|ch| ch.is_ascii_alphanumeric() || "-._~".contains(ch)), "{text}");
}

#[test]
fn garbage_and_truncated_input_is_an_error() {
    let c = catalog();
    let good = encode_design(&crop_watch_known_good(), CROP_WATCH);
    for bad in ["", "garbage", "v1", "v1.", "....", "\u{1F680}", &good[..good.len() / 2]] {
        assert!(decode_design(bad, &c).is_err(), "{bad:?}");
    }
}

#[test]
fn wrong_version_is_an_error() {
    let c = catalog();
    let good = encode_design(&crop_watch_known_good(), CROP_WATCH);
    let old = good.replacen("v1.", "v0.", 1);
    assert_eq!(decode_design(&old, &c), Err(ShareError::UnsupportedVersion));
    let future = good.replacen("v1.", "v9.", 1);
    assert_eq!(decode_design(&future, &c), Err(ShareError::UnsupportedVersion));
}

#[test]
fn unknown_ids_are_an_error() {
    let c = catalog();
    let mut d = crop_watch_known_good();
    d.orbit = Some("orbit-of-doom".into());
    assert!(matches!(decode_design(&encode_design(&d, CROP_WATCH), &c), Err(ShareError::UnknownId(_))));
    let good = crop_watch_known_good();
    assert!(matches!(decode_design(&encode_design(&good, "no-such-scenario"), &c), Err(ShareError::UnknownId(_))));
    let mut d = crop_watch_known_good();
    d.instruments.push("flux-capacitor".into());
    assert!(matches!(decode_design(&encode_design(&d, CROP_WATCH), &c), Err(ShareError::UnknownId(_))));
}
