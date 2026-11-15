# SpaceOooops

A browser game for the 2026 NASA Space Apps Challenge, "Space Mission Design Game". Play it here, with no install or login:
https://kulunchick.github.io/SpaceOooops/

30-second demo video: https://youtu.be/p-rJV9GkExQ

Team SpaceOooops (Kyiv): Mykyta Kalachuk, Artem Voloshyn, Danilo Andronov, Mykhailo Korotaiev, Anastasiia Savenko,
Mykola Krasovskyi.

## What it is

Students rarely get to experiment with a whole space mission. SpaceOooops lets you design, launch and fly a small mission and
see how every choice trades off against the others: a heavier Instrument costs Mass, Power, Data and money, and a cheaper
Orbit shortens the Lifetime. You can play solo or as a team of up to four on one screen.

## How to play

1. Choose solo play, or team play with 2 to 4 players on one screen: each player owns one or more Roles and has a
   Personal Objective worth bonus points.
2. Pick a Scenario and read the brief: objective, required Science Value, minimum Lifetime and Budget.
   There are seven Scenarios. Three are set in Angola, home of the challenge author: Crop Watch (easy), Connect the
   Villages (medium) and Space Weather Sentinel (hard). Four are set in countries of Space Apps partner agencies:
   Glacier Watch (Argentina, easy), Amazon Watch (Brazil, medium), Cyclone Watch (India, medium) and Disaster Response
   (Japan, hard).
3. Design the mission in four Role tabs: Science (Instruments), Spacecraft (structure, power, attitude control), Launch & Orbit
   (Orbit and Launch Vehicle) and Budget & Comms. Live gauges show Budget, Mass, Power, Data and Lifetime, and every number
   has a "why it matters" tip. Crop Watch adds first-time hints.
4. Pass the Launch Review. It blocks Designs that break a hard constraint and explains why. In team play every Role votes
   Go or No-Go, and a No-Go shows its objection.
5. Watch the Run, a day-by-day simulation with seeded Random Events, played back over an orbit animation with live
   dashboards. Solar flares and ground-station outages pause the Run and ask you what to do (enter safe mode or ride it
   out; wait or buy time on another station). If you skip, the spacecraft takes the safe default.
6. Read the Debrief: Success or failure, the Score line by line, the causes in plain language with this Run's numbers,
   your decisions and their effects, each Role's Personal Objective, a Real-Mission Match to a real mission of a similar
   kind, and a comparison with your previous Run. Then change the Design and fly again.

Every Run is saved in the browser's history (newest first, with "Load this Design"), and "Share design" copies a link that
opens the same Scenario and Design in any browser.

The terms above are defined in `GLOSSARY.md`. The plan is in `.scratch/mission-design-game/spec.md`.

## Tech stack

- Rust compiled to WebAssembly
- Leptos 0.8 (client-side rendered UI)
- Trunk (build and dev server)
- A pure-Rust `sim` module holds all game rules and is tested natively with `cargo test`
- A low-poly 3D scene behind the UI, drawn with three.js 0.186.1 (vendored, MIT) through a small JS bridge; without
  WebGL the game falls back to a static background (ADR 0003)
- Static hosting on GitHub Pages, no backend

## NASA resources used

