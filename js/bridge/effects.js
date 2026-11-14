// Random Events and Decisions in the Run scene (Scene Spec `effects` and `stations`, see src/scene/effects.rs).
//
// Rust decides which effects are active and when each one starts and ends, in Run days. This file only draws
// them, and draws each one purely from where the playback position is inside it: no timers, so a paused Run
// freezes a flare wave in mid-flight, 4x runs it four times faster and a skip leaves nothing behind.
//
//   flare             a wave leaves the Sun and hits the spacecraft at HIT of the flare day (the craft flashes)
//   safeMode          the solar wings feather and the running lights dim (eased in and out at the window's ends)
//   stationDark       the station's beacon goes dark
//   contact           a beam between the spacecraft and the station, drawn while the station is in view
//   failure           the part sparks from HIT of the failure day, then stays dark to the end of the Run
//   batteryShortfall  the battery blinks amber for the day
//
// The spacecraft's part of it goes through `craft.setCondition(...)` (spacecraft.js), which any craft has, so the
// Debrief's failure shots can put a craft into the same states without this module.
//
// Ground stations sit on `run.earthFixed` (the Earth group, rebuilt on a quality change), so they are re-attached
// in apply. For browser tests the canvas carries `data-effects` (active kinds), `data-flare` (wave progress drawn,
// 0..1, empty without a flare), `data-feather`, `data-dark-parts`, `data-stations` (`id:on|dark` pairs) and
// `data-contact` (the station a beam is drawn to, empty when none).
import * as THREE from '../../vendor/three/three.module.js';
import { latLonToVector } from './earth.js';

/** Share of the flare day at which the wave reaches the spacecraft (failures on that day spark from here too). */
const HIT = 0.6;
/** Days over which safe mode feathers the wings in, and back out at its end. */
const SAFE_EASE_DAYS = 0.3;
const BEACON_ON = 0x7dffb0;
const BEACON_OFF = 0x2a2f3a;
const BEAM = 0x6fe3ff;
const FLARE = 0xffa23a;
const AMBER = 0xffb02e;
const RED = 0xff3b3b;
const tint = new THREE.Color();

const smooth = (x) => {
  const t = Math.min(1, Math.max(0, x));
  return t * t * (3 - 2 * t);
};

function additive(color, opacity) {
  return new THREE.MeshBasicMaterial({
    color, transparent: true, opacity, depthWrite: false, blending: THREE.AdditiveBlending, side: THREE.DoubleSide,
  });
}

/** A low-poly ground station: a pad, a dish on a mast, a beacon lamp and a vertical light beam above it. */
function buildStation() {
  const g = new THREE.Group();
  const grey = new THREE.MeshStandardMaterial({ color: 0xd8dde6, flatShading: true, roughness: 0.6 });
  const pad = new THREE.Mesh(new THREE.CylinderGeometry(0.03, 0.034, 0.006, 6), grey);
  pad.position.y = 0.003;
  const mast = new THREE.Mesh(new THREE.CylinderGeometry(0.004, 0.006, 0.03, 5), grey);
  mast.position.y = 0.021;
  const dish = new THREE.Mesh(new THREE.ConeGeometry(0.018, 0.01, 7, 1, true), grey);
  dish.rotation.x = Math.PI;
  dish.position.y = 0.038;
  const lamp = new THREE.Mesh(new THREE.OctahedronGeometry(0.009, 0), new THREE.MeshBasicMaterial({ color: BEACON_ON }));
  lamp.position.y = 0.05;
  const beam = new THREE.Mesh(new THREE.CylinderGeometry(0.004, 0.012, 0.22, 6, 1, true), additive(BEACON_ON, 0.5));
  beam.position.y = 0.05 + 0.11;
  g.add(pad, mast, dish, lamp, beam);
  // Not to scale, like everything in the Run: big enough to find on the globe.
  g.scale.setScalar(1.6);
  g.userData = { lamp, beam };
  return g;
}

/**
 * The flare: a train of shock rings travelling along the group's +y (from the Sun towards the spacecraft),
 * spreading as they go, and a thin streak behind the leading ring.
 */
