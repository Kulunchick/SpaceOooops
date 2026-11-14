# 08: Sources, credits and README

**What to build:** A Sources page, reachable from the title screen, lists where the numbers and ideas come from:
- NASA CubeSat 101.
- "Design and Develop Science Missions" (mission segments).
- NASA's Webb "build it yourself" telescope game, which inspired the Real-Mission Match.
- The public specs used for Launch Vehicles.
- The Space Agency Partner context: Angola's GGPEN wrote the challenge.

The page states clearly how SI tools were used: the code was written with Claude Code.

The README describes the project for judges: what it is, how to play, the tech stack (Rust, WebAssembly, Leptos), the NASA resources used, and the SI acknowledgement.

**Blocked by:** 01: Skeleton and deploy.

**Status:** resolved

- [x] The Sources page can be opened from the title screen in the deployed app
- [x] Every NASA resource used is listed with a public link
- [x] SI use is stated on the page and in the README (code generated with Claude Code)
- [x] The README covers the project summary, how to play, tech stack, NASA resources, SI acknowledgement, and how to run, test and deploy locally

## Comments

- 2026-11-14: Resolved on integration branch `mission-design-game` (merged to `main` at d78ab1e, deployed to https://kulunchick.github.io/SpaceOooops/). Acceptance criteria confirmed by `cargo test` (116 tests) and a scripted browser check of the deployed app.
