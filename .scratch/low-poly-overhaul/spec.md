Status: ready-for-agent

# Spec: Low-poly visual and audio overhaul

Deadline: Sunday 2026-11-15, 23:59 local time (the Space Apps submission deadline). Every ticket must leave the game in a
working, deployable state.

> **Contradicts ADR 0001** (Leptos DOM UI, orbit in SVG, no game engine). We reopen it because the goal is now a 3D
> low-poly look, which DOM/CSS/SVG cannot provide. The forms-in-DOM part of 0001 still holds: all forms, cards and
> dashboards stay Leptos DOM. Only a 3D scene is added behind them. A new ADR records this and partly supersedes 0001.

> Decisions Q6–Q11 of the grilling session were taken at their recommended answers (the user moved straight on to the
> spec). They are listed under Implementation Decisions. Reopen any of them before the matching ticket starts.

## Problem Statement

SpaceOooops plays well, but it looks like an engineering dashboard: dark panels, monospace numbers and one schematic SVG
orbit. Nothing on screen shows what the player is building. Picking deployable solar panels or a bigger structure only
changes numbers, and the spacecraft is never visible. The Run is a dot on an ellipse, and a solar flare is a coloured
flash. The game makes no sound. For a jury and for students opening the link, it does not feel like a game, and the
mission they designed never feels physical.

## Solution

The game gets a low-poly 3D look (flat-shaded faces, a limited palette, chunky geometry) and a reworked screen layout
around one persistent 3D scene. The camera moves between shots as the player goes from screen to screen:

- **Title**: a low-poly Earth, with Angola recognisable, and a small spacecraft circling it. The camera slowly drifts around them.
- **Scenario select**: the three Scenarios appear as markers on the globe. Choosing one flies the camera to it.
- **Brief**: the camera holds on the chosen Scenario's region while the brief panel is open.
- **Design**: a hangar shot. The spacecraft is assembled live in the centre from the current Design. Changing the
  structure resizes the bus, picking solar panels adds body-mounted or deployable wings, Instruments mount on the bus,
  and comms antennas and the attitude hardware appear when chosen. The four Role tabs become translucent side panels
  around the model.
- **Launch Review**: the assembled spacecraft sits on top of the chosen Launch Vehicle on a pad. On a Go, the rocket
  launches, and the player can skip the launch.
- **Run**: the spacecraft orbits Earth on the chosen Orbit (LEO, SSO and GEO visibly different) with Earth's shadow
  visible. Random Events play out in the scene: a solar flare sends a wave from the Sun, and a ground station outage
  turns a beacon on the ground dark. Dashboards and the event log float above the scene.
- **Debrief**: panels over a dimmed scene. On Success the spacecraft holds against Earth. On failure the scene shows
  what happened (tumbling, going dark, re-entering).

The game also gets sound: music (a calm track for menus and Design, a tenser one for the Run) and sound effects for UI
clicks, Go/No-Go, launch, Random Events, Success and failure. Music and effects have separate volume controls that are
remembered between visits.

Without WebGL, or with "reduced motion" turned on, the game stays fully playable: it falls back to the current SVG orbit
and a static background.

## User Stories

### First impression
1. As a first-time player, I want the title screen to open on a 3D low-poly Earth with a spacecraft orbiting it, so that I know at once this is a space game.
2. As a jury member, I want the game to look polished and coherent across every screen, so that the project stands out.
3. As a player, I want the scene to keep moving gently while I read, so that the game feels alive without distracting me.
4. As a player, I want camera moves between screens to be short and smooth, so that the transitions feel like one continuous world rather than page loads.
5. As a player, I want to skip or shorten any camera transition by clicking, so that the visuals never slow me down.

### Scenario select and brief
6. As a player, I want each Scenario shown as a marker on the globe over Angola, so that I see the Scenarios are tied to a real place.
7. As a player, I want the camera to fly to the region of the Scenario I choose, so that the choice feels tangible.
8. As a player, I want each Scenario card to keep its difficulty, objective and key constraints, so that the new look loses no information.
9. As a player, I want Scenario cards to have an illustration, so that the three Scenarios are easy to tell apart.

