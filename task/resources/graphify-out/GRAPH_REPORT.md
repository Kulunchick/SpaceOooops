# Graph Report - resources  (2026-11-15)

## Corpus Check
- Corpus is ~39,348 words - fits in a single context window. You may not need a graph.

## Summary
- 276 nodes · 404 edges · 12 communities (11 shown, 1 thin omitted)
- Extraction: 91% EXTRACTED · 9% INFERRED · 0% AMBIGUOUS · INFERRED: 37 edges (avg confidence: 0.82)
- Token cost: 392,115 input · 0 output

## Community Hubs (Navigation)
- Concept Maturity Levels
- Mission Design Process
- Launch Safety Documentation
- Launch Integration Ecosystem
- NASA Science Programs
- RF Licensing Regulation
- CubeSat Form Factor
- CSLI Selection Pathway
- Funding and Testing
- CML Technical Checklists
- Ground Station Operations
- Launch (isolated)

## God Nodes (most connected - your core abstractions)
1. `Design and Develop Science Missions (webinar deck)` - 20 edges
2. `Concept Maturity Level (CML)` - 18 edges
3. `Flight Certification Documentation` - 15 edges
4. `Orbital Debris Assessment Report` - 15 edges
5. `CML Matrix Technical Rows` - 12 edges
6. `Mission Integrator` - 12 edges
7. `CML Matrix` - 11 edges
8. `NASA CSLI` - 11 edges
9. `CubeSat` - 10 edges
10. `Space Mission Concept Development Using Concept Maturity Levels` - 9 edges

## Surprising Connections (you probably didn't know these)
- `Terminal Node Controller` --conceptually_related_to--> `Amateur License`  [AMBIGUOUS]
  nasa_csli_cubesat_101_508.pdf → nasa_csli_cubesat_101_508.pdf  _Bridges community 10 → community 5_
- `Concept of Operations (ConOps)` --references--> `Concept of Operations Outline, Appendix S`  [INFERRED]
  250323-design-and-develop-science-mission.pdf → 250323-design-and-develop-science-mission.pdf  _Bridges community 4 → community 1_
- `CubeSat` --conceptually_related_to--> `Two-Line Element`  [INFERRED]
  nasa_csli_cubesat_101_508.pdf → nasa_csli_cubesat_101_508.pdf  _Bridges community 6 → community 10_
- `Launch Vehicle Provider` --conceptually_related_to--> `Launch Vehicle`  [INFERRED]
  nasa_csli_cubesat_101_508.pdf → nasa_csli_cubesat_101_508.pdf  _Bridges community 2 → community 3_
- `Development Testing` --conceptually_related_to--> `Free Advice Design Considerations`  [INFERRED]
  nasa_csli_cubesat_101_508.pdf → nasa_csli_cubesat_101_508.pdf  _Bridges community 6 → community 8_

## Import Cycles
- None detected.

