# 07: Orbit animation and UI polish

**What to build:** During the Run, an SVG animation shows the spacecraft circling Earth on the chosen Orbit. LEO, SSO and GEO look visibly different. Earth's night side is marked so eclipses are visible, and Random Events flash on the spacecraft. The whole game gets the mission-control look: dark theme, monospace numbers, neon gauges. The layout works at laptop and tablet sizes, and gauge states are shown by an icon or text label as well as by colour.

**Blocked by:** 03: Run and basic Debrief.

**Status:** resolved

- [x] The orbit animation stays in sync with the Run playback, including speed-up and skip
- [x] LEO, SSO and GEO look distinct; eclipses are visible
- [x] Usable at common laptop and tablet widths
- [x] Gauge states can be read without colour (icon or label)
- [x] No NASA logos, flags or mission identifiers anywhere

## Comments

- 2026-11-14: "No NASA mission identifiers" is read as graphic identifiers (logos, insignia, patches, flags); real missions are named in plain text in the Real-Mission Match.
- 2026-11-14: Resolved on integration branch `mission-design-game` (merged to `main` at d78ab1e, deployed to https://kulunchick.github.io/SpaceOooops/). Acceptance criteria confirmed by `cargo test` (116 tests) and a scripted browser check of the deployed app.
