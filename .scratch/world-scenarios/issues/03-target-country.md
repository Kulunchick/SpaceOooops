# 03: Target Country: label, highlight, neutral wording

Spec: `../spec.md`. Take these tickets after `.scratch/more-random-events/` (agreed deadline-day priority).

**What to build:** Each Scenario has a Target Country (id and display name); the three existing ones are Angola. The
Scenario card shows the country, and cards are ordered Angola first, then by difficulty. The Scene Spec names the one
Target Country to highlight: Angola on the title screen, the current Scenario's on the brief and later screens, Angola
on the Scenario select screen (ticket 04 makes it follow the hover). The globe colours only that country, using an
outline per country in the land mask (for now only Angola's exists). Text that is not about one Scenario stops naming
Angola: the GEO Orbit explanation, the village relay explanation, the Scenario select intro, and the Sources
explanation (Angola as home, other countries chosen from Space Apps partners). Tests that assume every Scenario is in
Angola are generalised.

**Blocked by:** None (can start immediately).

**Status:** resolved

- [x] A Scenario without a Target Country fails catalog loading
- [x] Each card shows its Target Country; Angolan Scenarios come first, then by difficulty
- [x] The Scene Spec names the highlighted Target Country per screen as above
- [x] The globe highlights only the named country; with Angola named it looks as today
- [x] The listed general texts no longer say "Angola"; Scenario briefs, hints and constraints still do
- [x] Marker tests check "markers apart and near their Target Country" instead of "inside Angola"
- [x] End-to-end Scenario select checks compare the card count with the catalog instead of expecting exactly three, and check the country label, in WebGL and fallback projects
- [x] The glossary entry for Target Country matches what was built