## Hyperedges (group relationships)
- **CML spine progression from Cocktail Napkin to CDR** — 13_3547_a1b_cml_1_cocktail_napkin, 13_3547_a1b_cml_2_initial_feasibility, 13_3547_a1b_cml_3_trade_space, 13_3547_a1b_cml_4_point_design, 13_3547_a1b_cml_5_baseline_concept, 13_3547_a1b_cml_6_integrated_concept, 13_3547_a1b_cml_7_preliminary_implementation_baseline, 13_3547_a1b_cml_8_pdr_integrated_baseline, 13_3547_a1b_cml_9_cdr [EXTRACTED 1.00]
- **CML tool set (terminology, matrix, checklists)** — 13_3547_a1b_concept_maturity_level, 13_3547_a1b_cml_matrix, 13_3547_a1b_cml_checklists [EXTRACTED 1.00]
- **CML Matrix row bins aligned to NASA science + TMCO evaluation** — 13_3547_a1b_science_rows, 13_3547_a1b_technical_rows, 13_3547_a1b_management_rows, 13_3547_a1b_cost_rows, 13_3547_a1b_other_rows, 13_3547_a1b_tmco [EXTRACTED 1.00]
- **BioNutrients-2 walked through the six-step mission lifecycle** — 250323_design_and_develop_science_mission_bionutrients_2, 250323_design_and_develop_science_mission_identify_need_and_goals, 250323_design_and_develop_science_mission_define_concept, 250323_design_and_develop_science_mission_define_requirements, 250323_design_and_develop_science_mission_design, 250323_design_and_develop_science_mission_build_and_verify, 250323_design_and_develop_science_mission_implement [EXTRACTED 1.00]
- **BioNutrients-2 mission segments** — 250323_design_and_develop_science_mission_bionutrients_2, 250323_design_and_develop_science_mission_ng_18, 250323_design_and_develop_science_mission_space_segment_experiment_procedure, 250323_design_and_develop_science_mission_on_console_support, 250323_design_and_develop_science_mission_user_segment_sample_processing [EXTRACTED 1.00]
- **On-orbit sample chain: PWD hydration, incubation, MELFI freezing** — 250323_design_and_develop_science_mission_potable_water_dispenser, 250323_design_and_develop_science_mission_production_bag, 250323_design_and_develop_science_mission_incubator, 250323_design_and_develop_science_mission_melfi [EXTRACTED 1.00]
- **CubeSat Development Lifecycle Phases** — nasa_csli_cubesat_101_508_concept_development, nasa_csli_cubesat_101_508_securing_funding, nasa_csli_cubesat_101_508_proposal_to_csli, nasa_csli_cubesat_101_508_selection_and_manifesting, nasa_csli_cubesat_101_508_mission_coordination, nasa_csli_cubesat_101_508_regulatory_licensing, nasa_csli_cubesat_101_508_cubesat_hardware_fabrication_and_testing, nasa_csli_cubesat_101_508_mission_readiness_review, nasa_csli_cubesat_101_508_cubesat_to_dispenser_integration, nasa_csli_cubesat_101_508_dispenser_to_launch_vehicle_integration, nasa_csli_cubesat_101_508_launch, nasa_csli_cubesat_101_508_mission_operations [EXTRACTED 1.00]
- **CSLI Mission Models** — nasa_csli_cubesat_101_508_nasa_procured_launch_vehicle_mission_model, nasa_csli_cubesat_101_508_ors_rideshare_mission_model, nasa_csli_cubesat_101_508_nro_rideshare_mission_model, nasa_csli_cubesat_101_508_third_party_broker_mission_model, nasa_csli_cubesat_101_508_iss_deployment_mission_model [EXTRACTED 1.00]
- **Sources Feeding the Mission ICD** — nasa_csli_cubesat_101_508_interface_control_document, nasa_csli_cubesat_101_508_cubesat_design_specification, nasa_csli_cubesat_101_508_lsp_req_317_01, nasa_csli_cubesat_101_508_range_safety, nasa_csli_cubesat_101_508_gsfc_std_7000 [EXTRACTED 1.00]
- **FCC license application package** — nasa_csli_cubesat_101_508_experimental_license, nasa_csli_cubesat_101_508_iaru_coordination_letter, nasa_csli_cubesat_101_508_spacecap_notice, nasa_csli_cubesat_101_508_orbital_debris_assessment_report, nasa_csli_cubesat_101_508_remote_sensing_license [EXTRACTED 1.00]
- **Environmental verification testing** — nasa_csli_cubesat_101_508_day_in_the_life_testing, nasa_csli_cubesat_101_508_dynamic_environment_testing, nasa_csli_cubesat_101_508_thermal_vacuum_bakeout, nasa_csli_cubesat_101_508_testing_procedures_reports [EXTRACTED 1.00]
- **Deployment inhibit system** — nasa_csli_cubesat_101_508_remove_before_flight_pin, nasa_csli_cubesat_101_508_separation_switches, nasa_csli_cubesat_101_508_independent_inhibit, nasa_csli_cubesat_101_508_electrical_report [INFERRED 0.85]

## Communities (12 total, 1 thin omitted)

### Community 0 - "Concept Maturity Levels"
Cohesion: 0.06
Nodes (42): JPL A-Team, Announcement of Opportunity (AO), Assigned vs Competed Project Lifecycles, Baseline Commitment Review (BCR), CML 1 Cocktail Napkin, CML 2 Initial Feasibility, CML 3 Trade Space, CML 4 Point Design (+34 more)

### Community 1 - "Mission Design Process"
Cohesion: 0.08
Nodes (15): Beta Carotene, BioNutrients Project, BioNutrients-1, BioNutrients-1 Production Pack, BioNutrients-2, Fluorinated Ethylene Propylene (FEP) Bag, Genetically-Modified Yeast, Incubator (+7 more)

### Community 2 - "Launch Safety Documentation"
Cohesion: 0.09
Nodes (32): AFOSHSTD 48-9, AFSPCMAN 91-710, Battery Report, Compliance Letter, CubeSat Acceptance Checklist, CubeSat-to-Dispenser ICD, Day In The Life Testing, Dimensional Verification (+24 more)

