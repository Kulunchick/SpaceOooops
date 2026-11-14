// The launch pad (Scene Spec `shot.kind === 'pad'`, `pad`, `spacecraft`): Launch Review's scene and the launch
// sequence. The chosen Launch Vehicle (rocket.js) stands on a pad beside a service tower, the assembled spacecraft
// (spacecraft.js) on top of it, and four lamps on the tower show the Go / No-Go votes. When the Scene Spec says
// `pad.launching`, a short sequence plays: countdown 3-2-1, the fairing closes over the spacecraft, ignition, lift-off
// and ascent. A click anywhere ends it at once, like the Skip button. When it is over (or skipped) the window
// receives a `spaceooops-launch-done` event and the UI starts the Run. Nothing here touches what the Run computes.
//
// Everything is built in units of the rocket's display height H (about 1 to 2), so one set of proportions fits every
// Launch Vehicle. The pad sits far below the other scenes and is only drawn while its Shot is active or the camera is
// still close to it.
//
// For browser tests the canvas carries `data-pad-phase` (idle, countdown, ignition, liftoff, ascent, done),
// `data-pad-votes` (e.g. "go nogo pending pending"), `data-pad-rocket` (the class) and `data-pad-craft` ("1" when the
// spacecraft is on top).
import * as THREE from '../../vendor/three/three.module.js';
import { palette } from './palette.js';
import { registerShot } from './camera.js';
import { createSpacecraft } from './spacecraft.js';
import { createRocket } from './rocket.js';
import { play } from './audio.js';

/** Where the pad's centre is: well away from the title Earth at the origin and from the hangar at y = -40. */
export const PAD_ORIGIN = new THREE.Vector3(0, -100, 0);

// Timeline of the launch sequence, in seconds.
const T_IGNITION = 3;
const T_LIFTOFF = 4.2;
const T_END = 10;
const FAIRING_CLOSE = 1.4;

const SKY = new THREE.Color(0x2f5d8f);
const SPACE = new THREE.Color(palette.space);

// Shared by the Shot pose and the pad instance (one pad per renderer).
const view = { H: 1.5, lift: 0, launching: false, sway: 0 };

registerShot('pad', (_shot, seconds) => {
  const H = view.H;
  const climb = Math.min(view.lift / (3 * H), 1);
  const az = 0.5 + view.sway * Math.sin(seconds * 0.12);
  const dist = H * (1.95 + 1.5 * climb) + 0.5;
  const targetY = H * 0.46 + view.lift;
  const camY = H * 0.28 + view.lift * 0.32;
  const target = new THREE.Vector3(0, targetY, 0).add(PAD_ORIGIN);
  const position = new THREE.Vector3(Math.sin(az) * dist, camY, Math.cos(az) * dist).add(PAD_ORIGIN);
  // While launching the panels are gone: centre the rocket instead of leaving room beside it.
  return { position, target, framing: !view.launching };
});

const flat = (color, opts = {}) => new THREE.MeshStandardMaterial({ color, flatShading: true, roughness: 0.8, metalness: 0.1, ...opts });
const glow = (color) => new THREE.MeshBasicMaterial({ color });

/** Deterministic pseudo-random numbers, so the hills are the same on every visit. */
function rng(seed) {
  let s = seed;
  return () => {
    s = (s * 1664525 + 1013904223) >>> 0;
    return s / 4294967296;
  };
}

/** The ground, the hills and the pad slab: everything that does not depend on the rocket. */
function buildGround() {
  const g = new THREE.Group();
  const grass = new THREE.Mesh(new THREE.CylinderGeometry(40, 40, 0.3, 10), flat(0x4a7a45));
  grass.position.y = -0.16;
  g.add(grass);
  const rand = rng(7);
  const hillColours = [0x3f7a4a, 0x5a8a4a, 0x6f8f4f, 0xb5a45a];
  for (let i = 0; i < 16; i++) {
    const a = (i / 16) * Math.PI * 2 + rand() * 0.3;
    const r = 11 + rand() * 9;
    const h = 1.4 + rand() * 2.6;
    const hill = new THREE.Mesh(new THREE.ConeGeometry(2.6 + rand() * 2.2, h, 6 + Math.floor(rand() * 2)), flat(hillColours[i % 4]));
    hill.position.set(Math.cos(a) * r, h / 2 - 0.1, Math.sin(a) * r);
    hill.rotation.y = rand() * 3;
    g.add(hill);
  }
  return g;
}

