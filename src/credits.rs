//! Credits: the external assets in `data/credits.toml`, shared by the Sources page and the README tests.
//! Pure Rust, no web dependencies. To add an asset, add an `[[asset]]` entry to the data file, then update the
//! README block between the `credits` markers (and the `ai-assets` block if `ai_generated` is true): the test
//! `the_readme_asset_list_is_exactly_the_credits_file` prints the exact text to paste.
use serde::Deserialize;

/// One external asset, as listed in `data/credits.toml`.
#[derive(Clone, Debug, Deserialize)]
pub struct Asset {
    pub name: String,
    pub author: String,
    pub source: String,
    pub licence: String,
    pub licence_file: String,
    /// `library`, `font`, `model`, `image`, `audio`, `music` or `texture`.
    pub kind: String,
    pub ai_generated: bool,
    pub files: Vec<String>,
}

#[derive(Deserialize)]
struct File {
    asset: Vec<Asset>,
}

const CREDITS: &str = include_str!("../data/credits.toml");

/// Every credited asset, in file order.
pub fn assets() -> Vec<Asset> {
    toml::from_str::<File>(CREDITS).expect("data/credits.toml is valid").asset
}

/// One README line for an asset.
pub fn readme_line(a: &Asset) -> String {
    format!("- {}, by {}. {} ({}). Source: {}", a.name, a.author, a.licence, a.kind, a.source)
}

/// The README asset list: one line per asset, in file order.
pub fn readme_list(assets: &[Asset]) -> String {
    assets.iter().map(readme_line).collect::<Vec<_>>().join("\n")
}

/// The sentence about AI-made images, music and sound, driven by each asset's `ai_generated` field.
pub fn ai_statement(assets: &[Asset]) -> String {
    let made: Vec<String> =
        assets.iter().filter(|a| a.ai_generated).map(|a| format!("{} ({})", a.name, a.kind)).collect();
    if made.is_empty() {
        "No image, music or sound effect in this game was made with AI tools.".to_string()
    } else {
        format!(
            "These assets were made with AI tools: {}. Every other asset is credited to its human author.",
            made.join(", ")
        )
    }
}
