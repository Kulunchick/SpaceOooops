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
    if value == value.trunc() && value.abs() >= 10.0 {
        format!("{value:.0}")
    } else if value.abs() < 0.1 && value != 0.0 {
        format!("{value:.4}")
    } else {
        format!("{value:.1}")
    }
}

pub fn percent(fraction: f64) -> String {
    format!("{:.0}%", fraction * 100.0)
}
