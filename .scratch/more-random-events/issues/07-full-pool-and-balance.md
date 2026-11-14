# 07: Full pool and balance

**What to build:** All 59 events of `events-draft.md` are in the game with final English text, and their numbers are tuned so the new events add variety without deciding the Mission: one event shifts Science Value by a few percent, and all new events of a Run together stay within about ±10% for the reference Designs. Every Scenario's Runs get 5 to 8 new events.

**Blocked by:** 02, 04, 05, 06

**Status:** ready-for-agent

- [ ] The catalog file holds 59 events: 40 real, 10 fantasy, 9 lucky, 21 of them interactive; a catalog test pins these counts
- [ ] Every event of `events-draft.md` is present; any that had to be rewritten to fit the mechanics is noted under Comments in this file
- [ ] No event text names a real country or agency (checked against a deny-list); the rival is "a rival agency"
- [ ] All three Scenarios are set to 5 to 8 new events per Run
- [ ] A sweep over seeds for the reference Designs shows Science Value with new events within about ±10% of the same Run with new events off
- [ ] A sweep over seeds shows no Run ended by a new event, and 3 or 4 interactive new events per Run
- [ ] Over ten Runs with different seeds, most of the pool has appeared at least once
- [ ] Playwright: a Run with a known seed shows a new event in the log with its mark and a question that can be answered, in both the WebGL and fallback projects
