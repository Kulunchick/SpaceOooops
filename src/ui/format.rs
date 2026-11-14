//! Number formatting for display only.

pub fn money(usd: f64) -> String {
    if usd.abs() >= 1_000_000.0 {
        format!("${:.2}M", usd / 1_000_000.0)
    } else if usd.abs() >= 1_000.0 {
        format!("${:.0}k", usd / 1_000.0)
    } else {
        format!("${usd:.0}")
    }
}

pub fn num(value: f64) -> String {
    let value = value + 0.0; // -0.0 + 0.0 is +0.0: never print "-0"
    if value == value.trunc() && value.abs() >= 10.0 {
        format!("{value:.0}")
    } else if value.abs() < 0.1 && value != 0.0 {
        format!("{value:.4}")
    } else {
        format!("{value:.1}")
    }
}

/// A Data volume: MB below 1 GB, GB above ("108.9 GB" rather than "108900 MB").
pub fn data_volume(mb: f64) -> String {
    let mb = mb + 0.0; // never "-0"
    if mb.abs() >= 1_000.0 {
        format!("{:.1} GB", mb / 1_000.0)
    } else if mb.abs() >= 10.0 {
        format!("{mb:.0} MB")
    } else {
        format!("{mb:.1} MB")
    }
}

pub fn percent(fraction: f64) -> String {
    format!("{:.0}%", fraction * 100.0)
}
