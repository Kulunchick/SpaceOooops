Status: resolved

# Spec: World Scenarios

## Problem Statement

Every Scenario in the game is set in Angola. That was a deliberate choice: the challenge author is Angola's national
space programme office, and it ties the game to a real place. But it also means the player only ever learns one
corner of the world, only three kinds of mission (crop imaging, a GEO relay, space weather), and never sees how the
same engineering trade-offs play out against other problems: clouds that hide the ground, a disaster that needs an
image now, a storm that must be watched all day, a glacier that shrinks over years.

The game also quietly assumes Angola everywhere, not only in the Scenarios: the ground stations are always near Luanda
and Johannesburg, the Scenario select screen always looks at Angola, and some Orbit and Instrument explanations say
"in view of Angola". A Mission somewhere else would read wrong.

## Solution

Four new Scenarios, one per country of a 2026 Space Apps partner agency, each with a problem unlike the Angolan three.
The three Angolan Scenarios stay as they are and stay first.

| Order | Target Country | Scenario | Difficulty | Intended path | Ground stations | Real-Mission Match |
|---|---|---|---|---|---|---|
| 1 | Brazil | Amazon deforestation watch | Medium | Radar or optics against near-constant cloud | Cuiabá (INPE), backup Alcântara | Amazonia-1 |
| 2 | Japan | First images after an earthquake or tsunami | Hard | Radar, heavy Data, dense ground contact, tight Budget | Tsukuba (JAXA), backup Okinawa | ALOS-2 (Daichi-2) |
| 3 | India | Cyclone watch over the Bay of Bengal | Medium | GEO with a new weather imager | Bengaluru / Hassan (ISTRAC) | INSAT-3DR |
| 4 | Argentina | Patagonian glacier retreat | Easy | Optics on SSO, a long Lifetime to see the trend | Córdoba, Teófilo Tabanera (CONAE), backup Ushuaia | SAOCOM |

Two new Instruments make the new lessons possible: a synthetic-aperture radar (SAR), shared by Brazil and Japan, that
sees through cloud and at night but is heavy, power-hungry and produces a lot of Data; and a weather imager for GEO,
for India.

Each Scenario has a Target Country. The globe outlines it, places the Scenario's pin and ground stations in it, and
highlights it while its Scenario is hovered or selected. On the Scenario select screen the camera turns to the pin of
the hovered or selected Scenario. The title screen keeps Angola highlighted and the camera over Africa.

Explanations that are not about one Scenario stop naming Angola and speak of "the target region".

## User Stories

