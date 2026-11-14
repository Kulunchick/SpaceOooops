//! Player settings, kept as `key=value` lines in browser local storage by the UI (`ui/settings_store.rs`).
//! Pure text in, text out, so it is testable natively. Unknown keys are ignored: later settings (such as the
//! quality level) add a field and a line here without breaking stored text.

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Settings {
    /// Music volume, 0.0 to 1.0.
    pub music: f64,
    /// Sound effects volume, 0.0 to 1.0.
    pub effects: f64,
    /// Mutes everything: gains are exactly 0 and nothing is started.
    pub muted: bool,
}

impl Default for Settings {
    /// Music sits well below effects so it never drowns a click.
    fn default() -> Self {
        Settings { music: 0.3, effects: 0.8, muted: false }
    }
}

impl Settings {
    pub fn to_text(&self) -> String {
        format!("music={}\neffects={}\nmuted={}", self.music, self.effects, u8::from(self.muted))
    }

    /// Reads stored text; anything missing or unreadable keeps its default.
    pub fn from_text(text: &str) -> Self {
        let mut s = Settings::default();
        let volume = |v: &str| v.trim().parse::<f64>().ok().filter(|x| x.is_finite()).map(|x| x.clamp(0.0, 1.0));
        for line in text.lines() {
            let Some((key, value)) = line.split_once('=') else { continue };
            match key.trim() {
                "music" => s.music = volume(value).unwrap_or(s.music),
                "effects" => s.effects = volume(value).unwrap_or(s.effects),
                "muted" => s.muted = value.trim() == "1",
                _ => {}
            }
        }
        s
    }
}
