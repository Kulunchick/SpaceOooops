//! Scene Spec: the Debrief shots (low-poly ticket 08). The verdict picks the Shot and the Cue (Success or
//! failure); the main cause of a failed Run picks the failure visual. Seeded Runs, like `tests/debrief.rs`.
mod common;

use common::*;
use spaceooops::scene::{self, Cue, Fate, SceneInputs, SceneSpec, Screen, Shot};
use spaceooops::sim::{load_catalog, run, CauseKind, Catalog, Design, Pick, ReliabilityTier::Basic, RunResult};

fn debrief_spec(c: &Catalog, d: &Design, scenario: &str, r: &RunResult) -> SceneSpec {
    scene::build(&SceneInputs {
        catalog: Some(c),
        design: Some(d),
        chosen: Some(scenario),
        run: Some(r),
        ..SceneInputs::on(Screen::Debrief)
    })
}

fn run_spec(c: &Catalog, d: &Design, scenario: &str, r: &RunResult, playback: f64) -> SceneSpec {
    scene::build(&SceneInputs {
        catalog: Some(c),
        design: Some(d),
        chosen: Some(scenario),
        run: Some(r),
        playback,
        ..SceneInputs::on(Screen::Run)
    })
}

/// Every Design and Scenario the tests fly: the known-good ones and variants that fail in different ways.
fn designs() -> Vec<(&'static str, Design)> {
    let mut out = vec![
        (CROP_WATCH, crop_watch_known_good()),
        (CONNECT_THE_VILLAGES, connect_the_villages_known_good()),
        (SPACE_WEATHER_SENTINEL, space_weather_sentinel_known_good()),
    ];
    for (scenario, base) in out.clone() {
        let mut all_basic = base.clone();
        for slot in [&mut all_basic.solar, &mut all_basic.battery, &mut all_basic.attitude, &mut all_basic.comms] {
            if let Some(p) = slot {
                *p = Pick::new(&p.id, Basic);
            }
        }
        all_basic.computer_tier = Basic;
        out.push((scenario, all_basic));
    }
    let mut small_battery = crop_watch_known_good();
    small_battery.battery = Some(Pick::new("battery-s", Basic));
    out.push((CROP_WATCH, small_battery));
    let mut uhf = crop_watch_known_good();
    uhf.comms = Some(Pick::new("comms-uhf", Basic));
    out.push((CROP_WATCH, uhf));
    let mut leo = crop_watch_known_good();
    leo.orbit = Some("leo-400".to_string());
    leo.launch_vehicle = Some("station-deploy".to_string());
    out.push((CROP_WATCH, leo));
    out
}

/// Every Run of `designs()` over seeds 0..150 as (catalog-free) tuples.
fn runs(c: &Catalog) -> Vec<(&'static str, Design, RunResult)> {
    let mut out = Vec::new();
    for (scenario, design) in designs() {
        for seed in 0..150 {
            out.push((scenario, design.clone(), run(c, &design, scenario, seed)));
        }
    }
    out
}

fn fate_of(spec: &SceneSpec) -> Fate {
    spec.debrief.as_ref().expect("a Debrief scene").fate
}

#[test]
fn a_success_holds_the_hero_shot_with_the_spacecraft() {
    let c = load_catalog().unwrap();
    let d = crop_watch_known_good();
    let r = (0..50).map(|s| run(&c, &d, CROP_WATCH, s)).find(|r| r.debrief.success).expect("a Success");
    let spec = debrief_spec(&c, &d, CROP_WATCH, &r);
    assert_eq!(spec.shot, Shot::DebriefHero);
    assert_eq!(fate_of(&spec), Fate::Hero);
    assert!(spec.debrief.as_ref().unwrap().success);
    assert!(spec.spacecraft.is_some(), "the Design's spacecraft is in the Debrief scene");
    assert!(spec.dimmed, "the scene is dimmed behind the Debrief panel");
}

#[test]
fn the_verdict_picks_the_shot_for_every_seeded_run() {
    let c = load_catalog().unwrap();
    let (mut successes, mut failures) = (0, 0);
    for (scenario, design, r) in runs(&c) {
        let spec = debrief_spec(&c, &design, scenario, &r);
        if r.debrief.success {
            successes += 1;
            assert_eq!(spec.shot, Shot::DebriefHero);
            assert_eq!(fate_of(&spec), Fate::Hero);
        } else {
            failures += 1;
            let fate = fate_of(&spec);
            assert_ne!(fate, Fate::Hero, "a failed Run never shows the Success shot");
            assert_eq!(spec.shot, Shot::DebriefFailure { fate });
            assert!(!spec.debrief.as_ref().unwrap().success);
        }
    }
    assert!(successes > 0 && failures > 0, "the sample has both verdicts ({successes}/{failures})");
}

