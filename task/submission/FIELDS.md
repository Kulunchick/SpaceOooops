# SpaceOooops: project submission fields

Copy each block into the matching field of the Space Apps project form. The demo link is already filled in.

---

## Project Name

```
SpaceOooops
```

---

## High-Level Project Summary

```
SpaceOooops is a free browser game where you design, launch and fly a small Earth-orbit space mission, then find out why it worked or failed. Design it. Launch it. Find out why it failed.

You pick one of seven Scenarios, from watching drought over Angola's farmland to disaster response in Japan. You choose Instruments, spacecraft Components, an Orbit and a Launch Vehicle while live gauges show Budget, Mass, Power, Data and Lifetime. Every choice costs something: a better Instrument adds Mass, Power use, Data and money. A Launch Review blocks designs that break a hard limit and explains why. The mission then flies day by day with solar flares, ground-station outages and other random events that ask you to decide. A Debrief explains the result in plain language and matches your design to a real mission.

It can be played solo or by a team of up to four on one screen, each player owning a Role with its own goal. Students rarely get to try a whole mission from idea to operation. Here they can fail safely, change one thing and fly again.
```

---

## Link to Project Demo

```
https://youtu.be/p-rJV9GkExQ
```

30-second video. Voice-over generated with ElevenLabs.

---

## Link to Final Project

```
https://kulunchick.github.io/SpaceOooops/
```

---

## Detailed Project Description

