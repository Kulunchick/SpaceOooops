# SpaceOooops

A web game where players design a small Earth-orbit space mission, launch it, watch it play out, and learn from the result.

## Mission flow

**Scenario**:
A predefined mission brief: the objective to achieve, the Science Value required, the minimum Lifetime, and the constraints the Design must respect.
_Avoid_: Level, quest

**Target Country**:
The country a Scenario serves: it holds the Scenario's pin and ground stations, and is outlined and highlighted on the globe (only that one country) once that Scenario is chosen; Angola is highlighted on the title and Scenario select screens. The Scenario card shows it, and Angolan Scenarios are listed first. Angola for the original three Scenarios.
_Avoid_: Region, area, target (alone)

**Mission**:
One attempt at a Scenario: a Design, a Launch Review, a Run and a Debrief.
_Avoid_: Game, session

**Design**:
Every choice the players make before launch: Components, Instruments, Orbit and Launch Vehicle.
_Avoid_: Build, config, loadout

**Launch Review**:
The pre-launch check that blocks any Design breaking a hard constraint and explains why.
_Avoid_: Validation, preflight

**Go/No-Go**:
The team vote at Launch Review; launch is allowed only when every Role votes Go. Skipped in solo play.
_Avoid_: Approval, ready check

**Run**:
The day-by-day simulated flight of a Design from launch to end of Lifetime, driven by a seed.
_Avoid_: Playthrough, sim

**Random Event**:
An unplanned occurrence during a Run (solar flare, component failure, ground station outage). Beyond the built-in ones the catalog has three groups: real (it really happens to missions, and the Debrief says how), fantasy (it cannot happen; the event log marks it Unexplained) and lucky (good news, shown apart from bad news). Some ask the player a question. A Score bonus from one is its own Score line and never changes Science Value.
_Avoid_: Incident, hazard

**Unexplained**:
The mark the event log gives a fantasy Random Event, one that has no real-life explanation.

**Debrief**:
The post-Run report: whether the Mission succeeded, why, and its Score.
_Avoid_: Results screen, summary

## Spacecraft

**Component**:
A spacecraft subsystem part with mass, power, cost and reliability (structure, power, battery, comms, computer, attitude control).
_Avoid_: Part, module, item

**Instrument**:
A scientific payload that produces Data and Science Value.
_Avoid_: Sensor, payload (when meaning one instrument)

**Orbit**:
The path the spacecraft flies; it shapes coverage, eclipses, contact time with the ground and Lifetime.
_Avoid_: Trajectory

**Launch Vehicle**:
The rocket class that carries the spacecraft to Orbit, limited by payload capacity and cost.
_Avoid_: Rocket, launcher

**Reliability Tier**:
How proven a Component is (Basic or Flight-proven); it sets the chance of failure during a Run and the cost.
_Avoid_: Quality, grade

## Resources

**Budget**:
Money available for the whole Mission, in US dollars.

**Mass**:
Total spacecraft mass in kg; must fit the Launch Vehicle's capacity to the chosen Orbit.

**Power**:
Electrical generation versus consumption in watts, including battery reserve for time in Earth's shadow.
_Avoid_: Energy

**Data**:
Data volume produced by Instruments versus what the comms link can send to the ground each day.
_Avoid_: Bandwidth (when meaning volume)

**Lifetime**:
How long the spacecraft keeps operating, in months.
_Avoid_: Duration

## Outcome

**Science Value**:
The points of scientific return a Run accumulates on days when Instruments work and their Data reaches the ground.

**Success**:
A Mission outcome where the Run reaches the Scenario's required Science Value and the spacecraft survives the minimum Lifetime.
_Avoid_: Win

**Score**:
Science Value plus bonuses for unspent Budget and Mass margin and for met Personal Objectives, minus penalties.
_Avoid_: Points (when meaning the total)

**Real-Mission Match**:
The real mission shown in the Debrief whose objective and design most resemble the player's.
_Avoid_: Comparison, reference mission

## Team play

**Role**:
A responsibility area owned by one player in team play: Science, Spacecraft, Launch & Orbit, or Budget & Comms. In solo play one player holds every Role.
_Avoid_: Class, seat

**Personal Objective**:
A Role's private bonus goal that pulls against other Roles' priorities (e.g. Budget & Comms: spend under 80% of Budget).
_Avoid_: Side quest, secret goal

## Presentation

**Scene Spec**:
The plain-data description of what the 3D scene should show and what sound should play right now, built from the game state.
_Avoid_: Scene state, render state

**Shot**:
The camera framing the 3D scene uses for a screen (for example the title orbit around Earth).
_Avoid_: Camera, view

**Cue**:
A music track or one-shot sound effect the game asks to be played.
_Avoid_: Sound event, trigger

**Graphics**:
The player's Low/High setting for how much the 3D scene draws (face count and pixel ratio). Stored under the key `quality` for old saves.
_Avoid_: Quality (reads as Reliability Tier)

**Motion**:
The player's Full/Reduced setting for animation. Full is the default on every device; Reduced skips the launch sequence, cuts camera moves, stops idle drift and turns CSS animations off. The operating system's reduced-motion preference is not followed.
_Avoid_: Animation setting, reduce-motion

**Model part**:
One drawn piece of the 3D spacecraft model (`Part` in the Scene Spec): the bus, a Component's hardware or an Instrument, identified by its catalog option id. A rendering term only; the game rules speak of Components and Instruments.
_Avoid_: Part (for a Component), module

**Fate**:
How the Debrief scene shows a failed Mission's spacecraft: tumbling, dark, reentry or adrift (hero on Success).
_Avoid_: Crash type
