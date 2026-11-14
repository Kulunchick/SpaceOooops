Status: resolved

# Spec: SpaceOooops — Space Mission Design Game

Event: 2026 NASA Space Apps Challenge, "Space Mission Design Game". Submission deadline: Sunday 2026-11-15, 23:59 local time.

## Problem Statement

Students and space enthusiasts rarely get to experiment with a whole space mission, from concept to operation. They learn orbits, power, launch vehicles and budgets as separate topics, so the trade-offs between them stay abstract. A heavier Instrument also means more Mass, more Power, more Data and more cost. A cheaper Orbit means a shorter Lifetime. These consequences are not something they can *feel*. Existing tools either focus on one discipline or are professional software (such as GMAT) that beginners can't approach. Small teams of up to four people also have no simple way to negotiate competing priorities together, as a real mission team does.

## Solution

SpaceOooops is a browser game with no install, hosted as a static site. Players pick a Scenario and work through a Mission:

1. **Scenario brief**: the objective, the required Science Value, the minimum Lifetime and the Budget.
2. **Design**: four tabs, one per Role. Science picks Instruments. Spacecraft picks Components. Launch & Orbit picks the Orbit and Launch Vehicle. Budget & Comms picks the comms link and watches the money. Resource gauges for Budget, Mass, Power, Data and Lifetime update live as choices change, and every parameter has a plain-language "why" tooltip.
3. **Launch Review**: blocks any Design that breaks a hard constraint and explains why. In team play, each Role then votes Go/No-Go, and launch needs a Go from every Role.
4. **Run**: a day-by-day simulation of the flight, played back in about 30 seconds. It shows an orbit animation, live resource dashboards and seeded Random Events.
5. **Debrief**: shows Success or failure, the Score, and the *causes* in plain language ("your battery was too small: 36 min in Earth's shadow each orbit drained it"). It also shows which Personal Objectives were met, a Real-Mission Match, and a comparison with the previous Run.
6. **Iterate**: go back to the Design, change it and fly again.

There are three Scenarios of rising difficulty, tied to the challenge author's country, Angola:

- **Crop Watch** (easy, tutorial): drought and crop monitoring over Angola from SSO with an imager.
- **Connect the Villages** (medium): communications for remote regions from GEO; expensive launch, tight Mass.
- **Space Weather Sentinel** (hard): a multi-Instrument science mission in LEO; small Budget, heavy Data.

## User Stories

### Starting
1. As a first-time player, I want to open the game from a link with no install or sign-up, so that I can start in seconds.
2. As a player, I want a title screen that briefly explains what the game is, so that I know what I'm about to do.
3. As a player, I want to choose solo play or team play (2–4 players), so that the game fits who is at the screen.
4. As a team, we want to assign a player name to each Role, so that everyone knows which tab they own.
5. As a solo player, I want to hold every Role myself without voting steps, so that solo play stays fast.
6. As a player, I want to see the three Scenarios with difficulty, objective and key constraints, so that I can pick one that suits me.
7. As a beginner, I want the easy Scenario to guide me with hints, so that it doubles as a tutorial.

### Scenario brief
8. As a player, I want a Scenario brief stating the objective, the required Science Value, the minimum Lifetime and the Budget, so that I know what counts as Success.
9. As a player, I want the brief to explain *why* this mission matters (e.g. drought monitoring for Angolan farmers), so that the goal feels real.
10. As a team, we want each Role's Personal Objective shown on that Role's tab, so that each player knows their own bonus goal.

### Design: general
11. As a player, I want the Design split into Role tabs (Science, Spacecraft, Launch & Orbit, Budget & Comms), so that the problem is broken into understandable parts.
12. As a player, I want to see every option's Mass, Power, cost, Data and Reliability Tier, so that I can compare options.
13. As a player, I want always-visible resource gauges (Budget, Mass, Power, Data, Lifetime) that update instantly when I change something, so that I see each decision's consequences right away.
14. As a player, I want a gauge to turn amber when its margin is thin and red when it is violated, so that problems stand out.
15. As a beginner, I want a plain-language tooltip on every parameter explaining what it is and why it matters, so that I learn as I play.
16. As a player, I want to see which options are incompatible with my current choices and why (e.g. a camera needs reaction wheels), so that I understand dependencies.
17. As a player, I want to change any choice at any time before launch, so that I can explore trade-offs freely.