#[test]
fn every_cause_the_sim_can_produce_maps_to_a_visual() {
    use CauseKind::*;
    // Written out by hand (not derived from the code): the visual for each cause when no failed part is known.
    let expected = [
        (BatteryTooSmall, Fate::Dark),
        (DataLost, Fate::Adrift),
        (ComponentFailure, Fate::Adrift),
        (ScienceShortfall, Fate::Adrift),
        (LifetimeShortfall, Fate::Reentry),
        (PoorCoverage, Fate::Adrift),
        (PoorInstrumentFit, Fate::Adrift),
        (RandomEvents, Fate::Adrift),
        (NotLaunched, Fate::Adrift),
        (WentWell, Fate::Adrift),
        (ScoreLimit, Fate::Adrift),
    ];
    // An exhaustive match: a new CauseKind will not compile until it is listed above too.
    for (kind, _) in expected {
        match kind {
            BatteryTooSmall | DataLost | ComponentFailure | ScienceShortfall | LifetimeShortfall | PoorCoverage
            | PoorInstrumentFit | RandomEvents | NotLaunched | WentWell | ScoreLimit => {}
        }
    }
    for (kind, fate) in expected {
        assert_eq!(scene::fate_for_cause(kind, None), fate, "{kind:?}");
        assert_ne!(scene::fate_for_cause(kind, None), Fate::Hero, "{kind:?} on a failure is never the hero shot");
    }
}

#[test]
fn a_failed_component_picks_the_visual_by_what_it_does() {
    use spaceooops::scene::ComponentClass::*;
    let f = |part| scene::fate_for_cause(CauseKind::ComponentFailure, Some(part));
    assert_eq!(f(Attitude), Fate::Tumbling);
    assert_eq!(f(Solar), Fate::Dark);
    assert_eq!(f(Battery), Fate::Dark);
    assert_eq!(f(Comms), Fate::Adrift);
    assert_eq!(f(Computer), Fate::Adrift);
}

#[test]
fn each_failure_visual_is_reached_by_some_real_run_and_follows_its_main_cause() {
    let c = load_catalog().unwrap();
    let mut seen = std::collections::BTreeSet::new();
    for (scenario, design, r) in runs(&c) {
        if r.debrief.success {
            continue;
        }
        let spec = debrief_spec(&c, &design, scenario, &r);
        let fate = fate_of(&spec);
        let main = r.debrief.causes.first().map(|c| c.kind);
        match main {
            Some(CauseKind::BatteryTooSmall) => assert_eq!(fate, Fate::Dark),
            Some(CauseKind::LifetimeShortfall) => assert_eq!(fate, Fate::Reentry),
            Some(CauseKind::ComponentFailure) => {
                assert!(matches!(fate, Fate::Tumbling | Fate::Dark | Fate::Adrift), "{fate:?}")
            }
            _ => assert_eq!(fate, Fate::Adrift, "{main:?}"),
        }
        seen.insert(format!("{fate:?}"));
    }
    for want in ["Tumbling", "Dark", "Reentry", "Adrift"] {
        assert!(seen.contains(want), "no seeded Run reached {want}: saw {seen:?}");
    }
}

#[test]
fn the_failure_visual_reaches_the_bridge_as_a_camel_case_shot_and_scene() {
    let c = load_catalog().unwrap();
    let (scenario, design, r) = runs(&c)
        .into_iter()
        .find(|(_, _, r)| !r.debrief.success && r.debrief.causes.first().is_some_and(|c| c.kind == CauseKind::LifetimeShortfall))
        .expect("a Run cut short by Lifetime");
    let json = scene::to_json(&debrief_spec(&c, &design, scenario, &r));
    let v: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(v["shot"]["kind"], "debriefFailure");
    assert_eq!(v["shot"]["fate"], "reentry");
    assert_eq!(v["debrief"]["fate"], "reentry");
    assert_eq!(v["debrief"]["success"], false);
}

#[test]
fn a_debrief_without_a_run_has_no_scene_and_keeps_the_old_shot() {
    let spec = scene::build(&SceneInputs::on(Screen::Debrief));
    assert!(spec.debrief.is_none());
    assert_eq!(spec.shot, Shot::TitleOrbit);
}

