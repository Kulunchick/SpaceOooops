// The Run scene: the Design's spacecraft flying round the 3D Earth on the chosen Orbit, with Earth's shadow
// (Scene Spec `shot.kind === 'orbit'` and `orbit`, see src/scene/orbit.rs).
//
// The Scene Spec's playback position is the only clock: the spacecraft's angle and Earth's turn are drawn from
// the latest spec, eased over the short gap between two specs, never advanced on a timer of its own. Paused
// playback means a still spacecraft. Not to scale: the orbit radius is the spec's display radius (log-ish
// altitude, Earth radius 1), so LEO and GEO both fit in the shot.
//
// Frames (reachable for the Effects of ticket 05 and the Debrief shots of ticket 08 via the returned object):
//   sunDirection   unit vector towards the Sun in the Run (world space, horizontal: an equinox Sun)
//   sunFrame       group whose +x points at the Sun and +y is Earth's axis; Earth's shadow lies along -x
//   plane          the orbit plane inside sunFrame, tilted by the inclination about the Sun line
//   carrier        the orbiting group (moves along the orbit, -y towards Earth); holds `craft.group`
//   earthFixed     the Earth group, turned by the spec's `earthTurn` during the Run (place ground things here
//                  with latLonToVector from earth.js; it is rebuilt when the quality changes)
//   sun            the Sun mesh
//
// Angle convention (same as the SVG view): angle 0 is the middle of Earth's shadow, straight behind Earth, and
// grows in the direction of flight. A point at angle a sits at radius * (-cos a, 0, sin a) in `plane`.
//
// For browser tests the canvas carries `data-orbit` (Orbit id, empty off the Run), `data-orbit-angle` (the angle
// drawn this frame), `data-orbit-target` (the latest spec's angle) and `data-in-shadow`.
import * as THREE from '../../vendor/three/three.module.js';
import { palette } from './palette.js';
import { registerShot } from './camera.js';
import { createSpacecraft } from './spacecraft.js';

const TAU = Math.PI * 2;
const Y = new THREE.Vector3(0, 1, 0);
const UP = new THREE.Vector3(0, 1, 0);
export const RUN_SUN = new THREE.Vector3(5, 0, 4).normalize();
/** Turns the Sun frame's +x onto RUN_SUN. */
const FRAME_YAW = Math.atan2(-RUN_SUN.z, RUN_SUN.x);
/** Where the camera looks from, in the Sun frame: a little to the sunward side, above the equator. */
const VIEW = new THREE.Vector3(0.22, 0.5, 1).normalize().applyAxisAngle(Y, FRAME_YAW);
const TAN_HALF_FOV = Math.tan((40 / 2) * (Math.PI / 180));
/** Minimum half-angle of the drawn eclipse arc, so GEO's tiny shadow still shows (as in the SVG view). */
const MIN_ARC_HALF = 0.1;
const SHADOW_COLOUR = 0x9d8cff;

// Shared by the Shot pose and the orbit scene (one per renderer): how far back the camera stands.
const framing = { distance: 6, target: new THREE.Vector3() };

registerShot('orbit', (shot) => {
  // Before the first frame has measured the clear stage, fit the orbit with a safe margin.
  const distance = framing.distance || Math.max(4.6, (shot.radius ?? 1.4) * 4.2);
  return { position: VIEW.clone().multiplyScalar(distance).add(framing.target), target: framing.target.clone(), framing: false };
});

function wrap(a) {
  return ((a % TAU) + TAU) % TAU;
}

/** Shortest signed turn from a to b, in (-PI, PI]. */
function turn(a, b) {
  let d = wrap(b - a);
  if (d > Math.PI) d -= TAU;
  return d;
}

function basic(color, opts = {}) {
  return new THREE.MeshBasicMaterial({ color, ...opts });
}

function buildSun() {
  const group = new THREE.Group();
  const core = new THREE.Mesh(new THREE.IcosahedronGeometry(1, 1), basic(0xffd75e));
  const glow = new THREE.Mesh(
    new THREE.IcosahedronGeometry(1.35, 1),
    basic(0xffd890, { transparent: true, opacity: 0.6, depthWrite: false, blending: THREE.AdditiveBlending }),
  );
  group.add(core, glow);
  return group;
}

/**
 * Earth's shadow behind Earth (along -x of the Sun frame), fading out with distance. Not to scale, like the
 * orbit: it runs from Earth's rim to exactly the width of the orbit's eclipse arc where it crosses the orbit,
 * so the spacecraft is inside the drawn shadow exactly when the Scene Spec says it is in eclipse.
 */
