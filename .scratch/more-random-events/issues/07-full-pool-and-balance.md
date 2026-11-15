# 07: Full pool and balance

**What to build:** All 59 events of `events-draft.md` are in the game with final English text, and their numbers are tuned so the new events add variety without deciding the Mission: one event shifts Science Value by a few percent, and all new events of a Run together stay within about ±10% for the reference Designs. Every Scenario's Runs get 5 to 8 new events.

**Blocked by:** 02, 04, 05, 06

**Status:** resolved

- [x] The catalog file holds 59 events: 40 real, 10 fantasy, 9 lucky, 21 of them interactive; a catalog test pins these counts
- [x] Every event of `events-draft.md` is present; any that had to be rewritten to fit the mechanics is noted under Comments in this file
- [x] No event text names a real country or agency (checked against a deny-list); the rival is "a rival agency"
- [x] All three Scenarios are set to 5 to 8 new events per Run (every Scenario in the catalog, five at the time of writing)
- [x] A sweep over seeds for the reference Designs shows Science Value with new events within about ±10% of the same Run with new events off
- [x] A sweep over seeds shows no Run ended by a new event, and 3 or 4 interactive new events per Run
- [x] Over ten Runs with different seeds, most of the pool has appeared at least once
- [x] Playwright: a Run with a known seed shows a new event in the log with its mark and a question that can be answered, in both the WebGL and fallback projects

## Comments

Events rewritten to fit the mechanics (numbers as shipped in `data/events.toml`):

- **15, Storage full of debug logs**: "a large computer does not notice" cannot exist (there is one computer, only its tier is chosen). A Flight-proven computer cleans the logs up instead; otherwise half the storage for 2 days.
- **14, Inches instead of centimetres**: "2 days looking the wrong way" is Instruments off for 2 days (no useful Data), not a pointing change.
- **17, The lens cover froze shut** and **F9, A busload of alien tourists**: "optics off" is plain Instruments off (1 day warming, 3 days waiting; 1 day blinded); both happen only with a camera (an Instrument with `optical = true`).
- **27, The operator fell asleep** and **30, System update on the console mid-pass**: "pass missed, a day's Data delayed" is a day without downlink; the Data waits on board.
- **21, Condensation on the lens**: "less Science Value 5 days" is 30% less Data for 5 days.
- **16, A reaction wheel starts to vibrate**: "less Data from pointing Instruments" is 30% less Data for 3 days from all Instruments.
- **37, Rescuers ask for flood images**: now camera only, as the draft says (it had no condition).
- **12, A panel did not open all the way**: the shake gamble's consequence now states its price (pointing 50% worse for 2 days) instead of "it probably opens", which repeated the odds the sim appends.

Balance, measured with the shipped numbers (200 seeds per reference Design, defaults taken; then answering every
question against its default): the mean Science Value shift is -0.8% (Crop Watch), -1.0% (Connect the Villages),
-1.8% (Space Weather Sentinel), -1.1% (Amazon Watch), -0.2% (Glacier Watch); the worst single Run moves -4.6%;
answering against the defaults moves the means by -0.1% to +0.4%. The success rate is the same with new events on
and off for every reference Design but Glacier Watch (173 off, 172 on of 200). No number needed retuning; the
largest single events (aliens, an anti-satellite test, inches/centimetres, the dragon, the inspector) cost about 3%
in the shortest Run (Space Weather Sentinel, 180 days).