1. As a player, I want Scenarios in several countries, so that the game shows space missions solving problems around the world.
2. As a player, I want the Angolan Scenarios to stay first, so that the game still opens with its tutorial.
3. As a judge, I want the Angolan core kept, so that the game stays tied to the challenge author.
4. As a judge, I want every new country to be a Space Apps partner, so that the choice is relevant to the challenge.
5. As a player, I want each Scenario card to name its Target Country, so that I know where I am going before I open it.
6. As a player, I want the cards in one list ordered Angola first then by difficulty, so that I find an easy start quickly.
7. As a player on the Scenario select screen, I want the globe to turn to the pin of the card I hover or select, so that I see where each Mission is.
8. As a player, I want the camera to rest on Angola when nothing is hovered, so that the screen opens where it does today.
9. As a player, I want the hovered or selected Scenario's Target Country outlined and highlighted, so that I recognise it on a low-poly globe.
10. As a player, I want only one country highlighted at a time, so that the highlight tells me which Scenario I am looking at.
11. As a player on the title screen, I want Angola highlighted and the camera over Africa as today, so that the game's home is unchanged.
12. As a player, I want the brief's Region shot centred on the Scenario's pin, so that the brief starts over the right country.
13. As a player, I want ground stations in the Target Country of my Scenario, so that a Brazilian Mission does not downlink to Luanda.
14. As a student, I want ground station names to be real stations of the national agency, so that I learn who operates them.
15. As a player, I want a backup station where the agency has one, so that a ground station outage behaves as it does today.
16. As a player in the Brazil Scenario, I want optics to be cheap but often blind under cloud, so that I face a real trade-off.
17. As a player in the Brazil Scenario, I want radar to see through cloud at a cost in Mass, Power, Data and Budget, so that paying for it is a choice and not a rule.
18. As a student, I want the brief to explain why the Amazon is cloudy and why that matters for deforestation alerts, so that the trade-off makes sense.
19. As a player in the Japan Scenario, I want the Mission to need images quickly and in any weather, so that it is the hardest Scenario.
20. As a player in the Japan Scenario, I want radar's heavy Data to need a strong downlink, so that comms matter.
21. As a player in the Japan Scenario, I want a tight Budget, so that I must give something up.
22. As a player in the India Scenario, I want a low Orbit to miss most cyclones, so that I learn why weather satellites sit in GEO.
23. As a player in the India Scenario, I want a weather imager that only makes sense in GEO, so that the Orbit and the Instrument are chosen together.
24. As a student, I want the India brief to contrast GEO for weather with GEO for communications (Connect the Villages), so that I see one Orbit serve two purposes.
25. As a player in the Argentina Scenario, I want an easy Mission with plain optics on SSO, so that it is a second gentle start.
26. As a player in the Argentina Scenario, I want a long minimum Lifetime, so that I learn that trends need years of data.
27. As a student, I want the Argentina brief to explain why glaciers are measured over years, so that the Lifetime requirement makes sense.
28. As a player, I want every new Scenario to have a known Design that succeeds, so that every Scenario is winnable.
29. As a player, I want tempting wrong Designs to fail Launch Review or the Run, so that each Scenario teaches its lesson.
30. As a player, I want existing Instruments and Orbits to work sensibly in new Scenarios, so that I can try my own ideas.
31. As a player, I want the new Instruments available in every Scenario, so that the Design screen stays one catalog.
32. As a player, I want the new Instruments to be useless or weak where they do not fit, so that they do not become the answer to everything.
33. As a student, I want each new Instrument to explain in plain words what it measures and what it costs, so that I can choose it with understanding.
34. As a player, I want the Debrief to show a real national mission like mine, so that my Design connects to a real one.
35. As a student, I want each new Scenario's brief to say why it matters to people in that country, so that the goal feels real.
36. As a player, I want tutorial hints in each new Scenario, so that I am not lost in an unfamiliar problem.
37. As a player, I want every Orbit and Instrument explanation to read correctly in any Scenario, so that no text says "Angola" in Japan.
38. As a player, I want the Scenario select screen text to say the pins are around the world, so that it matches what I see.
39. As a player, I want the Sources screen to explain why Angola is the home and why the other countries were chosen, so that the choice is honest.
40. As a player, I want the new Random Events to happen in the new Scenarios too, so that every Run gets the same surprises.
41. As a player, I want ground station outages in the new Scenarios to behave like today's, so that the rules do not change by country.
42. As a player, I want a shared Design of a new Scenario to replay the same way, so that sharing still works.
43. As a team, we want the new Scenarios playable with up to four Roles, so that team play needs no new rules.
44. As a player on a device without WebGL, I want each new Scenario to have an illustration, so that the select screen works without the globe.
45. As a player on a phone, I want seven cards to fit the select screen, so that the list stays usable.
46. As a developer, I want a Scenario without a coverage or fit number for some Orbit or Instrument to fail catalog loading, so that a silent zero cannot slip in.

## Implementation Decisions

Order of work, agreed for the deadline day (2026-11-15):

- The More Random Events spec goes first and in full (all 59 events, questions, 3D effects, star models, sounds).
- This spec comes after. A shared groundwork ticket blocks the four country tickets; the country tickets are done in
  the order of the table (Brazil, Japan, India, Argentina).
- Checkpoint at 20:00 local time. If behind, cut in this order: event sounds, star models other than the UFO, event 3D
  effects not yet done (those events stay log-only), then Scenarios from the end of the queue (Argentina, India, Japan).

Groundwork (one ticket, blocks the four):

- **Scenario gains a Target Country.** A country id plus display name, used for the card label, the highlight and the
  globe outline. The existing three get Angola.
- **Scenario gains its ground stations.** A list of named stations with latitude and longitude, primary first. It
  replaces the fixed Luanda and Johannesburg pair in the scene. The Angolan three keep today's two stations as their
  values. The Run's outage logic does not change; it still uses the per-Scenario outage tuning.
- **Coverage and fit become complete.** Catalog loading rejects a Scenario that lacks a coverage number for any Orbit or
  a fit number for any Instrument, instead of silently using zero. Every existing entry is filled in explicitly
  (zero where zero is meant).
- **The Scene Spec names the highlighted Target Country.** The title screen names Angola; the Scenario select screen
  names the hovered or selected Scenario's country and otherwise Angola; the brief and later screens name the
  current Scenario's. The JS side colours the facets of the named country only.
- **The Scenario select Shot follows the hovered or selected pin**, reusing the existing Region-style framing by
  latitude and longitude; with nothing hovered it rests on Angola as today. The fixed Angola centre stays only as that
  default.
- **Country outlines are hand-traced**, coarse polygons in the same style as today's Angola outline, one per Target
  Country (Japan as two or three islands). They live in the JS land mask beside the existing continent polygons. The
  land mask's "is this point Angola" check becomes "which Target Country is this point in".
- **Neutral wording** for text that is not about one Scenario: the GEO Orbit explanation, the village relay
  explanation, the Scenario select intro and the Sources explanation. Scenario-specific text (briefs, hints,
  constraints) keeps naming its country.
- **Card order and label**: Angola first, then the rest by difficulty; each card shows its Target Country.
- **Glossary**: add Target Country; reword definitions that assume Angola.

New Instruments:

- **Synthetic-aperture radar (SAR).** Heavy, power-hungry, high Data rate, expensive; sees through cloud and at night.
  High fit in Brazil and Japan, low elsewhere.