### Design: Science
18. As the Science Role, I want to pick one or more Instruments, so that the Mission produces Science Value.
19. As the Science Role, I want to see how well each Instrument suits the current Scenario, so that I pick relevant ones.
20. As the Science Role, I want to see each Instrument's daily Data output, so that I can coordinate with Budget & Comms.

### Design: Spacecraft
21. As the Spacecraft Role, I want to pick a Structure (CubeSat 3U / 6U / 12U or a ~100 kg SmallSat), so that I set how much fits on board.
22. As the Spacecraft Role, I want to see that the Structure limits how many Instruments and Components fit, so that I understand volume constraints.
23. As the Spacecraft Role, I want to choose body-mounted or deployable solar panels, so that I balance Power against cost and Mass.
24. As the Spacecraft Role, I want to choose battery size S / M / L, so that the spacecraft survives time in Earth's shadow.
25. As the Spacecraft Role, I want to choose attitude control (magnetorquers or reaction wheels), so that pointing Instruments work.
26. As the Spacecraft Role, I want to choose a Basic or Flight-proven Reliability Tier, so that I trade cost against the chance of failure.

### Design: Launch & Orbit
27. As the Launch & Orbit Role, I want to choose LEO 400 km, SSO 600 km or GEO, so that the Orbit suits the objective.
28. As the Launch & Orbit Role, I want each Orbit's coverage, time in shadow, contact time with the ground and effect on Lifetime explained simply, so that I understand the trade-offs.
29. As the Launch & Orbit Role, I want to choose a Launch Vehicle class with its capacity to each Orbit and its cost, so that I can get the spacecraft up.
30. As the Launch & Orbit Role, I want a clear warning when the spacecraft is too heavy for the Launch Vehicle to the chosen Orbit, so that I fix it before launch.
31. As the Launch & Orbit Role, I want each Launch Vehicle's tooltip to name a real example, so that I connect the class to real rockets.

### Design: Budget & Comms
32. As the Budget & Comms Role, I want to choose a comms band (UHF / S-band / X-band), so that I balance downlink capacity against Power and cost.
33. As the Budget & Comms Role, I want to see daily Data produced versus daily Data that can be downlinked, so that I know whether data will pile up.
34. As the Budget & Comms Role, I want a cost breakdown (spacecraft, Instruments, launch, operations), so that I see where the money goes.

### Launch Review and Go/No-Go
35. As a player, I want a Launch Review screen listing every broken hard constraint with a plain-language reason, so that I know exactly what to fix.
36. As a player, I want launch blocked until no hard constraint is broken, so that I can't fly an impossible spacecraft.
37. As a player, I want soft warnings (thin margins, risky choices) shown without blocking launch, so that I can choose to take risks.
38. As a team, we want each Role to vote Go or No-Go, with launch only on unanimous Go, so that we must agree as a team.
39. As a Role voting No-Go, I want to see what my objection is about (e.g. my Personal Objective would be missed), so that I can argue my case.

### Run
40. As a player, I want the Run to play back day by day in about 30 seconds, so that I watch the Mission unfold without waiting long.
41. As a player, I want an orbit animation of the spacecraft around Earth for the chosen Orbit, so that the Mission feels tangible.
42. As a player, I want live dashboards for battery charge, stored Data, Science Value and days elapsed, so that I see resources being used in real time.
43. As a player, I want Random Events (solar flare, Component failure, ground-station outage) announced as they happen, so that I understand surprises.
44. As a player, I want the chance of Component failure to depend on its Reliability Tier, so that my reliability choices matter.
45. As a player, I want to speed up or skip the playback, so that replays stay quick.
46. As a player, I want the same Design with the same seed to give the same result, so that comparisons are fair.

### Debrief
47. As a player, I want a clear Success or failure verdict, so that I know whether we achieved the objective.
48. As a player, I want the causes of failures and shortfalls explained in plain language, so that I learn *why*, not just *what*.
49. As a player, I want my Score broken down (Science Value, Budget margin bonus, Mass margin bonus, Personal Objective bonuses, penalties), so that I see what I was rewarded for.
50. As a team, we want to see which Role met its Personal Objective, so that everyone's contribution shows.
51. As a player, I want a Real-Mission Match showing a real mission with a similar objective and design, so that I relate my Design to actual space missions.
52. As a player, I want this Run compared with my previous Run, so that I see whether my changes helped.
53. As a player, I want a "Modify design" button that returns to the Design with my choices intact, so that I can iterate quickly.
54. As a player, I want a "Fly again" option with a new seed, so that I can test how robust my Design is.

