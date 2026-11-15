//! Scene Spec: the new Random Events (More Random Events ticket 08) as six effect kinds in the Run scene, each
//! with its own Cue. Built from the Run timeline and the playback position only, like the older effects.
mod common;

use common::*;
use spaceooops::scene::{self, Cue, Effect, SceneInputs, Screen};
use spaceooops::sim::{run, Catalog, Design, EventKind, RunResult};

/// (scene kind in the data file, optional model, catalog id) of the six test events.
const SIX: [(&str, Option<&str>, &str); 6] = [
    ("fly-by", Some("ufo"), "t-fly-by"),
    ("hit", None, "t-hit"),
    ("link-interference", None, "t-link"),
    ("panel-shading", None, "t-shading"),
    ("bonus-glow", None, "t-glow"),
    ("visitor", Some("whale"), "t-visitor"),
];

/// A passive event of the given scene kind (a small Power change, so the Run is barely touched).
fn event_toml(scene: &str, model: Option<&str>, id: &str) -> String {
    let model = model.map(|m| format!("model = \"{m}\"\n")).unwrap_or_default();
    format!(
        "[[events]]\nid = \"{id}\"\ngroup = \"lucky\"\ntitle = \"Test {id}\"\nlog = \"Something happened ({id}).\"\n\
         scene = \"{scene}\"\n{model}effects = [{{ mechanic = \"power\", strength = 0.01, days = 3 }}]\n\n"
    )
}

fn catalog_of(events: &[(&str, Option<&str>, &str)], count: u32) -> Catalog {
    let toml: String = events.iter().map(|(s, m, i)| event_toml(s, *m, i)).collect();
    catalog_with_events(&toml, count, count)
}

fn spec_effects(c: &Catalog, d: &Design, r: &RunResult, playback: f64) -> Vec<Effect> {
    scene::build(&SceneInputs {
        catalog: Some(c),
        design: Some(d),
        chosen: Some(CROP_WATCH),
        run: Some(r),
        playback,
        ..SceneInputs::on(Screen::Run)
    })
    .effects
}

/// The day (1-based) each new Random Event fired on, with its catalog id and scene days.
fn fired(r: &RunResult) -> Vec<(u32, String, u32)> {
    r.timeline
        .iter()
        .flat_map(|t| {
            t.events.iter().filter(|e| e.kind == EventKind::Catalog).map(move |e| {
                let c = e.catalog.as_ref().unwrap();
                (t.day, c.id.clone(), c.scene_days)
            })
        })
        .collect()
}

fn is_new(e: &Effect) -> bool {
    e.id.contains("-event-")
}

#[test]
fn each_of_the_six_kinds_is_in_the_scene_on_its_days_with_a_stable_id_and_nothing_outside() {
    let c = catalog_of(&SIX, 6);
    let d = crop_watch_known_good();
    let r = run(&c, &d, CROP_WATCH, 7);
    let fired = fired(&r);
    assert_eq!(fired.len(), 6, "all six test events fire");
    let wanted = ["flyBy", "hit", "linkInterference", "panelShading", "bonusGlow", "visitor"];
    let mut kinds = Vec::new();
    for (day, id, scene_days) in &fired {
        let (start, days) = (f64::from(*day) - 1.0, f64::from(*scene_days));
        let want_id = format!("day-{day}-event-{id}");
        let all: Vec<Effect> = spec_effects(&c, &d, &r, start + 1e-6).into_iter().filter(is_new).collect();
        let e = all.iter().find(|e| e.id == want_id).unwrap_or_else(|| panic!("{want_id} in the scene"));
        assert_eq!((e.start, e.days), (start, days.min(r.timeline.len() as f64 - start)));
        // Active through its last day, gone just after, and not there just before it starts.
        assert!(spec_effects(&c, &d, &r, start + days.min(r.timeline.len() as f64 - start)).iter().any(|x| x.id == want_id)
            || start + days >= r.timeline.len() as f64);
        assert!(!spec_effects(&c, &d, &r, start).iter().any(|x| x.id == want_id), "not before its first day");
        assert!(!spec_effects(&c, &d, &r, start + days + 1e-6).iter().any(|x| x.id == want_id), "not after its days");
        kinds.push(serde_json::to_value(e).unwrap()["kind"].as_str().unwrap().to_string());
    }
    kinds.sort();
    let mut wanted: Vec<String> = wanted.iter().map(|s| s.to_string()).collect();
    wanted.sort();
    assert_eq!(kinds, wanted, "one effect of every kind");
}

#[test]
fn an_effect_never_outlasts_the_run() {
    let c = catalog_of(&SIX, 6);
    for d in [crop_watch_known_good(), all_basic_crop_watch()] {
        for seed in 0..40 {
            let r = run(&c, &d, CROP_WATCH, seed);
            for t in 1..=r.timeline.len() {
                for e in spec_effects(&c, &d, &r, t as f64) {
                    assert!(e.start + e.days <= r.timeline.len() as f64 + 1e-9, "seed {seed}: {}", e.id);
                }
            }
        }
    }
}

