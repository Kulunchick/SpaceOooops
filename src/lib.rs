//! Library crate: the pure-Rust parts with no web/UI deps: `sim` (the game rules), `scene` (the Scene Spec and
//! its Cues), `settings` (stored player settings) and `credits` (the asset credits file).
//! The UI lives in the binary (`main.rs`, `ui/`) and may use all of them, never the reverse.
pub mod credits;
pub mod scene;
pub mod settings;
pub mod sim;
