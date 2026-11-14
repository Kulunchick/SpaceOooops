# 03: Run and basic Debrief

**What to build:** Once Launch Review passes, the player clicks Launch and watches the Run: a day-by-day simulation from launch to end of Lifetime, played back in about 30 seconds. Live dashboards show battery charge, stored Data, cumulative Science Value and days elapsed. Random Events are announced as they happen: Component failure (its chance set by Reliability Tier), solar flare, ground-station outage. The player can speed up or skip the playback. The Run ends in a Debrief showing Success or failure and the Score, with two buttons: "Modify design" (back to the Design with choices intact) and "Fly again" (new seed).

This ticket adds one call to the `sim` interface: **Run (Design, Scenario, seed)**. It returns a timeline of daily snapshots plus a Debrief (Success, Score).

The simulation follows the model in the spec:
- Orbit sets the eclipse fraction, ground-contact time and the Lifetime cap.
- The battery must cover the eclipse; otherwise operating time is lost.
- Instruments generate Data; comms band × contact time sets the downlink; anything beyond storage is lost.
- Science Value accrues from delivered Data, weighted by each Instrument's fit to the objective.

A seeded PRNG makes the Run fully deterministic. The UI only plays back the timeline.

**After this ticket the game is the minimum submittable product: solo play on Crop Watch, end to end, deployed.**

**Blocked by:** 02: Crop Watch: Scenario brief, Design and Launch Review.

**Status:** ready-for-agent

- [ ] Launch → Run playback (about 30 s, speed-up/skip) → Debrief works in the deployed app
- [ ] The Debrief shows Success or failure (Success = required Science Value reached and minimum Lifetime survived) and the Score
- [ ] "Modify design" keeps every choice; "Fly again" reruns with a new seed
- [ ] Test: the same Design, Scenario and seed give an identical timeline and Debrief
- [ ] Test: different seeds can produce different Random Events
- [ ] Test: Flight-proven fails less often than Basic over many seeds
- [ ] Test: a known-good Crop Watch Design reaches Success with a fixed seed; a clearly bad one (e.g. battery too small) does not
- [ ] Scoring weights live in data files, not code