function buildShadow(radius, half) {
  const group = new THREE.Group();
  const shade = (opacity) => basic(0x000000, { transparent: true, opacity, depthWrite: false, side: THREE.DoubleSide });
  const segment = (near, far, rNear, rFar, opacity) => {
    // A cylinder's +y end is turned to -x: that is the far end.
    const mesh = new THREE.Mesh(new THREE.CylinderGeometry(rFar, rNear, far - near, 32, 1, true), shade(opacity));
    mesh.rotation.z = Math.PI / 2;
    mesh.position.x = -(near + far) / 2;
    group.add(mesh);
  };
  const reach = Math.max(0.05, radius * Math.cos(half));
  const width = Math.max(0.2, radius * Math.sin(half));
  segment(0, reach, 1.002, width, 0.45);
  const tail = 6;
  const steps = 4;
  for (let i = 0; i < steps; i++) {
    segment(reach + (i * tail) / steps, reach + ((i + 1) * tail) / steps, width, width, 0.45 * (1 - (i + 0.5) / steps));
  }
  return group;
}

export function createOrbitScene({ scene, camera, earth, sunLight, ambient, canvas }) {
  const root = new THREE.Group();
  root.name = 'run-orbit';
  root.visible = false;
  scene.add(root);

  const sunFrame = new THREE.Group();
  sunFrame.rotation.y = FRAME_YAW;
  root.add(sunFrame);

  let shadow = null;

  const sun = buildSun();
  sunFrame.add(sun);

  const plane = new THREE.Group();
  sunFrame.add(plane);

  // Orbit ring (a thin torus in the plane) and the eclipse arc, centred on angle 0.
  const ringMat = basic(palette.accent, { transparent: true, opacity: 0.75 });
  const ring = new THREE.Mesh(new THREE.TorusGeometry(1, 0.006, 3, 160), ringMat);
  ring.rotation.x = -Math.PI / 2;
  plane.add(ring);
  let arc = null;
  let arcKey = '';

  const carrier = new THREE.Group();
  plane.add(carrier);
  const craft = createSpacecraft();
  carrier.add(craft.group);
  // A halo round the spacecraft so it is easy to find from far out; violet in Earth's shadow.
  const haloMat = basic(palette.accent, { transparent: true, opacity: 0.8, side: THREE.DoubleSide, depthWrite: false });
  const halo = new THREE.Mesh(new THREE.RingGeometry(0.92, 1, 24), haloMat);
  root.add(halo);

  // The run craft darkens in eclipse through its condition (`craft.setCondition`), which gives it its own copies
  // of the part materials, so the hangar's model is untouched. effects.js sets the rest of the condition.

  const sunHome = sunLight.position.clone();
  const ambientHome = ambient.intensity;
  let active = false;
  let reduced = false;
  let view = null;
  let partsKey = '';
  let drawn = { angle: 0, earthTurn: 0 };
  let dark = 0;
  let stageEl = null;
  let offsetOn = false;
  let craftScale = 0.3;
  const world = new THREE.Vector3();

  function stage() {
    if (!stageEl || !stageEl.isConnected) stageEl = document.querySelector('[data-scene-frame]');
    return stageEl;
  }

  function setArc(radius, fraction) {
    const half = Math.max(MIN_ARC_HALF, Math.PI * fraction);
    const key = `${radius}:${half}`;
    if (key === arcKey) return;
    arcKey = key;
    if (arc) {
      plane.remove(arc);
      arc.geometry.dispose();
      arc.material.dispose();
    }
    arc = null;
    if (shadow) {
      sunFrame.remove(shadow);
      shadow.traverse((o) => {
        o.geometry?.dispose();
        o.material?.dispose();
      });
    }
    shadow = buildShadow(radius, Math.max(MIN_ARC_HALF, Math.PI * fraction));
    sunFrame.add(shadow);
    if (fraction <= 0) return;
    arc = new THREE.Mesh(
      new THREE.TorusGeometry(radius, 0.02 + 0.008 * radius, 4, Math.max(8, Math.round(half * 40)), 2 * half),
      basic(SHADOW_COLOUR, { transparent: true, opacity: 0.85 }),
    );
    // The torus starts at its +x and runs counter-clockwise; turned so it spans angle -half..half (see header).
    arc.rotation.set(-Math.PI / 2, 0, Math.PI - half);
    plane.add(arc);
  }

  function apply(spec, reducedMotion) {
    reduced = reducedMotion;
    const next = spec?.shot?.kind === 'orbit' ? spec.orbit : null;
    const wasActive = active;
    active = Boolean(next);
    root.visible = active;
    canvas.dataset.orbit = next?.id ?? '';
    if (!active) {
      view = null;
      earth.rotation.y = 0;
      sunLight.position.copy(sunHome);
      ambient.intensity = ambientHome;
      clearOffset();
      return;
    }
    // A new Run (playback went back), another Orbit or a fresh arrival on the Run: cut, do not sweep round.
    const jump = !wasActive || !view || next.id !== view.id || next.playback < view.playback;
    view = next;
    if (jump) drawn = { angle: next.angle, earthTurn: next.earthTurn };
    sunLight.position.copy(RUN_SUN).multiplyScalar(10);
    // A dim sky light, so the night side of Earth reads as night.
    ambient.intensity = 0.1;
    ring.scale.setScalar(next.radius);
    // Tilted about the Sun line, so the point behind Earth (angle 0) stays in the middle of the shadow.
    plane.rotation.set((-next.inclinationDeg * Math.PI) / 180, 0, 0);
    setArc(next.radius, next.shadowFraction);
    const parts = spec.spacecraft?.parts;
    const key = parts ? JSON.stringify(parts) : '';
    if (parts && key !== partsKey) {
      partsKey = key;
      craft.update(parts, true);
    }
    canvas.dataset.orbitTarget = next.angle.toFixed(3);
    canvas.dataset.inShadow = String(next.inShadow);
  }

  function clearOffset() {
    if (offsetOn) camera.clearViewOffset();
    offsetOn = false;
  }

  /** Per frame: ease towards the latest spec, place the spacecraft, frame the orbit inside the clear stage. */
  function frame(dt) {
    if (!active || !view) {
      if (offsetOn) clearOffset();
      return;
    }
    const k = reduced ? 1 : 1 - Math.exp(-dt * 25);
    drawn.angle = wrap(drawn.angle + turn(drawn.angle, view.angle) * k);
    drawn.earthTurn = wrap(drawn.earthTurn + turn(drawn.earthTurn, view.earthTurn) * k);
    earth.rotation.y = drawn.earthTurn;
    canvas.dataset.orbitAngle = drawn.angle.toFixed(3);

    const r = view.radius;
    carrier.position.set(-Math.cos(drawn.angle) * r, 0, Math.sin(drawn.angle) * r);
    // Bottom face (antenna) towards Earth, long axis away from it.
    carrier.quaternion.setFromUnitVectors(UP, carrier.position.clone().normalize());

    // Darken in Earth's shadow (the spec decides when; the change itself is eased).
    const wantDark = view.inShadow ? 1 : 0;
    dark += (wantDark - dark) * (reduced ? 1 : 1 - Math.exp(-dt * 10));
    craft.setCondition({ light: 1 - 0.82 * dark });
    haloMat.color.setHex(palette.accent).lerp(new THREE.Color(SHADOW_COLOUR), dark);
    canvas.dataset.inShadow = String(view.inShadow);

    // Framing: fit the orbit (and the Sun beside it) inside the DOM element marked `data-scene-frame`.
    const w = window.innerWidth;
    const h = window.innerHeight;
    const rect = stage()?.getBoundingClientRect();
    const sw = rect && rect.width > 40 ? rect.width : w;
    const sh = rect && rect.height > 40 ? rect.height : h;
    const halfHeight = r * 1.3;
    const halfWidth = r * 1.1 + 1.05;
    const fit = Math.max(halfHeight, halfWidth / (sw / sh));
    framing.distance = Math.max(3.2, fit / (TAN_HALF_FOV * (sh / h)));
    // The Sun sits just outside the orbit, on its sunward side, so the shadow behind Earth reads at once.
    const sunSize = 0.1 * framing.distance / 4.6;
    sun.scale.setScalar(sunSize);
    sun.position.set(r + 0.55, 0, 0);
    // Spacecraft size grows with the shot so it stays visible from GEO's distance; not to scale.
    craftScale = 0.4 * (framing.distance / 4.6);
    carrier.scale.setScalar(craftScale);
    carrier.getWorldPosition(world);
    halo.position.copy(world);
    halo.quaternion.copy(camera.quaternion);
    halo.scale.setScalar(0.16 * framing.distance / 4.6);

    // Shift the projection so Earth lands in the middle of the stage.
    const tx = rect ? w / 2 - (rect.left + rect.width / 2) : 0;
    const ty = rect ? h / 2 - (rect.top + rect.height / 2) : 0;
    if (Math.abs(tx) > 0.5 || Math.abs(ty) > 0.5) {
      camera.setViewOffset(w, h, tx, ty, w, h);
      offsetOn = true;
    } else if (offsetOn) {
      clearOffset();
    }
  }

  return {
    apply,
    frame,
    setReducedMotion(on) {
      reduced = on;
    },
    get active() {
      return active;
    },
    sunDirection: RUN_SUN,
    sunFrame,
    plane,
    carrier,
    craft,
    sun,
    /** The ring round the spacecraft; its colour is reset every frame, so effects may tint it after `frame`. */
    halo,
    earthFixed: earth,
    dispose() {
      earth.rotation.y = 0;
      sunLight.position.copy(sunHome);
      ambient.intensity = ambientHome;
      scene.remove(root);
      carrier.remove(craft.group);
      craft.dispose();
      root.traverse((o) => {
        o.geometry?.dispose();
        o.material?.dispose?.();
      });
    },
  };
}
