// The Design hangar: the live spacecraft on a turntable stand, the chosen Orbit around a small Earth on the left
// and the chosen Launch Vehicle's rocket on the right (Scene Spec `shot.kind === 'hangar'`, `spacecraft`, `hangar`).
//
// The hangar sits far below the title Earth and is only drawn while its Shot is active (or the camera is still
// close to it). The camera frames the model inside the DOM element marked `data-scene-drag` (the empty stage
// between the Design panels): the projection is shifted so the model sits in the stage's centre, and dragging on
// the stage turns the model. Nothing outside that element takes pointer input, so panels stay clickable.
//
// For browser tests the canvas carries `data-shot`, `data-parts` (part ids, space-separated) and `data-yaw`.
import * as THREE from '../../vendor/three/three.module.js';
import { palette } from './palette.js';
import { registerShot } from './camera.js';
import { createEarth } from './earth.js';
import { createSpacecraft } from './spacecraft.js';
import { createRocket } from './rocket.js';

/** Where the hangar floor's centre is in the world: well away from the title Earth at the origin. */
export const HANGAR_ORIGIN = new THREE.Vector3(0, -40, 0);
const STAND_TOP = 0.22;
const MAST = 0.24;
const TAN_HALF_FOV = Math.tan((40 / 2) * (Math.PI / 180));
const VIEW_DIR = new THREE.Vector3(0.18, 0.36, 1).normalize();

// Shared by the Shot pose and the hangar instance (there is one hangar per renderer).
const framing = { target: new THREE.Vector3().copy(HANGAR_ORIGIN).setY(HANGAR_ORIGIN.y + 0.6), distance: 2.4 };

registerShot('hangar', () => ({
  position: framing.target.clone().addScaledVector(VIEW_DIR, framing.distance),
  target: framing.target.clone(),
  framing: false,
}));

function flat(color, opts = {}) {
  return new THREE.MeshStandardMaterial({ color, flatShading: true, roughness: 0.8, metalness: 0.1, ...opts });
}

/** Floor, walls, ribs and lamps. */
function buildRoom() {
  const room = new THREE.Group();
  const floor = new THREE.Mesh(new THREE.CylinderGeometry(7, 7.2, 0.2, 8), flat(palette.hangarFloor));
  floor.position.y = -0.1;
  room.add(floor);
  const grid = new THREE.GridHelper(8, 40, palette.hangarRib, 0x1f2b44);
  grid.position.y = 0.002;
  room.add(grid);
  // A faceted back wall ring with ribs, open towards the camera.
  const wall = flat(palette.hangarWall, { side: THREE.DoubleSide });
  const rib = flat(palette.hangarRib);
  const sides = 10;
  for (let i = 0; i < sides; i++) {
    const a0 = Math.PI * 0.62 + (i / sides) * Math.PI * 1.76;
    const a = a0 + Math.PI * 0.088;
    if (Math.sin(a) > 0.55) continue; // leave the camera side open
    const x = Math.cos(a) * 6;
    const z = Math.sin(a) * 6 - 0.5;
    const panel = new THREE.Mesh(new THREE.BoxGeometry(3.6, 5, 0.12), wall);
    panel.position.set(x, 2.5, z);
    panel.lookAt(0, 2.5, -0.5);
    room.add(panel);
    const r = new THREE.Mesh(new THREE.BoxGeometry(0.25, 5.2, 0.3), rib);
    r.position.set(Math.cos(a0) * 5.9, 2.6, Math.sin(a0) * 5.9 - 0.5);
    r.lookAt(0, 2.6, -0.5);
    room.add(r);
  }
  // Lamp strips overhead.
  const lamp = flat(palette.hangarLight, { emissive: palette.hangarLight, emissiveIntensity: 0.9 });
  for (const x of [-1.6, 0, 1.6]) {
    const strip = new THREE.Mesh(new THREE.BoxGeometry(0.12, 0.05, 3), lamp);
    strip.position.set(x, 4.2, -1.5);
    room.add(strip);
  }
  // Stripes on the floor around the stand.
  const stripe = flat(palette.accent, { emissive: palette.accent, emissiveIntensity: 0.15 });
  const ring = new THREE.Mesh(new THREE.TorusGeometry(0.42, 0.01, 3, 8), stripe);
  ring.rotation.x = Math.PI / 2;
  ring.position.y = 0.006;
  room.add(ring);
  return room;
}

