# 04: Scenario select camera follows the pin

Spec: `../spec.md`. Take these tickets after `.scratch/more-random-events/` (agreed deadline-day priority).

**What to build:** On the Scenario select screen, hovering or selecting a card turns the globe to that Scenario's pin
and highlights its Target Country. With nothing hovered or selected the camera rests on Angola, as today. The brief's
Region shot is unchanged. Reduced motion is respected the same way other Shot changes respect it.

**Blocked by:** 03

**Status:** ready-for-agent

- [ ] The Scene Spec's Shot on the select screen frames the hovered or selected Scenario's pin
- [ ] With nothing hovered or selected the Shot frames Angola
- [ ] The highlighted Target Country follows the hovered or selected card
- [ ] Keyboard focus on a card behaves like hover
- [ ] Scene Spec tests cover all three cases; the WebGL end-to-end run hovers a card without errors
