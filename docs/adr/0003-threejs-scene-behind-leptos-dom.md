# A three.js scene behind the Leptos DOM, described by a pure-Rust Scene Spec

Partly supersedes ADR 0001. The game now needs a low-poly 3D look, which DOM, CSS and SVG cannot give. Forms, cards, gauges and panels stay Leptos DOM, as 0001 decided. Behind them, one full-viewport WebGL canvas is mounted once at the app root and kept across every screen. three.js draws it through a thin JavaScript bridge (`js/bridge/`) called from Rust with `wasm-bindgen`. three.js is pinned to one exact version and vendored in the repo (`vendor/three/`); Trunk copies it and the bridge as-is, with no bundler and no runtime CDN. What the scene shows is decided by a pure-Rust Scene Spec module (`src/scene/`, no web dependencies, tested with `cargo test`); the bridge only decides how to draw it. Without WebGL the canvas is not used, a static background shows instead, and the game plays exactly as before.

## Considered Options

- **Bevy**: a real engine, but rebuilding the form-heavy UI in it would not fit the deadline, and the wasm bundle is heavy.
- **`three-d` crate**: all Rust, but fewer examples and riskier to debug in a day.
- **Loading three.js from a CDN**: smaller repo, but the deployed game would depend on a third-party host at runtime.

## Consequences

- Rust and JavaScript meet at one narrow API (`init`, `apply(sceneSpec)`, quality, reduced motion, audio). The Scene Spec travels as JSON.
- The bridge's look is checked by eye in the browser; only the Scene Spec and smoke tests (Playwright) are automated.
- three.js is shipped whole (about 2 MB uncompressed, much less gzipped) because there is no bundler to tree-shake it.
