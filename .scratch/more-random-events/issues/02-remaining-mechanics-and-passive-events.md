# 02: Remaining mechanics and the events without a question

**What to build:** The rest of the closed set of mechanics works, so every event in `events-draft.md` that has no question and no Design condition can be played: Data lost, storage reduced, pointing degraded, battery capacity reduced, Component failure chance scaled, Budget spent or gained, Science Value bonus, an extra day of Data. Those events are added to the catalog file, lucky ones included (for example 18, 19, 27, 29, 30, F3, F4, F5, F7, F8, G1, G4, G5, G6, G8). Budget changes count in the Mission's cost and Score.

**Blocked by:** 01

**Status:** resolved

- [x] Each new mechanic is covered through the Run interface with a one-event test catalog: snapshots, cost or Science Value move as the event states
- [x] Money spent goes through the existing extra-cost path; money gained raises the Budget left and shows in the Score's unspent-Budget bonus
- [x] A scaled failure chance changes only the threshold, not the number or order of random draws
- [x] An event with two effects (for example Instruments off, then a permanent Power gain) applies both
- [x] No mechanic marks a Component as failed or ends the Run
- [x] All events of `events-draft.md` without a question and without a Design condition are in the catalog file and load
