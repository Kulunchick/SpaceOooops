// Spacecraft models.
//
// createSpacecraft() is the live model built from the Scene Spec's `spacecraft.parts` (see src/scene/spacecraft.rs).
// It is one THREE.Group that any Shot can place (hangar, on the rocket, in orbit, tumbling); it knows nothing of the
// hangar. Each part is a child group named by its stable part id (the catalog option id), so other modules find it
// with `craft.part(id)` or `craft.partsOfKind(kind)`. Parts are diffed by id: a new part snaps in, a removed part
// snaps out, a part whose finish or size changed is rebuilt with a small pop.
//
// Units are metres; the bus is centred on the group's origin with its long axis along +y.
//
// createTitleCraft() is the small generic craft circling Earth on the title screen.
import * as THREE from '../../vendor/three/three.module.js';
import { palette } from './palette.js';

// ---------------------------------------------------------------- materials (shared, never disposed per part)
const materials = new Map();

/** A flat-shaded palette material, cached by its settings. */
function mat(color, { metal = 0.15, rough = 0.65, emissive = 0x000000, opacity = 1, side = THREE.FrontSide } = {}) {
  const key = [color, metal, rough, emissive, opacity, side].join('|');
  if (!materials.has(key)) {
    materials.set(key, new THREE.MeshStandardMaterial({
      color, metalness: metal, roughness: rough, emissive, flatShading: true, side,
      transparent: opacity < 1, opacity, depthWrite: opacity === 1,
    }));
  }
  return materials.get(key);
}

/** The finish of a part: Flight-proven hardware wears gold foil and gold trim, Basic hardware plain metal. */
function finishOf(finish) {
  const proven = finish === 'flightProven';
  return {
    proven,
    body: proven ? mat(palette.foil, { metal: 0.55, rough: 0.35 }) : mat(palette.alu, { metal: 0.3, rough: 0.55 }),
    trim: proven ? mat(palette.foil, { metal: 0.6, rough: 0.3 }) : mat(palette.aluDark, { metal: 0.3, rough: 0.6 }),
  };
}

// ---------------------------------------------------------------- geometry helpers (local frame: stands on y=0)
function box(w, h, d, material, x = 0, y = 0, z = 0) {
  const m = new THREE.Mesh(new THREE.BoxGeometry(w, h, d), material);
  m.position.set(x, y, z);
  return m;
}

function cyl(rTop, rBottom, h, material, segments = 8, x = 0, y = 0, z = 0) {
  const m = new THREE.Mesh(new THREE.CylinderGeometry(rTop, rBottom, h, segments), material);
  m.position.set(x, y, z);
  return m;
}

/** A shallow low-poly dish opening towards +y, `dia` wide and `depth` deep, its rim at y = depth. */
function dish(dia, depth, material, segments = 8) {
  const r = dia / 2;
  const pts = [];
  for (let i = 0; i <= 4; i++) {
    const x = (r * i) / 4;
    pts.push(new THREE.Vector2(Math.max(x, 0.0005), depth * (x / r) ** 2));
  }
  return new THREE.Mesh(new THREE.LatheGeometry(pts, segments), material);
}

// ---------------------------------------------------------------- part builders
// Each builder gets the part spec ({ size, count, shape, finish }) and returns an Object3D in the part's own frame.

function buildBus({ size, finish }) {
  const [x, y, z] = size;
  const g = new THREE.Group();
  if (finish === 'outline') {
    // No structure chosen yet: an empty wire frame with a faint skin.
    const edges = new THREE.LineSegments(
      new THREE.EdgesGeometry(new THREE.BoxGeometry(x, y, z)),
      new THREE.LineBasicMaterial({ color: palette.accent }),
    );
    g.add(edges, box(x, y, z, mat(palette.accent, { opacity: 0.08 })));
    return g;
  }
  const rail = Math.min(x, z) * 0.08;
  g.add(box(x - rail * 0.6, y - rail * 0.6, z - rail * 0.6, mat(palette.alu, { metal: 0.35, rough: 0.5 })));
  // Corner rails along y, and end frames.
  for (const sx of [-1, 1]) {
    for (const sz of [-1, 1]) {
      g.add(box(rail, y + rail * 0.8, rail, mat(palette.frame, { metal: 0.4, rough: 0.5 }), (sx * (x - rail)) / 2, 0, (sz * (z - rail)) / 2));
    }
  }
  for (const sy of [-1, 1]) {
    g.add(box(x, rail * 0.7, z, mat(palette.frame, { metal: 0.4, rough: 0.5 }), 0, (sy * (y - rail * 0.7)) / 2, 0));
  }
  // Panel seams: one dark band per CubeSat unit along y.
  const units = Math.max(1, Math.round(y / 0.1));
  for (let i = 1; i < units; i++) {
    g.add(box(x + 0.001, 0.003, z + 0.001, mat(palette.frame), 0, -y / 2 + (i * y) / units, 0));
  }
  return g;
}

