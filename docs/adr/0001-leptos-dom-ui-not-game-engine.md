# Leptos DOM UI with a pure-Rust simulation core, not a game engine

The game is mostly forms, component cards and resource dashboards, with one orbit animation. We build the UI with Leptos (client-side, built with Trunk) rendering to the DOM and styled with CSS, drawing the orbit in SVG. All mission rules and the day-by-day Run live in a separate `sim` module with no web dependencies, so they can be tested with plain `cargo test`.

## Considered Options

- **Bevy / macroquad**: real game engines, but form-heavy UI is painful in them and the wasm bundle is heavy.
- **egui/eframe**: the fastest to build, but it looks like an engineering tool rather than a game.
- **Rust core with a TypeScript UI**: drops the "all Rust" goal for no real gain.
