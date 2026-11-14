# Graph Report - SpaceOooops  (2026-11-14)

## Corpus Check
- Corpus is ~49,465 words - fits in a single context window. You may not need a graph.

## Summary
- 1001 nodes · 2443 edges · 38 communities (32 shown, 6 thin omitted)
- Extraction: 84% EXTRACTED · 16% INFERRED · 0% AMBIGUOUS · INFERRED: 400 edges (avg confidence: 0.79)
- Token cost: 378,786 input · 0 output

## Community Hubs (Navigation)
- Scenario Test Fixtures
- Catalog and Launch Review
- Spec, Glossary and Issues
- Run Simulation and Events
- Debrief and Objectives
- Review and Run Tests
- Debrief Cause Analysis
- Design Model and Sharing
- Challenge Brief and ADRs
- Orbit View and Playback
- History Compare and Stats
- Local Storage History
- Team Roles and Go/No-Go
- Deploy Workflow
- Interactive Decision Tests
- Scenario Select and Brief
- Debrief UI Sections
- CubeSat Flight Certification
- Launch Review Findings
- CubeSat Licensing
- Debrief Screen Actions
- CSLI Selection Process
- Design Screen UI
- App State Signals
- Spacecraft Tab Widgets
- App Shell and Links
- CubeSat Mission Models
- CubeSat Design Spec
- CubeSat Testing and Inhibits
- Resource Dashboards
- CubeSat Operations
- Favicon
- Agent Issue Tracking
- CubeSat 101 Overview
- Sources Screen
- Crate Root

## God Nodes (most connected - your core abstractions)
1. `crop_watch_known_good()` - 85 edges
2. `AppState` - 64 edges
3. `Catalog` - 58 edges
4. `Design` - 50 edges
5. `Spec: SpaceOooops Space Mission Design Game` - 44 edges
6. `load_catalog()` - 43 edges
7. `run_with_decisions()` - 30 edges
8. `SpaceOooops Glossary` - 26 edges
9. `Facts` - 24 edges
10. `launch_review()` - 24 edges

## Surprising Connections (you probably didn't know these)
- `No NASA logos/endorsement policy` --semantically_similar_to--> `Issue 07: Orbit animation and UI polish`  [INFERRED] [semantically similar]
  README.md → .scratch/mission-design-game/issues/07-orbit-animation-and-polish.md
- `bundled_data_files_load()` --calls--> `load_catalog()`  [INFERRED]
  tests/catalog.rs → src/sim/catalog.rs
- `every_option_explains_itself()` --calls--> `load_catalog()`  [INFERRED]
  tests/catalog.rs → src/sim/catalog.rs
- `every_design_parameter_gauge_and_component_category_has_a_why_it_matters_text()` --calls--> `load_catalog()`  [INFERRED]
  tests/debrief.rs → src/sim/catalog.rs
- `every_real_mission_in_the_catalog_has_a_source_link_and_a_fact()` --calls--> `load_catalog()`  [INFERRED]
  tests/debrief.rs → src/sim/catalog.rs

