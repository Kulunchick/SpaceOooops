// The JS bridge: the narrow API Rust calls (see src/ui/scene_bridge.rs). It decides only *how* to draw what
// the Scene Spec says; it never decides *what* to show. Loaded with a dynamic import, no bundler.
//
//   init(canvas, quality) -> Promise<boolean>   true when WebGL is available and the renderer is running
//   apply(sceneSpec)                            the latest Scene Spec (parsed JSON, camelCase)
//   setQuality('low' | 'high'), setReducedMotion(bool)
//   play(cue), setMusic(track), setVolumes(music, sfx, muted)   audio (audio.js, loaded separately by Rust)
import * as THREE from '../../vendor/three/three.module.js';
import { palette } from './palette.js';
import { createEarth, EARTH_RADIUS } from './earth.js';
import { createCameraRig } from './camera.js';
import { createTitleCraft } from './spacecraft.js';
import { createHangar } from './hangar.js';
import { createMarkers } from './markers.js';
import { createOrbitScene } from './orbit.js';
import { createPad } from './pad.js';
import { createEffects } from './effects.js';
import { createDebrief } from './debrief.js';

let renderer = null;
let scene = null;
let camera = null;
let rig = null;
let world = null;
let reducedMotion = false;
let quality = 'high';
let spec = null;
let canvasEl = null;
let markers = null;
let sunLight = null;
let skyLight = null;
// A quality change that arrived while the launch sequence was playing waits for its end (rebuilding the world would restart it).
let pendingQuality = null;
const LAUNCH_PHASES = ['countdown', 'ignition', 'liftoff', 'ascent'];

/** Whether this browser can create a WebGL context at all. */
export function hasWebGL() {
  try {
    const probe = document.createElement('canvas');
    return Boolean(window.WebGLRenderingContext && (probe.getContext('webgl2') || probe.getContext('webgl')));
  } catch {
    return false;
  }
}

// The Sun's resting place: it lights Africa from the upper right. On the globe and region Shots (Scenario select,
// Brief) it moves with the camera instead, keeping this same angle to it, so every Target Country is lit as Angola is.
const SUN_HOME = new THREE.Vector3(5, 2, 4);
const SUN_LEAD = Math.atan2(SUN_HOME.x, SUN_HOME.z) - (17.5 * Math.PI) / 180;
let sunFollows = false;

function followSun() {
  const lon = Math.atan2(camera.position.x, camera.position.z) + SUN_LEAD;
  const r = Math.hypot(SUN_HOME.x, SUN_HOME.z);
  sunLight.position.set(Math.sin(lon) * r, SUN_HOME.y, Math.cos(lon) * r);
}

function buildWorld() {
  // The Run and Debrief scenes keep the Sun's position at creation to restore it later: give them the resting one.
  sunLight.position.copy(SUN_HOME);
  const { earth, clouds, stars, highlight } = createEarth(quality);
  highlight(spec?.highlightedCountry ?? 'angola');
  const craft = createTitleCraft();
  craft.scale.setScalar(1.6);
  const orbit = new THREE.Group();
  orbit.rotation.set(0.5, 0, 0.35);
  orbit.add(craft);
  scene.add(earth, stars, orbit);
  markers = createMarkers(scene);
  if (spec) markers.set(spec.markers);
  const hangar = createHangar({ scene, camera, canvas: canvasEl });
  if (spec) hangar.apply(spec, reducedMotion);
  const run = createOrbitScene({ scene, camera, earth, sunLight, ambient: skyLight, canvas: canvasEl });
  if (spec) run.apply(spec, reducedMotion);
  const effects = createEffects({ scene, run, canvas: canvasEl, camera });
  if (spec) effects.apply(spec);
  const pad = createPad({ scene, camera, canvas: canvasEl });
  if (spec) pad.apply(spec, reducedMotion);
  const debrief = createDebrief({ scene, camera, earth, sunLight, canvas: canvasEl });
  if (spec) debrief.apply(spec, reducedMotion);
  return { earth, clouds, highlight, stars, craft, orbit, hangar, run, effects, pad, debrief };
}

function disposeWorld() {
  if (!world) return;
  world.hangar.dispose();
  world.effects.dispose();
  world.run.dispose();
  world.pad.dispose();
  world.debrief.dispose();
  for (const obj of [world.earth, world.stars, world.orbit]) {
    scene.remove(obj);
    obj.traverse((o) => {
      o.geometry?.dispose();
      o.material?.dispose();
    });
  }
  markers?.dispose();
  markers = null;
  world = null;
}

function resize() {
  const w = window.innerWidth;
  const h = window.innerHeight;
  // Low quality draws at a pixel ratio of 1; High at up to 2 (1.5 on a phone, where the screen is small and dense).
  const cap = quality === 'low' ? 1 : w <= 600 ? 1.5 : 2;
  renderer.setPixelRatio(Math.min(window.devicePixelRatio || 1, cap));
  renderer.setSize(w, h, false);
  camera.aspect = w / h;
  camera.updateProjectionMatrix();
  rig.setFraming(w > 900 ? 1.2 : 0, w > 900 ? 0 : -0.9);
}

const ORBIT_RADIUS = EARTH_RADIUS * 1.45;