function cellPlate(w, h, material, cellMaterial, trimMaterial) {
  const g = new THREE.Group();
  g.add(box(w, h, 0.003, trimMaterial));
  // A grid of cells, a little proud of the substrate.
  const cols = Math.max(1, Math.round(w / 0.045));
  const rows = Math.max(1, Math.round(h / 0.045));
  const cw = (w * 0.9) / cols;
  const ch = (h * 0.92) / rows;
  for (let i = 0; i < cols; i++) {
    for (let j = 0; j < rows; j++) {
      const m = (i + j) % 2 === 0 ? material : cellMaterial;
      g.add(box(cw * 0.9, ch * 0.88, 0.002, m, -w * 0.45 + cw * (i + 0.5), -h * 0.46 + ch * (j + 0.5), 0.002));
    }
  }
  return g;
}

function buildBodyPanels({ size, finish }) {
  const [x, y, z] = size;
  const f = finishOf(finish);
  const g = new THREE.Group();
  const cells = [mat(palette.cell, { metal: 0.4, rough: 0.3 }), mat(palette.cellLight, { metal: 0.4, rough: 0.3 })];
  for (const s of [-1, 1]) {
    const side = cellPlate(z * 0.86, y * 0.94, cells[0], cells[1], f.trim);
    side.rotation.y = (s * Math.PI) / 2;
    side.position.x = (s * x) / 2;
    g.add(side);
  }
  // The back wall carries the attitude hardware and markers; only the front gets cells besides the sides.
  const front = cellPlate(x * 0.86, y * 0.5, cells[0], cells[1], f.trim);
  front.position.set(0, -y * 0.22, z / 2);
  g.add(front);
  return g;
}

/** Two wings on hinges at +x and -x. Each wing is a pivot group named 'wing+' / 'wing-' (`setCondition({ feather })` turns them edge-on about .x). */
function buildSolarWings({ size, count, finish }, bus) {
  const [height, span] = size;
  const f = finishOf(finish);
  const g = new THREE.Group();
  const cells = [mat(palette.cell, { metal: 0.4, rough: 0.3 }), mat(palette.cellLight, { metal: 0.4, rough: 0.3 })];
  const segments = Math.max(1, count);
  const segment = span / segments;
  for (const s of [1, -1]) {
    const pivot = new THREE.Group();
    pivot.name = s > 0 ? 'wing+' : 'wing-';
    pivot.position.x = (s * bus[0]) / 2;
    const arm = box(0.012, height * 0.25, 0.012, f.trim, s * 0.006, 0, 0);
    pivot.add(arm);
    for (let i = 0; i < segments; i++) {
      // The panel lies in the x-y plane with its cells facing +z (the front).
      const panel = cellPlate(segment * 0.96, height, cells[0], cells[1], f.trim);
      panel.position.x = s * (0.012 + segment * (i + 0.5));
      pivot.add(panel);
    }
    g.add(pivot);
  }
  return g;
}

