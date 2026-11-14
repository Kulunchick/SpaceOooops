# 09: Remaining screens and the Sources page

**What to build:** History, team setup and Sources get the new low-poly skin and panel layout. The Sources page and the README list every external asset (models, fonts, sounds, music, images, three.js) from the credits data file, each with its author, source and licence. The README's AI acknowledgement section and the Sources page state which images and music were made with AI tools. No model, texture or image carries NASA logos, insignia, patches or flags.

**Blocked by:** 01: 3D skeleton.

**Status:** resolved

- [x] History, team setup and Sources use the new skin
- [x] The Sources page lists every entry in the credits data file
- [x] The README asset list and AI disclosure match the credits data file
- [x] History "Load this Design" and share links work as before
- [x] A manual check confirms no NASA marks in any asset

## Comments

- 2026-11-15: Resolved on integration branch `low-poly-overhaul` (not pushed, not deployed). Acceptance criteria confirmed by `cargo test` (228 tests), a release Trunk build and the Playwright suite (75 tests, with and without WebGL). The look was checked in screenshots; sounds were not checked by ear.