### Design: live spacecraft assembly
10. As a player, I want to see my spacecraft in 3D while I design it, so that my choices feel physical.
11. As a Spacecraft player, I want the bus to change size when I switch between 3U, 6U, 12U and SmallSat, so that I see what the structure choice means.
12. As a Spacecraft player, I want body-mounted and deployable solar panels to look different on the model, so that I understand the trade-off.
13. As a Spacecraft player, I want the battery and computer choice shown subtly (a size or label on the model), so that every Component has a visible footprint.
14. As a Spacecraft player, I want reaction wheels and magnetorquers to be visible as distinct hardware, so that attitude control is not invisible.
15. As a Budget & Comms player, I want UHF, S-band and X-band to show as different antennas, so that the comms choice is visible.
16. As a Science player, I want each Instrument I add to mount on the bus with its own shape, so that I see my payload grow.
17. As a player, I want a short "snap" animation and sound when a part is added or removed, so that every change gives feedback.
18. As a player, I want Flight-proven Components to look different from Basic ones (for example a gold foil finish), so that the Reliability Tier is visible.
19. As a player, I want to rotate the spacecraft model by dragging, so that I can look at it from every side.
20. As a player, I want the model to sit in a hangar setting, so that Design feels like building something.
21. As a team, we want the four Role tabs to sit as panels around the model, so that everyone at the screen sees the shared spacecraft while one Role edits.
22. As a player, I want the live gauges (Budget, Mass, Power, Data, Lifetime) to stay visible and readable over the scene, so that the numbers are never hidden behind visuals.
23. As a player, I want an Orbit choice to preview the orbit around a small Earth, so that LEO, SSO and GEO look different before launch.
24. As a player, I want a Launch Vehicle choice to show that rocket next to my spacecraft, so that I feel the size difference.

### Launch Review and launch
25. As a player, I want the Launch Review panel to appear with my spacecraft stacked on the chosen Launch Vehicle on the pad, so that the review feels like a real pre-launch moment.
26. As a team, we want Go and No-Go votes to give visual and audio feedback, so that each vote feels decisive.
27. As a player, I want a blocked Launch Review to keep explaining why it is blocked, so that the new look loses no learning value.
28. As a player, I want a short launch animation (ignition, lift-off, ascent) after Go, with a countdown sound, so that launching feels rewarding.
29. As a player, I want to skip the launch animation, so that repeat Runs are fast.

### Run
30. As a player, I want the spacecraft orbiting a 3D Earth during the Run, so that I watch my mission fly.
31. As a player, I want LEO, SSO and GEO to look clearly different in 3D (altitude, inclination, speed), so that I learn what an Orbit is.
32. As a player, I want Earth's shadow to be visible and the spacecraft to darken in eclipse, so that I see why the battery matters.
33. As a player, I want the scene to stay in sync with Run playback, including speed-up, pause and skip, so that visuals and numbers never disagree.
34. As a player, I want a solar flare to appear as a wave from the Sun hitting the spacecraft, with an alarm sound, so that the Random Event is dramatic and clear.
35. As a player, I want a ground station outage to show the ground beacon going dark, so that I understand what failed.
36. As a player, I want a Component failure to show on the model (a part sparks or goes dark), so that I see which part broke.
37. As a player, I want the safe-mode decision to show visibly on the spacecraft (for example panels feathering and lights dimming), so that my decision has a visible effect.
38. As a player, I want ground contact passes to show as a beam between the spacecraft and a station, so that I understand downlinking Data.
39. As a player, I want the dashboards and event log to sit over the scene without hiding the spacecraft, so that I can watch both.
40. As a player, I want the Run music to be tenser than the menu music, so that the flight feels like the climax.

### Debrief
41. As a player, I want the Debrief to keep every section it has today (verdict, Score, causes, decisions, Personal Objectives, Real-Mission Match, comparison), so that no learning value is lost.
42. As a player, I want a successful Mission to end on a calm hero shot with a success jingle, so that winning feels good.
43. As a player, I want a failed Mission to show its main cause visually (tumbling, going dark, re-entering) with a failure sound, so that failure is memorable and understandable.
44. As a player, I want "Modify design" to fly the camera back to the hangar with my spacecraft intact, so that iterating feels seamless.

