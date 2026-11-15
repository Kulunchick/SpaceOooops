// The new Random Events in the Run scene (More Random Events ticket 08). Rust decides which of the six effect
// kinds is active and for how long (src/scene/effects.rs); this file only draws them, from the playback position
// inside each effect, like effects.js: no timers, so a pause freezes them and a skip leaves nothing behind.
//
//   flyBy             a rock with a tail crosses the view past the spacecraft
//   hit               a streak strikes a part (the Design's solar panel, or the bus) in a flash; the part sparks
//   linkInterference  a jagged static line from the spacecraft towards Earth; the contact beam flickers out
//   panelShading      a dark cloud drifts over the spacecraft and the panels dim
//   bonusGlow         a golden shell breathes round the spacecraft, with sparkles rising
//   visitor           a small saucer holds beside the spacecraft and bobs, with a pale beam towards it
//
// Everything is generic low-poly geometry in the spacecraft's neighbourhood, sized by the halo ring (one halo
// radius = U), so it reads at phone size. `effect.model` names a star model (ticket 09 adds js/bridge/star_models.js);
// an unknown or absent name draws the generic shape. With reduced motion the effect holds one still pose.
//
// `frame(effects, playback)` returns what the spacecraft itself must show, for effects.js to merge into its
// condition: `{ dim, spark, linkBreak }`. The canvas carries `data-event-fx`, `kind:progress` pairs.
import * as THREE from '../../vendor/three/three.module.js';
import { MODELS } from './star_models.js';

const smooth = (x) => {
  const t = Math.min(1, Math.max(0, x));
  return t * t * (3 - 2 * t);
};
const Y = new THREE.Vector3(0, 1, 0);
const clamp01 = (x) => Math.min(1, Math.max(0, x));

function additive(color, opacity) {
  return new THREE.MeshBasicMaterial({
    color, transparent: true, opacity, depthWrite: false, blending: THREE.AdditiveBlending, side: THREE.DoubleSide,
  });
}

function flat(color, opts = {}) {
  return new THREE.MeshStandardMaterial({ color, flatShading: true, roughness: 0.7, ...opts });
}

// ---- builders: each returns a group sized to about one unit, plus `userData.parts` for the animation

function buildFlyBy() {
  const g = new THREE.Group();
  const rock = new THREE.Mesh(new THREE.IcosahedronGeometry(0.45, 0), flat(0x8a8f9c));
  const tail = new THREE.Mesh(new THREE.ConeGeometry(0.3, 2.2, 6, 1, true), additive(0xffc27a, 0.5));
  tail.rotation.z = Math.PI / 2; // points back along -x, the way the rock came from
  tail.position.x = -1.3;
  g.add(rock, tail);
  g.userData = { rock };
  return g;
}

function buildHit() {
  const g = new THREE.Group();
  const streak = new THREE.Mesh(new THREE.ConeGeometry(0.12, 2.4, 5, 1, true), additive(0xfff0c0, 0.9));
  const burst = new THREE.Mesh(new THREE.OctahedronGeometry(0.5, 0), additive(0xff8a3a, 0.9));
  g.add(streak, burst);
  g.userData = { streak, burst };
  return g;
}

function buildLink() {
  const g = new THREE.Group();
  const points = 14;
  const geometry = new THREE.BufferGeometry();
  geometry.setAttribute('position', new THREE.BufferAttribute(new Float32Array(points * 3), 3));
  const line = new THREE.Line(geometry, new THREE.LineBasicMaterial({ color: 0xff7a7a, transparent: true, opacity: 0.9, depthTest: false }));
  line.renderOrder = 10;
  line.frustumCulled = false;
  g.add(line);
  // Chunky static flecks along the line, so it reads at phone size.
  const flecks = [];
  for (let i = 0; i < 7; i++) {
    const fleck = new THREE.Mesh(new THREE.OctahedronGeometry(0.07, 0), additive(0xff9a9a, 0.9));
    g.add(fleck);
    flecks.push(fleck);
  }
  g.userData = { line, points, flecks };
  return g;
}

function buildShade() {
  const g = new THREE.Group();
  const cloud = new THREE.Mesh(
    new THREE.CircleGeometry(1.6, 7),
    new THREE.MeshBasicMaterial({ color: 0x05060c, transparent: true, opacity: 0.6, depthWrite: false, side: THREE.DoubleSide }),
  );
  g.add(cloud);
  g.userData = { cloud };
  return g;
}

function buildGlow() {
  const g = new THREE.Group();
  const shell = new THREE.Mesh(new THREE.IcosahedronGeometry(1, 1), additive(0xffd75e, 0.3));
  g.add(shell);
  const stars = [];
  for (let i = 0; i < 8; i++) {
    const star = new THREE.Mesh(new THREE.OctahedronGeometry(0.09, 0), additive(0xfff2a8, 0.9));
    g.add(star);
    stars.push(star);
  }
  g.userData = { shell, stars };
  return g;
}

