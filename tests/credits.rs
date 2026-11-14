//! Credits: every external asset shipped with the game is listed in `data/credits.toml` with its licence.
use std::path::{Path, PathBuf};

use serde::Deserialize;

#[derive(Deserialize)]
struct Credits {
    asset: Vec<Asset>,
}

#[derive(Deserialize)]
struct Asset {
    name: String,
    author: String,
    source: String,
    licence: String,
    licence_file: String,
    kind: String,
    #[allow(dead_code)]
    ai_generated: bool,
    files: Vec<String>,
}

const ROOT: &str = env!("CARGO_MANIFEST_DIR");

fn credits() -> Credits {
    let text = std::fs::read_to_string(Path::new(ROOT).join("data/credits.toml")).unwrap();
    toml::from_str(&text).unwrap()
}

/// External asset folders: every file in them except licence texts must be credited.
const EXTERNAL_DIRS: &[&str] = &["vendor", "assets/fonts", "assets/models", "assets/audio"];

fn files_under(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            files_under(&path, out);
        } else {
            out.push(path);
        }
    }
}

#[test]
fn every_credited_asset_names_its_author_source_and_licence() {
    for a in credits().asset {
        for (field, value) in [("author", &a.author), ("source", &a.source), ("licence", &a.licence), ("kind", &a.kind)] {
            assert!(!value.trim().is_empty(), "{} has no {field}", a.name);
        }
        assert!(Path::new(ROOT).join(&a.licence_file).is_file(), "{}: licence file {} missing", a.name, a.licence_file);
        for f in &a.files {
            assert!(Path::new(ROOT).join(f).is_file(), "{}: file {f} missing", a.name);
        }
    }
}

#[test]
fn threejs_and_both_fonts_are_credited() {
    let names: Vec<String> = credits().asset.into_iter().map(|a| a.name).collect();
    for wanted in ["three.js", "Russo One", "Atkinson Hyperlegible"] {
        assert!(names.iter().any(|n| n.contains(wanted)), "{wanted} not credited: {names:?}");
    }
}

#[test]
fn every_file_in_an_external_asset_folder_is_credited() {
    let credited: Vec<String> = credits().asset.into_iter().flat_map(|a| a.files).collect();
    let mut found = Vec::new();
    for dir in EXTERNAL_DIRS {
        files_under(&Path::new(ROOT).join(dir), &mut found);
    }
    for path in found {
        let rel = path.strip_prefix(ROOT).unwrap().to_string_lossy().replace('\\', "/");
        let file_name = path.file_name().unwrap().to_string_lossy().to_uppercase();
        if file_name.starts_with("LICENSE") || file_name.starts_with("LICENCE") || file_name.starts_with("OFL") {
            continue;
        }
        assert!(credited.contains(&rel), "{rel} is not listed in data/credits.toml");
    }
}

// ---- The shared credits module (Sources page and README) ----

use spaceooops::credits as shared;

fn readme() -> String {
    std::fs::read_to_string(Path::new(ROOT).join("README.md")).unwrap().replace("\r\n", "\n")
}

/// The text between two marker comments in the README.
fn readme_block(start: &str, end: &str) -> String {
    let text = readme();
    let from = text.find(start).unwrap_or_else(|| panic!("README has no {start} marker")) + start.len();
    let to = text.find(end).unwrap_or_else(|| panic!("README has no {end} marker"));
    text[from..to].trim().to_string()
}

#[test]
fn the_credits_module_reads_the_same_entries_as_the_data_file() {
    let names: Vec<String> = shared::assets().into_iter().map(|a| a.name).collect();
    let expected: Vec<String> = credits().asset.into_iter().map(|a| a.name).collect();
    assert_eq!(names, expected);
}

#[test]
fn the_readme_asset_list_is_exactly_the_credits_file() {
    let expected = shared::readme_list(&shared::assets());
    let found = readme_block("<!-- credits:start -->", "<!-- credits:end -->");
    assert_eq!(
        found, expected,
        "README asset list is out of step with data/credits.toml. Replace the block between the credits markers with:\n{expected}"
    );
}

#[test]
fn the_readme_ai_disclosure_matches_the_credits_file() {
    let expected = shared::ai_statement(&shared::assets());
    let found = readme_block("<!-- ai-assets:start -->", "<!-- ai-assets:end -->");
    assert_eq!(
        found, expected,
        "README AI disclosure is out of step with data/credits.toml. Replace the block between the ai-assets markers with:\n{expected}"
    );
}

fn asset(name: &str, kind: &str, ai: bool) -> shared::Asset {
    shared::Asset {
        name: name.into(),
        author: "A".into(),
        source: "https://example.org".into(),
        licence: "CC0".into(),
        licence_file: "x".into(),
        kind: kind.into(),
        ai_generated: ai,
        files: vec![],
    }
}

#[test]
fn with_no_ai_assets_the_statement_says_none_were_made_with_ai() {
    let s = shared::ai_statement(&[asset("Tree", "model", false)]);
    assert!(s.contains("No image, music or sound"), "{s}");
}

#[test]
fn ai_generated_assets_are_named_in_the_statement() {
    let s = shared::ai_statement(&[
        asset("Tree", "model", false),
        asset("Menu theme", "music", true),
        asset("Skybox", "image", true),
    ]);
    assert!(s.contains("Menu theme") && s.contains("Skybox"), "{s}");
    assert!(!s.contains("Tree"), "{s}");
    assert!(!s.contains("No image, music or sound"), "{s}");
}

#[test]
fn today_no_credited_asset_is_ai_generated() {
    assert!(shared::assets().iter().all(|a| !a.ai_generated));
}