### History and sharing
55. As a player, I want my past Runs kept in the browser, so that I can come back to them.
56. As a player, I want a Share link that encodes my Design, so that friends can load and fly it.
57. As a player opening a shared link, I want the shared Design loaded into its Scenario, so that I can fly it or improve it.

### Learning and accessibility
58. As a beginner with no space background, I want every concept introduced in simple English when I first meet it, so that I'm never lost.
59. As a player, I want a sources page listing the NASA and other resources the numbers come from, so that I can learn more.
60. As a player on a laptop or tablet, I want the layout to work at common screen sizes, so that I can play on what I have.
61. As a colour-blind player, I want gauge states shown by icon or text as well as colour, so that I can read them.

## Implementation Decisions

- **Overall shape**: a single Rust project compiled to WebAssembly, with two modules:
  - **`sim`**: pure Rust with no web or UI dependencies. It owns the catalog and Scenario data model, the resource maths, Launch Review, the day-by-day Run, Random Events, scoring, Debrief causes and the Real-Mission Match. This is the deep module: a small interface over all the game rules.
  - **UI**: Leptos, client-side only, built with Trunk. It renders the DOM and is styled with CSS. The orbit animation is SVG. It holds no game rules; it calls `sim` and renders what comes back. See ADR-0001.
- **`sim` public interface**, the only seam. It has three operations:
  - **Load catalog**: parses the bundled data files into Scenarios, Components, Instruments, Orbits, Launch Vehicles and Real-Mission Match entries.
  - **Launch Review (Design, Scenario)**: returns resource totals and margins for the five gauges, a list of hard-constraint violations and a list of soft warnings. Each item has a plain-language reason. The UI also calls this live on every change to drive the gauges.
  - **Run (Design, Scenario, seed)**: returns a timeline of daily snapshots (battery charge, stored Data, cumulative Science Value, Component health, Random Events that day) plus a Debrief: Success, the Score breakdown, a list of causes, Personal Objective results and the Real-Mission Match. The UI plays the timeline back; it never re-simulates.
- **Determinism**: the Run uses a seeded PRNG inside `sim`. The same Design, Scenario and seed always give an identical timeline and Debrief.
- **Simulation model** (simplified, with one-day steps):
  - Orbit sets the eclipse fraction, ground-contact minutes per day, the Lifetime cap (drag in LEO), coverage suitability per Scenario objective and the required Launch Vehicle capacity.
  - Power balance: average generation over the sunlit fraction against consumption. The battery must cover the eclipse; a shortfall reduces operating time that day.
  - Data: Instruments generate Data, and the comms band times contact time sets the downlink. Undelivered Data fills onboard storage, and anything beyond storage is lost and earns no Science Value.
  - Science Value accrues per day from delivered Data, weighted by how well each Instrument suits the Scenario objective and Orbit.
  - Random Events are drawn daily: failure probability by Reliability Tier, plus solar flares and ground-station outages, with severity tuned per Scenario.
  - The Run ends at the end of Lifetime, or earlier on a fatal failure.