#[test]
fn other_screens_have_no_debrief_scene() {
    let c = load_catalog().unwrap();
    let d = crop_watch_known_good();
    let r = run(&c, &d, CROP_WATCH, 1);
    let spec = run_spec(&c, &d, CROP_WATCH, &r, 3.0);
    assert!(spec.debrief.is_none());
}

#[test]
fn every_run_that_failed_to_launch_still_gets_a_failure_shot() {
    let c = load_catalog().unwrap();
    let mut d = crop_watch_known_good();
    d.launch_vehicle = None; // never passes Launch Review
    let r = run(&c, &d, CROP_WATCH, 1);
    assert!(!r.debrief.success);
    let spec = debrief_spec(&c, &d, CROP_WATCH, &r);
    assert!(matches!(spec.shot, Shot::DebriefFailure { .. }));
}

// ---------------------------------------------------------------- Cues

fn verdict_cues(prev: &SceneSpec, now: &SceneSpec) -> Vec<Cue> {
    scene::one_shots(prev, now).into_iter().filter(|c| matches!(c, Cue::Success | Cue::MissionFailed)).collect()
}

#[test]
fn arriving_from_the_run_plays_the_verdict_cue_once() {
    let c = load_catalog().unwrap();
    let all = runs(&c);
    let (s, d, win) = all.iter().find(|(_, _, r)| r.debrief.success).unwrap();
    let (fs, fd, loss) = all.iter().find(|(_, _, r)| !r.debrief.success && !r.timeline.is_empty()).unwrap();

    let end = win.timeline.len() as f64;
    let on_run = run_spec(&c, d, s, win, end);
    let debrief = debrief_spec(&c, d, s, win);
    assert_eq!(verdict_cues(&on_run, &debrief), vec![Cue::Success]);
    // Re-rendering the Debrief (an equal spec, or any later one) is silent.
    assert!(verdict_cues(&debrief, &debrief.clone()).is_empty());

    let on_run = run_spec(&c, fd, fs, loss, loss.timeline.len() as f64);
    let debrief = debrief_spec(&c, fd, fs, loss);
    assert_eq!(verdict_cues(&on_run, &debrief), vec![Cue::MissionFailed]);
    assert!(verdict_cues(&debrief, &debrief.clone()).is_empty());
}

#[test]
fn skipping_from_the_middle_of_the_run_still_plays_the_verdict() {
    let c = load_catalog().unwrap();
    let d = crop_watch_known_good();
    let r = (0..50).map(|s| run(&c, &d, CROP_WATCH, s)).find(|r| r.debrief.success).unwrap();
    let mid = run_spec(&c, &d, CROP_WATCH, &r, 2.5);
    assert_eq!(verdict_cues(&mid, &debrief_spec(&c, &d, CROP_WATCH, &r)), vec![Cue::Success]);
}

#[test]
fn a_debrief_reached_from_anywhere_but_the_run_is_silent() {
    let c = load_catalog().unwrap();
    let d = crop_watch_known_good();
    let r = run(&c, &d, CROP_WATCH, 1);
    let debrief = debrief_spec(&c, &d, CROP_WATCH, &r);
    for screen in [Screen::History, Screen::Title, Screen::Design] {
        let before = scene::build(&SceneInputs { catalog: Some(&c), design: Some(&d), ..SceneInputs::on(screen) });
        assert!(verdict_cues(&before, &debrief).is_empty(), "{screen:?}");
    }
}

#[test]
fn leaving_the_debrief_plays_no_verdict_cue() {
    let c = load_catalog().unwrap();
    let d = crop_watch_known_good();
    let r = run(&c, &d, CROP_WATCH, 1);
    let debrief = debrief_spec(&c, &d, CROP_WATCH, &r);
    let design = scene::build(&SceneInputs { catalog: Some(&c), design: Some(&d), ..SceneInputs::on(Screen::Design) });
    assert!(verdict_cues(&debrief, &design).is_empty());
    assert!(scene::Cue::Success.name() != scene::Cue::MissionFailed.name());
}

#[test]
fn modify_design_returns_to_the_hangar_with_the_same_spacecraft() {
    let c = load_catalog().unwrap();
    let d = crop_watch_known_good();
    let r = run(&c, &d, CROP_WATCH, 1);
    let debrief = debrief_spec(&c, &d, CROP_WATCH, &r);
    let design = scene::build(&SceneInputs { catalog: Some(&c), design: Some(&d), ..SceneInputs::on(Screen::Design) });
    assert_eq!(design.shot, Shot::Hangar);
    assert_eq!(design.spacecraft, debrief.spacecraft, "the same Design builds the same spacecraft");
    assert!(design.debrief.is_none());
}
