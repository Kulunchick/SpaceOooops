# More Random Events: agreed decisions and event list

Outcome of the grilling session on 2026-11-15. Not yet a spec or tickets.

## Decisions

1. Pool: 59 new Random Events: 40 real (told with humour), 10 fantasy, 9 lucky. The four existing events stay as they are.
2. Every new event changes the Run only through existing levers (Power, Data, downlink, pointing, Budget, Science Value, Instrument downtime, failure chance). No new event ends the Mission by itself.
3. No real countries or agencies. The rival is an unnamed "rival agency".
4. Each new event happens at most once per Run. Variety between Runs comes from the seed; nothing is remembered between Runs.
5. A Run gets 5-8 new events regardless of its length (a count per Run, not a daily chance), of which 3-4 ask the player a question.
6. One event shifts the result by a few percent; all new events of a Run together usually move Science Value by no more than ±10%.
7. About a third of the events depend on the Design: some need a certain choice to happen, others are softened by a choice. The Debrief can say what would have prevented them.
8. One shared pool for all Scenarios; a Scenario sets only how many events a Run gets.
9. Fantasy events carry an "Unexplained" mark in the event log. No "realistic mode" setting.
10. Each real event has one short "how this happens in real life" line in the Debrief (no agency names). Fantasy events have none.
11. Events are data in a new `data/events.toml` (text, mechanic type, strength, Design condition, 3D effect type, question and options). The code keeps about 8 mechanic types.
12. 3D: 5-6 reusable effect types (fly-by, hit, link interference, panel shading, bonus glow, visitor beside the spacecraft); unique models only for 3-4 stars such as the UFO.
13. New events draw from their own random stream, so the four existing events fall exactly as before for the same seed.
14. The rescue-imagery bonus (37) is its own Score line, not Science Value.
15. Team play: questions are answered the same way as the flare and outage questions today. Tying a question to a Role is a possible later ticket.
16. Game text is English. Numbers below are provisional until balanced against decision 6.

## Events

`?` marks an event with a question (21 in total).

### Space and orbit

| # | Event | What happens |
|---|---|---|
| 1 | Debris on a collision course | ? Dodge (Instruments off 2 days) / risk it (small chance of a hit on the panels, less Power). More likely in a low Orbit |
| 2 | Micrometeoroid through a panel | -5% Power to the end of the Run. Deployable panels are hit more often |
| 3 | Bit flipped by a cosmic ray | Computer reboots, Instruments off 1 day. A Flight-proven computer loses nothing |
| 4 | A bad week over the South Atlantic | Radiation noise in the images, less Data for 3 days. Low Orbit only |
| 5 | Geomagnetic storm puffs up the atmosphere | Pointing worse for 3 days. Low Orbit only |
| 6 | Atomic oxygen eats the coating | -3% Power to the end. Low Orbit only |
| 7 | Meteor shower | ? Turn edge-on (Instruments off 1 day) / keep working (chance of a hit) |
| 8 | Static discharge on the hull | Radio silent 1 day. More likely in a high Orbit |
| 9 | The Moon blinds the star tracker | Pointing worse 1 day. Only with precise attitude control |
| 10 | The station looks straight into the Sun | Noise drowns the signal, no downlink for 1 day |
| 11 | Someone else's anti-satellite weapon test | Debris cloud, a week of dodging: Instruments off 2 days |

### Spacecraft gremlins

| # | Event | What happens |
|---|---|---|
| 12 | A panel did not open all the way | ? Shake it with the reaction wheels (chance: it opens, otherwise pointing is off 2 days) / leave it (-10% Power for good). Deployable panels only |
| 13 | Software update on a Friday evening | ? Upload now (chance: more Data, otherwise Instruments off 3 days) / wait for Monday (nothing) |
| 14 | Inches instead of centimetres | A pointing command in the wrong units: 2 days looking the wrong way |
| 15 | Storage full of debug logs | Half the room for Data for 2 days. A large computer does not notice |
| 16 | A reaction wheel starts to vibrate | Blurred images, less Data from pointing Instruments 3 days. More likely with Basic |
| 17 | The lens cover froze shut | ? Warm it in the Sun (Instruments off 1 day) / wait (optics off 3 days). Optical Instrument only |
| 18 | The thermostat lost its mind | Cold battery, lower capacity 3 days |
| 19 | The antenna snagged while unfolding | Half the downlink for 2 days |
| 20 | The spacecraft got scared of its own shadow | Safe mode for no reason, Instruments off 1 day |
| 21 | Condensation on the lens | ? Bake it off (Instruments off 1 day) / shoot through it (less Science Value 5 days). Optics only |
| 22 | A sensor took Earth for the Sun | Panels turned away, less Power 1 day |

