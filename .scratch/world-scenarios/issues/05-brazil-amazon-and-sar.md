# 05: Brazil: Amazon deforestation watch and the radar

Spec: `../spec.md`. Take these tickets after `.scratch/more-random-events/` (agreed deadline-day priority).

**What to build:** A Medium Scenario over the Brazilian Amazon, and a new Instrument, a synthetic-aperture radar (SAR):
heavy, power-hungry, high Data rate, expensive, sees through cloud and at night. In this Scenario optical Instruments
have low fit because of near-constant cloud and the radar has high fit; in the Angolan Scenarios the radar has low fit.
The brief explains why the Amazon is cloudy and why fast deforestation alerts matter. Includes the Brazil outline, the
pin, ground stations Cuiabá (INPE) and backup Alcântara, Amazonia-1 as a Real-Mission Match, a no-WebGL illustration,
tutorial hints, event tuning in today's ranges, and Sources entries. Station coordinates and mission facts are checked
against public sources first.

**Blocked by:** 01, 02, 03, 04

**Status:** ready-for-agent

- [ ] A known-good radar Design on SSO passes Launch Review cleanly and reaches Success on a fixed seed and on the large majority of seeds
- [ ] An optical Design on SSO launches but falls short of the required Science Value
- [ ] The radar appears on the Design screen with a plain-words explanation of what it does and what it costs
- [ ] The radar does not become the best answer in the Angolan Scenarios
- [ ] Brazil is outlined, highlighted when its card is hovered or selected, and holds the pin and stations
- [ ] The Debrief can show Amazonia-1 for a matching Design
- [ ] The card has an illustration in the fallback project; end-to-end checks pass with the new card count
- [ ] The sweep passes with the new Scenario and Instrument