function buildVisitor() {
  const g = new THREE.Group();
  const body = new THREE.Mesh(new THREE.SphereGeometry(0.6, 8, 4), flat(0xb9c4d8, { metalness: 0.3 }));
  body.scale.y = 0.28;
  const dome = new THREE.Mesh(new THREE.SphereGeometry(0.28, 7, 4, 0, Math.PI * 2, 0, Math.PI / 2), flat(0x7be0c8, { emissive: 0x1a5a4a }));
  dome.position.y = 0.08;
  const lamp = new THREE.Mesh(new THREE.OctahedronGeometry(0.07, 0), new THREE.MeshBasicMaterial({ color: 0xffe27a }));
  lamp.position.set(0.5, -0.02, 0);
  const beam = new THREE.Mesh(new THREE.ConeGeometry(0.5, 1.4, 7, 1, true), additive(0xbfffe8, 0.18));
  beam.position.y = -0.8;
  g.add(body, dome, lamp, beam);
  g.userData = { body, lamp, beam };
  return g;
}

const BUILDERS = {
  flyBy: buildFlyBy,
  hit: buildHit,
  linkInterference: buildLink,
  panelShading: buildShade,
  bonusGlow: buildGlow,
  visitor: buildVisitor,
};

/** A named star model, when ticket 09 has one; otherwise the generic shape. */
function modelFor(effect, generic, low) {
  const make = effect.model ? MODELS[effect.model] : null;
  if (!make) return generic;
  const model = make(low);
  disposeGroup(generic);
  // A star model keeps its own pose (no tumbling, no spinning); `tick` moves its wings, tail and lights.
  model.userData = { ...model.userData, star: true, beam: { material: { opacity: 0 } } };
  return model;
}

function disposeGroup(g) {
  g.traverse((o) => {
    o.geometry?.dispose();
    o.material?.dispose();
  });
}

