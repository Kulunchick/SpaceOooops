# 01: First new Random Events, from data to event log

**What to build:** A Run shows a handful of new Random Events in the event log, and they change the Run's numbers. Events are defined in a new catalog data file. At the start of a Run the sim plans, from a second random stream derived from the seed, how many new events happen (between the Scenario's minimum and maximum), which ones (each at most once) and on which distinct days. Three mechanics work, each for N days or to the end of the Run: Instruments off, Power changed by a share, downlink changed by a share or off. About 8 events without a question and without a Design condition ship, taken from `events-draft.md` (for example 10, 11, 14, 20, 22, 23, 26, 40). No new event ends the Mission. See `spec.md` for the decisions.

**Blocked by:** None (can start immediately)

**Status:** resolved

- [x] A new catalog data file holds the events (id, group, title, log text, mechanic, strength, length, scene effect type) and loads into the Catalog; a malformed entry fails catalog tests with a clear message
- [x] Each Scenario has a minimum and maximum number of new events per Run; zero switches them off
- [x] With new events off, a Run is identical to today's for the same Design, Scenario and seed
- [x] With new events on, solar flares, outages and failure rolls still fall on the same days as before for the same seed (the existing random stream is untouched)
- [x] Same Design, Scenario and seed give the same new events on the same days; different seeds give different sets
- [x] No new event appears twice in a Run, and none ends the Mission, across a sweep of seeds
- [x] Each of the three mechanics is covered through the Run interface with a one-event test catalog: the day snapshots move as the event states and return to normal when it ends
- [x] The Random Event carries the catalog event id, its group and its scene effect type; the event log shows its text on its day
- [x] Playback slows on a day with a new event, as it does for existing events
- [x] Existing tests pass (run against new events off, or re-pinned once)