## Import Cycles
- 2-file cycle: `src/sim/resources.rs -> src/sim/review.rs -> src/sim/resources.rs`
- 2-file cycle: `src/sim/causes.rs -> src/sim/debrief.rs -> src/sim/causes.rs`
- 2-file cycle: `src/sim/debrief.rs -> src/sim/objectives.rs -> src/sim/debrief.rs`
- 2-file cycle: `src/sim/debrief.rs -> src/sim/real_mission.rs -> src/sim/debrief.rs`
- 2-file cycle: `src/sim/debrief.rs -> src/sim/run.rs -> src/sim/debrief.rs`
- 2-file cycle: `src/sim/decisions.rs -> src/sim/events.rs -> src/sim/decisions.rs`
- 2-file cycle: `src/sim/catalog.rs -> src/sim/objectives.rs -> src/sim/catalog.rs`
- 3-file cycle: `src/sim/catalog.rs -> src/sim/objectives.rs -> src/sim/debrief.rs -> src/sim/catalog.rs`
- 3-file cycle: `src/sim/causes.rs -> src/sim/debrief.rs -> src/sim/run.rs -> src/sim/causes.rs`
- 4-file cycle: `src/sim/catalog.rs -> src/sim/objectives.rs -> src/sim/debrief.rs -> src/sim/causes.rs -> src/sim/catalog.rs`
- 4-file cycle: `src/sim/catalog.rs -> src/sim/objectives.rs -> src/sim/debrief.rs -> src/sim/real_mission.rs -> src/sim/catalog.rs`
- 4-file cycle: `src/sim/catalog.rs -> src/sim/objectives.rs -> src/sim/debrief.rs -> src/sim/run.rs -> src/sim/catalog.rs`
- 4-file cycle: `src/sim/catalog.rs -> src/sim/objectives.rs -> src/sim/debrief.rs -> src/sim/score.rs -> src/sim/catalog.rs`
- 5-file cycle: `src/sim/catalog.rs -> src/sim/objectives.rs -> src/sim/debrief.rs -> src/sim/causes.rs -> src/sim/review.rs -> src/sim/catalog.rs`
- 5-file cycle: `src/sim/catalog.rs -> src/sim/objectives.rs -> src/sim/debrief.rs -> src/sim/run.rs -> src/sim/review.rs -> src/sim/catalog.rs`
- 5-file cycle: `src/sim/catalog.rs -> src/sim/objectives.rs -> src/sim/debrief.rs -> src/sim/causes.rs -> src/sim/decisions.rs -> src/sim/catalog.rs`
- 5-file cycle: `src/sim/catalog.rs -> src/sim/objectives.rs -> src/sim/debrief.rs -> src/sim/causes.rs -> src/sim/events.rs -> src/sim/catalog.rs`
- 5-file cycle: `src/sim/catalog.rs -> src/sim/objectives.rs -> src/sim/debrief.rs -> src/sim/causes.rs -> src/sim/resources.rs -> src/sim/catalog.rs`
- 5-file cycle: `src/sim/catalog.rs -> src/sim/objectives.rs -> src/sim/debrief.rs -> src/sim/run.rs -> src/sim/causes.rs -> src/sim/catalog.rs`
- 5-file cycle: `src/sim/catalog.rs -> src/sim/objectives.rs -> src/sim/debrief.rs -> src/sim/run.rs -> src/sim/decisions.rs -> src/sim/catalog.rs`

## Hyperedges (group relationships)
- **sim public interface three operations** — _scratch_mission_design_game_spec_load_catalog, _scratch_mission_design_game_spec_launch_review_op, _scratch_mission_design_game_spec_run_op, _scratch_mission_design_game_spec_sim_public_interface [EXTRACTED 1.00]
- **Mission flow: Scenario, Design, Launch Review, Run, Debrief** — glossary_scenario, glossary_design, glossary_launch_review, glossary_run, glossary_debrief, glossary_mission [EXTRACTED 1.00]
- **Three Angola-tied Scenarios of rising difficulty** — _scratch_mission_design_game_spec_crop_watch, _scratch_mission_design_game_spec_connect_the_villages, _scratch_mission_design_game_spec_space_weather_sentinel [EXTRACTED 1.00]
- **Backendless static-site architecture** — docs_adr_0002_no_backend_local_team_play_static_github_pages, docs_adr_0002_no_backend_local_team_play_localstorage_history, docs_adr_0002_no_backend_local_team_play_url_encoded_designs, docs_adr_0002_no_backend_local_team_play_local_team_play [EXTRACTED 1.00]
- **Rejected UI technology options** — docs_adr_0001_leptos_dom_ui_not_game_engine_bevy_macroquad, docs_adr_0001_leptos_dom_ui_not_game_engine_egui_eframe, docs_adr_0001_leptos_dom_ui_not_game_engine_rust_core_typescript_ui [EXTRACTED 1.00]
- **Agent issue-tracking workflow** — docs_agents_issue_tracker_scratch_feature_dirs, docs_agents_issue_tracker_wayfinding, docs_agents_triage_labels_triage_roles [INFERRED 0.85]
- **CubeSat path from selection to orbit** — task_resources_nasa_csli_cubesat_101_508_manifesting, task_resources_nasa_csli_cubesat_101_508_mission_readiness_review, task_resources_nasa_csli_cubesat_101_508_dispenser_integration, task_resources_nasa_csli_cubesat_101_508_lv_integration, task_resources_nasa_csli_cubesat_101_508_launch, task_resources_nasa_csli_cubesat_101_508_mission_operations [EXTRACTED 1.00]
- **Power inhibit safety chain verified by DITL** — task_resources_nasa_csli_cubesat_101_508_rbf_pin, task_resources_nasa_csli_cubesat_101_508_separation_switches, task_resources_nasa_csli_cubesat_101_508_electrical_inhibit, task_resources_nasa_csli_cubesat_101_508_ditl_testing, task_resources_nasa_csli_cubesat_101_508_electrical_report [EXTRACTED 1.00]
- **Requirement sources rolled into the mission ICD** — task_resources_nasa_csli_cubesat_101_508_icd, task_resources_nasa_csli_cubesat_101_508_cds, task_resources_nasa_csli_cubesat_101_508_lsp_req_317_01, task_resources_nasa_csli_cubesat_101_508_range_safety, task_resources_nasa_csli_cubesat_101_508_federal_statutes [EXTRACTED 1.00]

