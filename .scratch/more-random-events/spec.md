Status: resolved

# Spec: More Random Events

The agreed event list (59 events, with what each does and which ask a question) is in `events-draft.md` next to this
file. It is part of this spec. Numbers in it are provisional until balanced (see Implementation Decisions).

## Problem Statement

A Run has only four kinds of Random Event: solar flare, Component failure, ground station outage and battery shortfall.
After two or three Missions the player has seen all of them, and every Run reads the same. All of them are bad news,
none of them is funny, and only two ask the player anything. The Run is the part of the game the player watches rather
than plays, so it is where the game most needs surprise.

## Solution

A Run now also draws from a pool of 59 new Random Events: 40 real ones told with humour (debris on a collision course,
a software update on a Friday evening, birds nesting in the antenna dish), 10 fantasy ones marked "Unexplained" (a UFO
parks alongside, a space whale blocks the Sun) and 9 lucky ones (a sponsor chips in, a rare sight lands in frame).

Each Run gets 5 to 8 of them, 3 or 4 of which ask the player a question with two options and a stated price for each.
No event repeats within a Run, and a different seed gives a different set, so about ten Runs pass before the pool is
exhausted. Every event changes the Run through things the player already manages (Power, Data, pointing, Budget,
Science Value, Instrument time), none ends the Mission by itself, and about a third depend on the Design, so they are
feedback on it rather than noise. Each real event comes with one line in the Debrief on how it happens in real life.

## User Stories

1. As a player, I want a Run to contain events I have not seen before, so that flying again stays interesting.
2. As a player, I want 5 to 8 new Random Events in a Run whatever its length, so that a short Mission is not empty and a long one is not flooded.
3. As a player, I want no new event to happen twice in one Run, so that the event log does not read as a loop.
4. As a player, I want a different seed to give a different set of events, so that two Runs of the same Design differ.
5. As a player, I want the same seed, Design and answers to give the same Run, so that a shared Design replays the same way.
6. As a player, I want some events to be funny, so that the Run is entertaining and not only a report.
7. As a player, I want a few fantasy events such as aliens, a dragon or a space whale, so that the Run sometimes surprises me completely.
8. As a student, I want fantasy events clearly marked "Unexplained" in the event log, so that I can tell a joke from real engineering.
9. As a student, I want each real event to carry a short "how this happens in real life" line in the Debrief, so that I learn something from what hit me.
10. As a player, I want lucky events too (extra Budget, extra downlink, bonus Science Value), so that the unexpected is not always bad.
11. As a player, I want 3 or 4 of a Run's new events to ask me a question, so that I act during the Run and do not only watch.
12. As a player, I want each option of a question to state what it costs and what it buys, so that I can choose on the trade-off.
13. As a player, I want an option I cannot afford to be shown but disabled with the reason, so that I understand why it is closed.
14. As a player, I want a question I do not answer (skipped playback) to take a safe default, so that skipping never breaks the Run.
15. As a player, I want a gamble option to say that it is a gamble, so that a bad outcome feels like my risk and not a trick.
16. As a player, I want the Debrief to say what each of my answers led to, so that I can judge my choices afterwards.
17. As a player, I want every event to have a real effect on the Run, so that events matter and are not decoration.
18. As a player, I want no new event to end the Mission by itself, so that a good Design never dies to a space whale.
19. As a player, I want the new events together to move the result by only about a tenth, so that the Design still decides Success.
20. As a player with a Design on the edge, I want events to be able to tip it, so that I learn the value of margin.
21. As a player, I want some events to happen only with certain choices (low Orbit, deployable panels, an optical Instrument, UHF radio), so that events reflect what I built.
22. As a player, I want some events to be softened by my choices (a Flight-proven computer shrugs off a bit flip), so that paying for reliability visibly pays off.
23. As a student, I want the Debrief to say when a different choice would have prevented or softened an event, so that I know what to change.
24. As a player, I want the rival to be an unnamed "rival agency", so that the game jokes about no real country.
25. As a player, I want helping rescuers with flood images to show as its own line in the Score, so that the good deed is rewarded without deciding Success.
26. As a player, I want Budget gained or spent in an event to count in the Mission's cost and Score, so that money events are real.
27. As a player, I want to see each event in the 3D scene (something flies by, hits the spacecraft, shades the panels, jams the link, glows, or visits), so that I notice it without reading the log.
28. As a player, I want the UFO and a few other stars to have their own model, so that the best jokes land visually.
29. As a player, I want a sound for each kind of scene effect, so that events are heard as well as seen.
30. As a player on a device without WebGL, I want every event fully told in the event log, so that I miss nothing but the picture.
31. As a player who asked for reduced motion, I want events without animation and without the playback slowing for them, so that my setting is respected.
32. As a team, we want questions answered the same way as the flare and outage questions today, so that team play needs no new rules.
33. As a player, I want the playback to slow on a day with an event, so that I can see it at any speed.
34. As a player returning to an old shared Design, I want solar flares, outages and failures to fall on the same days as before for the same seed, so that the Run is recognisably the same flight.
35. As a designer of the game, I want to add or reword an event by editing a data file, so that the pool can grow without programming.
36. As a designer of the game, I want a broken event definition to fail the catalog tests with a clear message, so that a typo never reaches a player.
37. As a designer of the game, I want a Scenario to set how many events its Runs get, so that difficulty can differ by Scenario.

