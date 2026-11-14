//! Player settings, kept as `key=value` lines in browser local storage by the UI (`ui/settings_store.rs`).
//! Pure text in, text out, so it is testable natively. Unknown keys are ignored: later settings add a field
//! and a line here without breaking stored text.

/// How much the 3D scene draws: Low skips antialiasing, uses fewer faces and a pixel ratio of 1.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Quality {
    Low,
    High,
}

/// What the browser tells us about the machine, gathered by the UI (any hint may be missing).
#[derive(Clone, PartialEq, Debug, Default)]
pub struct DeviceHints {
    /// `navigator.hardwareConcurrency`.
    pub cores: Option<u32>,
    /// `navigator.deviceMemory` in GB (the browser rounds it down to 0.25 ... 8).
    pub memory_gb: Option<f64>,
    /// The unmasked WebGL renderer string.
    pub renderer: Option<String>,
}

impl Quality {
    pub fn name(self) -> &'static str {
        match self {
            Quality::Low => "low",
            Quality::High => "high",
        }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        match name.trim() {
            "low" => Some(Quality::Low),
            "high" => Some(Quality::High),
            _ => None,
        }
    }

    /// The default for a machine nobody has chosen for: Low on a software renderer, on two cores or fewer, or on
    /// 2 GB of memory or less. A missing hint is never a reason for Low.
    pub fn default_for(hints: &DeviceHints) -> Quality {
        let software = hints.renderer.as_deref().is_some_and(|r| {
            let r = r.to_lowercase();
            ["swiftshader", "llvmpipe", "softpipe", "software", "basic render"].iter().any(|s| r.contains(s))
        });
        let few_cores = hints.cores.is_some_and(|c| c <= 2);
        let little_memory = hints.memory_gb.is_some_and(|m| m <= 2.0);
        if software || few_cores || little_memory { Quality::Low } else { Quality::High }
    }

    /// The level in force: the player's saved choice when there is one, otherwise the device default.
    pub fn choose(saved: Option<Quality>, hints: &DeviceHints) -> Quality {
        saved.unwrap_or_else(|| Quality::default_for(hints))
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Settings {
    /// Music volume, 0.0 to 1.0.
    pub music: f64,
    /// Sound effects volume, 0.0 to 1.0.
    pub effects: f64,
    /// Mutes everything: gains are exactly 0 and nothing is started.
    pub muted: bool,
    /// The player's own quality choice; `None` until they make one (then the device default applies).
    pub quality: Option<Quality>,
}

impl Default for Settings {
    /// Music sits well below effects so it never drowns a click.
    fn default() -> Self {
        Settings { music: 0.3, effects: 0.8, muted: false, quality: None }
    }
}

impl Settings {
    pub fn to_text(&self) -> String {
        let mut text = format!("music={}\neffects={}\nmuted={}", self.music, self.effects, u8::from(self.muted));
        if let Some(q) = self.quality {
            text.push_str(&format!("\nquality={}", q.name()));
        }
        text
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
                "quality" => s.quality = Quality::from_name(value),
                _ => {}
            }
        }
        s
    }
}