### Ground and people

| # | Event | What happens |
|---|---|---|
| 23 | The intern commanded the wrong satellite | Instruments off 1 day while it is sorted out |
| 24 | A digger cut the station's cable | ? Urgent repair (Budget, link back in a day) / wait (3 days without downlink) |
| 25 | Birds nested in the antenna dish | ? Evict them (fine from the conservationists, Budget) / wait for the chicks (-30% downlink 5 days) |
| 26 | Thunderstorm over the station | No downlink 1 day |
| 27 | The operator fell asleep on the night shift | Pass missed, a day's Data delayed |
| 28 | An official lost the frequency licence | ? Pay a lawyer (Budget) / stay off the air 2 days |
| 29 | Budget cut | The ministry needed a new fountain: less Budget |
| 30 | System update on the console mid-pass | "Do not turn off your computer": pass missed |
| 31 | A radio amateur jams the frequency by accident | -50% downlink 2 days. UHF radio only |
| 32 | A neighbour on the same frequency | ? Agree a schedule (-20% downlink 4 days) / shout over them (chance of losing all contact for 2 days) |
| 33 | A rival agency's inspector satellite comes right up close | ? Close the lens covers and wait (Instruments off 2 days) / ignore it (chance the camera is "accidentally" dazzled by a laser) |
| 34 | The rival jams the link over its own territory | ? Buy time on another station (Budget) / wait (less Data 2 days) |
| 35 | Schoolchildren try to hack the link | ? Change the keys (no downlink 1 day) / ignore them (chance: 3 days of broadcasting their playlist) |
| 36 | The scientists argue about where to look | ? Target A (ordinary science) / target B (chance of bonus Science Value or 2 empty days) |
| 37 | Rescuers ask for flood images | ? Help (2 days without planned science, Score bonus) / decline. Camera only |
| 38 | Journalists want a pretty picture | ? A day of shooting (-1 day of science, sponsors add Budget) / decline |
| 39 | Nobody paid for the cloud archive | ? Pay (Budget) / lose a day's Data |
| 40 | The control-room key went on holiday with the engineer | Pass missed, 1 day without downlink |

### Fantasy ("Unexplained")

| # | Event | What happens |
|---|---|---|
| F1 | A UFO parks alongside | ? Say hello (chance: bonus Science Value nobody will believe; otherwise they shade the panels 2 days) / pretend to be debris (Instruments off 1 day) |
| F2 | A space whale swims past | Blocks the Sun, less Power 1 day. With a camera: bonus Science Value for the photo |
| F3 | Russell's teapot knocks on the hull | Pointing off 1 day |
| F4 | Gremlins chew the wiring | Less Power 3 days, then they get bored |
| F5 | Aliens borrow the satellite for experiments | Instruments off 2 days. Returned with washed panels: +2% Power |
| F6 | A dragon takes the satellite for something shiny | ? Dim the lights (Instruments off 2 days) / scare it off by radio (chance of a scratched panel) |
| F7 | A witch on a broom crosses the orbit | Dodge, Instruments off 1 day |
| F8 | Time loop | The satellite lives one day twice: an extra day of Data |
| F9 | A busload of alien tourists | Flash photography, optics blinded 1 day. Optics only |
| F10 | A space cat chases the downlink beam | ? Switch the beam off for a day / keep playing (less Data 2 days, but the cat is happy) |

### Lucky

| # | Event | What happens |
|---|---|---|
| G1 | A sponsor chips in | More Budget |
| G2 | Radio amateurs around the world pick up your signal | More downlink 3 days. UHF radio only |
| G3 | A rare sight lands in frame | Eruption, ocean bloom or aurora: bonus Science Value with a fitting Instrument |
| G4 | A neighbouring station gives away passes | More downlink 5 days |
| G5 | The programmers compress Data better | +5% downlink to the end of the Run |
| G6 | Quiet Sun | A month of lower Component failure chance |
| G7 | A photo from the satellite goes viral | Grant: more Budget. Camera only |
| G8 | The panels beat the estimate | +3% Power to the end |
| G9 | A university invites joint observations | ? Agree (bonus Science Value, less Data 2 days) / decline |