## Implementation Decisions

- **Scope of "new events"**: the four existing Random Events and their two questions keep their code, texts and tuning.
  The new events are an addition beside them.
- **Events are data**: a new catalog file holds one entry per event: id, group (real / fantasy / lucky), title, log
  text, mechanic and strength, optional Design condition, optional Design mitigation, scene effect type, real-life line
  (real events only) and, for an interactive event, the question, its two options and the default. It loads into the
  Catalog like the other data files, and all player-facing text comes from the sim core as it does today.
- **Mechanics**: the code implements a small closed set of mechanics, each timed (for N days or to the end of the Run):
  Power changed by a share; Instruments off; downlink changed by a share or off; Data lost or storage reduced; pointing
  degraded; battery capacity reduced; Component failure chance scaled; Budget spent or gained; Science Value bonus;
  Score bonus; an extra day of Data. An event that needs anything outside this set is rewritten to fit, not given its
  own code.
- **No new event ends the Mission**: mechanics never set a Component as failed and never end the Run.
- **Options with a chance**: an option may name two outcomes and a chance. The roll is drawn when the Run is planned,
  not when the player answers.
- **Planning a Run**: at the start of the Run the sim plans the new events from the seed: how many (between the
  Scenario's minimum and maximum, default 5 to 8), which ones (from the events whose Design condition holds, each at
  most once, with 3 or 4 interactive ones when enough are eligible), on which days (distinct days, spread over the
  planned days), and every chance roll. The plan depends only on the seed, the Design and the Scenario, never on the
  player's answers, so the days before a question still never depend on its answer.
- **Own random stream**: planning uses a second random stream derived from the seed. The existing stream keeps the same
  draws in the same order, so flares, outages and failure rolls fall as before. Results can still differ from before
  where a new event changes the state those events act on.
- **Scenario tuning**: each Scenario gets a minimum and maximum number of new events per Run. Zero switches them off.
- **RandomEvent shape**: the event kind gains one variant for catalog events; a Random Event of that kind carries the
  catalog event id, its group and its scene effect type. Prompt and choice fields are reused unchanged, so the existing
  question UI and `Decision` flow work as they are.
- **Budget**: money spent goes through the existing extra-cost path; money gained raises the Budget left. An option the
  remaining Budget cannot pay for is unavailable with a reason, as the station-time option is today.
