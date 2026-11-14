# 05: Random Events in the Run scene

**What to build:** Random Events and the player's Decisions play out visibly and audibly in the Run scene:

- **Solar flare:** a wave travels from the Sun and hits the spacecraft, with an alarm sound.
- **Ground station outage:** the station's beacon on the ground goes dark.
- **Ground contact pass:** a beam links the spacecraft to the station.
- **Component failure:** the failed part sparks or goes dark on the model.
- **Safe mode:** the panels feather and the lights dim.

Each effect has a start and a duration in Run days, so it follows the playback clock through speed-up and skip. The event captions in the event log stay as they are.

The Scene Spec gains Effects, built from the Run timeline and the Decisions, plus a Cue for each Random Event.

**Blocked by:** 03: Run in 3D; 04: Sound foundation.

**Status:** ready-for-agent

- [ ] For seeded Runs, each effect is active on the days its Random Event or Decision occurs, and on no other day (`cargo test`)
- [ ] Each Random Event kind emits its own Cue (`cargo test`)
- [ ] Effects stay in sync through pause, speed-up and skip
- [ ] Safe mode visibly changes the spacecraft
- [ ] The no-WebGL fallback keeps today's SVG event flashes
