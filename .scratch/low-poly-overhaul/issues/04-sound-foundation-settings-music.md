# 04: Sound: foundation, settings and music

**What to build:** The game has sound.

- **Music:** a calm track plays on the title, Scenario select, Brief and Design screens, and a tenser track plays during the Run. Switching between them crossfades.
- **Sound effects:** UI clicks and part snaps on the Design screen.
- **Silence until the first click:** nothing plays before the player's first click, because browsers block audio until then.
- **Settings:** a small control in a corner of the screen offers music volume, effects volume and mute, and the settings are remembered between visits. Mute is fully silent.
- **Fallback:** audio also works when WebGL is not available.

Under the hood:
- the JS bridge gains `play(cue)`, `setMusic(track)` and `setVolumes`, built on WebAudio;
- the Scene Spec gains Cues: the music track that should play now, and one-shot sounds triggered by a state change;
- the first tracks and sounds are CC0/CC-BY placeholders (Kenney audio, OpenGameArt or similar), all listed in the credits data file. AI tracks supplied by the team can replace them later.

**Blocked by:** 02: Live spacecraft assembly (for the part-snap cue).

**Status:** ready-for-agent

- [ ] No audio before the first user gesture
- [ ] Music Cue is the menu track outside the Run and the Run track during it (`cargo test`)
- [ ] Adding or removing a part emits a snap Cue (`cargo test`)
- [ ] Crossfade between tracks
- [ ] Music and effects volumes, plus mute, persist across a reload (Playwright)
- [ ] Mute is fully silent
- [ ] Audio works in the no-WebGL fallback
- [ ] Every audio file is listed with its licence in the credits data file