## Communities (38 total, 6 thin omitted)

### Community 0 - "Scenario Test Fixtures"
Cohesion: 0.05
Nodes (71): CatalogError, Invalid, Parse, load_catalog(), parse(), validate(), bundled_data_files_load(), every_option_explains_itself() (+63 more)

### Community 1 - "Catalog and Launch Review"
Cohesion: 0.05
Nodes (63): AttitudeControl, Battery, Catalog, CAUSES, CausesFile, Comms, COMPONENTS, ComponentsFile (+55 more)

### Community 2 - "Spec, Glossary and Issues"
Cohesion: 0.08
Nodes (77): Issue 01: Skeleton and deploy, Issue 02: Crop Watch scenario brief, Design and Launch Review, Issue 03: Run and basic Debrief, Issue 04: Detailed Debrief and learning layer, Issue 05: Connect the Villages and Space Weather Sentinel (full catalog), Issue 06: Team play: Roles, Personal Objectives, Go/No-Go, Issue 07: Orbit animation and UI polish, Issue 08: Sources, credits and README (+69 more)

### Community 3 - "Run Simulation and Events"
Cohesion: 0.06
Nodes (61): DecisionRules, EventTuning, safe_mode_flare(), BUY_STATION_TIME, chosen(), day_adjective(), Decision, DecisionOption (+53 more)

### Community 4 - "Debrief and Objectives"
Cohesion: 0.07
Nodes (53): steps, name, run, with, path, targets, tool, ObjectivesFile (+45 more)

### Community 5 - "Review and Run Tests"
Cohesion: 0.09
Nodes (55): crop_watch_known_good(), a_pointing_instrument_marks_magnetorquers_incompatible(), all_basic_tier_shortens_lifetime_to_a_thin_margin(), basic_tier_on_a_critical_component_warns(), battery_that_only_just_covers_the_eclipse_warns(), cost_over_budget_blocks_launch(), data_above_downlink_warns_without_blocking(), empty_design_is_incomplete_not_a_panic() (+47 more)

### Community 6 - "Debrief Cause Analysis"
Cohesion: 0.11
Nodes (48): CauseRules, battery(), Cause, CauseKind, BatteryTooSmall, ComponentFailure, DataLost, LifetimeShortfall (+40 more)

### Community 7 - "Design Model and Sharing"
Cohesion: 0.09
Nodes (34): TierSpec, Design, Pick, ReliabilityTier, Basic, FlightProven, check(), decode_design() (+26 more)

### Community 8 - "Challenge Brief and ADRs"
Cohesion: 0.06
Nodes (45): ADR 0001: Leptos DOM UI, not a game engine, Bevy / macroquad (rejected), egui/eframe (rejected), Leptos client-side DOM UI (built with Trunk), Rust core with TypeScript UI (rejected), sim module (pure-Rust simulation core), SVG orbit animation, ADR 0002: No backend, local team play (+37 more)

### Community 9 - "Orbit View and Playback"
Cohesion: 0.06
Nodes (30): EventKind, BatteryShortfall, ComponentFailure, GroundStationOutage, SolarFlare, event_class(), EventLog(), icon() (+22 more)

### Community 10 - "History Compare and Stats"
Cohesion: 0.09
Nodes (15): BudgetCommsTab(), CostTable(), ComparisonSection(), Row, rows(), view_row(), data_volume(), money() (+7 more)

