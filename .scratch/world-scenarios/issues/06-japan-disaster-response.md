# 06: Japan: first images after an earthquake or tsunami

Spec: `../spec.md`. Take these tickets after `.scratch/more-random-events/` (agreed deadline-day priority).

**What to build:** A Hard Scenario over Japan. Images are needed fast and in any weather, so the radar from ticket 05 is
the intended Instrument; its heavy Data needs a strong downlink; the Budget is tight, so something must be given up.
"Fast" is expressed with existing levers (coverage, fit, required Science Value, Budget) and the brief, not a new
metric. Includes the Japan outline (two or three islands), the pin, ground stations Tsukuba (JAXA) and backup Okinawa,
ALOS-2 (Daichi-2) as a Real-Mission Match, a no-WebGL illustration, tutorial hints, event tuning, and Sources entries,
checked against public sources first.

**Blocked by:** 05

**Status:** resolved

- [x] A known-good radar Design passes Launch Review cleanly and reaches Success on a fixed seed and on the large majority of seeds
- [x] The same Design with an S-band radio cannot keep up with the radar's Data and fails
- [x] An optical Design falls short of the required Science Value
- [x] Japan is outlined, highlighted when its card is hovered or selected, and holds the pin and stations
- [x] The Debrief can show ALOS-2 for a matching Design
- [x] Illustration in the fallback project; end-to-end checks pass; the sweep passes