### Layout and UI skin
45. As a player, I want panels, buttons, cards and gauges restyled to match the low-poly look (faceted shapes, a limited palette, chunky type), so that UI and scene feel like one game.
46. As a player, I want every text to stay readable over the 3D scene (enough contrast, backdrop blur or solid panels), so that the look never costs legibility.
47. As a player, I want gauge states to stay readable without colour, so that the existing accessibility guarantee holds.
48. As a player on a laptop or tablet, I want the new layout to work at common widths, so that it plays wherever students are.
49. As a player on a phone, I want the game to stay usable even if the 3D scene is reduced, so that a small screen does not lock me out.
50. As a player, I want the History, Sources, team setup and title screens restyled too, so that no screen looks left behind.

### Sound and music
51. As a player, I want background music on the title, Scenario select and Design screens, so that the game has atmosphere.
52. As a player, I want sound effects for clicks, part snaps, Go/No-Go, launch, Random Events, Success and failure, so that actions give feedback.
53. As a player, I want separate controls for music and sound effects (mute and volume), so that I can play in a classroom.
54. As a player, I want my sound settings remembered between visits, so that I don't have to mute again.
55. As a player, I want no sound before my first click, so that the page doesn't blast audio, as browsers require.
56. As a player, I want music to crossfade between menu and Run tracks, so that changes are not jarring.
57. As a team in a classroom, I want sound off to really mean silent, so that we never disturb others.

### Performance, fallback and accessibility
58. As a student on an old school PC, I want the game to run smoothly, so that the visuals don't make it unplayable.
59. As a player, I want a quality toggle (Low / High) that turns off shadows and reduces detail, so that I can fix a slow game myself.
60. As a player whose browser has no WebGL, I want the game to fall back to the current 2D orbit and a static background, so that I can still play everything.
61. As a player with "reduced motion" turned on, I want camera moves and idle animations turned off or minimal, so that the game is comfortable.
62. As a player, I want the first screen to appear quickly while 3D assets load in the background, so that I'm not stuck on a blank page.
63. As a player, I want gameplay to be identical with or without the 3D scene, so that the visuals are never required to win.

### Sharing and continuity
64. As a player, I want share links and History to work exactly as before, so that the overhaul breaks nothing.
65. As a player opening a share link, I want to land on the Design with the shared spacecraft already assembled in 3D, so that the link shows the design at once.

### Credits and compliance
66. As a jury member, I want every external asset (models, fonts, sounds, music, images) listed with its source and licence on the Sources page and in the README, so that the project is clean on licensing.
67. As a jury member, I want the README and Sources page to state where AI tools produced images or music, so that the Space Apps AI disclosure is complete.
68. As the team, we want no NASA logos, insignia, mission patches or flags in any model, texture or image, so that the existing branding rule holds.

## Implementation Decisions

### Architecture
- **New ADR** partly superseding ADR 0001. Forms, cards, gauges and panels stay Leptos DOM. A full-viewport WebGL
  canvas sits behind the DOM, rendered with **three.js** through a thin JavaScript bridge called from Rust with
  `wasm-bindgen`. We considered Bevy (too slow to rebuild the form UI by the deadline, heavy wasm) and the `three-d`
  crate (all-Rust, but fewer examples and riskier to debug in a day). three.js is pinned to an exact version and
  bundled with the site (no runtime CDN dependency), and the bundle is copied by Trunk.
- **`sim` stays untouched.** The overhaul adds no game rules and must not change any Run, Score or Launch Review outcome.

### New module: Scene description (the one test seam)
- A new pure-Rust module, with no web dependencies and next to `sim` in the library crate, turns game state into a
  **Scene Spec**: a plain data description of what the 3D scene and the audio should show right now. It is the only
  place that decides *what* to show. The bridge decides only *how* to draw and play it.
- Inputs: the Catalog, the Design, the current screen (shot), the chosen Scenario, and during the Run the Run result,
  the playback position and the player's Decisions.