### Community 11 - "Local Storage History"
Cohesion: 0.10
Nodes (20): FirstTimeHints(), clear(), HistoryRecord, KEY, LINE_TAG, load(), MAX_RECORDS, parse_all() (+12 more)

### Community 12 - "Team Roles and Go/No-Go"
Cohesion: 0.11
Nodes (15): Role, .ALL, BudgetComms, LaunchOrbit, Science, Spacecraft, ObjectiveOutlook, GoNoGo() (+7 more)

### Community 13 - "Deploy Workflow"
Cohesion: 0.08
Nodes (24): runs-on, concurrency, cancel-in-progress, group, environment, needs, runs-on, steps (+16 more)

### Community 14 - "Interactive Decision Tests"
Cohesion: 0.32
Nodes (22): run_with_decisions(), a_choice_does_not_change_when_later_random_events_happen(), a_run_without_interactive_events_has_no_decision_notes(), buying_station_time_costs_budget_and_shows_in_the_total_cost(), buying_station_time_never_takes_the_mission_over_its_budget(), catalog(), catalog_with_budget_left(), days_before_a_decision_are_identical_whatever_the_choice() (+14 more)

### Community 15 - "Scenario Select and Brief"
Cohesion: 0.11
Nodes (13): BriefScreen(), ScenarioSelectScreen(), Screen, Brief, Debrief, Design, History, LaunchReview (+5 more)

### Community 16 - "Debrief UI Sections"
Cohesion: 0.12
Nodes (5): CausesSection(), DecisionsSection(), ObjectivesSection(), RealMissionSection(), InfoTip()

### Community 17 - "CubeSat Flight Certification"
Cohesion: 0.16
Nodes (17): Battery Report, Compliance Letter, Flight Certification Documentation, Mission-Specific Interface Control Document (ICD), LSP-REQ-317.01 Program Level Requirements, Materials List, Missile System Prelaunch Safety Package (MSPSP), NASA-Procured Launch Vehicle Mission Model (+9 more)

### Community 18 - "Launch Review Findings"
Cohesion: 0.13
Nodes (15): FindingKind, BasicTierOnCritical, BatteryBarelyCoversEclipse, BatteryTooSmallForEclipse, DataAboveDownlink, Incomplete, NeedsReactionWheels, NegativePower (+7 more)

### Community 19 - "CubeSat Licensing"
Cohesion: 0.17
Nodes (14): Amateur License (FCC Part 97), Experimental License (FCC Part 5, Form 442 / STA), Federal Communications Commission (FCC), Federal Statutes (47 CFR Part 97, 15 CFR Part 960), International Amateur Radio Union (IARU), Launch, Margin, Mass Properties Report (+6 more)

### Community 20 - "Debrief Screen Actions"
Cohesion: 0.14
Nodes (5): DebriefActions(), DebriefScreen(), ScoreSection(), StatsSection(), Verdict()

### Community 21 - "CSLI Selection Process"
Cohesion: 0.15
Nodes (13): Announcement of Partnership Opportunity (AoPO), Cooperative Research And Development Agreement (CRADA), NASA CubeSat Launch Initiative (CSLI), Deliverables, Educational Launch of Nanosatellites (ELaNa), Example CSLI CubeSat Missions (LightSail, KickSat, MinXSS, BisonSat, STMSat-1, TJ3Sat, Dellingr), Feasibility Review, Selection and Manifesting (+5 more)

### Community 22 - "Design Screen UI"
Cohesion: 0.20
Nodes (5): Finding, DesignScreen(), findings(), LaunchReviewScreen(), RoleBanner()

### Community 24 - "Spacecraft Tab Widgets"
Cohesion: 0.22
Nodes (4): CardData, slot_section(), SpacecraftTab(), TierSelect()

### Community 25 - "App Shell and Links"
Cohesion: 0.24
Nodes (5): App(), listen_for_links(), open_link_from_url(), PREFIX, TitleScreen()

### Community 26 - "CubeSat Mission Models"
Cohesion: 0.20
Nodes (10): CubeSat Dispenser, CubeSat-to-Dispenser Integration and Testing, ISS Deployment Mission Model (NanoRacks), Launch Vehicle (LV), Dispenser-to-Launch Vehicle Integration, Mission Models, National Reconnaissance Office (NRO) Rideshare Mission Model, Operationally Responsive Space (ORS) Rideshare Mission Model (+2 more)

