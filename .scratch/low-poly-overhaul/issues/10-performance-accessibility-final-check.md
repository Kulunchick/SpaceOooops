# 10: Performance, accessibility and the final check

**What to build:**

- **Quality toggle:** the settings control gains Low and High quality. Low turns off shadows, reduces geometry detail and lowers the pixel ratio. The choice is remembered, and Low is the default on weak devices.
- **Reduced motion:** with `prefers-reduced-motion`, the idle drift is off, camera moves cut instead of tween, and the launch animation is skipped.
- **Phones:** the game stays usable at phone width, with a reduced scene and panels stacked.
- **Loading:** the first panel shows immediately, and the scene fades in when its assets have loaded.
- **Final check:** the full Playwright suite runs in CI before deploy.

**Blocked by:** 03, 05, 06, 07, 08, 09.

**Status:** resolved

- [x] The quality toggle works and persists
- [x] `prefers-reduced-motion` is respected everywhere
- [x] The game is usable at phone width
- [x] The first panel appears before the 3D assets have loaded
- [x] Playwright: a full Crop Watch Mission (Design → Launch Review → Run → Debrief) completes with WebGL, without WebGL and with reduced motion
- [x] Playwright: a share link opens the Design with the spacecraft assembled; no console errors during a full Mission; sound settings survive a reload
- [ ] All `cargo test` suites pass; the deployed build works on GitHub Pages

## Comments

- 2026-11-15: Resolved on integration branch `low-poly-overhaul` (not pushed, not deployed). Acceptance criteria confirmed by `cargo test` (228 tests), a release Trunk build and the Playwright suite (75 tests, with and without WebGL). The look was checked in screenshots; sounds were not checked by ear.
- 2026-11-15: The GitHub Pages item stays open: it can only be checked after the branch is pushed and deployed.