export function createEventEffects({ scene, run, canvas = null, camera = null }) {
  const root = new THREE.Group();
  root.name = 'event-effects';
  scene.add(root);

  const live = new Map(); // effect id -> group
  const craftPos = new THREE.Vector3();
  const target = new THREE.Vector3();
  const right = new THREE.Vector3(1, 0, 0);
  const up = new THREE.Vector3(0, 1, 0);
  const toward = new THREE.Vector3();
  const box = new THREE.Box3();

  const reduced = () => canvas?.dataset.reducedMotion === 'true';

  function partCentre(id, out) {
    const part = id ? run.craft.part(id) : null;
    if (!part) return out.copy(craftPos);
    box.setFromObject(part);
    return box.isEmpty() ? out.copy(craftPos) : box.getCenter(out);
  }

  function busId() {
    return run.craft.partsOfKind('bus')[0]?.userData.partId ?? null;
  }

  /** Per frame. Returns the spacecraft's share of the effects: `{ dim, spark, linkBreak }`. */
  function frame(effects, playback) {
    const patch = { dim: 0, spark: null, linkBreak: false };
    const seen = new Set();
    const still = reduced();
    run.carrier.getWorldPosition(craftPos);
    const U = 2.2 * (run.halo.scale.x || 0.16); // generous: the halo is only the size of the craft
    if (camera) {
      right.set(1, 0, 0).applyQuaternion(camera.quaternion);
      up.set(0, 1, 0).applyQuaternion(camera.quaternion);
    }
    const status = [];
    for (const e of effects) {
      const builder = BUILDERS[e.kind];
      if (!builder) continue;
      seen.add(e.id);
      let g = live.get(e.id);
      if (!g) {
        g = modelFor(e, builder(), canvas?.dataset.quality === 'low');
        live.set(e.id, g);
        root.add(g);
      }
      const into = playback - e.start;
      const p = still ? 0.5 : clamp01(into / e.days); // reduced motion: one still pose, mid-way
      const fade = still ? 1 : smooth(into / 0.15) * smooth((e.days - into) / 0.15);
      const t = still ? 0 : playback;
      const parts = g.userData;
      g.scale.setScalar(U);
      status.push(`${e.kind}:${p.toFixed(2)}`);
      parts.tick?.(t);

      if (e.kind === 'flyBy') {
        // Crosses from one side to the other, above the craft, tumbling.
        const x = -5 + 10 * p;
        g.position.copy(craftPos).addScaledVector(right, x * U).addScaledVector(up, 1.6 * U);
        if (parts.star) g.rotation.set(0, 0, still ? 0 : Math.sin(t * 3) * 0.1); // nose along the way it travels
        else parts.rock?.rotation.set(t * 5, t * 7, 0);
        g.scale.setScalar(U * (0.4 + 0.6 * fade));
      } else if (e.kind === 'hit') {
        // The streak comes in over the first 40%, then the burst and the sparks.
        partCentre(e.component ?? busId(), target);
        const inbound = clamp01(p / 0.4);
        const after = clamp01((p - 0.4) / 0.6);
        g.position.copy(target);
        parts.streak.visible = inbound < 1;
        toward.copy(right).multiplyScalar(4).addScaledVector(up, 2.5); // where the streak comes from
        parts.streak.position.copy(toward).multiplyScalar(1 - inbound);
        parts.streak.quaternion.setFromUnitVectors(Y, toward.normalize().negate());
        parts.burst.visible = inbound >= 1;
        parts.burst.scale.setScalar(0.4 + 1.6 * smooth(after * 3));
        parts.burst.material.opacity = 0.9 * (1 - after);
        g.scale.setScalar(U * 0.8);
        if (inbound >= 1) patch.spark = { id: e.component ?? busId(), amount: Math.min(0.99, after * 2) };
      } else if (e.kind === 'linkInterference') {
        // A jagged line from the spacecraft down towards Earth's centre; the contact beam flickers.
        const len = 3 * U;
        toward.copy(craftPos).multiplyScalar(-1).normalize();
        g.position.copy(craftPos);
        g.scale.setScalar(1);
        const pos = parts.line.geometry.attributes.position;
        const side = toward.clone().cross(up).normalize();
        for (let i = 0; i < parts.points; i++) {
          const k = i / (parts.points - 1);
          const jitter = still ? (i % 2 ? 0.25 : -0.25) : Math.sin(t * 90 + i * 12.9898) * 0.35;
          pos.setXYZ(i, toward.x * k * len + side.x * jitter * U, toward.y * k * len + side.y * jitter * U, toward.z * k * len + side.z * jitter * U);
        }
        pos.needsUpdate = true;
        parts.flecks.forEach((f, i) => {
          f.position.set(pos.getX(2 * i + 1), pos.getY(2 * i + 1), pos.getZ(2 * i + 1));
          f.material.opacity = 0.9 * fade * (still ? 1 : 0.4 + 0.6 * Math.abs(Math.sin(t * 55 + i * 3)));
        });
        parts.line.material.opacity = 0.9 * fade * (still ? 1 : 0.5 + 0.5 * Math.abs(Math.sin(t * 40)));
        patch.linkBreak = true;
      } else if (e.kind === 'panelShading') {
        // A dark cloud drifts across the craft, and the craft is dimmed while it covers it.
        const x = -2.5 + 5 * p;
        g.position.copy(craftPos).addScaledVector(right, x * U).addScaledVector(up, 0.2 * U);
        g.quaternion.copy(camera ? camera.quaternion : g.quaternion);
        parts.cloud.material.opacity = 0.6 * fade;
        patch.dim = Math.max(patch.dim, 0.65 * fade);
      } else if (e.kind === 'bonusGlow') {
        g.position.copy(craftPos);
        const breathe = still ? 1 : 1 + 0.12 * Math.sin(t * 12);
        g.scale.setScalar(U * 1.25 * breathe * (0.6 + 0.4 * fade));
        parts.shell.material.opacity = 0.3 * fade;
        parts.stars.forEach((s, i) => {
          const rise = still ? (i / 8) : (t * 1.5 + i / 8) % 1;
          const a = (i / 8) * Math.PI * 2 + (still ? 0 : t * 2);
          s.position.set(Math.cos(a) * 1.1, -0.6 + 1.8 * rise, Math.sin(a) * 1.1);
          s.material.opacity = 0.9 * fade * Math.sin(rise * Math.PI);
        });
      } else if (e.kind === 'visitor') {
        // Arrives, holds beside the craft (bobbing a little), and leaves.
        const arrive = smooth(into / 0.4) * smooth((e.days - into) / 0.4);
        const bob = still ? 0 : Math.sin(t * 6) * 0.12;
        // A star model is bigger than the generic saucer, so it holds closer, to stay on a phone-width screen.
        const beside = parts.star ? 1.5 : 2.2;
        g.position.copy(craftPos).addScaledVector(right, beside * U).addScaledVector(up, (parts.star ? 0.9 : 1.1 + bob) * U);
        g.scale.setScalar(U * (still ? 1 : arrive));
        if (parts.star) g.rotation.set(0.25, -0.5, 0); // a three-quarter view from the camera
        else if (!still) g.rotation.y = t * 1.2;
        parts.beam.material.opacity = 0.18 * (still ? 1 : arrive);
      }
    }
    for (const [id, g] of live) {
      if (seen.has(id)) continue;
      root.remove(g);
      disposeGroup(g);
      live.delete(id);
    }
    if (canvas) canvas.dataset.eventFx = status.join(' ');
    return patch;
  }

  return {
    frame,
    dispose() {
      for (const g of live.values()) disposeGroup(g);
      live.clear();
      scene.remove(root);
    },
  };
}
