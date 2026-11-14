# 02: Crop Watch: Scenario brief, Design and Launch Review

**What to build:** A solo player picks the Crop Watch Scenario: drought and crop monitoring over Angola from SSO. They read its brief: the objective, required Science Value, minimum Lifetime, Budget, and why the mission matters.

They then build a Design across four Role tabs:
- **Science**: Instruments.
- **Spacecraft**: Structure, solar panels, battery, attitude control, Reliability Tier.
- **Launch & Orbit**: Orbit, Launch Vehicle.
- **Budget & Comms**: comms band, cost breakdown.

Gauges for Budget, Mass, Power, Data and Lifetime are always visible and update on every change. They turn amber for thin margins and red for violations. The Launch Review screen lists every broken hard constraint and every soft warning with a plain-language reason. "Launch" is enabled only when no hard constraint is broken.

All numbers come from human-readable data files bundled into the build, never hard-coded. They are approximately realistic, from NASA CubeSat 101 and public specs. This ticket only needs the minimal catalog for Crop Watch: SSO, one or two Launch Vehicle classes, two options per Component category, 2–3 Instruments, and the Basic and Flight-proven Reliability Tiers.

This ticket adds two calls to the `sim` interface:
- **Load catalog**.
- **Launch Review (Design, Scenario)**: returns resource totals and margins, hard-constraint violations and soft warnings, each with a reason.

The UI holds no rules; it renders what `sim` returns.

**Blocked by:** 01: Skeleton and deploy.

**Status:** ready-for-agent

- [ ] Scenario select → brief → Design works in the deployed app
- [ ] Gauges show the Launch Review totals and update instantly on every choice
- [ ] Options show Mass, Power, cost, Data and Reliability Tier
- [ ] Incompatible options are shown with the reason (e.g. imager needs reaction wheels)
- [x] Hard constraints block launch with reasons: Mass over Launch Vehicle capacity to the Orbit, cost over Budget, negative average Power balance, a pointing Instrument without reaction wheels, Structure volume exceeded, no comms
- [x] Soft warnings appear without blocking launch: thin margins, battery that only just covers the eclipse, Data above downlink, Basic tier on critical Components
- [x] A test loads the bundled data files; each hard constraint has one `sim` test that asserts the violation and its reason through the public interface
- [x] A test shows a known-good Crop Watch Design passes Launch Review
- [ ] All UI text is in English and uses GLOSSARY.md terms
