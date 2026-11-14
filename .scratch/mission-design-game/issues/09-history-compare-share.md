# 09: History, comparison with the previous Run, Share

**What to build:** Past Runs are kept in the browser, so a returning player can see them. The Debrief compares this Run with the previous Run of the same Scenario: Score, Science Value, cost, Mass and Lifetime, each marked as up or down. A Share button produces a URL that encodes the Design and Scenario; opening it loads that Design, ready to fly or modify. No backend (ADR-0002).

**Blocked by:** 03: Run and basic Debrief.

**Status:** resolved

- [x] Run history survives a page reload (browser local storage) and can be viewed
- [x] The Debrief shows the change from the previous Run of the same Scenario
- [x] The Share link round-trips: opening it loads exactly the same Design and Scenario
- [x] A broken or outdated Share link shows a friendly message instead of crashing
- [x] Test: a Design survives serialization and deserialization through the `sim` public interface

## Comments

- 2026-11-14: Resolved on integration branch `mission-design-game` (merged to `main` at d78ab1e, deployed to https://kulunchick.github.io/SpaceOooops/). Acceptance criteria confirmed by `cargo test` (116 tests) and a scripted browser check of the deployed app.
