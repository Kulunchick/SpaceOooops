# 04: Detailed Debrief and learning layer

**What to build:** The Debrief teaches *why*:
- **Causes** of failure and shortfall, in plain language. For example: "Your battery was too small: 36 min in Earth's shadow each orbit drained it"; "X-band was needed: 40% of your Data never reached the ground"; "Your Basic reaction wheel failed on day 112".
- **Score breakdown**: Science Value, Budget-margin bonus, Mass-margin bonus, penalties.
- **Real-Mission Match**: a real mission with a similar objective and design, with one sentence about it.

Every parameter in the Design gets a "why it matters" tooltip in simple English. The Crop Watch Scenario shows first-time hints, so it doubles as the tutorial.

**Blocked by:** 03: Run and basic Debrief.

**Status:** ready-for-agent

- [ ] The Debrief lists causes that come from `sim`, never made up in the UI
- [ ] Test: an undersized battery produces a battery cause
- [ ] Test: Data above downlink produces a lost-Data cause
- [ ] Test: a Component failure that ends the Run early produces a cause naming the Component and the day
- [ ] Test: the Score breakdown adds up to the Score
- [ ] `sim` picks the Real-Mission Match from a data file of real missions; a test checks the match is sensible for a known Design
- [ ] Every Design parameter has a tooltip; first-time hints appear only in Crop Watch
