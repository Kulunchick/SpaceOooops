# 08: New events in the 3D scene, with sound

**What to build:** A new Random Event is seen and heard in the Run scene. Six reusable effect kinds are added to the Scene Spec, each built from the Run timeline like the existing effects: fly-by (something passes the spacecraft), hit (a strike on a Model part), link interference (the contact beam breaks up), panel shading (the solar panels go dark), bonus glow (a lucky event), visitor (something holds beside the spacecraft). Each event names its effect kind in the catalog file. Each kind has one Cue. Without WebGL or with reduced motion the events stay text only.

**Blocked by:** 01

**Status:** ready-for-agent

- [ ] The Scene Spec lists an effect for each new event on its days, with a stable id, the right start and length, and nothing outside its days
- [ ] An effect lasts as long as its event's mechanic, cut at the end of the Run, and a pause freezes it
- [ ] Each of the six kinds is drawn in the low-poly style with generic geometry and reads at phone size
- [ ] Each kind plays its Cue once when its effect starts
- [ ] With reduced motion no effect animates and playback does not slow for it; without WebGL the Run plays with text only
- [ ] Both Graphics settings (Low and High) draw the effects
- [ ] Playwright: the effects specs cover at least one new effect kind in the WebGL project and its absence in the fallback project