function buildAntenna({ size, count, shape, finish }) {
  const [w, h, d] = size;
  const f = finishOf(finish);
  const g = new THREE.Group();
  if (shape === 'whip') {
    // A deployer box with tape-measure whips splayed out of its corners.
    g.add(box(0.08, 0.008, 0.08, f.body, 0, 0.004, 0));
    const tape = mat(palette.instrumentWhite, { metal: 0.5, rough: 0.4 });
    for (let i = 0; i < Math.max(1, count); i++) {
      const a = (i / count) * Math.PI * 2 + Math.PI / 4;
      const pivot = new THREE.Group();
      pivot.position.set(Math.cos(a) * 0.035, 0.008, Math.sin(a) * 0.035);
      pivot.rotation.set(Math.sin(a) * 0.5, 0, -Math.cos(a) * 0.5);
      pivot.add(box(Math.max(w, 0.003), h, 0.0015, tape, 0, h / 2, 0));
      g.add(pivot);
    }
  } else if (shape === 'patch') {
    g.add(box(w, h, d, f.body, 0, h / 2, 0));
    g.add(box(w * 0.82, h * 0.6, d * 0.82, mat(0xe8e2d0, { rough: 0.8 }), 0, h + h * 0.3, 0));
    g.add(box(w * 0.42, h * 0.5, d * 0.42, mat(palette.copper, { metal: 0.7, rough: 0.35 }), 0, h * 1.6, 0));
  } else {
    // dish: a short mast, the reflector and a feed on a tripod.
    const mast = h * 0.25;
    g.add(cyl(w * 0.06, w * 0.08, mast, f.trim, 6, 0, mast / 2, 0));
    const reflector = dish(w, h * 0.45, mat(palette.instrumentWhite, { rough: 0.5, side: THREE.DoubleSide }), 8);
    reflector.position.y = mast;
    g.add(reflector);
    if (f.proven) {
      const back = dish(w * 0.98, h * 0.44, mat(palette.foil, { metal: 0.6, rough: 0.3, side: THREE.BackSide }), 8);
      back.position.y = mast - 0.002;
      g.add(back);
    }
    g.add(cyl(0.004, 0.004, h * 0.75, f.trim, 5, 0, mast + h * 0.4, 0));
    g.add(cyl(w * 0.07, w * 0.05, h * 0.12, mat(palette.instrumentDark), 6, 0, mast + h * 0.8, 0));
  }
  return g;
}

function buildReactionWheels({ size, count, finish }) {
  const [w, h, d] = size;
  const f = finishOf(finish);
  const g = new THREE.Group();
  g.add(box(w, h * 0.45, d, f.body, 0, h * 0.225, 0));
  const wheel = mat(palette.alu, { metal: 0.7, rough: 0.25 });
  const hub = mat(palette.instrumentDark);
  const n = Math.max(1, count);
  const r = Math.min(w, d) * 0.2;
  for (let i = 0; i < n; i++) {
    const a = (i / n) * Math.PI * 2;
    const x = Math.cos(a) * w * 0.22;
    const z = Math.sin(a) * d * 0.22;
    g.add(cyl(r, r, h * 0.4, wheel, 10, x, h * 0.65, z));
    g.add(cyl(r * 0.35, r * 0.35, h * 0.45, hub, 6, x, h * 0.68, z));
  }
  return g;
}

function buildMagnetorquers({ size, count, finish }) {
  const [w, h, d] = size;
  const f = finishOf(finish);
  const g = new THREE.Group();
  const copper = mat(palette.copper, { metal: 0.75, rough: 0.3 });
  const r = h / 2;
  // Two torque rods crossing the face, each with end caps, and one flat air-core coil.
  const rodA = cyl(r, r, w, copper, 6, 0, r, -d * 0.25);
  rodA.rotation.z = Math.PI / 2;
  const rodB = cyl(r, r, w * 0.8, copper, 6, 0, r, d * 0.1);
  rodB.rotation.z = Math.PI / 2;
  g.add(rodA, rodB);
  for (const [rod, len] of [[rodA, w], [rodB, w * 0.8]]) {
    for (const s of [-1, 1]) {
      g.add(cyl(r * 1.4, r * 1.4, r * 1.2, f.trim, 6, (s * len) / 2, r, rod.position.z));
    }
  }
  if (count >= 3) {
    const coil = new THREE.Mesh(new THREE.TorusGeometry(d * 0.22, r * 0.6, 4, 10), copper);
    coil.rotation.x = Math.PI / 2;
    coil.position.set(0, r * 0.7, d * 0.38);
    g.add(coil);
  }
  return g;
}

