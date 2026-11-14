// The Debrief scene (low-poly ticket 08; Scene Spec `shot.kind` 'debriefHero' | 'debriefFailure' and `debrief`, see
// src/scene/debrief.rs). The Design's spacecraft hangs near Earth, which the title-screen globe already draws:
//
//   hero      Success: the craft holds steady in the sunlight, turning very slowly.
//   tumbling  attitude control failed: the craft spins end over end inside a red tumble ring.
//   dark      power or the battery failed: the craft's lights and parts go dark, a dead-grey halo marks it.
//   reentry   Lifetime too short: the craft falls into the atmosphere, a fireball streaming behind it.
//   adrift    any other failure: the craft drifts, dim, its running lights low.
//
// These are idle animations of a finished Run, so they run on the render clock (`frame(dt, t)`), unlike the Run
// scene. With reduced motion each holds a still pose that still reads as the failure (the tumble ring and a
// skewed craft, a darkened craft, the fireball at the limb, a dim drifting craft). Rust decides which visual:
// this file only draws `spec.debrief.fate`.
//
// For browser tests the canvas carries `data-debrief` (the fate drawn, empty off the Debrief) and
// `data-debrief-spin` (the craft's rotation this frame).
import * as THREE from '../../vendor/three/three.module.js';
import { registerShot } from './camera.js';
import { createSpacecraft } from './spacecraft.js';

const Y = new THREE.Vector3(0, 1, 0);

/**
 * Per fate: where the craft hangs and where the camera stands (world space, Earth radius 1 at the origin),
 * how big the craft is drawn (it is not to scale) and how steadily the camera looks at it.
 */
const POSES = {
  hero: { craft: [1.2, 0.35, 1.65], camera: [3.7, 1.5, 5.6], scale: 1.3 },
  tumbling: { craft: [1.15, 0.45, 1.7], camera: [3.7, 1.6, 5.6], scale: 1.3 },
  dark: { craft: [1.2, 0.3, 1.7], camera: [3.7, 1.4, 5.6], scale: 1.3 },
  reentry: { craft: [0.35, 0.45, 1.2], camera: [2.9, 1.5, 4.4], scale: 1.1 },
  adrift: { craft: [1.15, 0.3, 1.7], camera: [3.7, 1.4, 5.6], scale: 1.3 },
};


for (const kind of ['debriefHero', 'debriefFailure']) {
  registerShot(kind, (shot, seconds) => {
    const fate = kind === 'debriefHero' ? 'hero' : shot.fate ?? 'adrift';
    const p = POSES[fate] ?? POSES.adrift;
    const sway = Math.sin(seconds * 0.12) * 0.12;
    // On a narrow screen the panel sits below the picture: aim the camera below the subject so it fills the upper
    // part of the screen (the side-panel framing shift pushed it out of the frame on a phone).
    const narrow = window.innerWidth <= 900;
    return {
      position: new THREE.Vector3(p.camera[0] + sway, p.camera[1], p.camera[2]),
      target: new THREE.Vector3(p.craft[0] * 0.4, p.craft[1] * 0.4 - (narrow ? 1.1 : 0), p.craft[2] * 0.4),
      framing: !narrow,
    };
  });
}

function basic(color, opts = {}) {
  return new THREE.MeshBasicMaterial({ color, ...opts });
}

/** A red ring tilted round the craft: the path of its tumble, readable even when still. */
function buildTumbleRing() {
  const ring = new THREE.Mesh(new THREE.TorusGeometry(0.5, 0.012, 3, 24, Math.PI * 1.55), basic(0xff4b4b, { transparent: true, opacity: 0.85 }));
  ring.rotation.set(1.1, 0.3, 0.5);
  return ring;
}

/** A dead-grey halo for the dark craft, so it can be found against space. */
function buildDeadHalo() {
  const halo = new THREE.Mesh(new THREE.RingGeometry(0.55, 0.6, 12), basic(0x6b7686, { transparent: true, opacity: 0.55, side: THREE.DoubleSide, depthWrite: false }));
  return halo;
}

/** The fireball streaming behind a re-entering craft: nested additive cones pointing back along its flight. */
function buildFireball() {
  const group = new THREE.Group();
  const layers = [
    [0.55, 2.0, 0xff4a1a, 0.5],
    [0.38, 1.6, 0xff8a2a, 0.6],
    [0.2, 1.2, 0xffe08a, 0.8],
  ];
  for (const [radius, length, color, opacity] of layers) {
    const cone = new THREE.Mesh(
      new THREE.ConeGeometry(radius, length, 7, 1, true),
      basic(color, { transparent: true, opacity, depthWrite: false, blending: THREE.AdditiveBlending, side: THREE.DoubleSide }),
    );
    // The cone's tip (+y) towards the craft; its wide end streams away behind.
    cone.rotation.x = Math.PI;
    cone.position.y = length / 2;
    group.add(cone);
  }
  return group;
}

