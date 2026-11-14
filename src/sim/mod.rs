//! Pure simulation core. Must never depend on web/UI code.

/// One-line game pitch shown on the title screen (placeholder public API until ticket 02).
pub fn tagline() -> &'static str {
    "Design the mission. Survive the physics."
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tagline_is_the_game_pitch() {
        assert_eq!(tagline(), "Design the mission. Survive the physics.");
    }
}
