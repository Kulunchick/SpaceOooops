// Camera rig: turns the Scene Spec's Shot into a camera pose and eases between poses.
// Each Shot kind maps to a function (shot, seconds) -> { position, target }. Later tickets add their Shots
// with `registerShot` from their own module, so this file need not change for every new Shot.
import * as THREE from '../../vendor/three/three.module.js';
import { latLonToVector } from './earth.js';

const shots = new Map();

/** Adds or replaces the pose function for one Shot kind (the `kind` field of the Scene Spec's shot). */
export function registerShot(kind, pose) {
  shots.set(kind, pose);
}

// Title: the camera looks at Earth from above southern Africa and drifts slowly side to side, so Angola stays
// in view while the globe appears to turn.
registerShot('titleOrbit', (_shot, seconds) => {
  const drift = Math.sin(seconds * 0.05) * 18;
  const bob = Math.sin(seconds * 0.031) * 6;
  return {
    position: latLonToVector(-4 + bob, 14 + drift, 4.4),
    target: new THREE.Vector3(0, 0, 0),
  };
});

const FALLBACK_KIND = 'titleOrbit';

export function createCameraRig(camera) {
  let shot = { kind: FALLBACK_KIND };
  let snap = true;
  let flying = false;
  const framing = { x: 0, y: 0 };
  // The unshifted camera position; the framing shift is applied on top each frame.
  const position = new THREE.Vector3();
  const target = new THREE.Vector3();
  const desiredTarget = new THREE.Vector3();

  function pose(seconds) {
    const fn = shots.get(shot.kind) ?? shots.get(FALLBACK_KIND);
    return fn(shot, seconds);
  }

  return {
    setShot(next) {
      if (!next) return;
      shot = next;
    },
    /** A click during a flight skips it: the camera cuts to the Shot. Clicks while the camera is at rest do nothing. */
    skip() {
      if (flying) snap = true;
    },
    /** Shifts the picture so the subject sits beside (wide screens) or above (narrow screens) the DOM panels. */
    setFraming(x, y) {
      framing.x = x;
      framing.y = y;
    },
    /** True while the camera is still easing toward its Shot. */
    get flying() {
      return flying;
    },
    /** Cuts straight to the current Shot on the next update instead of easing (reduced motion, first frame). */
    cut() {
      snap = true;
    },
    /** Moves the camera toward the current Shot. With reduced motion the idle drift stops and moves are cuts. */
    update(dt, seconds, reducedMotion) {
      const p = pose(reducedMotion ? 0 : seconds);
      desiredTarget.copy(p.target);
      if (snap || reducedMotion) {
        position.copy(p.position);
        target.copy(desiredTarget);
        snap = false;
      } else {
        const k = 1 - Math.exp(-dt * 2.5);
        position.lerp(p.position, k);
        target.lerp(desiredTarget, k);
      }
      flying = position.distanceTo(p.position) + target.distanceTo(desiredTarget) > 0.02;
      camera.position.copy(position);
      camera.lookAt(target);
      // The shift scales with the distance, so the subject sits at the same place on screen for every Shot.
      const scale = camera.position.distanceTo(target) / 4.4;
      camera.translateX(-framing.x * scale);
      camera.translateY(-framing.y * scale);
    },
  };
}
