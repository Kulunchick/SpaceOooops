# 05: Connect the Villages and Space Weather Sentinel

**What to build:** Two more Scenarios become playable:
- **Connect the Villages** (medium): communications for remote regions from GEO, with an expensive launch and tight Mass.
- **Space Weather Sentinel** (hard): a multi-Instrument science mission in LEO 400 km, with a small Budget and heavy Data.

This completes the catalog:
- **Orbits**: LEO 400 km, SSO 600 km, GEO. Each has its eclipse fraction, contact time, Lifetime effect (drag in LEO) and coverage fit for each objective.
- **Structures**: 3U, 6U, 12U, SmallSat.
- **Instruments**: around 6.
- **Launch Vehicles**: every class, with capacity to each Orbit and a tooltip naming a real example.
- **Batteries**: S, M, L.
- **Comms**: every band.

**Blocked by:** 03: Run and basic Debrief.

**Status:** ready-for-agent

- [ ] Both Scenarios appear in Scenario select with difficulty and key constraints, and are fully playable in the deployed app
- [ ] Test: every Scenario has at least one known-good Design that reaches Success with a fixed seed
- [ ] Test: every Scenario has at least one tempting but wrong Design that fails Launch Review or the Run, so the trade-off is real (e.g. GEO with a launcher that is too small)
- [ ] Changing the Orbit visibly changes Lifetime, Power (eclipse) and Data (contact time) in the gauges
- [ ] All new numbers live in data files
