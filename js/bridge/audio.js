// Audio for the game: WebAudio music with crossfades, one-shot Cues, and the volume/mute settings.
// Standalone: it never touches the renderer, so it works when WebGL is missing. Rust decides *what* to
// play (src/scene/cues.rs); this file decides only *how*.
//
//   play(cue)               a one-shot Cue by name; a name without a sample plays a short synthesised tone
//   setMusic(track)         'menu' | 'run' | null; crossfades from the current track
//   setVolumes(music, sfx, muted = false)   0..1 each; muted makes both gains exactly 0 and starts nothing
//   getState()              what is audible, for browser tests (also mirrored to <html data-audio-*>)
//
// Nothing plays and no AudioContext exists before the first user gesture (pointerdown / keydown / touchend).
// Adding a sound: put the file in assets/audio/ (and data/credits.toml) and add a line to SAMPLES or TRACKS.

const SAMPLES = {
  snap: 'snap.ogg',
  click: 'click.ogg',
  go: 'vote-go.ogg',
  nogo: 'vote-nogo.ogg',
  engine: 'launch-engine.ogg',
  ignition: 'launch-ignition.ogg',
  // 'launch' (the Cue when Launch is pressed) and 'countdown' (one tick per second, played by pad.js) have no
  // sample: they are the synthesised tone.
};
const TRACKS = {
  menu: 'music-menu.mp3',
  run: 'music-run.mp3',
};
const CROSSFADE_SECONDS = 2;
const SFX_BOOST = 1; // effects are short and quiet; the default volumes keep music well below them

let ctx = null;
let unlocked = false;
let musicBus = null;
let sfxBus = null;
let volumes = { music: 0.3, sfx: 0.8, muted: false };
let wantedTrack = null;
let current = null; // { track, gain, source }
let lastCue = '';
let sourcesStarted = 0;
let musicToken = 0;
let fading = 0; // tracks currently fading out (a crossfade in progress)
const buffers = new Map(); // url -> Promise<AudioBuffer | null>

const assetUrl = (file) => new URL(`../../assets/audio/${file}`, import.meta.url).href;

function effective(kind) {
  if (volumes.muted) return 0;
  return kind === 'music' ? volumes.music : volumes.sfx;
}

function publish() {
  const d = document.documentElement.dataset;
  d.audioUnlocked = String(unlocked);
  d.audioTrack = current?.track ?? '';
  d.audioWanted = wantedTrack ?? '';
  d.audioMusicGain = String(effective('music'));
  d.audioSfxGain = String(effective('sfx'));
  d.audioMuted = String(volumes.muted);
  d.audioLastCue = lastCue;
  d.audioSources = String(sourcesStarted);
  d.audioFading = String(fading);
}

export function getState() {
  return {
    unlocked,
    track: current?.track ?? null,
    wantedTrack,
    musicGain: effective('music'),
    sfxGain: effective('sfx'),
    muted: volumes.muted,
    lastCue,
    sourcesStarted,
    fading,
  };
}

function loadBuffer(url) {
  if (!buffers.has(url)) {
    buffers.set(
      url,
      fetch(url)
        .then((r) => (r.ok ? r.arrayBuffer() : Promise.reject(new Error(String(r.status)))))
        .then((data) => ctx.decodeAudioData(data))
        .catch((e) => {
          console.warn('audio file unavailable:', url, e);
          return null;
        }),
    );
  }
  return buffers.get(url);
}

function applyGains() {
  if (!ctx) return;
  const t = ctx.currentTime;
  for (const [bus, kind] of [[musicBus, 'music'], [sfxBus, 'sfx']]) {
    const v = effective(kind);
    bus.gain.cancelScheduledValues(t);
    bus.gain.setValueAtTime(v, t);
  }
}

function unlock() {
  if (unlocked) return;
  const Ctor = window.AudioContext || window.webkitAudioContext;
  if (!Ctor) return;
  try {
    ctx = new Ctor();
  } catch (e) {
    console.warn('AudioContext unavailable:', e);
    return;
  }
  musicBus = ctx.createGain();
  sfxBus = ctx.createGain();
  musicBus.connect(ctx.destination);
  sfxBus.connect(ctx.destination);
  unlocked = true;
  applyGains();
  ctx.resume?.().catch(() => {});
  reconcileMusic();
  publish();
}

