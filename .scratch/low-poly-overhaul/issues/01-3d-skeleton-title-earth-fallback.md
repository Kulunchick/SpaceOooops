# 01: 3D skeleton: low-poly Earth on the title screen, with a fallback

**What to build:** Opening the game shows the title screen over a full-viewport, flat-shaded, low-poly 3D scene. Earth is an icosphere with coloured faces for ocean, land, ice and clouds, and Angola is recognisable. A small spacecraft circles Earth, and the camera drifts slowly. The title panel uses the new low-poly skin: a palette as CSS custom properties, faceted panels, a chunky display font and a readable body font. The canvas is mounted once and stays behind every screen, so later tickets only change what it shows. If the browser has no WebGL, the title shows a static background instead, and the game is fully playable. This ticket also lays the foundations every later ticket builds on:

- a new ADR that partly supersedes ADR 0001: forms stay in the Leptos DOM, and a three.js canvas sits behind them;
- a thin JS bridge with `init`, `apply(sceneSpec)` and WebGL detection, with three.js pinned to an exact version and bundled by Trunk (not loaded from a CDN at runtime);
- a pure-Rust Scene Spec module with no web dependencies, in its first form: a Shot per screen, starting with the title shot;
- a credits data file that lists every external asset (three.js, fonts) with its source and licence;
- Playwright smoke tests in CI.

**Blocked by:** None (can start immediately).

**Status:** ready-for-agent

- [ ] New ADR recorded; ADR 0001 notes it is partly superseded
- [ ] Title screen shows the low-poly Earth with an orbiting spacecraft and a slow camera drift
- [ ] Title panel uses the new skin; text is readable over the scene
- [ ] Without WebGL a static background is shown and the game still starts and plays
- [ ] Scene Spec module exists; `cargo test` covers screen → Shot for the title
- [ ] Existing `sim` tests pass unchanged
- [ ] Playwright runs in CI before deploy: the page loads with no console errors, the canvas initialises, and the fallback appears with WebGL disabled
- [ ] Credits data file lists three.js and the fonts with their licences
- [ ] Deployed build works on GitHub Pages