- Output (shape, not final names):
  - **Spacecraft model**: a list of parts. Each part has a kind (bus, solar wing, body panels, antenna, Instrument,
    reaction wheel, magnetorquer, battery or computer marker), dimensions derived from catalog data (bus size from
    the structure's U-count, wing span from panel type), a mount point, and a finish from the Reliability Tier.
    Instruments get distinct shapes per Instrument id.
  - **Shot**: a named camera target per screen (title orbit, globe at Scenario, hangar, pad, launch, orbit-follow,
    debrief hero, debrief failure).
  - **Orbit view**: altitude (display-scaled), inclination, angular position from playback, and eclipse state, reusing
    the same quantities the SVG orbit view uses today.
  - **Effects**: active scene effects for the current playback day (flare wave, station dark, contact beam, Component
    failure on part X, safe mode), each with a start and duration in days so effects stay in sync with speed-up and skip.
  - **Cues**: the music track that should be playing (menu, Run, none) and the one-shot sound effects triggered by
    a state transition (part added or removed, vote cast, launch, Random Event, verdict).
  - **Scenario markers**: positions on the globe (latitude and longitude) for the three Scenarios.
- Catalog data gains only what the scene needs and cannot derive: per-Instrument visual shape, per-antenna kind, and
  Scenario marker coordinates, added to the existing data files. No gameplay numbers change.

### JS bridge (scene renderer and audio)
- One small JS module exposes a narrow API to Rust: `init(canvas, quality)`, `apply(sceneSpec)`, `setQuality`,
  `setReducedMotion`, and audio `play(cue)`, `setMusic(track)`, `setVolumes(music, sfx)`. It returns whether WebGL
  is available. Rust serialises the Scene Spec and sends it on every change (Design edit, screen change, playback tick).
- The renderer diffs incoming specs: parts are added or removed with a snap tween, the camera eases between shots,
  and effects run on the playback clock it was given (never on its own timer), so pause, speed-up and skip stay in sync.
- Low-poly look: flat shading, a limited shared palette, procedural geometry for Earth (icosphere with coloured faces
  for ocean, land, ice and cloud shells), the spacecraft parts, the Sun, stars and orbit lines. Launch Vehicles, the
  pad, the hangar props and ground stations come from CC0 glTF packs (Kenney Space Kit / Quaternius) or simple
  procedural shapes when no fitting CC0 model exists.
- Audio uses WebAudio. Sound effects come from CC0 packs (Kenney audio), with WebAudio synthesis where no sample fits.
  Music starts as CC0/CC-BY placeholder tracks. AI-generated tracks supplied by the team replace them if they arrive
  before the deadline. Audio is unlocked on the first user gesture, and music crossfades between tracks.

### Leptos UI
- The app root mounts the canvas once and keeps it across all screens. A Leptos effect builds the Scene Spec from
  `AppState` and pushes it to the bridge.
- **Layout rework**: each screen is laid out as panels over the scene instead of a full-page document. Title, Scenario
  select and Brief use a single centred or side panel over the globe. Design uses side panels for the Role tabs, the
  gauges along an edge, and the model centred. Launch Review uses a side panel next to the pad. Run uses dashboards
  and the event log at the edges, keeping the centre clear. Debrief uses a scrollable panel over a dimmed scene.
  History, Sources and team setup use panels in the same skin.
- **New skin**: a low-poly palette as CSS custom properties, faceted panel shapes via `clip-path`, chunky display font
  (CC0/OFL) plus a readable body font, and translucent panels with a backdrop blur and a solid fallback. Gauges keep
  their icon or text states.
- Model drag-rotate on the Design screen only. Elsewhere the canvas ignores pointer input so panels stay clickable.
- **Settings**: a small persistent control (top corner) for music volume, effects volume, mute, and quality Low/High,
  stored in `localStorage` like History.
- **Fallback**: if the bridge reports no WebGL, the canvas is not used, a static background image is shown, and the
  Run screen renders today's SVG orbit view. `prefers-reduced-motion` disables idle drift and camera tweens (cuts
  instead) and the launch animation. Audio still works in the fallback.
