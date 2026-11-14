# 01: Coverage and fit with no silent zero

Spec: `../spec.md`. Take these tickets after `.scratch/more-random-events/` (agreed deadline-day priority).

**What to build:** Catalog loading rejects a Scenario that has no coverage number for some Orbit or no fit number for
some Instrument, instead of quietly treating it as zero. Every existing entry is written out explicitly, zero where
zero is meant, so that adding a Scenario or an Instrument later cannot leave a hole unnoticed. Nothing changes for the
player.

**Blocked by:** None (can start immediately).

**Status:** ready-for-agent

- [ ] Loading fails, with a message naming the Scenario and the Orbit or Instrument, when a coverage or fit number is missing
- [ ] Every existing Orbit has coverage for all three Scenarios and every Instrument has fit for all three, explicitly
- [ ] Every existing Run, Score and Debrief is unchanged (existing tests pass without re-pinning)
- [ ] A catalog test covers the rejection
