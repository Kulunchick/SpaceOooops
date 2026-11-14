//! Fresh seeds for each Run. `sim` stays deterministic; only the UI picks a random seed.
use leptos::web_sys::js_sys::Math;

pub fn new_seed() -> u64 {
    let half = || (Math::random() * 4_294_967_296.0) as u64;
    (half() << 32) | half()
}