for (const type of ['pointerdown', 'keydown', 'touchend']) {
  window.addEventListener(type, unlock, { capture: true, passive: true });
}

// UI clicks: any button press. (Part snaps and other state-driven sounds come from Rust Cues.)
document.addEventListener(
  'click',
  (e) => {
    if (e.target instanceof Element && e.target.closest('button, [role="button"], a[href]')) play('click');
  },
  { capture: true },
);

/** Short synthesised tone for a Cue name that has no sample (or whose sample failed to load). */
function playTone(name) {
  let h = 0;
  for (const ch of name) h = (h * 31 + ch.charCodeAt(0)) >>> 0;
  const osc = ctx.createOscillator();
  const env = ctx.createGain();
  osc.type = 'triangle';
  osc.frequency.value = 330 + (h % 12) * 55;
  const t = ctx.currentTime;
  env.gain.setValueAtTime(0.0001, t);
  env.gain.exponentialRampToValueAtTime(0.5, t + 0.01);
  env.gain.exponentialRampToValueAtTime(0.0001, t + 0.18);
  osc.connect(env).connect(sfxBus);
  osc.start(t);
  osc.stop(t + 0.2);
  sourcesStarted += 1;
}

export function play(cue) {
  if (unlocked) lastCue = String(cue);
  if (unlocked && effective('sfx') > 0) {
    const file = SAMPLES[cue];
    if (!file) {
      playTone(String(cue));
    } else {
      loadBuffer(assetUrl(file)).then((buf) => {
        if (effective('sfx') <= 0) return; // muted while loading
        if (!buf) return playTone(String(cue));
        const src = ctx.createBufferSource();
        const g = ctx.createGain();
        g.gain.value = SFX_BOOST;
        src.buffer = buf;
        src.connect(g).connect(sfxBus);
        src.start();
        sourcesStarted += 1;
        publish();
      });
    }
  }
  publish();
}

function fadeOut(entry) {
  const t = ctx.currentTime;
  entry.gain.gain.cancelScheduledValues(t);
  entry.gain.gain.setValueAtTime(entry.gain.gain.value, t);
  entry.gain.gain.linearRampToValueAtTime(0, t + CROSSFADE_SECONDS);
  entry.source.stop(t + CROSSFADE_SECONDS + 0.05);
  fading += 1;
  publish();
  entry.source.onended = () => {
    entry.gain.disconnect();
    fading -= 1;
    publish();
  };
}

/** Makes what is playing match what is wanted: nothing before the first gesture or while silent. */
function reconcileMusic() {
  if (!unlocked) return;
  const silent = effective('music') <= 0;
  const target = silent ? null : wantedTrack;
  if ((current?.track ?? null) === target) return;
  const token = ++musicToken;
  const old = current;
  current = null;
  if (old) fadeOut(old);
  if (!target || !TRACKS[target]) return publish();
  loadBuffer(assetUrl(TRACKS[target])).then((buf) => {
    if (token !== musicToken || !buf) return; // superseded, or the file is missing
    const gain = ctx.createGain();
    const t = ctx.currentTime;
    gain.gain.setValueAtTime(0, t);
    gain.gain.linearRampToValueAtTime(1, t + (old ? CROSSFADE_SECONDS : 0.5));
    const source = ctx.createBufferSource();
    source.buffer = buf;
    source.loop = true;
    source.connect(gain).connect(musicBus);
    source.start();
    sourcesStarted += 1;
    current = { track: target, gain, source };
    publish();
  });
  publish();
}

export function setMusic(track) {
  wantedTrack = track && TRACKS[track] ? track : null;
  reconcileMusic();
  publish();
}

export function setVolumes(music, sfx, muted = false) {
  const clamp = (v, fallback) => (Number.isFinite(v) ? Math.min(1, Math.max(0, v)) : fallback);
  volumes = { music: clamp(music, volumes.music), sfx: clamp(sfx, volumes.sfx), muted: Boolean(muted) };
  applyGains();
  reconcileMusic();
  publish();
}

publish();
