//! Rust side of the audio bridge (`js/bridge/audio.js`). Loaded on its own, never through the renderer's
//! `index.js`, so sound works when WebGL is missing. Calls made before the module has loaded are kept
//! (the latest music track and volumes; one-shot Cues are dropped, since nothing can play before the
//! first click anyway).
use wasm_bindgen::prelude::*;

#[wasm_bindgen(inline_js = r#"
let audio = null;
let pending = { track: null, volumes: null };
const ready = (async () => {
    try {
        audio = await import(new URL('js/bridge/audio.js', document.baseURI).href);
        if (pending.volumes) audio.setVolumes(...pending.volumes);
        audio.setMusic(pending.track);
    } catch (e) {
        console.warn('Audio unavailable:', e);
    }
})();
export function audio_set_music(track) {
    pending.track = track;
    if (audio) audio.setMusic(track);
}
export function audio_set_volumes(music, sfx, muted) {
    pending.volumes = [music, sfx, muted];
    if (audio) audio.setVolumes(music, sfx, muted);
}
export function audio_play(cue) {
    if (audio) audio.play(cue);
}
"#)]
extern "C" {
    /// The music track that should play now ("menu" or "run"); the bridge crossfades.
    pub fn audio_set_music(track: &str);
    /// Music volume, effects volume (0..1) and mute.
    pub fn audio_set_volumes(music: f64, sfx: f64, muted: bool);
    /// A one-shot Cue by name.
    pub fn audio_play(cue: &str);
}