```
PLAY IT
Game (no install, no login): https://kulunchick.github.io/SpaceOooops/
Source code: https://github.com/Kulunchick/SpaceOooops

WHAT IT DOES
SpaceOooops is a game about the trade-offs of space mission design. One Mission is four steps: Design, Launch Review, Run and Debrief.

1. Choose solo play, or team play with 2 to 4 players on one screen.
2. Pick a Scenario and read its brief: the objective, the Science Value required, the minimum Lifetime and the Budget. There are seven Scenarios. Three are set in Angola, the home of the challenge author: Crop Watch (easy), Connect the Villages (medium) and Space Weather Sentinel (hard). Four more are set in countries of Space Apps partner agencies: Amazon Watch (Brazil), Glacier Watch (Argentina), Cyclone Watch (India) and Disaster Response (Japan).
3. Design the mission in four Role tabs: Science (9 Instruments), Spacecraft (structure from CubeSat 3U to a 100 kg SmallSat, solar panels, battery, attitude control, onboard computer, each in a Basic or Flight-proven reliability tier), Launch & Orbit (3 Orbits: 400 km low Earth orbit, 600 km sun-synchronous, geostationary; 6 Launch Vehicle classes) and Budget & Comms (UHF, S-band or X-band radio). Live gauges show Budget, Mass, Power, Data and Lifetime, and every number has a "why it matters" tip. Easy Scenarios add first-time hints.
4. Pass the Launch Review. It blocks a Design that breaks a hard constraint, for example a spacecraft too heavy for its Launch Vehicle, and says why. In team play every Role votes Go or No-Go, and a No-Go shows its objection.
5. Watch the Run: a day-by-day simulation shown over a 3D orbit animation with live dashboards. Random Events happen along the way. Some pause the Run and ask a question: enter safe mode for a solar flare or ride it out, wait out a ground-station outage or buy time on another station, dodge a piece of debris or risk it. Some options are gambles with a stated chance. If you skip a question, the spacecraft takes the safe default.
6. Read the Debrief: Success or failure, the Score line by line, the causes in plain language with this Run's numbers, your decisions and their effects, each Role's Personal Objective, a Real-Mission Match and a comparison with your previous Run. Then change the Design and fly again.

Every Run is saved in the browser's history with a "Load this Design" button. "Share design" copies a link that opens the same Scenario and Design in any browser.

HOW IT WORKS
All game rules live in one simulation module written in Rust, with no user-interface code in it. It computes the resource budgets, runs the Launch Review, flies the Run from a seed (the same seed and Design always give the same Run), and builds the Debrief. Every game number sits in plain data files with its source written next to it, and values we tuned for play are marked as game estimates. The user interface is built with Leptos and compiled to WebAssembly. A low-poly 3D scene (Earth, the spacecraft model built from the chosen parts, the launch, the orbit) is drawn with three.js behind the interface. Without WebGL the game falls back to a static background and stays fully playable. There is no backend: the game is a set of static files on GitHub Pages.

BENEFITS AND INTENDED IMPACT
- Students see a whole mission at once, not one topic at a time. Changing one part moves five gauges.
- There is no single correct answer. Different combinations of Instruments, Orbits and Launch Vehicles can meet the same objective.
- The game explains why. The Launch Review and the Debrief give causes and numbers, not only pass or fail.
- Failure is cheap. A Run takes minutes, and the comparison with the previous Run shows what the change did.
- Team play creates real negotiation. Each Role has a Personal Objective that pulls against the others, for example Budget & Comms wants to spend under 80% of the Budget while Science wants more Science Value.
- The Real-Mission Match connects the player's Design to one of 13 real missions, such as NASA's Dellingr CubeSat, AngoSat-2, Amazonia-1, ALOS-2, SAOCOM and INSAT-3DR.

We hope teachers and space clubs can use it as a 20-minute classroom activity, with one shared screen and four students arguing over one spacecraft.

CREATIVITY
- Random Events come in three kinds: real (each has a short note on how it happens to real missions), lucky (good news) and "Unexplained" (clearly marked fantasy events, such as a space cat chasing the downlink beam). The jokes keep players reading the event log, where the real engineering lessons are.
- The Go/No-Go vote and Personal Objectives turn a single-player optimisation puzzle into a team discussion.
- The 3D spacecraft is assembled from the parts the player picked, and a failed Mission ends with a matching fate: tumbling, dark, re-entry or adrift.

FACTORS WE CONSIDERED
- Access and cost: free, runs in a browser, no install, no account, no server. Nothing is sent anywhere; history stays in the browser.
- Low-end devices: a Low/High Graphics setting, a Full/Reduced Motion setting, a non-WebGL fallback, and layouts tested at phone width.
- Readability: body text uses Atkinson Hyperlegible, a typeface designed for legibility. Concepts are explained in simple terms inside the game, with a glossary of terms used the same way on every screen.
- Honesty about numbers: the in-game Sources page lists where values come from and says which are primary sources, secondary sources or game estimates.
- Energy: static hosting and client-side simulation, so no servers run for the game.

TOOLS AND LANGUAGES
Rust, WebAssembly, Leptos 0.8, Trunk, JavaScript, three.js 0.186.1, CSS, TOML data files, Playwright browser tests, GitHub Actions and GitHub Pages. The simulation is covered by native Rust tests, and the release build is checked by browser tests before every deploy.

USE OF AI TOOLS
The idea, the game design and the specification are the team's own work. The code was written with Claude Code, an AI coding assistant by Anthropic, working from the team's specification; this is also stated in the repository README and on the in-game Sources page. The text of this submission was edited with AI assistance. Voice-over generated with ElevenLabs. No image, music or sound effect in the game was made with AI tools, and the demo video shows real gameplay footage.

NOTICES
SpaceOooops uses no NASA logos, insignia, mission patches or flags. Real missions are named in plain text only, as factual references. SpaceOooops is not endorsed by NASA.
```

---

## Use of Super Intelligence (SI)

Dropdown: select the options that match "code generation" (or the closest wording for coding assistants / large language models), "text" editing, and "audio" generation. Do not select image or video generation. Do not select "I did not use SI".

```
Yes.

Code: the game's code was written with Claude Code, an AI coding assistant by Anthropic, working from a specification prepared by the team. The idea, the game design, the rules and the choice of scenarios are the team's own work. This is acknowledged in the repository README, on the in-game Sources page and in the project description.

Text: the wording of this submission page was edited with AI assistance.

Audio: the voice-over of the demo video was generated with ElevenLabs. This is acknowledged in the video description, in the project description and in the video file's metadata.

Images, video, music and sound effects: none were made with AI tools. The demo video is a recording of real gameplay. The game's music and sounds are CC0 assets by human authors, listed with their sources in the game.

No generated content uses or modifies NASA logos, flags or mission identifiers.
```

---

## NASA Data

