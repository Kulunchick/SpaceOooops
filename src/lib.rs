//! Library crate: holds the pure-Rust `sim` (no web/UI deps).
//! The UI lives in the binary (`main.rs`, `ui/`) and may use `sim`, never the reverse.
pub mod sim;