function buildStand() {
  const stand = new THREE.Group();
  const base = new THREE.Mesh(new THREE.CylinderGeometry(0.2, 0.27, STAND_TOP - 0.03, 8), flat(palette.frame, { metalness: 0.3 }));
  base.position.y = (STAND_TOP - 0.03) / 2;
  const top = new THREE.Mesh(new THREE.CylinderGeometry(0.18, 0.18, 0.03, 8), flat(palette.aluDark, { metalness: 0.4 }));
  top.position.y = STAND_TOP - 0.015;
  const glow = new THREE.Mesh(
    new THREE.TorusGeometry(0.19, 0.006, 3, 8),
    flat(palette.accent, { emissive: palette.accent, emissiveIntensity: 0.6 }),
  );
  glow.rotation.x = Math.PI / 2;
  glow.position.y = STAND_TOP;
  stand.add(base, top, glow);
  const turntable = new THREE.Group();
  turntable.position.y = STAND_TOP;
  const mast = new THREE.Mesh(new THREE.CylinderGeometry(0.012, 0.02, MAST, 6), flat(palette.aluDark, { metalness: 0.4 }));
  mast.position.y = MAST / 2;
  turntable.add(mast);
  stand.add(turntable);
  return { stand, turntable };
}

/** The Orbit preview: a small Earth on a holo pedestal with the orbit ring and a moving dot. */
function buildOrbitPreview() {
  const group = new THREE.Group();
  const { earth, stars } = createEarth('low');
  stars.geometry.dispose();
  stars.material.dispose();
  const globe = new THREE.Group();
  globe.add(earth);
  const pedestal = new THREE.Mesh(new THREE.CylinderGeometry(0.1, 0.16, 0.5, 6), flat(palette.frame, { metalness: 0.3 }));
  pedestal.position.y = 0.25;
  const emitter = new THREE.Mesh(
    new THREE.CylinderGeometry(0.11, 0.11, 0.02, 6),
    flat(palette.accent, { emissive: palette.accent, emissiveIntensity: 0.5 }),
  );
  emitter.position.y = 0.51;
  const plane = new THREE.Group(); // tilted by the inclination
  const ring = new THREE.Mesh(
    new THREE.TorusGeometry(1, 0.012, 3, 48),
    new THREE.MeshBasicMaterial({ color: palette.accent }),
  );
  ring.rotation.x = Math.PI / 2;
  const dot = new THREE.Mesh(new THREE.OctahedronGeometry(0.045, 0), new THREE.MeshBasicMaterial({ color: palette.ice }));
  plane.add(ring, dot);
  globe.add(plane);
  group.add(pedestal, emitter, globe);
  return { group, globe, earth, plane, ring, dot };
}