/** Slab, launch table, tower with swing arms, lamp board and floodlights for a rocket of display height H. */
function buildPad(H, rocketHalfWidth) {
  const g = new THREE.Group();
  const concrete = flat(0x9aa3ad, { roughness: 0.9 });
  const dark = flat(0x2b3240, { metalness: 0.4 });
  const steel = flat(0xc9462c, { metalness: 0.2 }); // the classic red-orange service tower
  const slab = new THREE.Mesh(new THREE.CylinderGeometry(0.62 * H, 0.66 * H, 0.04 * H, 8), concrete);
  slab.position.y = 0.02 * H;
  g.add(slab);
  const table = new THREE.Mesh(new THREE.CylinderGeometry(rocketHalfWidth * 1.5, rocketHalfWidth * 1.7, 0.05 * H, 8), dark);
  table.position.y = 0.065 * H;
  g.add(table);
  // The flame trench: a dark slot in the slab in front of the table.
  const trench = new THREE.Mesh(new THREE.BoxGeometry(0.1 * H, 0.012 * H, 0.5 * H), flat(0x14181f));
  trench.position.set(0, 0.042 * H, 0.32 * H);
  g.add(trench);

  // Tower.
  const towerX = -(rocketHalfWidth + 0.14 * H);
  const tw = 0.075 * H;
  const th = 1.08 * H;
  const tower = new THREE.Group();
  tower.position.x = towerX;
  const post = 0.011 * H;
  for (const sx of [-1, 1]) {
    for (const sz of [-1, 1]) {
      const p = new THREE.Mesh(new THREE.BoxGeometry(post, th, post), steel);
      p.position.set(sx * tw / 2, 0.04 * H + th / 2, sz * tw / 2);
      tower.add(p);
    }
  }
  const rungs = 11;
  for (let i = 0; i <= rungs; i++) {
    const y = 0.04 * H + (i / rungs) * th;
    for (const [w, d, x, z] of [[tw, post, 0, tw / 2], [tw, post, 0, -tw / 2], [post, tw, tw / 2, 0], [post, tw, -tw / 2, 0]]) {
      const r = new THREE.Mesh(new THREE.BoxGeometry(w, post * 0.7, d), steel);
      r.position.set(x, y, z);
      tower.add(r);
    }
    if (i < rungs) {
      const dy = th / rungs;
      const brace = new THREE.Mesh(new THREE.BoxGeometry(post * 0.6, Math.hypot(tw, dy), post * 0.6), steel);
      brace.position.set(0, y + dy / 2, tw / 2);
      brace.rotation.z = (i % 2 ? 1 : -1) * Math.atan2(tw, dy);
      tower.add(brace);
    }
  }
  const top = new THREE.Mesh(new THREE.BoxGeometry(tw * 1.2, 0.02 * H, tw * 1.2), dark);
  top.position.y = 0.04 * H + th + 0.01 * H;
  tower.add(top);
  const mast = new THREE.Mesh(new THREE.CylinderGeometry(0.003 * H, 0.005 * H, 0.12 * H, 5), dark);
  mast.position.y = 0.04 * H + th + 0.08 * H;
  tower.add(mast);
  const beacon = new THREE.Mesh(new THREE.OctahedronGeometry(0.014 * H, 0), glow(0xff4b3a));
  beacon.position.y = 0.04 * H + th + 0.15 * H;
  tower.add(beacon);

  // Swing arms hinge on the tower's far corner and reach to the rocket; they swing away at ignition.
  const arms = [];
  const reach = -towerX - tw / 2 - rocketHalfWidth * 0.6;
  for (const f of [0.38, 0.62, 0.8]) {
    const hinge = new THREE.Group();
    hinge.position.set(tw / 2, 0.04 * H + th * f, -tw / 2);
    const arm = new THREE.Mesh(new THREE.BoxGeometry(reach, 0.016 * H, 0.016 * H), dark);
    arm.position.set(reach / 2, 0, tw / 2);
    // a short brace under the arm's tip
    const clamp = new THREE.Mesh(new THREE.BoxGeometry(0.03 * H, 0.03 * H, 0.03 * H), dark);
    clamp.position.set(reach, 0, tw / 2);
    hinge.add(arm, clamp);
    hinge.userData.f = f;
    tower.add(hinge);
    arms.push(hinge);
  }
  g.add(tower);

  // Go / No-Go lamps on the tower's front face, top to bottom in Role order.
  const board = new THREE.Mesh(new THREE.BoxGeometry(0.05 * H, 0.2 * H, 0.012 * H), dark);
  board.position.set(towerX, 0.04 * H + th * 0.86 - 0.08 * H, tw / 2 + 0.012 * H);
  g.add(board);
  const lamps = [];
  for (let i = 0; i < 4; i++) {
    const lamp = new THREE.Mesh(new THREE.OctahedronGeometry(0.0185 * H, 0), new THREE.MeshBasicMaterial({ color: 0x4b5568 }));
    lamp.position.set(towerX, board.position.y + 0.075 * H - i * 0.05 * H, tw / 2 + 0.028 * H);
    lamps.push(lamp);
    g.add(lamp);
  }

  // Floodlight masts at the slab's edge.
  for (const [x, z] of [[0.6 * H, 0.5 * H], [-0.55 * H, 0.62 * H]]) {
    const pole = new THREE.Mesh(new THREE.CylinderGeometry(0.006 * H, 0.009 * H, 0.5 * H, 5), dark);
    pole.position.set(x, 0.27 * H, z);
    const lampHead = new THREE.Mesh(new THREE.BoxGeometry(0.06 * H, 0.025 * H, 0.03 * H), glow(0xfff1c0));
    lampHead.position.set(x, 0.53 * H, z);
    lampHead.lookAt(0, 0.4 * H, 0);
    g.add(pole, lampHead);
  }
  return { group: g, lamps, arms, towerX, tw };
}

