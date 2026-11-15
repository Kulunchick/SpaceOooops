# 02: Ground stations from the Scenario

Spec: `../spec.md`. Take these tickets after `.scratch/more-random-events/` (agreed deadline-day priority).

**What to build:** Each Scenario lists its own ground stations (name, latitude, longitude; primary first, backup
optional). The 3D scene shows the current Scenario's stations instead of a fixed pair. The three Angolan Scenarios list
today's two stations (near Luanda, near Johannesburg), so nothing looks different yet. The Run's ground station outage
logic is unchanged.

**Blocked by:** None (can start immediately).

**Status:** resolved

- [x] A Scenario without at least one ground station fails catalog loading
- [x] The Scene Spec's ground stations come from the current Scenario
- [x] The three existing Scenarios show the same stations as before
- [x] The scene effects test checks that a Scenario's stations come from its own data, not that they are in Angola