function buildBatteryMarker({ size, count, finish }) {
  const [w, h, d] = size;
  const f = finishOf(finish);
  const g = new THREE.Group();
  g.add(box(w * 1.08, h * 0.25, d * 1.05, f.trim, 0, h * 0.125, 0));
  const n = Math.max(1, count);
  const r = Math.min(w / n, h) * 0.45;
  const cell = mat(palette.batteryCell, { metal: 0.3, rough: 0.5 });
  for (let i = 0; i < n; i++) {
    const c = cyl(r, r, d, cell, 6, -w / 2 + (w * (i + 0.5)) / n, h * 0.25 + r, 0);
    c.rotation.x = Math.PI / 2;
    g.add(c);
  }
  return g;
}

function buildComputerMarker({ size, finish }) {
  const [w, h, d] = size;
  const f = finishOf(finish);
  const g = new THREE.Group();
  g.add(box(w * 1.06, h * 0.4, d * 1.06, f.trim, 0, h * 0.2, 0));
  g.add(box(w, h * 0.4, d, mat(palette.pcb, { rough: 0.7 }), 0, h * 0.6, 0));
  g.add(box(w * 0.35, h * 0.5, d * 0.35, mat(palette.instrumentDark), -w * 0.15, h, 0));
  g.add(box(w * 0.08, h * 0.5, d * 0.08, mat(palette.led, { emissive: palette.led }), w * 0.35, h, d * 0.3));
  return g;
}

function buildInstrument({ size, shape }) {
  const [w, h, d] = size;
  const g = new THREE.Group();
  const dark = mat(palette.instrumentDark, { metal: 0.3, rough: 0.6 });
  const white = mat(palette.instrumentWhite, { rough: 0.6 });
  const lens = mat(palette.lens, { metal: 0.8, rough: 0.15 });
  const r = Math.min(w, d) / 2;
  switch (shape) {
    case 'telescope': {
      g.add(box(w, h * 0.22, d, dark, 0, h * 0.11, 0));
      g.add(cyl(r * 0.78, r * 0.85, h * 0.62, white, 8, 0, h * 0.53, 0));
      g.add(cyl(r * 0.95, r * 0.8, h * 0.16, dark, 8, 0, h * 0.92, 0));
      g.add(cyl(r * 0.6, r * 0.6, 0.002, lens, 8, 0, h * 0.84, 0));
      break;
    }
    case 'thermal-camera': {
      g.add(box(w, h * 0.55, d, dark, 0, h * 0.275, 0));
      g.add(cyl(r * 0.62, r * 0.7, h * 0.3, lens, 8, 0, h * 0.7, 0));
      const fin = mat(palette.alu, { metal: 0.6, rough: 0.4 });
      // Radiator fins along one side.
      for (let i = 0; i < 4; i++) g.add(box(w * 0.18, h * 0.45, d * 0.04, fin, -w / 2 - w * 0.09, h * 0.3, -d * 0.36 + i * d * 0.24));
      break;
    }
    case 'wide-camera': {
      g.add(box(w * 0.8, h * 0.6, d * 0.8, white, 0, h * 0.3, 0));
      g.add(cyl(r * 0.45, r * 0.32, h * 0.35, dark, 8, 0, h * 0.75, 0));
      g.add(cyl(r * 0.3, r * 0.3, 0.002, lens, 8, 0, h * 0.93, 0));
      break;
    }
    case 'folded-dish': {
      g.add(box(w * 0.5, h * 0.18, d * 0.5, dark, 0, h * 0.09, 0));
      g.add(cyl(0.006, 0.006, h * 0.55, white, 5, 0, h * 0.45, 0));
      const reflector = dish(Math.max(w, d) * 1.6, h * 0.25, mat(0xd6c38a, { metal: 0.6, rough: 0.35, side: THREE.DoubleSide }), 6);
      reflector.position.y = h * 0.72;
      reflector.rotation.z = 0.35;
      g.add(reflector);
      break;
    }
    case 'helix': {
      g.add(cyl(r * 1.1, r * 1.1, 0.004, mat(palette.alu, { metal: 0.6 }), 8, 0, 0.002, 0));
      const turns = 4;
      const points = [];
      for (let i = 0; i <= turns * 8; i++) {
        const a = (i / 8) * Math.PI * 2;
        points.push(new THREE.Vector3(Math.cos(a) * r * 0.55, (h * i) / (turns * 8), Math.sin(a) * r * 0.55));
      }
      const curve = new THREE.CatmullRomCurve3(points);
      g.add(new THREE.Mesh(new THREE.TubeGeometry(curve, turns * 8, 0.0025, 4), mat(palette.copper, { metal: 0.7, rough: 0.3 })));
      break;
    }
    case 'particle-stack': {
      for (let i = 0; i < 4; i++) g.add(cyl(r * (0.95 - i * 0.1), r * (0.95 - i * 0.1), h * 0.14, i % 2 ? dark : white, 8, 0, h * (0.07 + i * 0.15), 0));
      g.add(cyl(r * 0.35, r * 0.45, h * 0.35, white, 6, 0, h * 0.78, 0));
      g.add(cyl(r * 0.25, r * 0.25, 0.002, lens, 6, 0, h * 0.96, 0));
      break;
    }
    case 'probe-boom': {
      g.add(box(w * 0.6, h * 0.25, d * 0.6, dark, 0, h * 0.125, 0));
      const boom = h * 2.2;
      const arm = new THREE.Group();
      arm.rotation.z = -0.6;
      arm.position.y = h * 0.25;
      arm.add(cyl(0.003, 0.003, boom, white, 4, 0, boom / 2, 0));
      const ball = new THREE.Mesh(new THREE.IcosahedronGeometry(r * 0.4, 0), mat(palette.copper, { metal: 0.8, rough: 0.25 }));
      ball.position.y = boom;
      arm.add(ball);
      g.add(arm);
      break;
    }
    case 'radar-panel': {
      // A low electronics box with a long, flat antenna panel unfolded across it, tiled with radiating patches.
      g.add(box(w * 0.7, h * 0.3, d * 0.7, dark, 0, h * 0.15, 0));
      const span = Math.max(w, d) * 2.6;
      const depth = Math.min(w, d) * 0.9;
      const panel = mat(palette.instrumentWhite, { metal: 0.2, rough: 0.5 });
      g.add(box(span, h * 0.08, depth, panel, 0, h * 0.34, 0));
      const tile = mat(palette.alu, { metal: 0.7, rough: 0.35 });
      const n = 6;
      for (let i = 0; i < n; i++) {
        g.add(box(span / n * 0.8, h * 0.02, depth * 0.8, tile, -span / 2 + (span * (i + 0.5)) / n, h * 0.39, 0));
      }
      break;
    }
    default:
      g.add(box(w, h, d, dark, 0, h / 2, 0));
  }
  return g;
}

