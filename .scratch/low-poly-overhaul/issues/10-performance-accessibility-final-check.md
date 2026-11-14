# 10: Performance, accessibility and the final check

**What to build:**

- **Quality toggle:** the settings control gains Low and High quality. Low turns off shadows, reduces geometry detail and lowers the pixel ratio. The choice is remembered, and Low is the default on weak devices.
- **Reduced motion:** with `prefers-reduced-motion`, the idle drift is off, camera moves cut instead of tween, and the launch animation is skipped.
- **Phones:** the game stays usable at phone width, with a reduced scene and panels stacked.
- **Loading:** the first panel shows immediately, and the scene fades in when its assets have loaded.
- **Final check:** the full Playwright suite runs in CI before deploy.

**Blocked by:** 03, 05, 06, 07, 08, 09.

**Status:** ready-for-agent

- [ ] The quality toggle works and persists
- [ ] `prefers-reduced-motion` is respected everywhere
- [ ] The game is usable at phone width
- [ ] The first panel appears before the 3D assets have loaded
- [ ] Playwright: a full Crop Watch Mission (Design → Launch Review → Run → Debrief) completes with WebGL, without WebGL and with reduced motion
- [ ] Playwright: a share link opens the Design with the spacecraft assembled; no console errors during a full Mission; sound settings survive a reload
- [ ] All `cargo test` suites pass; the deployed build works on GitHub Pages