function buildFlare() {
  const g = new THREE.Group();
  const rings = [];
  for (let i = 0; i < 4; i++) {
    const ring = new THREE.Mesh(new THREE.TorusGeometry(1, 0.09, 4, 18), additive(i ? FLARE : 0xffe6b0, 0.85));
    ring.rotation.x = Math.PI / 2; // facing +y, the way it travels
    const holder = new THREE.Group();
    holder.add(ring);
    holder.userData = { lag: i * 0.05, ring };
    g.add(holder);
    rings.push(holder);
  }
  const streak = new THREE.Mesh(new THREE.CylinderGeometry(0.012, 0.03, 1, 6, 1, true), additive(FLARE, 0.4));
  g.add(streak);
  // Drawn over Earth: the wave reaches the spacecraft even when it sits in Earth's shadow (as in the sim).
  g.traverse((o) => {
    if (o.material) o.material.depthTest = false;
    o.renderOrder = 10;
  });
  g.userData = { rings, streak };
  g.visible = false;
  return g;
}

export function createEffects({ scene, run, canvas, camera = null }) {
  const root = new THREE.Group();
  root.name = 'run-effects';
  scene.add(root);

  const flare = buildFlare();
  root.add(flare);
  const beam = new THREE.Mesh(new THREE.CylinderGeometry(0.006, 0.006, 1, 6, 1, true), additive(BEAM, 0.6));
  beam.visible = false;
  root.add(beam);

  const stations = new Map(); // id -> group
  let spec = null;
  let active = false;

  const sunPos = new THREE.Vector3();
  const craftPos = new THREE.Vector3();
  const stationPos = new THREE.Vector3();
  const normal = new THREE.Vector3();
  const dir = new THREE.Vector3();
  const Y = new THREE.Vector3(0, 1, 0);

  function syncStations(list) {
    const seen = new Set();
    for (const s of list ?? []) {
      seen.add(s.id);
      let g = stations.get(s.id);
      if (!g) {
        g = buildStation();
        g.name = `station-${s.id}`;
        stations.set(s.id, g);
      }
      // The Earth group may have been rebuilt (quality change): put the station back on the current one.
      if (g.parent !== run.earthFixed) run.earthFixed.add(g);
      const up = latLonToVector(s.lat, s.lon);
      g.position.copy(up);
      g.quaternion.setFromUnitVectors(Y, up);
    }
    for (const [id, g] of stations) {
      if (seen.has(id)) continue;
      g.parent?.remove(g);
      stations.delete(id);
    }
  }

  function apply(next) {
    spec = next;
    active = Boolean(next?.orbit) && next?.shot?.kind === 'orbit';
    syncStations(active ? next.stations : []);
    canvas.dataset.effects = active ? (next.effects ?? []).map((e) => e.kind).join(' ') : '';
    if (!active) {
      flare.visible = false;
      beam.visible = false;
      canvas.dataset.flare = '';
      canvas.dataset.contact = '';
      canvas.dataset.stations = '';
      canvas.dataset.feather = '';
      canvas.dataset.darkParts = '';
    }
  }

  /** Per frame, after the orbit scene has placed the spacecraft. */
  function frame() {
    if (!active || !spec) return;
    const playback = spec.orbit.playback;
    const effects = spec.effects ?? [];
    const into = (e) => playback - e.start; // days into the effect

    // ---- the spacecraft's condition
    let feather = 0;
    let flash = 0;
    let warn = 0;
    const dark = [];
    const warnParts = [];
    let spark = null;
    for (const e of effects) {
      const t = into(e);
      if (e.kind === 'safeMode') {
        feather = Math.max(feather, smooth(t / SAFE_EASE_DAYS) * smooth((e.days - t) / SAFE_EASE_DAYS));
      } else if (e.kind === 'failure') {
        if (t >= HIT) dark.push(e.part);
        if (t >= HIT && t <= 1) spark = { id: e.part, amount: (t - HIT) / (1 - HIT) };
      } else if (e.kind === 'batteryShortfall') {
        warnParts.push(e.part);
        warn = Math.max(warn, 0.5 + 0.5 * Math.sin(t * Math.PI * 2 * 5));
      } else if (e.kind === 'flare') {
        flash = Math.max(flash, Math.max(0, 1 - Math.abs(t - HIT) / 0.12));
      }
    }
    run.craft.setCondition({ feather, dim: feather, dark, warnParts, warn, spark, flash });
    canvas.dataset.feather = feather.toFixed(2);
    canvas.dataset.darkParts = dark.join(' ');
    // The ring round the spacecraft carries the news at a distance: amber in safe mode or on a battery warning,
    // red while a part sparks, white-hot as the flare hits.
    const haloColour = run.halo.material.color;
    haloColour.lerp(tint.setHex(AMBER), Math.max(0.75 * feather, 0.8 * warn));
    if (spark) haloColour.lerp(tint.setHex(RED), 0.9 * (1 - 0.5 * spark.amount));
    else if (dark.length) haloColour.lerp(tint.setHex(RED), 0.45);
    haloColour.lerp(tint.setHex(0xfff0d0), flash);

    run.carrier.getWorldPosition(craftPos);
    if (camera) {
      const p = craftPos.clone().project(camera);
      canvas.dataset.craftXy = `${Math.round(((p.x + 1) / 2) * window.innerWidth)} ${Math.round(((1 - p.y) / 2) * window.innerHeight)}`;
    }
    run.sun.getWorldPosition(sunPos);

    // ---- the flare wave, from the Sun to the spacecraft and on past it
    const wave = effects.find((e) => e.kind === 'flare');
    flare.visible = Boolean(wave);
    if (wave) {
      const s = Math.min(1, Math.max(0, into(wave)));
      const reach = sunPos.distanceTo(craftPos);
      dir.subVectors(craftPos, sunPos).normalize();
      flare.position.copy(sunPos);
      flare.quaternion.setFromUnitVectors(Y, dir);
      for (const holder of flare.userData.rings) {
        // Progress of this ring: 0 leaving the Sun, 1 at the spacecraft (at HIT of the day), beyond it after.
        const k = Math.max(0, s - holder.userData.lag) / HIT;
        holder.visible = k > 0.01 && k < 1.5;
        holder.position.set(0, k * reach, 0);
        holder.scale.setScalar(0.06 + 0.2 * k);
        const fade = Math.min(1, k * 5) * Math.max(0, 1 - Math.max(0, k - 1) * 2);
        holder.userData.ring.material.opacity = 0.85 * fade;
      }
      // The streak runs from the Sun to the leading ring, and fades once the wave has passed.
      const front = Math.min(s / HIT, 1.3) * reach;
      const { streak } = flare.userData;
      streak.visible = front > 0.01;
      streak.scale.set(1, Math.max(0.01, front), 1);
      streak.position.set(0, front / 2, 0);
      streak.material.opacity = 0.45 * Math.max(0, 1 - Math.max(0, s - HIT) / (1 - HIT));
      // The Sun swells as the flare leaves it.
      run.sun.scale.multiplyScalar(1 + 0.6 * Math.max(0, 1 - s / 0.25) * Math.min(1, s * 20));
      canvas.dataset.flare = s.toFixed(3);
    } else {
      canvas.dataset.flare = '';
    }

    // ---- ground stations: beacons, outages and the contact beam
    const darkStations = new Set(effects.filter((e) => e.kind === 'stationDark').map((e) => e.station));
    const pulse = 0.75 + 0.25 * Math.sin(playback * Math.PI * 2 * 3);
    const states = [];
    for (const [id, g] of stations) {
      const off = darkStations.has(id);
      const { lamp, beam: light } = g.userData;
      lamp.material.color.setHex(off ? BEACON_OFF : BEACON_ON);
      light.visible = !off;
      light.material.opacity = 0.45 * pulse;
      states.push(`${id}:${off ? 'dark' : 'on'}`);
    }
    canvas.dataset.stations = states.join(' ');

    const contact = effects.find((e) => e.kind === 'contact' && stations.has(e.station) && !darkStations.has(e.station));
    let linked = '';
    if (contact) {
      const g = stations.get(contact.station);
      g.getWorldPosition(stationPos);
      normal.copy(stationPos).normalize();
      // Only while the station can see the spacecraft (above its horizon).
      if (dir.subVectors(craftPos, stationPos).dot(normal) > 0) {
        const length = dir.length();
        beam.position.copy(stationPos).addScaledVector(dir, 0.5);
        beam.quaternion.setFromUnitVectors(Y, dir.normalize());
        beam.scale.set(1, length, 1);
        beam.material.opacity = 0.35 + 0.3 * (0.5 + 0.5 * Math.sin(playback * Math.PI * 2 * 6));
        linked = contact.station;
      }
    }
    beam.visible = Boolean(linked);
    canvas.dataset.contact = linked;
  }

  return {
    apply,
    frame,
    dispose() {
      for (const g of stations.values()) {
        g.parent?.remove(g);
        g.traverse((o) => {
          o.geometry?.dispose();
          o.material?.dispose();
        });
      }
      stations.clear();
      scene.remove(root);
      root.traverse((o) => {
        o.geometry?.dispose();
        o.material?.dispose();
      });
    },
  };
}
