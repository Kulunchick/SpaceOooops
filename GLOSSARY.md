# SpaceOooops

A web game where players design a small Earth-orbit space mission, launch it, watch it play out, and learn from the result.

## Mission flow

**Scenario**:
A predefined mission brief: the objective to achieve, the Science Value required, the minimum Lifetime, and the constraints the Design must respect.
_Avoid_: Level, quest

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
An unplanned occurrence during a Run (solar flare, component failure, ground station outage).
_Avoid_: Incident, hazard

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
