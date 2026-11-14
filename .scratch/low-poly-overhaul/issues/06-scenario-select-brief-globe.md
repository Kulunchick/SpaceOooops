# 06: Scenario select and Brief on the globe

**What to build:** On Scenario select, the three Scenarios (Crop Watch, Connect the Villages, Space Weather Sentinel) appear as markers on the low-poly globe over Angola. Choosing one flies the camera to its region, and the Brief opens as a panel while the camera holds on that region. The Scenario cards get the new skin and an illustration slot. Placeholder art fills the slot until the team supplies AI images, which then get listed and disclosed in the credits data file. Difficulty, objective and key constraints stay on every card.

The Scenario data gains marker coordinates (latitude and longitude), and the Scene Spec exposes them.

**Blocked by:** 01: 3D skeleton.

**Status:** ready-for-agent

- [ ] Each Scenario's marker position comes from its data (`cargo test`)
- [ ] Choosing a Scenario flies the camera to its marker, and a click skips the flight
- [ ] Cards keep all their information and get an illustration slot
- [ ] New layout for Scenario select and Brief
- [ ] Without WebGL both screens work over the static background