const builders = {
  bus: buildBus,
  bodyPanels: buildBodyPanels,
  solarWings: buildSolarWings,
  antenna: buildAntenna,
  reactionWheels: buildReactionWheels,
  magnetorquers: buildMagnetorquers,
  batteryMarker: buildBatteryMarker,
  computerMarker: buildComputerMarker,
  instrument: buildInstrument,
};

const UP = new THREE.Vector3(0, 1, 0);

/** The part's group in the spacecraft frame: positioned on its face and turned to face outwards. */
function buildPart(spec, busSize) {
  const holder = new THREE.Group();
  holder.name = spec.id;
  holder.userData = { partId: spec.id, kind: spec.kind, key: partKey(spec) };
  const body = (builders[spec.kind] ?? buildInstrument)(spec, busSize);
  holder.add(body);
  const centred = spec.kind === 'bus' || spec.kind === 'bodyPanels' || spec.kind === 'solarWings';
  if (!centred) {
    holder.position.fromArray(spec.position);
    holder.quaternion.setFromUnitVectors(UP, new THREE.Vector3().fromArray(spec.facing).normalize());
  }
  return holder;
}

/** Everything that changes a part's look; a part is rebuilt when this changes. */
function partKey(spec) {
  return JSON.stringify([spec.kind, spec.shape, spec.size, spec.position, spec.facing, spec.count, spec.finish]);
}

function disposeTree(obj) {
  obj.traverse((o) => {
    o.geometry?.dispose();
    if (o.material && ![...materials.values()].includes(o.material)) o.material.dispose();
  });
}