### Community 27 - "CubeSat Design Spec"
Cohesion: 0.22
Nodes (8): Attitude Determination and Control System (ADCS), Burn Wire Deployable Restraint, CubeSat Design Specification (CDS), CubeSat, CubeSat Acceptance Checklist (CAC), CubeSat Unit (1U, 1.5U, 2U, 3U, 6U, 12U), Dimensional Verifications, UL Listed Batteries

### Community 28 - "CubeSat Testing and Inhibits"
Cohesion: 0.33
Nodes (8): Day In The Life (DITL) Testing, Electrical Inhibit, Electrical Report, Engineering Test Unit (ETU), FlatSat, Mission Readiness Review (MRR), Remove Before Flight (RBF) Pin, Separation Switches

### Community 30 - "CubeSat Operations"
Cohesion: 0.33
Nodes (6): Ground Station, Joint Space Operations Center (JSpOC), Mission Integrator, Mission Operations, Two-Line Element (TLE), Terminal Node Controller (TNC)

### Community 31 - "Favicon"
Cohesion: 0.50
Nodes (5): SpaceOooops Favicon (planet with orbit), Tilted Orange Orbit Ring, Blue Planet, Space Mission Theme, Orbiting Body (moon/spacecraft)

### Community 32 - "Agent Issue Tracking"
Cohesion: 0.50
Nodes (5): Issue tracker: Local Markdown, .scratch/<feature-slug>/ spec and issue files, Wayfinding operations (map, child tickets, frontier, claim, resolve), Triage Labels, Triage roles (needs-triage, needs-info, ready-for-agent, ready-for-human, wontfix)

### Community 33 - "CubeSat 101 Overview"
Cohesion: 0.40
Nodes (4): CubeSat 101: Basic Concepts and Processes for First-Time CubeSat Developers, CubeSat Development Process (16 phases), Notional Timeline of Events/Deliverables, Technical Reference Documents (NASA-STD-8719.14A, GSFC-STD-7000A, UL 1642, MIL-STD-461F)

## Knowledge Gaps
- **113 isolated node(s):** `spaceooops`, `Parse`, `BatteryTooSmall`, `DataLost`, `ComponentFailure` (+108 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 252 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **6 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `AppState` connect `App State Signals` to `Catalog and Launch Review`, `Run Simulation and Events`, `Design Model and Sharing`, `Orbit View and Playback`, `History Compare and Stats`, `Local Storage History`, `Team Roles and Go/No-Go`, `Scenario Select and Brief`, `Debrief UI Sections`, `Debrief Screen Actions`, `Design Screen UI`, `Spacecraft Tab Widgets`, `App Shell and Links`, `Resource Dashboards`?**
  _High betweenness centrality (0.178) - this node is a cross-community bridge._
- **Are the 79 inferred relationships involving `crop_watch_known_good()` (e.g. with `a_design_that_cannot_launch_gives_its_launch_review_violations_as_causes()` and `a_success_still_says_what_went_right_and_what_held_the_score_down()`) actually correct?**
  _`crop_watch_known_good()` has 79 INFERRED edges - model-reasoned connections that need verification._
- **What connects `spaceooops`, `Parse`, `BatteryTooSmall` to the rest of the system?**
  _113 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Scenario Test Fixtures` be split into smaller, more focused modules?**
  _Cohesion score 0.05389876880984952 - nodes in this community are weakly interconnected._
- **Why does `Catalog` connect `Catalog and Launch Review` to `Scenario Test Fixtures`, `Run Simulation and Events`, `Debrief and Objectives`, `Debrief Cause Analysis`, `Design Model and Sharing`, `Local Storage History`, `Interactive Decision Tests`, `App State Signals`?**
  _High betweenness centrality (0.113) - this node is a cross-community bridge._
- **Are the 20 inferred relationships involving `Design` (e.g. with `all_basic()` and `fly()`) actually correct?**
  _`Design` has 20 INFERRED edges - model-reasoned connections that need verification._
- **Should `Catalog and Launch Review` be split into smaller, more focused modules?**
  _Cohesion score 0.050140056022408966 - nodes in this community are weakly interconnected._