- **Loading**: the first panel renders immediately. The scene fades in when its assets have loaded.

### Assets and credits
- All external assets live in the repo with their licence. A credits data file lists each asset's name, author,
  source URL and licence, and the Sources page and README are generated from it or kept in step with it.
- AI-generated images (Scenario card illustrations, skybox or backdrop) and music are supplied by the team as files.
  The agent cannot generate them. They are disclosed in the README AI section and on the Sources page.
- No NASA logos, insignia, patches or flags in any model, texture or image.

### Glossary
- New terms to add through `/domain-modeling`: **Scene Spec** (the data description of what to show and play),
  **Shot** (the camera framing for a screen), **Cue** (a music track or one-shot sound requested by the game).

### Suggested ticket order
Each ticket ends in a working, deployed game.
1. ADR, three.js bridge and canvas skeleton with WebGL detection and fallback.
2. Scene Spec module and the low-poly Earth on the title screen.
3. Live spacecraft assembly on the Design screen.
4. Run scene: orbit, eclipse and Random Event effects in sync with playback.
5. Layout rework and new UI skin across all screens.
6. Sound and music with settings.
7. Launch Review pad and launch, Debrief success and failure shots, Scenario markers and fly-to.
8. Quality toggle, reduced motion, browser smoke tests, Sources, README and AI disclosure.

Cut order if time runs short: launch animation, Debrief failure shot, Scenario fly-to, drag-rotate.

## Testing Decisions

- **Good tests check external behaviour.** For the Scene Spec, that means "given this Design, the spec contains
  deployable solar wings and an X-band antenna" or "on day N of this seeded Run with a flare, a flare effect is active",
  never how the spec is built internally.
- **Scene Spec module (native `cargo test`)**, in the same style as the existing integration tests over `sim`
  (catalog, run, decisions and share tests, with the shared test helpers):
  - every Component and Instrument choice in the catalog maps to a visible part, and removing it removes the part;
  - structure size drives bus dimensions monotonically (3U < 6U < 12U < SmallSat);
  - Reliability Tier changes a part's finish;
  - each screen maps to its Shot;
  - for seeded Runs, effects appear on the days the timeline's Random Events and Decisions occur, and none appear
    outside their duration;
  - music cue is menu outside the Run and Run during it, and verdict cues match Success or failure;
  - a sweep over all catalog combinations (like the existing sweep test) produces a Scene Spec without panics;
  - the Scene Spec never changes Run, Score or Launch Review results (the existing `sim` tests must pass unchanged).
- **Browser smoke tests** (Playwright against the Trunk-built site, run in CI before deploy):
  - the game loads and the canvas initialises with WebGL;
  - with WebGL disabled, the fallback shows and a full Mission (Crop Watch: Design → Launch Review → Run → Debrief) completes;
  - with `prefers-reduced-motion`, a full Mission completes;
  - a share link opens the Design with the spacecraft assembled;
  - no console errors during a full Mission;
  - sound settings persist across a reload.
- The three.js renderer's look and the audio mix are checked by hand in the browser. That is not automated.

## Out of Scope

- Changes to game rules, the catalog's gameplay numbers, scoring or Scenarios.
- A game engine rewrite (Bevy or similar) or moving the forms UI into the canvas.
- Networked play, accounts and leaderboards (ADR 0002 still holds).
- Voice-over, a narrated tutorial and localisation.
- Generating AI images or music. The team supplies these, and the agent only integrates them.
- Custom-modelled assets beyond procedural geometry and CC0 packs.
- Pixel-art or 2D sprite styling (considered and rejected in favour of low poly).

## Further Notes

- Bundle size matters for school networks. Keep three.js tree-shaken where possible, compress glTF, and keep audio short and compressed (OGG/MP3).
- The orbit display scale must stay "not to scale, log-ish altitude" as today, so that LEO and GEO both fit in one shot.
- The existing ticket 07 guarantees (sync with playback, distinct orbits, visible eclipses, colour-free gauge states,
  no NASA marks) carry over to the 3D scene.
- The untracked `task/resources/` folder (the CubeSat 101 PDF) is reference material, not a game asset.