const SNAP_IN = 0.38;
const SNAP_OUT = 0.25;

/** Overshooting ease for the snap-in. */
function easeOutBack(t) {
  const c = 1.9;
  return 1 + (c + 1) * (t - 1) ** 3 + c * (t - 1) ** 2;
}

/**
 * The live spacecraft. `update(parts)` takes the Scene Spec's part list; `tick(dt, reducedMotion)` runs the snap
 * animations. `part(id)` finds a part's group by its id; `partsOfKind(kind)` lists the groups of a kind.
 */
export function createSpacecraft() {
  const group = new THREE.Group();
  group.name = 'spacecraft';
  const live = new Map(); // id -> holder
  const leaving = new Set(); // holders snapping out
  let busSize = [0.1, 0.3, 0.1];
  let first = true;

  function animateIn(holder, kind, reduced) {
    const facing = holder.quaternion.clone();
    const offset = kind === 'bus' ? new THREE.Vector3(0, 0, 0) : new THREE.Vector3(0, 0.08, 0).applyQuaternion(facing);
    holder.userData.anim = reduced ? null : { t: 0, dir: 1, from: holder.position.clone().add(offset), to: holder.position.clone() };
    if (!reduced) holder.scale.setScalar(0.001);
  }

  function update(parts, reducedMotion = false) {
    const bus = parts?.find((p) => p.kind === 'bus');
    if (bus) busSize = bus.size;
    const seen = new Set();
    for (const spec of parts ?? []) {
      seen.add(spec.id);
      const current = live.get(spec.id);
      if (current && current.userData.key === partKey(spec)) continue;
      const holder = buildPart(spec, busSize);
      if (current) {
        // Same part, new look (finish, size, mount): swap in place with a small pop.
        group.remove(current);
        disposeTree(current);
        if (!reducedMotion && !first) holder.userData.anim = { t: 0, dir: 0, pop: true };
      } else if (!first) {
        animateIn(holder, spec.kind, reducedMotion);
      }
      group.add(holder);
      live.set(spec.id, holder);
    }
    for (const [id, holder] of live) {
      if (seen.has(id)) continue;
      live.delete(id);
      if (reducedMotion) {
        group.remove(holder);
        disposeTree(holder);
      } else {
        holder.userData.anim = { t: 0, dir: -1, from: holder.position.clone() };
        leaving.add(holder);
      }
    }
    first = false;
  }

  function tick(dt) {
    for (const holder of [...live.values(), ...leaving]) {
      const a = holder.userData.anim;
      if (!a) continue;
      a.t += dt;
      if (a.pop) {
        const k = Math.min(a.t / 0.25, 1);
        holder.scale.setScalar(1 + 0.15 * Math.sin(k * Math.PI));
        if (k >= 1) holder.userData.anim = null;
      } else if (a.dir > 0) {
        const k = Math.min(a.t / SNAP_IN, 1);
        holder.scale.setScalar(Math.max(0.001, easeOutBack(k)));
        holder.position.lerpVectors(a.from, a.to, Math.min(1, k * 1.4));
        if (k >= 1) {
          holder.scale.setScalar(1);
          holder.position.copy(a.to);
          holder.userData.anim = null;
        }
      } else {
        const k = Math.min(a.t / SNAP_OUT, 1);
        holder.scale.setScalar(Math.max(0.001, 1 - k * k));
        if (k >= 1) {
          leaving.delete(holder);
          group.remove(holder);
          disposeTree(holder);
        }
      }
    }
  }

  // ---------------------------------------------------------------- condition (Run effects, Debrief failure shots)
  // A craft that is given a condition owns copies of its materials, one per part, so a single part can go dark
  // without touching the shared materials (the hangar's model keeps its own look). The condition is plain data and
  // carries no clock: the caller says how far each change has gone.
  const condition = { light: 1, dark: [], dim: 0, feather: 0, warn: 0, warnParts: [], spark: null, flash: 0 };
  const partMats = new Map(); // `${partId}|${original uuid}` -> { mat, partId, color, emissive }
  const ownedMats = new Set();
  let lamps = null;
  let sparks = null;

  function partIdOf(o) {
    for (let p = o; p && p !== group; p = p.parent) if (p.userData?.partId) return p.userData.partId;
    return null;
  }

  function adoptMaterials() {
    const used = new Set();
    for (const holder of live.values()) {
      holder.traverse((o) => {
        if (!o.isMesh || !o.material?.color) return;
        if (ownedMats.has(o.material)) {
          used.add(o.material);
          return;
        }
        const id = partIdOf(o);
        const key = `${id}|${o.material.uuid}`;
        if (!partMats.has(key)) {
          const mat = o.material.clone();
          partMats.set(key, { mat, partId: id, color: o.material.color.clone(), emissive: o.material.emissive?.clone() ?? null });
          ownedMats.add(mat);
        }
        o.material = partMats.get(key).mat;
        used.add(o.material);
      });
    }
    // Copies of parts that were removed or rebuilt (their meshes disposed them already).
    for (const [key, entry] of partMats) {
      if (!used.has(entry.mat)) {
        partMats.delete(key);
        ownedMats.delete(entry.mat);
      }
    }
  }

  /** Two small running lights on top of the bus; they only exist on a craft that has a condition. */
  function ensureLamps() {
    if (!lamps) {
      lamps = new THREE.Group();
      lamps.name = 'running-lights';
      for (const [s, colour] of [[1, 0x7dffa0], [-1, 0xff6b6b]]) {
        const lamp = new THREE.Mesh(
          new THREE.OctahedronGeometry(0.012, 0),
          new THREE.MeshBasicMaterial({ color: colour }),
        );
        lamp.userData = { colour: new THREE.Color(colour), side: s };
        lamps.add(lamp);
      }
      group.add(lamps);
    }
    lamps.children.forEach((lamp) => {
      lamp.position.set((lamp.userData.side * busSize[0]) / 2, busSize[1] / 2 + 0.014, busSize[2] / 2);
    });
  }

  /** A burst of low-poly sparks, built once and placed on the failing part. */
  function ensureSparks() {
    if (sparks) return sparks;
    sparks = new THREE.Group();
    sparks.name = 'sparks';
    const geo = new THREE.TetrahedronGeometry(0.03, 0);
    for (let i = 0; i < 14; i++) {
      const m = new THREE.Mesh(geo, new THREE.MeshBasicMaterial({
        color: i % 3 ? 0xffb030 : 0xfff6d0, transparent: true, depthWrite: false, blending: THREE.AdditiveBlending,
      }));
      const a = i * 2.399963; // golden angle: an even spray in every direction
      const y = 1 - (2 * (i + 0.5)) / 14;
      const r = Math.sqrt(1 - y * y);
      m.userData.dir = new THREE.Vector3(Math.cos(a) * r, y, Math.sin(a) * r);
      sparks.add(m);
    }
    group.add(sparks);
    return sparks;
  }

  const tmpBox = new THREE.Box3();
  const tmpV = new THREE.Vector3();
  const tmpC = new THREE.Color();

  /**
   * Puts the craft into a state; any keys left out keep their value. All amounts are 0..1.
   *   light      overall brightness (Earth's shadow darkens the craft)
   *   dark       part ids that are dead: drawn nearly black, their lights out
   *   dim        safe mode: the craft's lights (running lights, status LEDs) dim
   *   feather    safe mode: the solar wings turn edge-on
   *   warn       blink level of `warnParts` (an amber warning glow)
   *   spark      { id, amount } a part sparking as it fails, `amount` from 0 (start) to 1 (spent); null for none
   *   flash      a white-hot hit (the solar flare reaching the craft)
   */
  function setCondition(next = {}) {
    Object.assign(condition, next);
    adoptMaterials();
    ensureLamps();
    const dark = new Set(condition.dark ?? []);
    const warned = new Set(condition.warnParts ?? []);
    const sparkId = condition.spark?.id ?? null;
    const sparkAmount = condition.spark ? Math.min(1, Math.max(0, condition.spark.amount)) : 0;
    const flicker = sparkId ? 0.5 + 0.5 * Math.abs(Math.sin(sparkAmount * 57)) : 0;
    const light = condition.light;
    const dim = condition.dim;
    for (const { mat, partId, color, emissive } of partMats.values()) {
      const dead = dark.has(partId);
      mat.color.copy(color).multiplyScalar(light * (dead ? 0.16 : 1) * (1 - 0.25 * dim));
      if (!mat.emissive) continue;
      if (emissive) mat.emissive.copy(emissive).multiplyScalar(dead ? 0 : light * (1 - 0.92 * dim));
      if (warned.has(partId)) mat.emissive.add(tmpC.setRGB(1, 0.55, 0.05).multiplyScalar(0.9 * condition.warn));
      if (partId === sparkId) mat.emissive.add(tmpC.setRGB(1, 0.6, 0.2).multiplyScalar(1.2 * flicker * (1 - sparkAmount)));
      if (condition.flash > 0) mat.emissive.add(tmpC.setRGB(1, 0.75, 0.45).multiplyScalar(0.8 * condition.flash));
    }
    for (const lamp of lamps.children) {
      const off = dark.has(busId()) ? 0 : 1;
      lamp.material.color.copy(lamp.userData.colour).multiplyScalar(off * (1 - 0.85 * dim) + 0.08);
      lamp.scale.setScalar(1 - 0.4 * dim);
    }
    for (const pivot of ['wing+', 'wing-']) {
      const wing = group.getObjectByName(pivot);
      if (wing) wing.rotation.x = condition.feather * 1.4;
    }
    // Sparks fly out of the failing part and burn out.
    const part = sparkId ? live.get(sparkId) : null;
    if (part && sparkAmount < 1) {
      const s = ensureSparks();
      s.visible = true;
      tmpBox.setFromObject(part);
      group.worldToLocal(tmpBox.getCenter(tmpV));
      s.position.copy(tmpV);
      const reach = 0.05 + 0.4 * sparkAmount;
      s.children.forEach((m, i) => {
        m.position.copy(m.userData.dir).multiplyScalar(reach * (0.6 + 0.4 * ((i * 7) % 5) / 4));
        m.material.opacity = (1 - sparkAmount) * (0.6 + 0.4 * Math.abs(Math.sin(sparkAmount * 40 + i)));
        m.scale.setScalar(1.6 - sparkAmount);
      });
    } else if (sparks) {
      sparks.visible = false;
    }
  }

  function busId() {
    for (const [id, h] of live) if (h.userData.kind === 'bus') return id;
    return null;
  }

  return {
    group,
    update,
    tick,
    setCondition,
    /** The current condition (see setCondition). */
    condition: () => ({ ...condition }),
    part: (id) => live.get(id) ?? null,
    partsOfKind: (kind) => [...live.values()].filter((h) => h.userData.kind === kind),
    ids: () => [...live.keys()],
    /** Bus size in metres, from the latest spec. */
    busSize: () => busSize,
    /** Whether a snap animation is still running. */
    animating: () => leaving.size > 0 || [...live.values()].some((h) => h.userData.anim),
    dispose() {
      for (const h of [...live.values(), ...leaving]) disposeTree(h);
      for (const extra of [lamps, sparks]) if (extra) disposeTree(extra);
      live.clear();
      leaving.clear();
      partMats.clear();
    },
  };
}

/** A small generic CubeSat: a gold-foil bus with two blue solar wings. About 0.1 units long. */
export function createTitleCraft() {
  const craft = new THREE.Group();
  const flat = (color) => new THREE.MeshStandardMaterial({ color, flatShading: true, roughness: 0.6, metalness: 0.2 });
  const bus = new THREE.Mesh(new THREE.BoxGeometry(0.035, 0.035, 0.06), flat(palette.craftGold));
  const wing = new THREE.BoxGeometry(0.07, 0.002, 0.04);
  const left = new THREE.Mesh(wing, flat(palette.craftPanel));
  const right = left.clone();
  left.position.x = -0.055;
  right.position.x = 0.055;
  const dish = new THREE.Mesh(new THREE.ConeGeometry(0.012, 0.012, 6), flat(palette.craftBody));
  dish.position.z = 0.036;
  dish.rotation.x = Math.PI / 2;
  craft.add(bus, left, right, dish);
  return craft;
}