- [CubeSat 101: Basic Concepts and Processes for First-Time CubeSat Developers](https://www.nasa.gov/wp-content/uploads/2017/03/nasa_csli_cubesat_101_508.pdf), for structure sizes and the shape of a small-satellite mission.
- [Design and Develop Science Missions](https://www.nasa.gov/wp-content/uploads/2025/04/250323-design-and-develop-science-mission.pdf), for the mission-segment framing.
- [Build It Yourself (NASA Webb learning game)](https://science.nasa.gov/mission/webb/game-build-it-yourself/), as inspiration for the Real-Mission Match.
- Background from the challenge resources: [GMAT](https://software.nasa.gov/software/GSC-19468-1), [Concept Maturity Levels](https://dataverse.jpl.nasa.gov/dataset.xhtml?persistentId=hdl:2014/44299), [Mission Design Lab](https://etd.gsfc.nasa.gov/capabilities/integrated-design-center/mission-design-lab/) and [Mission Viz](https://nasa.github.io/mission-viz/).

Launch Vehicle classes are generic and use public figures, cross-checked against secondary sources (Wikipedia pages for
[Electron](https://en.wikipedia.org/wiki/Electron_(rocket)) and [Falcon 9](https://en.wikipedia.org/wiki/Falcon_9)).
The challenge author is Angola's GGPEN ([ggpen.gov.ao](https://ggpen.gov.ao)) according to the challenge text.
The game has a Sources page, reachable from the title screen, with the same list.
It uses no NASA logos, insignia, mission patches or flags. Real missions, including some NASA missions, are named in plain
text only, as factual references for comparison; no endorsement is implied. SpaceOooops is not endorsed by NASA.

## Assets and licences

Every external asset shipped with the game (library, fonts, and later models, images, sounds and music) is listed in
`data/credits.toml`, shown on the in-game Sources page, and listed here. The list below must match the data file exactly;
`cargo test` (`tests/credits.rs`) fails if it does not, and prints the text to paste. To add an asset, add an `[[asset]]`
entry to `data/credits.toml`, then update this list.

<!-- credits:start -->
- three.js r186 (0.186.1), by three.js authors (mrdoob and contributors). MIT (library). Source: https://www.npmjs.com/package/three/v/0.186.1
- Russo One (display font), by Jovanny Lemonad. SIL Open Font License 1.1 (font). Source: https://fonts.google.com/specimen/Russo+One (via https://www.npmjs.com/package/@fontsource/russo-one 5.3.0)
- Atkinson Hyperlegible (body font), by Braille Institute of America. SIL Open Font License 1.1 (font). Source: https://fonts.google.com/specimen/Atkinson+Hyperlegible (via https://www.npmjs.com/package/@fontsource/atkinson-hyperlegible 5.3.0)
- Kenney UI SFX Set (click and snap sounds), by Kenney Vleugels (Kenney.nl). CC0 1.0 (sound). Source: https://kenney.nl/assets/ui-audio (click1.ogg, switch1.ogg)
- Outer Space Loop (menu music), by wipics. CC0 1.0 (music). Source: https://opengameart.org/content/outer-space-loop
- Space Flight (Run music), by wipics. CC0 1.0 (music). Source: https://opengameart.org/content/space-flight
- Kenney Sci-Fi Sounds (launch engine and ignition), by Kenney Vleugels (Kenney.nl). CC0 1.0 (sound). Source: https://kenney.nl/assets/sci-fi-sounds (spaceEngineLarge_000.ogg, lowFrequency_explosion_000.ogg)
- Kenney Interface Sounds (Go and No-Go votes), by Kenney Vleugels (Kenney.nl). CC0 1.0 (sound). Source: https://kenney.nl/assets/interface-sounds (confirmation_002.ogg, error_001.ogg)
<!-- credits:end -->

The static backdrop `assets/images/fallback-earth.svg` and the favicon are the team's own work.

## Acknowledgement of AI tools (SI)

The code of this project was written with Claude Code, an AI coding assistant by Anthropic, working from a specification
prepared by the project team. This is also stated on the in-game Sources page.

Images, music and sound, as recorded in `data/credits.toml` (`ai_generated`):

<!-- ai-assets:start -->
No image, music or sound effect in this game was made with AI tools.
<!-- ai-assets:end -->

Outside the game, in the submission material (`task/submission/`):

- The demo video's voice-over was generated with ElevenLabs (AI-generated speech). This is also written in the video
  file's metadata and its YouTube description. The footage is real gameplay.
- The team logo is an AI-generated image. The video thumbnail and `team-logo-700x450.png` carry a visible
  "AI-generated image" watermark; `team-logo.png` is the unmarked source file they are made from.
- The text of the submission page was edited with AI assistance.

## Prerequisites

    rustup target add wasm32-unknown-unknown
    cargo install trunk --locked

## Run locally

    trunk serve        # http://127.0.0.1:8080/

## Test

    cargo test         # runs natively, no browser needed (tests the `sim` and `scene` modules)

Browser smoke tests (Playwright, Node 24) run against the release build in `dist/`, served under `/SpaceOooops/`:

    trunk build --release --public-url /SpaceOooops/
    cd e2e && npm ci && npx playwright install chromium && npx playwright test

## Build and deploy

    trunk build --release --public-url /SpaceOooops/

Pushing to `main` runs `.github/workflows/deploy.yml`: `cargo test`, the release build, the Playwright smoke tests, then a deploy to GitHub Pages
(https://kulunchick.github.io/SpaceOooops/). In the repo settings, set Pages source to "GitHub Actions".

## Layout

- `src/sim/` pure-Rust simulation, no web/UI dependencies (library crate `spaceooops`): catalog, Launch Review, the
  Run, interactive decisions, Debrief causes, Score, Personal Objectives, Real-Mission Match and Share links
- `src/ui/` Leptos UI, one file per screen or component (binary `spaceooops-app`); `src/ui/run/` and `src/ui/debrief/`
  hold the Run and Debrief screens
- `data/*.toml` every game number and catalog entry, with sources noted next to the values
- `src/scene/` the Scene Spec: what the 3D scene shows, as plain data (library crate, no web dependencies)
- `js/bridge/` the three.js renderer the UI drives through `src/ui/scene_bridge.rs`; `vendor/three/` the pinned three.js
- `data/credits.toml` every external asset (library, fonts) with its source and licence
- `e2e/` Playwright browser smoke tests (dev only)
- `tests/` integration tests of the `sim` module through its public interface (`tests/sweep.rs` checks properties over
  a sweep of random Designs)
- `style/*.css` styles: `skin.css` (low-poly palette, fonts, scene layer, panel and button primitives), `title.css`,
  `main.css` (base and colour tokens), `design.css`, `run.css`,
  `decisions.css`, `learning.css`, `team.css`, `history.css`
- `GLOSSARY.md` the game's terms; `docs/adr/` design decisions
- `task/` the challenge text, its resources and the Space Apps submission guide
- `task/submission/` the submission: `FIELDS.md` (every project-form field), `description.html` (the styled project
  description), `images/` (gameplay screenshots) and `video/` (the demo, its English subtitles, the thumbnail, the
  team logo, the recorded takes and the scripts that rebuild the demo; `tts.py` needs `ELEVEN_LABS_API_KEY` in `.env`)