function launching() {
  return LAUNCH_PHASES.includes(canvasEl?.dataset.padPhase);
}

function frame(dt, seconds) {
  if (pendingQuality && !launching()) {
    const next = pendingQuality;
    pendingQuality = null;
    setQuality(next);
  }
  const t = reducedMotion ? 0 : seconds;
  // The title craft circles Earth once every ~24 s; clouds creep slowly.
  const angle = t * ((Math.PI * 2) / 24) + 1.2;
  world.craft.position.set(Math.cos(angle) * ORBIT_RADIUS, 0, -Math.sin(angle) * ORBIT_RADIUS);
  world.craft.rotation.y = angle;
  world.clouds.rotation.y = t * 0.01;
  // The title craft gives way to the Design's own spacecraft during the Run.
  world.orbit.visible = !world.run.active && !world.debrief.active;
  rig.update(dt, seconds, reducedMotion);
  if (sunFollows) followSun();
  markers?.update(seconds, reducedMotion);
  renderer.domElement.dataset.flying = String(rig.flying);
  world.hangar.frame(dt, t);
  world.run.frame(dt);
  world.effects.frame();
  world.pad.frame(dt, t);
  world.debrief.frame(dt, t);
  renderer.render(scene, camera);
}

export async function init(canvas, initialQuality = 'high') {
  if (renderer) return true;
  if (!hasWebGL()) return false;
  quality = initialQuality === 'low' ? 'low' : 'high';
  canvasEl = canvas;
  canvas.dataset.quality = quality;
  try {
    // Antialiasing is a context attribute: it is fixed by the level the page starts with (a canvas cannot get a
    // second context), so switching Low/High in game changes face count and pixel ratio but not antialiasing.
    renderer = new THREE.WebGLRenderer({ canvas, antialias: quality === 'high', powerPreference: 'low-power' });
  } catch (e) {
    console.warn('WebGL renderer could not start:', e);
    renderer = null;
    return false;
  }
  // Motion is the player's own setting (Rust puts it on <html data-motion>); the OS preference is not followed.
  reducedMotion = document.documentElement.dataset.motion === 'reduced';
  canvas.dataset.reducedMotion = String(reducedMotion);
  scene = new THREE.Scene();
  scene.background = new THREE.Color(palette.space);
  camera = new THREE.PerspectiveCamera(40, 1, 0.05, 200);
  rig = createCameraRig(camera);

  sunLight = new THREE.DirectionalLight(palette.sun, 2.6);
  sunLight.position.set(5, 2, 4);
  skyLight = new THREE.HemisphereLight(0x9fc4ff, 0x10131c, 0.7);
  scene.add(sunLight, skyLight);

  world = buildWorld();
  if (spec) rig.setShot(spec.shot);
  resize();
  window.addEventListener('resize', resize);
  // A click anywhere during a camera flight skips it.
  window.addEventListener('pointerdown', () => rig.skip(), true);

  const clock = new THREE.Timer();
  renderer.setAnimationLoop((timestamp) => {
    clock.update(timestamp);
    frame(Math.min(clock.getDelta(), 0.1), clock.getElapsed());
  });
  return true;
}

export function apply(sceneSpec) {
  spec = sceneSpec;
  if (rig) rig.setShot(sceneSpec.shot);
  const follows = ['globe', 'region'].includes(sceneSpec.shot?.kind);
  if (sunFollows && !follows && sunLight) sunLight.position.copy(SUN_HOME);
  sunFollows = follows;
  world?.highlight(sceneSpec.highlightedCountry ?? 'angola');
  // For browser tests: the highlighted Target Country and where the Shot looks.
  if (canvasEl) {
    canvasEl.dataset.highlight = sceneSpec.highlightedCountry ?? 'angola';
    canvasEl.dataset.shotAt = sceneSpec.shot?.lat == null ? '' : `${sceneSpec.shot.lat},${sceneSpec.shot.lon}`;
  }
  world?.hangar.apply(sceneSpec, reducedMotion);
  world?.run.apply(sceneSpec, reducedMotion);
  world?.effects.apply(sceneSpec);
  world?.pad.apply(sceneSpec, reducedMotion);
  world?.debrief.apply(sceneSpec, reducedMotion);
  markers?.set(sceneSpec.markers);
}

export function setQuality(next) {
  const q = next === 'low' ? 'low' : 'high';
  if (q === quality || !renderer) {
    quality = q;
    pendingQuality = null;
    return;
  }
  if (launching()) {
    pendingQuality = q;
    return;
  }
  pendingQuality = null;
  quality = q;
  canvasEl.dataset.quality = q;
  disposeWorld();
  world = buildWorld();
  resize();
}

export function setReducedMotion(on) {
  reducedMotion = Boolean(on);
  if (canvasEl) canvasEl.dataset.reducedMotion = String(reducedMotion);
  rig?.cut();
  world?.hangar.setReducedMotion(reducedMotion);
  world?.run.setReducedMotion(reducedMotion);
  world?.pad.setReducedMotion(reducedMotion);
  world?.debrief.setReducedMotion(reducedMotion);
}

// Audio lives in audio.js and is loaded by Rust directly (it must work without WebGL); re-exported for the contract.
export { play, setMusic, setVolumes, getState as getAudioState } from './audio.js';
