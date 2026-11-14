# 10: Interactive Random Events during the Run

**What to build:** Some Random Events pause the Run and ask the player to react. For example:
- Solar-flare warning: "Enter safe mode? (lose 2 days of science, protect the electronics)".
- Ground-station outage: "Buy extra station time? (costs Budget)".

The choice changes the rest of the Run, and the Debrief mentions the decisions that mattered. This is variant (c) from the design session; build it only if time allows.

**Blocked by:** 03: Run and basic Debrief; 07: Orbit animation and UI polish.

**Status:** resolved

- [x] At least two interactive Random Event types, with two choices each
- [x] Choices are passed back into `sim`, so the Run stays deterministic: the same seed and the same choices give the same result (tested)
- [x] The Debrief mentions the player's decisions and their effect
- [x] When the player doesn't interact (playback skipped), a sensible default choice is applied

## Comments

- 2026-11-14: Resolved on integration branch `mission-design-game` (merged to `main` at d78ab1e, deployed to https://kulunchick.github.io/SpaceOooops/). Acceptance criteria confirmed by `cargo test` (116 tests) and a scripted browser check of the deployed app.