export function createHangar({ scene, camera, canvas }) {
  const root = new THREE.Group();
  root.name = 'hangar';
  root.position.copy(HANGAR_ORIGIN);
  root.add(buildRoom());
  const lamp = new THREE.PointLight(palette.hangarLight, 6, 9, 1.6);
  lamp.position.set(0.5, 3.2, 1.5);
  const rim = new THREE.PointLight(0x7fa8ff, 3, 8, 1.6);
  rim.position.set(-2, 1.5, -2);
  root.add(lamp, rim);

  const { stand, turntable } = buildStand();
  root.add(stand);
  const craft = createSpacecraft();
  turntable.add(craft.group);

  const orbit = buildOrbitPreview();
  orbit.group.visible = false;
  root.add(orbit.group);

  let rocket = null;
  let rocketKey = '';
  const rocketHolder = new THREE.Group();
  root.add(rocketHolder);
  const pad = new THREE.Mesh(new THREE.CylinderGeometry(0.22, 0.26, 0.04, 8), flat(palette.frame, { metalness: 0.3 }));
  pad.position.y = 0.02;
  rocketHolder.add(pad);
  rocketHolder.visible = false;

  scene.add(root);

  let active = false;
  let reduced = false;
  let spec = null;
  let busSize = [0.1, 0.3, 0.1];
  let craftHalfWidth = 0.2;
  let rocketHeight = 0;
  let yaw = -0.5;
  let pitch = 0;
  let lastDrag = -Infinity;
  let elapsed = 0;
  let stageEl = null;
  const offset = { x: 0, y: 0, on: false };
  const layout = { earthX: -1, rocketX: 1, scale: 1 };

  // ---- drag to rotate, only on the stage element and only while the hangar is active
  let drag = null;
  function onPointerDown(e) {
    if (!active || e.button > 0) return;
    const stage = e.target instanceof Element ? e.target.closest('[data-scene-drag]') : null;
    if (!stage) return;
    drag = { id: e.pointerId, x: e.clientX, y: e.clientY };
    stage.setPointerCapture?.(e.pointerId);
    e.preventDefault();
  }
  function onPointerMove(e) {
    if (!drag || e.pointerId !== drag.id) return;
    yaw += (e.clientX - drag.x) * 0.012;
    pitch = Math.max(-0.5, Math.min(0.5, pitch + (e.clientY - drag.y) * 0.006));
    drag.x = e.clientX;
    drag.y = e.clientY;
    lastDrag = elapsed;
  }
  function onPointerUp(e) {
    if (drag && e.pointerId === drag.id) drag = null;
  }
  document.addEventListener('pointerdown', onPointerDown);
  document.addEventListener('pointermove', onPointerMove);
  document.addEventListener('pointerup', onPointerUp);
  document.addEventListener('pointercancel', onPointerUp);

  function stage() {
    if (!stageEl || !stageEl.isConnected) stageEl = document.querySelector('[data-scene-drag]');
    return stageEl;
  }

  function apply(next, reducedMotion) {
    spec = next;
    reduced = reducedMotion;
    active = next?.shot?.kind === 'hangar';
    canvas.dataset.shot = next?.shot?.kind ?? '';
    const parts = next?.spacecraft?.parts;
    if (parts) {
      craft.update(parts, reducedMotion);
      const bus = parts.find((p) => p.kind === 'bus');
      busSize = bus ? bus.size : busSize;
      const wings = parts.find((p) => p.kind === 'solarWings');
      craftHalfWidth = Math.max(busSize[0] / 2 + (wings ? wings.size[1] + 0.02 : 0.03), busSize[2] / 2 + 0.05);
      // The bus sits on the mast, its bottom face (with the antenna) clear of the stand.
      craft.group.position.y = MAST + busSize[1] / 2;
      canvas.dataset.parts = craft.ids().join(' ');
    }
    const preview = next?.hangar;
    const o = preview?.orbit;
    orbit.group.visible = Boolean(o);
    if (o) {
      orbit.ring.scale.setScalar(o.radius);
      orbit.plane.rotation.set(0, 0, 0);
      orbit.plane.rotation.x = (o.inclinationDeg * Math.PI) / 180;
      orbit.group.userData.radius = o.radius;
    }
    const lv = preview?.launchVehicle;
    rocketHolder.visible = Boolean(lv);
    const key = lv ? `${lv.class}:${lv.height}` : '';
    if (key !== rocketKey) {
      rocketKey = key;
      if (rocket) {
        rocketHolder.remove(rocket);
        rocket.traverse((x) => {
          x.geometry?.dispose();
          x.material?.dispose();
        });
        rocket = null;
      }
      if (lv) {
        rocket = createRocket(lv.class, lv.height);
        rocket.position.y = 0.04;
        rocketHolder.add(rocket);
      }
      rocketHeight = lv ? lv.height : 0;
    }
  }

  /** Per frame: idle turn, snap animations, framing inside the stage element. */
  function frame(dt, seconds) {
    elapsed += dt;
    const near = camera.position.distanceTo(root.position) < 25;
    root.visible = active || near;
    if (!root.visible) {
      clearOffset();
      return;
    }
    craft.tick(dt);
    if (!reduced && !drag && elapsed - lastDrag > 2.5) yaw += dt * 0.18;
    turntable.rotation.set(pitch * 0.4, yaw, 0);
    canvas.dataset.yaw = yaw.toFixed(2);
    orbit.earth.rotation.y = reduced ? 0 : seconds * 0.15;
    if (orbit.group.visible) {
      const a = reduced ? 0.9 : seconds * 0.9;
      orbit.dot.position.set(Math.cos(a), 0, -Math.sin(a)).multiplyScalar(orbit.ring.scale.x);
    }

    // Framing: the visible half-height at the model grows gently with the bus, so a bigger structure looks bigger,
    // and the previews sit at fixed fractions of the visible frame, so they always fit beside the model.
    const w = window.innerWidth;
    const h = window.innerHeight;
    const el = active ? stage() : null;
    const rect = el?.getBoundingClientRect();
    const sw = rect && rect.width > 40 ? rect.width : w;
    const sh = rect && rect.height > 40 ? rect.height : h;
    const busCube = Math.cbrt(busSize[0] * busSize[1] * busSize[2]);
    const halfHeight = 0.66 * (busCube / 0.144) ** 0.35;
    const stageAspect = sw / sh;
    const scale = halfHeight / 0.66;
    // Leave room for the previews on both sides of the model (wings included).
    const ring = 0.12 * (orbit.group.visible ? orbit.group.userData.radius : 1.2);
    const neededHalfWidth = craftHalfWidth + Math.max(0.5, ring * 2 + 0.14) * scale;
    const fit = Math.max(halfHeight, neededHalfWidth / stageAspect);
    framing.distance = Math.max(0.8, fit / (TAN_HALF_FOV * (sh / h)));
    const visibleHalfWidth = fit * stageAspect;
    const k = 1 - Math.exp(-dt * 4);
    layout.scale += (scale - layout.scale) * k;
    const earthX = -Math.max(visibleHalfWidth * 0.6, craftHalfWidth + (ring + 0.08) * scale);
    const rocketX = Math.max(visibleHalfWidth * 0.62, craftHalfWidth + 0.2 * scale);
    layout.earthX += (earthX - layout.earthX) * k;
    layout.rocketX += (rocketX - layout.rocketX) * k;
    const centreY = STAND_TOP + MAST + busSize[1] / 2;
    // Aim a little below the model so the stand and the floor show.
    framing.target.set(HANGAR_ORIGIN.x, HANGAR_ORIGIN.y + centreY - 0.15 * layout.scale, HANGAR_ORIGIN.z);
    const earthScale = 0.12 * layout.scale;
    orbit.globe.scale.setScalar(earthScale);
    orbit.globe.position.y = centreY + 0.05;
    orbit.group.position.set(layout.earthX, 0, -0.1);
    const pedestalScale = (centreY + 0.05 - earthScale * 1.2) / 0.5;
    orbit.group.children[0].scale.y = Math.max(0.2, pedestalScale);
    orbit.group.children[0].position.y = 0.25 * Math.max(0.2, pedestalScale);
    orbit.group.children[1].position.y = 0.5 * Math.max(0.2, pedestalScale) + 0.01;
    rocketHolder.position.set(layout.rocketX, 0, -0.25);
    // The spec's rocket height is relative; scale it so the tallest rocket fills most of the frame's height.
    rocketHolder.scale.setScalar(0.48 * layout.scale);

    // Shift the projection so the model's centre lands in the middle of the stage element.
    const tx = active && rect ? w / 2 - (rect.left + rect.width / 2) : 0;
    const ty = active && rect ? h / 2 - (rect.top + rect.height / 2) : 0;
    const ko = reduced ? 1 : 1 - Math.exp(-dt * 5);
    offset.x += (tx - offset.x) * ko;
    offset.y += (ty - offset.y) * ko;
    if (Math.abs(offset.x) > 0.5 || Math.abs(offset.y) > 0.5) {
      camera.setViewOffset(w, h, offset.x, offset.y, w, h);
      offset.on = true;
    } else {
      clearOffset();
    }
  }

  function clearOffset() {
    if (offset.on) {
      camera.clearViewOffset();
      offset.on = false;
    }
    offset.x = 0;
    offset.y = 0;
  }

  return {
    apply,
    frame,
    craft,
    setReducedMotion(on) {
      reduced = on;
    },
    dispose() {
      document.removeEventListener('pointerdown', onPointerDown);
      document.removeEventListener('pointermove', onPointerMove);
      document.removeEventListener('pointerup', onPointerUp);
      document.removeEventListener('pointercancel', onPointerUp);
      clearOffset();
      scene.remove(root);
      turntable.remove(craft.group);
      craft.dispose();
      root.traverse((o) => {
        o.geometry?.dispose();
        o.material?.dispose?.();
      });
    },
  };
}
