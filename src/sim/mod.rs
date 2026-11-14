//! Pure simulation core. Must never depend on web/UI code.
//! The module root only declares and re-exports; one file per concern.
mod catalog;
mod design;

pub use catalog::*;
pub use design::*;

/// One-line game pitch shown on the title screen.
pub fn tagline() -> &'static str {
    "Design the mission. Survive the physics."
}