- **Weather imager.** Coarse resolution, continuous viewing; high fit only with GEO in India, poor in low Orbits.
- No other new Components, Orbits or Launch Vehicles unless a country ticket finds a Scenario unwinnable without one;
  new parts are allowed but are not the default.

Scenario design:

- No new metrics. "Quick first image" (Japan) and "multi-year trend" (Argentina) are expressed with the existing levers:
  coverage per Orbit, fit per Instrument, minimum Lifetime, required Science Value, Budget, and the brief's text.
- Brazil: optical Instruments get low fit (cloud); SAR high. Known-good Design uses SAR on SSO; an optical Design on SSO
  launches but falls short of Science Value.
- Japan: Hard; SAR plus a strong downlink; tight Budget. An S-band radio cannot carry the radar's Data.
- India: GEO with the weather imager. A low Orbit has low coverage and misses the required Science Value.
- Argentina: Easy; optics on SSO; minimum Lifetime of several years. A short-lived Design fails on Lifetime.
- Each new Scenario gets its own event tuning (flare and outage chances) in the same ranges as today's.
- Each gets a Real-Mission Match entry with operator and a short factual blurb, and its purpose keyed by Scenario id.
- Each gets an illustration for the no-WebGL select screen, in the style of the existing three.
- Ground station coordinates and Real-Mission Match facts are checked against public sources before they are entered
  and are cited on the Sources screen.

Interaction with More Random Events:

- The event pool is shared by all Scenarios and names no place, so it needs no change. Its rule that event text names
  no real country applies to events only, not to Scenario briefs.

## Testing Decisions

A good test checks behaviour through a public seam (what a player can do and see), not how the data is laid out. Three
seams, all existing; no new ones.

1. **The public `sim` interface**, following the existing Scenario and sweep tests:
   - every new Scenario has a known-good Design that passes Launch Review cleanly and reaches Success on a fixed seed
     and on the large majority of seeds;
   - each Scenario's lesson as tempting-but-wrong Designs: optics in Brazil fall short; S-band in Japan cannot keep up
     with the radar's Data; a low Orbit in India falls short; a short Lifetime in Argentina fails;
   - the catalog has seven Scenarios with difficulty, key constraints and a Target Country;
   - catalog loading rejects a Scenario missing a coverage or fit number;
   - the existing sweep covers the new Scenarios and Instruments automatically.
2. **The Scene Spec builder**, following the existing marker, effects and Shot tests:
   - one marker per Scenario at its data position, markers apart from each other;
   - ground stations come from the current Scenario;
   - the Scenario select Shot frames the hovered or selected pin, and Angola when nothing is hovered;
   - the highlighted Target Country is Angola on the title screen and the current Scenario's country elsewhere;
   - today's "everything is in Angola" tests become "each Scenario's marker and stations are near its Target Country".
3. **End-to-end (Playwright)**, following the existing Scenario select specs, in both WebGL and fallback projects:
   seven cards with a Target Country label, the Angolan three first, an illustration on every card. The existing
   "exactly three cards" checks change to seven.

Country outlines and facet highlighting live only in JS, which has no unit tests. They are checked by eye and by the
WebGL end-to-end smoke run (the select screen renders and hovering a card does not error).

## Out of Scope

- New metrics such as time to first image or trend accuracy.
- Real orbital geometry: ground tracks, pass times, revisit computed from location. Location still matters only
  through the hand-tuned coverage and fit numbers.
- Highlighting all Target Countries at once.
- Fine country borders or borders for countries without a Scenario.
- Changing the three Angolan Scenarios' balance, briefs or difficulty.
- Grouping cards by region or tabs.
- Translating game text.
- Scenario-specific Random Events.

## Further Notes

- The submission deadline is 2026-11-15, 23:59 local time. Whatever is not finished at the 20:00 checkpoint is cut in
  the agreed order; the build that ships must be working, not nearly done.
- The Japan and India Scenarios carry new mechanics (radar, GEO imaging) and are therefore ahead of Argentina, which
  is closest to Crop Watch.
- Partner agencies behind the choice: Brazilian Space Agency, JAXA, ISRO, National Space Activities Commission of
  Argentina (CONAE), alongside the Angola National Space Programme Management Office.

## Comments

- 2026-11-15: Resolved on branch `more-random-events` (one integration branch for both specs), tickets 01 to 08. Deviations: Glacier Watch's minimum Lifetime is two years, not several, and its known-good Design wins on about 85% of seeds (Component failures over 730 days; a longer Lifetime drops it under the test's floor); its backup station is Tolhuin, Tierra del Fuego, because the Ushuaia station is only planned; Disaster Response's known-good Design launches with one Budget-margin warning, because a Budget tight enough to rule out the dearest part in every slot leaves no Design the 15% margin; Amazonia-1 and SAOCOM match by objective and Orbit, not by Instrument; India lists a backup station (Hassan); the Sun follows the camera on globe and region Shots so a highlighted country is lit.
