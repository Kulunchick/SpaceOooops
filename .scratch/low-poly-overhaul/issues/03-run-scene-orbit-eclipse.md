# 03: Run in 3D: orbit and Earth's shadow, in sync with playback

**What to build:** During the Run the spacecraft from the player's Design orbits the 3D Earth. LEO, SSO and GEO look clearly different in altitude (drawn on the same not-to-scale log-ish scale as today), inclination and angular speed. Earth's shadow is visible, and the spacecraft darkens in eclipse. The scene's only clock is the Run playback position, so pause, speed-up and skip stay in sync with the dashboards. The dashboards and the event log sit over the scene at its edges and keep the centre clear. Without WebGL, the Run renders today's SVG orbit view.

The Scene Spec gains an orbit view: display altitude, inclination, angular position from playback, and eclipse state. These come from the same quantities the SVG orbit view uses.

**Blocked by:** 02: Live spacecraft assembly.

**Status:** ready-for-agent

- [ ] The three Orbits are visibly distinct in 3D
- [ ] Eclipse state per playback day matches the Run timeline (`cargo test`)
- [ ] Angular position comes only from playback; pause, speed-up and skip stay in sync
- [ ] New Run layout: dashboards and event log over the scene without hiding the spacecraft
- [ ] Without WebGL the SVG orbit view is used and the Run plays as before
- [ ] Ticket 07 guarantees from the mission-design-game spec still hold
