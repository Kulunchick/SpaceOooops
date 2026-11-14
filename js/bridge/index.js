// The JS bridge: the narrow API Rust calls (see src/ui/scene_bridge.rs). It decides only *how* to draw what
// the Scene Spec says; it never decides *what* to show. Loaded with a dynamic import, no bundler.
//
//   init(canvas, quality) -> Promise<boolean>   true when WebGL is available and the renderer is running
//   apply(sceneSpec)                            the latest Scene Spec (parsed JSON, camelCase)
//   setQuality('low' | 'high'), setReducedMotion(bool)
//   play(cue), setMusic(track), setVolumes(music, sfx)   audio (ticket 04); no-ops until then
import * as THREE from '../../vendor/three/three.module.js';
import { palette } from './palette.js';
import { createEarth, EARTH_RADIUS } from './earth.js';
import { createCameraRig } from './camera.js';
import { createTitleCraft } from './spacecraft.js';

let renderer = null;
let scene = null;
let camera = null;
let rig = null;
let world = null;
let reducedMotion = false;
let quality = 'high';
let spec = null;

/** Whether this browser can create a WebGL context at all. */
export function hasWebGL() {
  try {
    const probe = document.createElement('canvas');
    return Boolean(window.WebGLRenderingContext && (probe.getContext('webgl2') || probe.getContext('webgl')));
  } catch {
    return false;
  }
}

function buildWorld() {
  const { earth, clouds, stars } = createEarth(quality);
  const craft = createTitleCraft();
  craft.scale.setScalar(1.6);
  const orbit = new THREE.Group();
  orbit.rotation.set(0.5, 0, 0.35);
  orbit.add(craft);
  scene.add(earth, stars, orbit);
  return { earth, clouds, stars, craft, orbit };
}

function disposeWorld() {
  if (!world) return;
  for (const obj of [world.earth, world.stars, world.orbit]) {
    scene.remove(obj);
    obj.traverse((o) => {
      o.geometry?.dispose();
      o.material?.dispose();
    });
  }
  world = null;
}

function resize() {
  const w = window.innerWidth;
  const h = window.innerHeight;
  renderer.setPixelRatio(Math.min(window.devicePixelRatio || 1, quality === 'low' ? 1 : 2));
  renderer.setSize(w, h, false);
  camera.aspect = w / h;
  camera.updateProjectionMatrix();
  rig.setFraming(w > 900 ? 1.2 : 0, w > 900 ? 0 : -0.9);
}

const ORBIT_RADIUS = EARTH_RADIUS * 1.45;

function frame(dt, seconds) {
  const t = reducedMotion ? 0 : seconds;
  // The title craft circles Earth once every ~24 s; clouds creep slowly.
  const angle = t * ((Math.PI * 2) / 24) + 1.2;
  world.craft.position.set(Math.cos(angle) * ORBIT_RADIUS, 0, -Math.sin(angle) * ORBIT_RADIUS);
  world.craft.rotation.y = angle;
  world.clouds.rotation.y = t * 0.01;
  rig.update(dt, seconds, reducedMotion);
  renderer.render(scene, camera);
}

export async function init(canvas, initialQuality = 'high') {
  if (renderer) return true;
  if (!hasWebGL()) return false;
  quality = initialQuality === 'low' ? 'low' : 'high';
  try {
    renderer = new THREE.WebGLRenderer({ canvas, antialias: quality === 'high', powerPreference: 'low-power' });
  } catch (e) {
    console.warn('WebGL renderer could not start:', e);
    renderer = null;
    return false;
  }
  reducedMotion = window.matchMedia?.('(prefers-reduced-motion: reduce)').matches ?? false;
  scene = new THREE.Scene();
  scene.background = new THREE.Color(palette.space);
  camera = new THREE.PerspectiveCamera(40, 1, 0.05, 200);
  rig = createCameraRig(camera);

  const sun = new THREE.DirectionalLight(palette.sun, 2.6);
  sun.position.set(5, 2, 4);
  scene.add(sun, new THREE.HemisphereLight(0x9fc4ff, 0x10131c, 0.7));

  world = buildWorld();
  if (spec) rig.setShot(spec.shot);
  resize();
  window.addEventListener('resize', resize);

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
}

export function setQuality(next) {
  const q = next === 'low' ? 'low' : 'high';
  if (q === quality || !renderer) {
    quality = q;
    return;
  }
  quality = q;
  disposeWorld();
  world = buildWorld();
  resize();
}

export function setReducedMotion(on) {
  reducedMotion = Boolean(on);
  rig?.cut();
}

// Audio: ticket 04 implements these in audio.js and re-exports them here.
export function play(_cue) {}
export function setMusic(_track) {}
export function setVolumes(_music, _sfx) {}
