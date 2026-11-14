# 05: Events that depend on the Design

**What to build:** An event can have a Design condition (it can only happen, or is more likely, with a certain choice: low or high Orbit, deployable panels, an optical Instrument, a camera, UHF radio, precise attitude control, a Basic Reliability Tier) and a Design mitigation (a choice that removes or softens its effect: a Flight-proven computer, a large computer). Planning picks only from events whose condition holds. The Debrief says which choice caused or softened such an event. The Design-aware events of `events-draft.md` ship (2, 3, 4, 5, 6, 8, 9, 12, 15, 16, 17, 21, 31, F2, F9, G2, G3, G7).

**Blocked by:** 02

**Status:** ready-for-agent

- [ ] Conditions and mitigations are data in the catalog file, from a closed set of Design facts; catalog tests reject an unknown fact
- [ ] Across a sweep of seeds an event never appears in a Run whose Design does not meet its condition
- [ ] A "more likely" condition raises how often the event is planned, shown over a sweep of seeds
- [ ] A mitigated event still appears in the log, with text saying the choice softened it, and its effect is reduced or zero
- [ ] The Debrief names the Design choice behind each conditioned or mitigated event that happened
- [ ] A Run still gets its planned number of events when conditions shrink the eligible pool, or as many as are eligible
