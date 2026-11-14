# SpaceOooops

A browser game for the 2026 NASA Space Apps Challenge, "Space Mission Design Game". Play it here, with no install or login:
https://kulunchick.github.io/SpaceOooops/

## What it is

Students rarely get to experiment with a whole space mission. SpaceOooops lets you design, launch and fly a small mission and
see how every choice trades off against the others: a heavier Instrument costs Mass, Power, Data and money, and a cheaper
Orbit shortens the Lifetime. You can play solo or as a team of up to four on one screen.

## How to play

1. Choose solo play, or team play with 2 to 4 players on one screen: each player owns one or more Roles and has a
   Personal Objective worth bonus points.
2. Pick a Scenario and read the brief: objective, required Science Value, minimum Lifetime and Budget.
   The three Scenarios are tied to Angola: Crop Watch (easy), Connect the Villages (medium) and Space Weather Sentinel (hard).
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

## Acknowledgement of AI tools (SI)

The code of this project was written with Claude Code, an AI coding assistant by Anthropic, working from a specification
prepared by the project team. This is also stated on the in-game Sources page.

## Prerequisites

    rustup target add wasm32-unknown-unknown
    cargo install trunk --locked

## Run locally

    trunk serve        # http://127.0.0.1:8080/

## Test

    cargo test         # runs natively, no browser needed (tests the `sim` module)

## Build and deploy

    trunk build --release --public-url /SpaceOooops/

Pushing to `main` runs `.github/workflows/deploy.yml`: `cargo test`, the release build, then a deploy to GitHub Pages
(https://kulunchick.github.io/SpaceOooops/). In the repo settings, set Pages source to "GitHub Actions".

## Layout

- `src/sim/` pure-Rust simulation, no web/UI dependencies (library crate `spaceooops`): catalog, Launch Review, the
  Run, interactive decisions, Debrief causes, Score, Personal Objectives, Real-Mission Match and Share links
- `src/ui/` Leptos UI, one file per screen or component (binary `spaceooops-app`); `src/ui/run/` and `src/ui/debrief/`
  hold the Run and Debrief screens
- `data/*.toml` every game number and catalog entry, with sources noted next to the values
- `tests/` integration tests of the `sim` module through its public interface (`tests/sweep.rs` checks properties over
  a sweep of random Designs)
- `style/*.css` dark mission-control styles: `main.css` (base and colour tokens), `design.css`, `run.css`,
  `decisions.css`, `learning.css`, `team.css`, `history.css`
- `GLOSSARY.md` the game's terms; `docs/adr/` design decisions
