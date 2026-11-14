# 07: Launch Review: rocket on the pad and lift-off

**What to build:** At Launch Review the spacecraft sits on top of the chosen Launch Vehicle on a launch pad. The Launch Vehicles and the pad are CC0 glTF models, or simple procedural shapes when no CC0 model fits. The Launch Review panel sits beside the pad, and a blocked review still explains why it is blocked. In team play each Go or No-Go vote gives visual and sound feedback. After launch is approved, a short countdown, ignition, lift-off and ascent play with sound, and then the Run starts. The launch animation can be skipped.

**Blocked by:** 02: Live spacecraft assembly; 04: Sound foundation.

**Status:** resolved

- [x] The pad Shot shows the chosen Launch Vehicle with the assembled spacecraft on top
- [x] The Launch Review explanations are unchanged
- [x] Go and No-Go votes emit Cues (`cargo test`) and give visual feedback
- [x] The launch animation plays with sound and can be skipped
- [x] The new models are listed in the credits data file
- [x] Without WebGL, Launch Review and launch work as today

## Comments

- 2026-11-15: Resolved on integration branch `low-poly-overhaul` (not pushed, not deployed). Acceptance criteria confirmed by `cargo test` (228 tests), a release Trunk build and the Playwright suite (75 tests, with and without WebGL). The look was checked in screenshots; sounds were not checked by ear.