```
1. CubeSat 101: Basic Concepts and Processes for First-Time CubeSat Developers (NASA CubeSat Launch Initiative)
https://www.nasa.gov/wp-content/uploads/2017/03/nasa_csli_cubesat_101_508.pdf
Used for CubeSat structure sizes (3U, 6U, 12U) and mass per unit, body-mounted solar power per unit, rideshare price ranges and typical small-mission budgets. These set the Design options and their approximate values.

2. Design and Develop Science Missions (NASA webinar slides, "Access to Space for All" systems engineering series)
https://www.nasa.gov/wp-content/uploads/2025/04/250323-design-and-develop-science-mission.pdf
Used for the framing of a mission as launch, space, ground and user segments, and for working from a science need to a mission design. The game's Scenario brief and Role tabs follow this.

3. Build It Yourself (NASA Webb learning game)
https://science.nasa.gov/mission/webb/game-build-it-yourself/
Inspiration for the Real-Mission Match in the Debrief, which relates the player's Design to a real mission.

4. NASA pages on real missions, used as primary sources for the Real-Mission Match:
- Dellingr CubeSat, NASA Goddard: https://svs.gsfc.nasa.gov/12602
- MinXSS-1 CubeSat: https://science.nasa.gov/technology/technology-highlights/high-precision-pointing-on-a-cubesat-enables-new-science

5. Background reading from the challenge resources on how real teams study and visualise mission concepts:
- General Mission Analysis Tool (GMAT): https://software.nasa.gov/software/GSC-19468-1
- Concept Maturity Levels (JPL): https://dataverse.jpl.nasa.gov/dataset.xhtml?persistentId=hdl:2014/44299
- Mission Design Lab (NASA Goddard): https://etd.gsfc.nasa.gov/capabilities/integrated-design-center/mission-design-lab/
- NASA Mission Viz: https://nasa.github.io/mission-viz/

Game values are simplified and only approximately realistic. The in-game Sources page carries the same list.
```

---

## Space Agency Partner Data and Other References

```
SPACE AGENCY PARTNER CONTEXT AND SOURCES
- Angola National Space Programme Management Office (GGPEN), author of the challenge: https://ggpen.gov.ao. Angola is the game's home; three Scenarios are set there and AngoSat-2 is one of the real missions (source: https://spaceinafrica.com/?p=19185, secondary).
- ISRO (India, Cyclone Watch): Master Control Facility, Hassan station position: https://www.mcf.gov.in/website/tos ; ISTRAC: https://www.istrac.gov.in/ (position of the Bengaluru station from Mapcarta, https://mapcarta.com/W801974072, secondary).
- JAXA (Japan, Disaster Response): list of centers, https://global.jaxa.jp/about/centers/index.html (station positions from Wikipedia, https://en.wikipedia.org/wiki/Tsukuba_Space_Center, and Wikidata, https://www.wikidata.org/wiki/Q11552678, secondary).
- National Space Activities Commission of Argentina, CONAE (Glacier Watch): Cordoba ground station page, https://www.argentina.gob.ar/ciencia/conae/centros-y-estaciones/estacion-terrena-cordoba/ubicacion (positions from Wikidata, https://www.wikidata.org/wiki/Q5693426 and https://www.wikidata.org/wiki/Q42373251, secondary).
- Brazilian Space Agency and INPE (Amazon Watch): INPE tracking and control service, https://www.gov.br/pt-br/servicos/obter-servico-de-rastreio-e-controle-para-instituicoes-nacionais-ou-internacionais-inpe ; Cuiaba station in the USGS Landsat station list, https://landsat.usgs.gov/CUB ; Alcantara position from Wikipedia, https://en.wikipedia.org/wiki/Alc%C3%A2ntara_Space_Center (secondary).
- ESA: eoPortal mission catalogue, used as a secondary source for real-mission facts: Planet Dove (https://www.eoportal.org/satellite-missions/planet), EOS SAT-1 (https://www.eoportal.org/satellite-missions/eos-sat-1), Amazonia-1 (https://www.eoportal.org/satellite-missions/amazonia-1), ALOS-2 (https://www.eoportal.org/satellite-missions/alos-2), SAOCOM (https://www.eoportal.org/satellite-missions/saocom), INSAT-3DR (https://www.eoportal.org/satellite-missions/insat-3dr), CSSWE (https://eoportal.org/satellite-missions/csswe).

OTHER REFERENCES FOR GAME FACTS
- Launch Vehicle classes are generic and use public figures from secondary sources: https://en.wikipedia.org/wiki/Electron_(rocket) and https://en.wikipedia.org/wiki/Falcon_9
- Real missions from operators' own pages: Arcturus, Astranis (https://www.astranis.com/blog/first-astranis-satellite-successfully-deployed-to-geo-beams-first-signals-down-to-alaska-257ba); Amazonas Nexus, Hispasat (https://www.hispasat.com/en/fleet-and-infrastructure/amazonas-nexus); EOS SAT-1, EOSDA (https://eos.com/blog/eosda-sent-its-first-agro-satellite-on-jan-3-to-orbit/); Swarm SpaceBEE (https://en.wikipedia.org/wiki/Swarm_Technologies, secondary).
- Solar flare frequency follows NOAA Space Weather Prediction Center figures for strong flares near solar maximum.

ASSETS SHIPPED WITH THE GAME
- three.js r186 (0.186.1), by three.js authors (mrdoob and contributors). MIT. https://www.npmjs.com/package/three/v/0.186.1
- Russo One (display font), by Jovanny Lemonad. SIL Open Font License 1.1. https://fonts.google.com/specimen/Russo+One
- Atkinson Hyperlegible (body font), by Braille Institute of America. SIL Open Font License 1.1. https://fonts.google.com/specimen/Atkinson+Hyperlegible
- Kenney UI SFX Set (click and snap sounds), by Kenney Vleugels (Kenney.nl). CC0 1.0. https://kenney.nl/assets/ui-audio
- Kenney Sci-Fi Sounds (launch engine and ignition), by Kenney Vleugels. CC0 1.0. https://kenney.nl/assets/sci-fi-sounds
- Kenney Interface Sounds (Go and No-Go votes), by Kenney Vleugels. CC0 1.0. https://kenney.nl/assets/interface-sounds
- Outer Space Loop (menu music), by wipics. CC0 1.0. https://opengameart.org/content/outer-space-loop
- Space Flight (Run music), by wipics. CC0 1.0. https://opengameart.org/content/space-flight
The Earth backdrop, the Scenario illustrations, the 3D models and the favicon are the team's own work.

TOOLS
- Rust (https://www.rust-lang.org) and WebAssembly
- Leptos 0.8, web UI framework (https://leptos.dev), MIT
- Trunk, build tool (https://trunkrs.dev)
- three.js, 3D rendering (https://threejs.org), MIT
- Playwright, browser tests and demo recording (https://playwright.dev)
- GitHub Actions and GitHub Pages, build and hosting
- ffmpeg, demo video editing (https://ffmpeg.org)
- ElevenLabs, AI voice-over for the demo video (https://elevenlabs.io)
- Claude Code by Anthropic, AI coding assistant (https://claude.com/claude-code)
```

