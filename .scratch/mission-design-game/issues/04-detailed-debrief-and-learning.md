# 04: Detailed Debrief and learning layer

**What to build:** The Debrief teaches *why*:
- **Causes** of failure and shortfall, in plain language. For example: "Your battery was too small: 36 min in Earth's shadow each orbit drained it"; "X-band was needed: 40% of your Data never reached the ground"; "Your Basic reaction wheel failed on day 112".
- **Score breakdown**: Science Value, Budget-margin bonus, Mass-margin bonus, penalties.
- **Real-Mission Match**: a real mission with a similar objective and design, with one sentence about it.

Every parameter in the Design gets a "why it matters" tooltip in simple English. The Crop Watch Scenario shows first-time hints, so it doubles as the tutorial.

**Blocked by:** 03: Run and basic Debrief.

**Status:** resolved

- [x] The Debrief lists causes that come from `sim`, never made up in the UI
- [x] Test: an undersized battery produces a battery cause
- [x] Test: Data above downlink produces a lost-Data cause
- [x] Test: a Component failure that ends the Run early produces a cause naming the Component and the day
- [x] Test: the Score breakdown adds up to the Score
- [x] `sim` picks the Real-Mission Match from a data file of real missions; a test checks the match is sensible for a known Design
- [x] Every Design parameter has a tooltip; first-time hints appear only in Crop Watch

## Comments

- 2026-11-14: Resolved on integration branch `mission-design-game` (merged to `main` at d78ab1e, deployed to https://kulunchick.github.io/SpaceOooops/). Acceptance criteria confirmed by `cargo test` (116 tests) and a scripted browser check of the deployed app.
