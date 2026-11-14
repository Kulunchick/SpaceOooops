# 08: Debrief: success and failure shots

**What to build:** The Debrief opens as a scrollable panel over a dimmed scene. On Success, the spacecraft holds in a calm hero shot against Earth, and a success jingle plays. On failure, the scene shows the main cause: tumbling when attitude control failed, going dark when power or the battery failed, re-entering when Lifetime was too short. A failure sound plays. "Modify design" flies the camera back to the hangar with the Design intact. Every current Debrief section stays: verdict, Score, causes, decisions, Personal Objectives, Real-Mission Match and comparison.

**Blocked by:** 05: Random Events in the Run scene.

**Status:** ready-for-agent

- [ ] The verdict picks the Shot and the Cue (success or failure), and the main cause picks the failure visual (`cargo test`)
- [ ] The new Debrief layout keeps every existing section
- [ ] "Modify design" returns to the hangar Shot with the spacecraft intact
- [ ] Without WebGL the Debrief works as today