- **Score**: a Score bonus from an event (rescuers' images) is its own Score line and does not touch Science Value.
- **Debrief**: every question answered or defaulted gets a note, as today. Every real event that happened gets its
  real-life line. An event with a Design condition or mitigation says what choice caused or softened it.
- **Balance target**: one event shifts Science Value by a few percent; all new events of a Run together stay within
  about ±10% for the reference Designs. The numbers in `events-draft.md` are tuned to this.
- **Event log**: fantasy events show an "Unexplained" mark; lucky events are visually distinct from bad ones.
- **Scene**: six reusable effect kinds are added to the Scene Spec (fly-by, hit, link interference, panel shading,
  bonus glow, visitor), each built from the Run timeline like the existing effects, with stable ids. A visitor or
  fly-by may name a model; three or four events get their own model (the UFO first), the rest share generic ones.
  One Cue per effect kind.
- **Fallbacks**: without WebGL and with reduced motion the events are text only, and playback slowing follows the
  existing rule.
- **Team play**: unchanged. Questions are not tied to a Role.
- **Language and naming**: game text in English. No real countries, agencies or marks; the rival is "a rival agency".
- **Glossary**: the Random Event entry is widened to cover the new groups; "Unexplained" is added as the mark for a
  fantasy Random Event.

## Testing Decisions

A good test here calls a public interface with a Design, a Scenario and a seed and checks what the player would see:
the events in the timeline, the snapshot numbers, the Debrief text, the Scene Spec. It does not check how the plan is
built or which internal state holds an effect.

No new seams. Three existing ones are used:

1. **The Run interface of the sim core** (`run` / `run_with_decisions` returning the timeline and Debrief), as in the
   existing Run, decisions, Debrief, Score and sweep tests. Covered here:
   - determinism: same inputs give the same result; days before a question do not depend on its answer;
   - across many seeds: 5 to 8 new events per Run, 3 or 4 interactive when eligible, none twice, none ends the Mission;
   - an event with a Design condition never appears when the condition does not hold; a mitigation softens its effect;
   - each mechanic, through a small test catalog with one event: the snapshot numbers move as stated;
   - questions: each option, the default on no answer, an unaffordable option falling back to the default;
   - the Debrief notes, real-life lines and the separate Score line;
   - the balance target, as a sweep over seeds for the reference Designs;
   - with new events switched off for a Scenario, a Run is identical to today's (this also keeps the existing pinned
     tests meaningful: they run against a catalog with new events off, or are re-pinned once).
2. **Catalog loading**, as in the existing catalog tests: the shipped event file loads; every event has the fields its
   mechanic and group require; ids are unique; the pool has the agreed size and split; no text names a real country or
   agency from a deny-list.
3. **The Scene Spec builder**, as in the existing scene effects tests: each effect kind appears for its event with the
   right start, length and stable id, and is absent outside its days.

End-to-end (Playwright), following the existing run and effects specs: one Run with a known seed shows a new event in
the log with its mark, a question can be answered, and the fallback (no WebGL) project shows the same text.

## Out of Scope

- Changing the four existing Random Events, their questions or their tuning.
- Remembering seen events between Runs.
- A "realistic mode" setting that turns fantasy events off.
- Events specific to one Scenario.
- Tying a question to a Role in team play.
- A unique 3D animation for every event.
- Translating game text.

## Further Notes

- Today, 2026-11-15, 23:59 local time is the Space Apps submission deadline. This spec does not fit before it. If any
  of it should ship in the submission, the first ticket should be a thin slice that is playable by itself: the catalog
  file, planning, the mechanics and the event log text for a subset of passive events, with no scene work.
- Suggested order after that: interactive events and Debrief notes; Design conditions and mitigations; the remaining
  events and balance; scene effects and Cues; star models.

## Comments

- 2026-11-15: Agreed priority for the deadline day: this spec goes first and in full (all 59 events, questions, 3D effects, star models, sounds), before `.scratch/world-scenarios/`. Checkpoint at 20:00 local time; if behind, cut event sounds, then star models other than the UFO, then unfinished 3D effects (those events stay log-only). This supersedes the thin-slice suggestion in Further Notes.
- 2026-11-15: Resolved on branch `more-random-events`, tickets 01 to 09. Decided during the work, each within the rule that an event is rewritten to fit the mechanics: event 15 is softened by a Flight-proven computer (the catalog has one computer); "optics off" is plain Instruments off; a failure chance can only be lowered; a Power or downlink loss applies after the Design's margin so it costs every Design, and gains were rewritten to Data or Science Value; a scene effect lasts as long as its event, at most 5 days; the rescue bonus is 40 Score points. Rewritten events are listed under Comments in ticket 07.