fn all_basic_crop_watch() -> Design {
    let mut d = crop_watch_known_good();
    for p in [&mut d.solar, &mut d.battery, &mut d.attitude] {
        if let Some(p) = p {
            p.tier = spaceooops::sim::ReliabilityTier::Basic;
        }
    }
    d
}

#[test]
fn the_model_name_travels_in_the_spec_and_is_absent_without_one() {
    let c = catalog_of(&SIX, 6);
    let d = crop_watch_known_good();
    let r = run(&c, &d, CROP_WATCH, 7);
    let json = |kind: &str| -> serde_json::Value {
        for (day, id, _) in fired(&r) {
            let e = spec_effects(&c, &d, &r, f64::from(day) - 0.5).into_iter().find(|e| e.id.ends_with(&id)).unwrap();
            let v = serde_json::to_value(&e).unwrap();
            if v["kind"] == kind {
                return v;
            }
        }
        panic!("no {kind}");
    };
    assert_eq!(json("flyBy")["model"], "ufo");
    assert_eq!(json("visitor")["model"], "whale");
    assert!(json("linkInterference").get("model").is_none());
    assert!(json("bonusGlow").get("model").is_none());
    let bare = catalog_of(&[("fly-by", None, "t-bare")], 1);
    let rb = run(&bare, &d, CROP_WATCH, 7);
    let day = fired(&rb)[0].0;
    let v = serde_json::to_value(&spec_effects(&bare, &d, &rb, f64::from(day) - 0.5)[0]).unwrap();
    assert!(v.get("model").is_none(), "an unnamed model is not in the JSON: {v}");
}

#[test]
fn a_hit_names_the_designs_solar_panel_part_or_the_bus() {
    let c = catalog_of(&[("hit", None, "t-hit")], 1);
    let d = crop_watch_known_good();
    let r = run(&c, &d, CROP_WATCH, 3);
    let day = fired(&r)[0].0;
    let at = |d: &Design| -> serde_json::Value {
        let e = spec_effects(&c, d, &r, f64::from(day) - 0.5).into_iter().find(is_new).unwrap();
        serde_json::to_value(e).unwrap()
    };
    assert_eq!(at(&d)["component"], d.solar.as_ref().unwrap().id.as_str());
    let mut no_solar = d.clone();
    no_solar.solar = None;
    assert!(at(&no_solar).get("component").is_none(), "no panel: the bus is hit");
}

#[test]
fn each_kind_has_its_own_cue_and_name() {
    let cues = [Cue::FlyBy, Cue::Hit, Cue::LinkInterference, Cue::PanelShading, Cue::BonusGlow, Cue::Visitor];
    let names: Vec<&str> = cues.iter().map(|c| c.name()).collect();
    assert_eq!(names, ["fly-by", "hit", "interference", "shading", "bonus", "visitor"]);
}

/// Plays a Run back in small steps and collects the Cues heard, with the day the playback was on.
fn heard(c: &Catalog, d: &Design, r: &RunResult, step: f64) -> Vec<(u32, Cue)> {
    let mut out = Vec::new();
    let mut previous = scene::build(&inputs(c, d, r, 0.0));
    let mut p = 0.0;
    while p < r.timeline.len() as f64 {
        p = (p + step).min(r.timeline.len() as f64);
        let current = scene::build(&inputs(c, d, r, p));
        out.extend(scene::one_shots(&previous, &current).into_iter().map(|cue| (p.ceil() as u32, cue)));
        previous = current;
    }
    out
}

fn inputs<'a>(c: &'a Catalog, d: &'a Design, r: &'a RunResult, playback: f64) -> SceneInputs<'a> {
    SceneInputs {
        catalog: Some(c),
        design: Some(d),
        chosen: Some(CROP_WATCH),
        run: Some(r),
        playback,
        ..SceneInputs::on(Screen::Run)
    }
}

#[test]
fn each_kind_plays_its_cue_once_when_its_effect_starts() {
    let d = crop_watch_known_good();
    let want = [Cue::FlyBy, Cue::Hit, Cue::LinkInterference, Cue::PanelShading, Cue::BonusGlow, Cue::Visitor];
    for ((scene_kind, model, id), cue) in SIX.iter().zip(want) {
        let c = catalog_of(&[(*scene_kind, *model, *id)], 1);
        let r = run(&c, &d, CROP_WATCH, 11);
        let day = fired(&r)[0].0;
        let got: Vec<(u32, Cue)> = heard(&c, &d, &r, 0.3).into_iter().filter(|(_, c)| *c == cue).collect();
        assert_eq!(got, vec![(day, cue)], "{scene_kind}: heard once, on its day");
    }
}

#[test]
fn a_skip_over_an_event_plays_no_cue_for_it() {
    let d = crop_watch_known_good();
    let c = catalog_of(&[("fly-by", None, "t-skip")], 1);
    let r = run(&c, &d, CROP_WATCH, 11);
    let day = f64::from(fired(&r)[0].0);
    let before = scene::build(&inputs(&c, &d, &r, day - 20.0));
    let after = scene::build(&inputs(&c, &d, &r, day + 20.0));
    assert!(scene::one_shots(&before, &after).is_empty());
}