### Community 3 - "Launch Integration Ecosystem"
Cohesion: 0.10
Nodes (31): APIC, CubeSat 101, Debris Assessment Software, Development Process Overview, Dispenser Standards, Dispenser-to-Launch Vehicle Integration, Federal Statutes, GSFC-STD-7000 (+23 more)

### Community 4 - "NASA Science Programs"
Cohesion: 0.09
Nodes (28): Access to Space for All Systems Engineering Webinar Series, Artemis Program, Astrophysics, Concept of Operations Outline, Appendix S, Dawn Mission, Design and Develop Science Missions (webinar deck), Earth Science, Exploration Mission (Presidential directive driven) (+20 more)

### Community 5 - "RF Licensing Regulation"
Cohesion: 0.12
Nodes (23): 15 CFR Part 960, 47 CFR Part 97, Amateur License, Commercial License, Experimental License, FCC, FCC Form 442, Government License (+15 more)

### Community 6 - "CubeSat Form Factor"
Cohesion: 0.10
Nodes (20): 3U Dispenser, 6U Dispenser, ADCS, Attitude Determination and Control System, Burn Wire, Cal Poly, CubeSat, CubeSat Design (+12 more)

### Community 7 - "CSLI Selection Pathway"
Cohesion: 0.12
Nodes (18): AMSAT, Announcement of Partnership Opportunity, BisonSat, Chasqui 1, Concept Development, CubeSat Developer, ELaNa, Feasibility Review (+10 more)

### Community 8 - "Funding and Testing"
Cohesion: 0.12
Nodes (18): CRADA, Crowdfunding, CubeSat Hardware Fabrication and Testing, Deliverables, Development Testing, Engineering Test Unit, Environmental Testing, FlatSat (+10 more)

### Community 9 - "CML Technical Checklists"
Cohesion: 0.14
Nodes (13): CML 4 Checklist Sheet, CML Checklists, Ground System / Mission Operations System Design, Instrument System Design, Launch Services, Master Equipment List (MEL), Mission Development, Planetary Protection (+5 more)

### Community 10 - "Ground Station Operations"
Cohesion: 0.29
Nodes (8): American Radio Relay League, CubeSat IRC Channel, Ground Station, JSpOC, Mission Operations, State Vector, Terminal Node Controller, Two-Line Element

## Ambiguous Edges - Review These
- `Terminal Node Controller` → `Amateur License`  [AMBIGUOUS]
  nasa_csli_cubesat_101_508.pdf · relation: conceptually_related_to

## Knowledge Gaps
- **96 isolated node(s):** `CML 1 Cocktail Napkin`, `CML 8 PDR (Integrated Baseline)`, `CML 9 CDR`, `Pre-Phase A`, `Phase A Concept & Technology Phase` (+91 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 100 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **1 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Flight Certification Documentation` connect `Launch Safety Documentation` to `Funding and Testing`, `Launch Integration Ecosystem`?**
  _High betweenness centrality (0.083) - this node is a cross-community bridge._
- **Are the 2 inferred relationships involving `Orbital Debris Assessment Report` (e.g. with `NASA-STD-8719.14A` and `NPR 8715.6A`) actually correct?**
  _`Orbital Debris Assessment Report` has 2 INFERRED edges - model-reasoned connections that need verification._
- **What connects `CML 1 Cocktail Napkin`, `CML 8 PDR (Integrated Baseline)`, `CML 9 CDR` to the rest of the system?**
  _96 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Concept Maturity Levels` be split into smaller, more focused modules?**
  _Cohesion score 0.05919661733615222 - nodes in this community are weakly interconnected._
- **Why does `CubeSat 101` connect `Launch Integration Ecosystem` to `Launch Safety Documentation`, `RF Licensing Regulation`, `CubeSat Form Factor`, `CSLI Selection Pathway`?**
  _High betweenness centrality (0.076) - this node is a cross-community bridge._
- **Should `Mission Design Process` be split into smaller, more focused modules?**
  _Cohesion score 0.08403361344537816 - nodes in this community are weakly interconnected._

### Low-confidence Hints
_AMBIGUOUS edges — the extractor was unsure. Verify before acting on these._

- **What is the exact relationship between `Terminal Node Controller` and `Amateur License`?**
  _Edge tagged AMBIGUOUS (relation: conceptually_related_to) - confidence is low._