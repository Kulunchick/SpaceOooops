# 06: Team play: Roles, Personal Objectives, Go/No-Go

**What to build:** On the title screen, players choose solo or team play (2–4 players) and assign a player name to each Role: Science, Spacecraft, Launch & Orbit, Budget & Comms. With fewer than four players, some players hold several Roles.

- **Role tabs**: each tab shows its owner and its Personal Objective, a bonus goal that pulls against other Roles (e.g. Budget & Comms: spend under 80% of Budget).
- **Go/No-Go at Launch Review**: each Role votes Go or No-Go. Launch needs a Go from every Role, and a No-Go shows what the objection is about.
- **Debrief**: shows which Roles met their Personal Objectives and the bonus each earned.
- **Solo play**: no voting, but Personal Objectives still show as optional bonuses.

Team play is local, on one screen, with no networking (ADR-0002).

**Blocked by:** 03: Run and basic Debrief.

**Status:** ready-for-agent

- [ ] Solo/team setup with Role → player-name assignment works in the deployed app
- [x] Personal Objectives are defined in data files and evaluated by `sim` in the Debrief
- [x] Test: Personal Objectives are correctly met or missed for known Designs, and their bonuses appear in the Score breakdown
- [x] Go/No-Go blocks launch until every Role votes Go; solo play skips it
- [x] The Debrief lists each Role, its player name and whether it met its Personal Objective