---

## YouTube upload

Visibility: Unlisted (or Public). Not Private. Audience: "No, it's not made for kids". Check the link in a private browser window before pasting it into the form.

Title:

```
SpaceOooops: Space Mission Design Game (NASA Space Apps Challenge 2026 demo)
```

Description:

```
SpaceOooops is a browser game where you design, launch and fly a small space mission, then find out why it worked or failed.

Play it (no install, no login): https://kulunchick.github.io/SpaceOooops/
Source code: https://github.com/Kulunchick/SpaceOooops

Challenge: Space Mission Design Game, 2026 NASA Space Apps Challenge
Team SpaceOooops: Mykyta Kalachuk, Artem Voloshyn, Danilo Andronov, Mykhailo Korotaiev, Anastasiia Savenko, Mykola Krasovskyi

Voice-over generated with ElevenLabs. The footage is real gameplay. Music: "Space Flight" by wipics (CC0).
The game's code was written with Claude Code, an AI coding assistant, from the team's specification.

SpaceOooops is not endorsed by NASA and uses no NASA logos or insignia.
```

If the video uses the menu track instead, change the music line to: `Music: "Outer Space Loop" by wipics (CC0).`

---

## Pre-submit checklist

- [ ] Video is 30 seconds or less (check the length YouTube shows, not only the file).
- [ ] YouTube link opens in a private window without login; pasted into the form.
- [ ] https://kulunchick.github.io/SpaceOooops/ opens in a private window and shows all seven Scenarios.
- [ ] Every field above is pasted, in English.
- [ ] SI dropdown options selected (code, text, audio).
- [ ] All six members appear on the team's Members tab and are registered for the same Local Event, none waitlisted.
- [ ] "Terms and Conditions" and "Confirm All Work is Original" boxes checked.
- [ ] "Submit for Judging" clicked before 23:59 local time, and the confirmation banner appeared.