- **Hard constraints** (block launch): Mass above the Launch Vehicle's capacity to the Orbit; cost above Budget; negative average Power balance; an Instrument that requires reaction wheels flown without them; Structure volume exceeded; no comms. **Soft warnings**: thin margins, a battery that only just covers the eclipse, Data generation above downlink, Basic Reliability Tier on critical Components.
- **Scoring**: Success requires the Scenario's required Science Value and survival to the minimum Lifetime. Score = Science Value + Budget-margin bonus + Mass-margin bonus + Personal Objective bonuses − penalties for an early end. Exact weights live in the data files so they can be balanced without code changes.
- **Data files**: every number lives in human-readable data files bundled into the build, not hard-coded. This covers Components, Instruments, Orbits, Launch Vehicles, Scenarios, Personal Objectives, Real-Mission Matches and scoring weights. Numbers are approximately realistic, drawn from NASA CubeSat 101 and public specs. Launch Vehicles are generic classes; tooltips name a real example (e.g. "like Rocket Lab Electron").
- **Catalog breadth**: Structures: 3U, 6U, 12U, SmallSat. Solar panels: body-mounted, deployable. Battery: S / M / L. Comms: UHF, S-band, X-band. Computer: one option. Attitude control: magnetorquers, reaction wheels. Around 6 Instruments. Orbits: LEO 400 km, SSO 600 km, GEO. Reliability Tiers: Basic, Flight-proven.
- **Team play**: local on one screen (ADR-0002). The Role → player-name mapping and the Go/No-Go votes are UI state. Personal Objectives are defined in data and evaluated by `sim` in the Debrief.
- **Persistence**: Run history is stored in browser local storage, and Share serializes the Design into the URL. No backend.
- **Hosting**: a static build deployed to GitHub Pages, with deployment set up from the very first vertical slice. The user's own domain is a fallback.
- **Language and branding**: all UI text is in English. No NASA logos, flags or mission identifiers. Use of SI-generated code is credited in the README and on a credits/sources page.
- **Delivery order** (cut from the bottom if time runs out):
  1. Solo play, one Scenario (Crop Watch), end to end, deployed.
  2. All three Scenarios and the full Debrief causes.
  3. Team play: Roles, Go/No-Go, Personal Objectives.
  4. Orbit animation and UI polish.
  5. History, comparison with the previous Run, Share.
  6. Interactive Random Events during the Run.

## Testing Decisions

- **One seam**: the public interface of `sim` (Load catalog, Launch Review, Run). All game rules are tested there, through `cargo test`, natively without a browser.
- **What makes a good test**: it sets up a Design and a Scenario, calls the public interface, and asserts on observable results such as violations and their reasons, gauge totals, Success, the Score breakdown, Debrief causes and timeline properties. Tests never reach into internal helpers or data-structure layout, so the simulation model can be rebalanced or refactored freely.
- **What gets tested**:
  - The bundled data files load and every Scenario is winnable: at least one known-good Design reaches Success with a fixed seed.
  - Each hard constraint blocks launch with the right reason (overweight for the Launch Vehicle, over Budget, negative Power, a pointing Instrument without reaction wheels, Structure overflow, no comms).
  - A battery too small for the eclipse produces a Debrief cause about the battery.
  - Data generation above downlink produces a Debrief cause about lost Data.
  - Determinism: the same seed gives an identical result, and different seeds can give different Random Events.
  - A Flight-proven Reliability Tier has a lower failure rate than Basic over many seeds.
  - The Score breakdown adds up to the Score.
  - Personal Objectives are evaluated correctly.
- **UI**: no automated tests; it is checked by hand in the browser after each slice.
- **Prior art**: none yet. The repo is a fresh `cargo new`. These will be the first tests.

## Out of Scope

- Networked multiplayer, online lobbies, leaderboards, accounts, cloud saves (ADR-0002).
- Lunar, interplanetary or deep-space missions; MEO and HEO orbits.
- Thermal and propulsion subsystems as playable choices (mentioned only in tooltips).
- High-fidelity orbital mechanics (no GMAT-style propagation).
- A sandbox mode without constraints; a difficulty selector separate from Scenarios.
- Localization beyond English.
- Automated UI or browser tests.
- The demo video and presentation (handled separately after the game exists).

## Further Notes

- **Judging criteria**: Impact, Creativity, Validity, Relevance, Presentation. Tying the Scenarios to Angola targets Relevance, since the challenge author is Angola's national space programme office (GGPEN). Realistic numbers with cited sources target Validity.
- **Submission requirements to keep in mind**: at least one NASA resource used and listed (CubeSat 101; "Design and Develop Science Missions" for the mission-segment framing; the NASA webb "Build it yourself" game as inspiration for the Real-Mission Match). Public link with no login. SI use acknowledged. No likeness of anyone under 18.
- **Demo**: either a video of 30 seconds or less, or at most 7 slides, never more. A voiceover generated with the user's ElevenLabs voice model must be acknowledged as SI-generated audio.
- **Glossary**: `GLOSSARY.md`. **ADRs**: `docs/adr/0001` (Leptos DOM UI and pure-Rust `sim`), `docs/adr/0002` (no backend, local team play).
