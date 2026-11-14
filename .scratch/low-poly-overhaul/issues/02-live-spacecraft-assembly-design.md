# 02: Live spacecraft assembly on the Design screen

**What to build:** On the Design screen the camera moves to a hangar shot, and the spacecraft is assembled in 3D from the current Design.

The model follows each choice:
- the structure sets the bus size (3U < 6U < 12U < SmallSat);
- body-mounted solar panels and deployable wings look different;
- UHF, S-band and X-band antennas each have their own shape;
- reaction wheels and magnetorquers are visible as distinct hardware;
- the battery and the computer leave a subtle marker on the bus;
- each Instrument mounts on the bus with its own shape;
- Flight-proven Components have a different finish from Basic ones.

Adding or removing a part plays a short snap animation. The player can rotate the model by dragging, and only on this screen.

The Design screen gets its new layout: the four Role tabs become translucent side panels around the model, and the gauges (Budget, Mass, Power, Data, Lifetime) sit along one edge, readable without colour. Choosing an Orbit previews it around a small Earth, and choosing a Launch Vehicle shows the rocket next to the spacecraft. A share link opens the Design with the spacecraft already assembled.

The Scene Spec gains the spacecraft model: a list of parts derived from the Design and the Catalog. The catalog data gains only the visual fields the scene cannot derive (an Instrument's shape, an antenna's kind). No gameplay numbers change.

**Blocked by:** 01: 3D skeleton.

**Status:** ready-for-agent

- [ ] Every Component and Instrument choice in the catalog maps to a visible part, and removing the choice removes the part (`cargo test`)
- [ ] Bus dimensions grow monotonically with structure size (`cargo test`)
- [ ] Reliability Tier changes a part's finish (`cargo test`)
- [ ] A sweep over all catalog combinations builds a Scene Spec without panics
- [ ] Hangar Shot with snap animations and drag-rotate
- [ ] New Design layout: Role side panels and the gauge strip; usable at laptop and tablet widths
- [ ] Orbit and Launch Vehicle choices show their previews
- [ ] A share link lands on the Design with the spacecraft assembled
- [ ] Without WebGL the Design screen works as before