export function createPad({ scene, camera, canvas }) {
  const root = new THREE.Group();
  root.name = 'pad';
  root.position.copy(PAD_ORIGIN);
  root.add(buildGround());
  const sun = new THREE.DirectionalLight(0xfff1d0, 1.6);
  sun.position.set(-4, 5, 6);
  const hemi = new THREE.HemisphereLight(0xbcd8ff, 0x4a6a40, 0.9);
  root.add(sun, hemi);
  const engineLight = new THREE.PointLight(0xff9a3d, 0, 8, 1.4);
  root.add(engineLight);
  scene.add(root);

  const craft = createSpacecraft();
  let stack = null; // { group, rocket, fairing, pad, flame, core, lamps, arms }
  let stackKey = '';
  let reduced = false;
  let active = false;
  let spec = null;
  let launchT = null; // seconds since the launch started, null while idle
  let finished = false;
  let lastTick = -1;
  const fired = { ignition: false };
  let votes = [];
  const pulse = [0, 0, 0, 0];
  let craftBox = { width: 0.4, height: 0.4 };

  const background = new THREE.Color();
  let sky = 0; // 0 = space, 1 = daylight sky
  const prev = { background: null, fog: null };

  // Smoke puffs, pooled.
  const puffGeo = new THREE.IcosahedronGeometry(1, 0);
  const puffs = [];
  for (let i = 0; i < 36; i++) {
    const m = new THREE.Mesh(puffGeo, new THREE.MeshBasicMaterial({ color: 0xe6e9ee, transparent: true, opacity: 0, depthWrite: false }));
    m.visible = false;
    m.userData = { age: 99, vx: 0, vz: 0 };
    root.add(m);
    puffs.push(m);
  }
  let puffCursor = 0;
  let puffAcc = 0;

  function emitPuff(H, rr) {
    const p = puffs[puffCursor++ % puffs.length];
    const a = Math.random() * Math.PI * 2;
    p.position.set(Math.cos(a) * rr * 1.2, 0.09 * H, Math.sin(a) * rr * 1.2);
    p.userData = { age: 0, vx: Math.cos(a) * (0.12 + Math.random() * 0.18) * H, vz: Math.sin(a) * (0.12 + Math.random() * 0.18) * H, size: rr * (1.2 + Math.random()) };
    p.visible = true;
  }

  function disposeStack() {
    if (!stack) return;
    root.remove(stack.group);
    stack.craftHolder.remove(craft.group); // the spacecraft and its shared materials outlive the stack
    stack.group.traverse((o) => {
      o.geometry?.dispose();
      o.material?.dispose?.();
    });
    stack = null;
  }

  function buildStack(lv) {
    disposeStack();
    const H = lv.height;
    const rocket = createRocket(lv.class, H);
    // The core's radius (rocket.js), and the half-width of the whole stack with its boosters.
    const rr = H * (lv.class === 'heavy' ? 0.05 : lv.class === 'small' ? 0.036 : 0.03);
    const pad = buildPad(H, lv.class === 'heavy' ? rr * 2.2 : rr);
    const group = new THREE.Group();
    group.add(pad.group);
    const rocketHolder = new THREE.Group();
    rocketHolder.position.y = 0.09 * H;
    rocketHolder.add(rocket);
    group.add(rocketHolder);
    const fairing = rocket.getObjectByName('fairing');
    // The spacecraft stands on the payload adapter, exposed until the fairing closes over it.
    const adapter = new THREE.Mesh(new THREE.CylinderGeometry(rr * 0.5, rr * 0.8, 0.03 * H, 8), flat(palette.rocketDark, { metalness: 0.4 }));
    adapter.position.y = rocket.userData.payloadY + 0.015 * H;
    rocket.add(adapter);
    const craftHolder = new THREE.Group();
    craftHolder.add(craft.group);
    rocket.add(craftHolder);
    // Exhaust: a bright cone under the rocket, scaled by thrust.
    const flame = new THREE.Group();
    const outer = new THREE.Mesh(new THREE.ConeGeometry(rr * (lv.class === 'heavy' ? 0.9 : 0.65), H * 0.34, 8, 1, true), glow(0xff8a2b));
    const inner = new THREE.Mesh(new THREE.ConeGeometry(rr * (lv.class === 'heavy' ? 0.55 : 0.4), H * 0.24, 8, 1, true), glow(0xffe08a));
    for (const c of [outer, inner]) {
      c.rotation.x = Math.PI;
      c.position.y = -c.geometry.parameters.height / 2;
      flame.add(c);
    }
    flame.position.y = 0.02 * H;
    flame.visible = false;
    rocketHolder.add(flame);
    root.add(group);
    stack = { group, rocket, rocketHolder, fairing, pad, flame, craftHolder, H, rr };
    view.H = H;
    place(lv);
  }

  /** Puts the spacecraft on the adapter, scaled so it (wings included) fits inside the fairing. */
  function place() {
    if (!stack) return;
    const { rocket, craftHolder, H, rr } = stack;
    // Scaled so the spacecraft, wings included, fits inside the fairing and stays a modest part of the stack.
    const s = Math.min((rr * 2 * 0.95) / craftBox.width, (H * 0.2) / craftBox.height);
    craftHolder.scale.setScalar(s);
    craft.group.position.y = craftBox.height / 2 + 0.01;
    craftHolder.position.y = rocket.userData.payloadY + 0.03 * H;
    stack.craftScale = s;
  }

  function setPhaseText(text, phase) {
    canvas.dataset.padPhase = phase;
    const el = document.getElementById('launch-count');
    if (el && el.textContent !== text) el.textContent = text;
  }

  function finish() {
    if (launchT === null || finished) return;
    finished = true;
    setPhaseText('', 'done');
    window.dispatchEvent(new CustomEvent('spaceooops-launch-done'));
  }

  function resetLaunch() {
    launchT = null;
    finished = false;
    lastTick = -1;
    fired.ignition = false;
    view.lift = 0;
    view.launching = false;
    if (stack) {
      stack.rocketHolder.position.y = 0.09 * stack.H;
      stack.rocketHolder.rotation.set(0, 0, 0);
      stack.flame.visible = false;
      if (stack.fairing) {
        stack.fairing.visible = false;
        stack.fairing.position.y = 0;
      }
      for (const a of stack.pad.arms) a.rotation.y = 0;
    }
    engineLight.intensity = 0;
    for (const p of puffs) p.visible = false;
    setPhaseText('', 'idle');
  }

  function startLaunch() {
    resetLaunch();
    launchT = 0;
    view.launching = true;
    if (stack?.fairing) {
      stack.fairing.visible = true;
      stack.fairing.position.y = 0.4 * stack.H;
    }
    if (reduced) finish();
  }

  // A click or key press anywhere ends the sequence at once (the Skip button does the same through the UI).
  function onPointerDown() {
    if (launchT !== null && launchT > 0.3) finish();
  }
  window.addEventListener('pointerdown', onPointerDown, true);

  function apply(next, reducedMotion) {
    spec = next;
    reduced = reducedMotion;
    const pad = next?.pad;
    active = next?.shot?.kind === 'pad' && Boolean(pad);
    canvas.dataset.padRocket = pad?.launchVehicle?.class ?? '';
    canvas.dataset.padCraft = '0';
    if (!pad) {
      if (launchT !== null) resetLaunch();
      return;
    }
    const parts = next.spacecraft?.parts;
    if (parts) {
      craft.update(parts, reducedMotion);
      const bus = parts.find((p) => p.kind === 'bus');
      const size = bus ? bus.size : [0.1, 0.3, 0.1];
      const wings = parts.find((p) => p.kind === 'solarWings');
      const half = Math.max(size[0] / 2 + (wings ? wings.size[1] + 0.02 : 0.03), size[2] / 2 + 0.05);
      craftBox = { width: half * 2, height: size[1] + 0.06 };
    }
    const lv = pad.launchVehicle;
    const key = `${lv.class}:${lv.height}:${craftBox.width.toFixed(3)}:${craftBox.height.toFixed(3)}`;
    if (key !== stackKey) {
      stackKey = key;
      buildStack(lv);
      resetLaunch();
    }
    canvas.dataset.padCraft = parts && stack ? '1' : '0';

    // Votes: a lamp pulses when its vote changes.
    const now = pad.votes ?? [];
    for (let i = 0; i < 4; i++) {
      if ((now[i] ?? 'pending') !== (votes[i] ?? 'pending')) pulse[i] = 0.7;
    }
    votes = now;
    canvas.dataset.padVotes = [0, 1, 2, 3].map((i) => votes[i] ?? 'pending').join(' ');

    if (pad.launching && launchT === null) startLaunch();
    else if (!pad.launching && launchT !== null) resetLaunch();
  }

  const LAMP = { go: 0x45e08a, nogo: 0xff4b4b, pending: 0x4b5568 };

  function frame(dt, seconds) {
    const near = camera.position.distanceTo(root.position) < 60;
    root.visible = active || near;
    // Daylight sky while on the pad, easing back to space; the ascent thins it out.
    const skyTarget = active ? 1 - Math.min(Math.max((view.lift / (stack?.H ?? 1) - 1.5) / 4, 0), 0.85) : 0;
    sky += (skyTarget - sky) * (reduced ? 1 : 1 - Math.exp(-dt * 3));
    if (active || sky > 0.01) {
      if (!prev.background) prev.background = scene.background;
      background.copy(SPACE).lerp(SKY, sky);
      scene.background = background;
      if (!scene.fog) scene.fog = new THREE.Fog(background, 8, 38);
      scene.fog.color.copy(background);
      scene.fog.near = 8;
      scene.fog.far = 14 + 40 * (1 - sky);
    } else if (prev.background) {
      scene.background = prev.background;
      scene.fog = null;
      prev.background = null;
    }
    if (!root.visible || !stack) return;
    craft.tick(dt);
    view.sway = reduced || launchT !== null ? 0 : 0.14;

    // Vote lamps.
    stack.pad.lamps.forEach((lamp, i) => {
      lamp.material.color.setHex(LAMP[votes[i] ?? 'pending']);
      pulse[i] = Math.max(0, pulse[i] - dt);
      lamp.scale.setScalar(1 + (reduced ? 0 : pulse[i] * 1.2));
    });

    if (launchT === null || finished) {
      engineLight.intensity = 0;
      return;
    }
    launchT += dt;
    const t = launchT;
    const { H, rr } = stack;

    // Countdown ticks, one per second.
    const tick = Math.floor(t);
    if (tick !== lastTick && tick < T_IGNITION) {
      lastTick = tick;
      play('countdown');
    }
    // The fairing closes over the spacecraft.
    if (stack.fairing) {
      const k = Math.min(t / FAIRING_CLOSE, 1);
      stack.fairing.position.y = 0.4 * H * (1 - (1 - (1 - k) ** 3));
    }
    // Swing arms retract just before ignition.
    const armK = Math.min(Math.max((t - (T_IGNITION - 0.6)) / 0.9, 0), 1);
    stack.pad.arms.forEach((a, i) => {
      a.rotation.y = armK * (1.25 + i * 0.05);
    });

    let phase = 'countdown';
    let text = String(Math.max(1, Math.ceil(T_IGNITION - t)));
    if (t >= T_IGNITION) {
      phase = 'ignition';
      text = 'Ignition';
      if (!fired.ignition) {
        fired.ignition = true;
        play('ignition');
        play('engine');
      }
    }
    if (t >= T_LIFTOFF) {
      phase = 'liftoff';
      text = 'Lift-off';
    }
    const dtl = Math.max(0, t - T_LIFTOFF);
    if (dtl > 3.2) phase = 'ascent';
    if (dtl > 3.2) text = '';
    view.lift = 0.14 * H * dtl * dtl;
    stack.rocketHolder.position.y = 0.09 * H + view.lift;
    // A small tremble from ignition on.
    const shake = t >= T_IGNITION ? Math.min(1, (t - T_IGNITION) / 0.6) * (dtl > 0 ? 0.5 : 1) : 0;
    stack.rocketHolder.rotation.z = Math.sin(t * 60) * 0.004 * shake;
    stack.rocketHolder.rotation.x = Math.cos(t * 53) * 0.004 * shake;

    // Exhaust and smoke.
    const thrust = t >= T_IGNITION ? Math.min(1, 0.35 + (t - T_IGNITION) * 0.5) : 0;
    stack.flame.visible = thrust > 0;
    stack.flame.scale.set(1 + 0.08 * Math.sin(t * 40), thrust * (0.8 + 0.25 * Math.sin(t * 55)), 1 + 0.08 * Math.cos(t * 47));
    engineLight.position.set(0, 0.2 * H + view.lift * 0.5, 0.2 * H);
    engineLight.intensity = thrust * 9 * (0.85 + 0.15 * Math.sin(t * 50));
    if (thrust > 0 && view.lift < 6 * H) {
      puffAcc += dt * 26 * thrust;
      while (puffAcc > 1) {
        puffAcc -= 1;
        emitPuff(H, rr);
      }
    }
    for (const p of puffs) {
      if (!p.visible) continue;
      const d = p.userData;
      d.age += dt;
      p.position.x += d.vx * dt;
      p.position.z += d.vz * dt;
      p.position.y += 0.05 * H * dt;
      const life = 2.6;
      const k = d.age / life;
      if (k >= 1) {
        p.visible = false;
        continue;
      }
      p.scale.setScalar(d.size * (0.6 + k * 3));
      p.material.opacity = 0.7 * (1 - k);
    }

    setPhaseText(text, phase);
    if (t >= T_END) finish();
  }

  return {
    apply,
    frame,
    setReducedMotion(on) {
      reduced = on;
    },
    dispose() {
      window.removeEventListener('pointerdown', onPointerDown, true);
      if (prev.background) scene.background = prev.background;
      scene.fog = null;
      disposeStack();
      scene.remove(root);
      craft.dispose();
      puffGeo.dispose();
      root.traverse((o) => {
        o.geometry?.dispose();
        o.material?.dispose?.();
      });
    },
  };
}