export function createDebrief({ scene, camera, earth, sunLight, canvas }) {
  const root = new THREE.Group();
  root.name = 'debrief';
  root.visible = false;
  scene.add(root);

  const craft = createSpacecraft();
  const holder = new THREE.Group();
  holder.add(craft.group);
  root.add(holder);

  const tumbleRing = buildTumbleRing();
  const deadHalo = buildDeadHalo();
  const fireball = buildFireball();
  for (const extra of [tumbleRing, deadHalo, fireball]) {
    extra.visible = false;
    root.add(extra);
  }

  let active = false;
  let reduced = false;
  let fate = 'hero';
  let partsKey = '';
  let light = 1;
  // The render-clock time the current verdict arrived at (set on its first frame), for one-off fades.
  let arrivedAt = null;
  const sunHome = sunLight.position.clone();
  const tmp = new THREE.Vector3();

  function apply(spec, reducedMotion) {
    reduced = reducedMotion;
    const next = spec?.debrief && (spec.shot?.kind === 'debriefHero' || spec.shot?.kind === 'debriefFailure') ? spec.debrief : null;
    const wasActive = active;
    active = Boolean(next);
    root.visible = active;
    canvas.dataset.debrief = next?.fate ?? '';
    if (!active) {
      sunLight.position.copy(sunHome);
      return;
    }
    if (!wasActive || next.fate !== fate) {
      fate = next.fate;
      light = fate === 'dark' ? 1 : fate === 'adrift' ? 0.55 : 1;
      arrivedAt = null;
    }
    const parts = spec.spacecraft?.parts;
    const key = parts ? JSON.stringify(parts) : '';
    if (parts && key !== partsKey) {
      partsKey = key;
      craft.update(parts, true);
    }
    tumbleRing.visible = fate === 'tumbling';
    deadHalo.visible = fate === 'dark';
    fireball.visible = fate === 'reentry';
    // The Run's light stays: a low-angle Sun that lights the craft from the side.
    sunLight.position.set(5, 2, 4);
  }

  /** Per frame: the idle animation of the verdict. `t` is the render clock (0 with reduced motion). */
  function frame(dt, t) {
    if (!active) return;
    if (arrivedAt === null) arrivedAt = t;
    const pose = POSES[fate] ?? POSES.adrift;
    const home = tmp.set(...pose.craft);
    holder.position.copy(home);
    holder.scale.setScalar(pose.scale);
    const rm = reduced;
    let wantLight = 1;
    let dim = 0;
    let dark = [];

    if (fate === 'hero') {
      holder.rotation.set(0.25, rm ? 0.6 : 0.6 + t * 0.12, 0.1);
    } else if (fate === 'tumbling') {
      // End over end about a skewed axis; the still pose is a craft caught skew with its ring.
      if (rm) holder.rotation.set(1.0, 0.5, 1.5);
      else holder.rotation.set(t * 1.3 + 1.0, t * 0.9 + 0.5, t * 1.7 + 1.5);
      tumbleRing.position.copy(home);
      tumbleRing.scale.setScalar(pose.scale * 0.9);
      tumbleRing.rotation.z = rm ? 0.5 : 0.5 + t * 0.8;
      craft.setCondition({ warn: rm ? 0.8 : 0.5 + 0.5 * Math.sin(t * 6), warnParts: craft.ids() });
    } else if (fate === 'dark') {
      holder.rotation.set(0.3, 0.8 + (rm ? 0 : t * 0.04), 0.15);
      // The lights die over three seconds (at once with reduced motion) and the craft flickers once, then stays dark.
      const age = rm ? 99 : Math.max(0, t - arrivedAt);
      wantLight = age > 3 ? 0.1 : 1 - 0.9 * (age / 3);
      dim = age > 2 ? 1 : age / 2;
      dark = craft.ids();
      deadHalo.position.copy(home);
      deadHalo.quaternion.copy(camera.quaternion);
      deadHalo.scale.setScalar(pose.scale * 0.8);
    } else if (fate === 'reentry') {
      // The fall repeats every ~11 s: from high above the limb into the thick air, burning more as it goes.
      const cycle = rm ? 0.7 : (t % 11) / 11;
      const radius = 1.42 - 0.3 * cycle;
      const dir = tmp.set(0.3, 0.38, 1).normalize();
      const pos = dir.clone().multiplyScalar(radius);
      holder.position.copy(pos);
      // The craft tumbles gently as it burns, nose to the flow.
      holder.rotation.set(0.6, rm ? 1.0 : 1.0 + t * 0.4, 0.8);
      // The flow runs along the limb; the fireball streams the other way, up and away from Earth.
      const away = pos.clone().add(new THREE.Vector3(0.45, 0.25, -0.35)).sub(pos).normalize();
      fireball.position.copy(pos);
      fireball.quaternion.setFromUnitVectors(Y, away);
      const burn = 0.5 + 0.5 * cycle;
      const s = 0.5 * burn;
      fireball.scale.set(s, s * (rm ? 1 : 0.9 + 0.1 * Math.sin(t * 18)), s);
      wantLight = 1;
      dim = 0;
      craft.setCondition({ flash: 0.35 + 0.45 * cycle, warn: 0.7, warnParts: craft.ids() });
    } else {
      // adrift: a slow, dim tumble that goes nowhere.
      holder.rotation.set(0.35, 0.5 + (rm ? 0 : t * 0.08), -0.2 + (rm ? 0 : Math.sin(t * 0.3) * 0.1));
      wantLight = 0.55;
      dim = 0.7;
    }

    // Test hook: the craft's pose this frame, to tell an idle animation from a held still pose.
    canvas.dataset.debriefSpin = holder.rotation.toArray().slice(0, 3).map((v) => v.toFixed(2)).join(' ');
    if (fate !== 'tumbling' && fate !== 'reentry') craft.setCondition({ warn: 0, flash: 0, warnParts: [] });
    light += (wantLight - light) * (rm ? 1 : 1 - Math.exp(-dt * 6));
    craft.setCondition({ light, dim, dark });
    craft.tick(dt);
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
    craft,
    dispose() {
      sunLight.position.copy(sunHome);
      scene.remove(root);
      holder.remove(craft.group);
      craft.dispose();
      root.traverse((o) => {
        o.geometry?.dispose();
        o.material?.dispose?.();
      });
    },
  };
}
