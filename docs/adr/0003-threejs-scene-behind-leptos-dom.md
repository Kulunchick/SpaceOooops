# A three.js scene behind the Leptos DOM, described by a pure-Rust Scene Spec

Partly supersedes ADR 0001. The game now needs a low-poly 3D look, which DOM, CSS and SVG cannot give. Forms, cards, gauges and panels stay Leptos DOM, as 0001 decided. Behind them, one full-viewport WebGL canvas is mounted once at the app root and kept across every screen. three.js draws it through a thin JavaScript bridge (`js/bridge/`) called from Rust with `wasm-bindgen`. three.js is pinned to one exact version and vendored in the repo (`vendor/three/`); Trunk copies it and the bridge as-is, with no bundler and no runtime CDN. What the scene shows is decided by a pure-Rust Scene Spec module (`src/scene/`, no web dependencies, tested with `cargo test`); the bridge only decides how to draw it. Without WebGL the canvas is not used, a static background shows instead, and the game plays exactly as before.

## Considered Options

- **Bevy**: a real engine, but rebuilding the form-heavy UI in it would not fit the deadline, and the wasm bundle is heavy.
- **`three-d` crate**: all Rust, but fewer examples and riskier to debug in a day.
- **Loading three.js from a CDN**: smaller repo, but the deployed game would depend on a third-party host at runtime.

## Consequences

- Rust and JavaScript meet at one narrow API (`init`, `apply(sceneSpec)`, graphics level, reduced motion, audio). The Scene Spec travels as JSON.
- The bridge's look is checked by eye in the browser; only the Scene Spec and smoke tests (Playwright) are automated.
- three.js is shipped whole (about 2 MB uncompressed, much less gzipped) because there is no bundler to tree-shake it.

## Decisions made while building

- **Two bridge files with externs.** `src/ui/scene_bridge.rs` holds the externs for the renderer (`js/bridge/index.js`); `src/ui/audio_bridge.rs` holds those for `js/bridge/audio.js`, which is loaded on its own so that sound works without WebGL.
- **UI click sound is chosen in JS, not as a Rust Cue.** `audio.js` plays `click` for any click on a button, role=button or link. A click is not a change of game state, so it has no place in the Scene Spec; everything state-driven (snap, launch, failure, votes) is a Rust Cue.
- **Reduced motion keeps the 3D Run, frozen.** With `prefers-reduced-motion` the Run stays in the 3D scene with camera moves cut, idle drift off and the launch sequence skipped; it does not fall back to the SVG orbit view. (The spec's Solution paragraph says fall back; its Implementation Decisions say motion off. The code follows the latter. The SVG view is for browsers without WebGL.)
- **Random Event days play slowly only while the 3D scene is live** (and motion is not reduced), so their effects and Cues can be seen and heard. This changes the pace of the playback, never the Run's results.
- **Graphics (Low/High) changes the face count and pixel ratio live; antialiasing is fixed when the page starts**, because a WebGL context's attributes cannot change on a canvas that already has one.
