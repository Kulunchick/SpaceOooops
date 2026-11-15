// Star models for the Random Events that have their own look: the UFO, the space
// whale, the dragon and the rival agency's inspector satellite. Original low-poly geometry in the game's palette;
// no flag, emblem or lettering of any real agency or country.
//
// Each entry is `make(low) -> THREE.Group`, about 2 units long, nose along +x, centred on the origin, so event_effects.js
// can place it like the generic shape (which is about 1.2 units across). `low` is the Low Graphics setting: fewer
// segments. Face budget (triangles): High under 700, Low under 350 for every model (checked in the report).
// A model may carry `userData.tick(t)`, called every frame with the playback time (0 under reduced motion), to
// move its own parts (wings, tail, lights).
import * as THREE from '../../vendor/three/three.module.js';
import { palette } from './palette.js';

// A little emissive of its own colour, so a model still reads on the night side of the Earth.
const flat = (color, opts = {}) =>
  new THREE.MeshStandardMaterial({ color, emissive: color, emissiveIntensity: 0.3, flatShading: true, roughness: 0.75, ...opts });
const glow = (color) => new THREE.MeshBasicMaterial({ color });
const seg = (low, hi, lo) => (low ? lo : hi);

function mesh(geometry, material, x = 0, y = 0, z = 0) {
  const m = new THREE.Mesh(geometry, material);
  m.position.set(x, y, z);
  return m;
}

/** A flat triangle fan (wing, fin, fluke) from points in the xy plane, drawn on both sides. */
function flatShape(points, material) {
  const shape = new THREE.Shape(points.map(([x, y]) => new THREE.Vector2(x, y)));
  return new THREE.Mesh(new THREE.ShapeGeometry(shape), material);
}

const twoSided = (color, opts = {}) => flat(color, { side: THREE.DoubleSide, ...opts });

// ---- UFO: a classic saucer, a glass dome, a ring of lights, three little feet. Spins its rim lights.
function ufo(low) {
  const g = new THREE.Group();
  const n = seg(low, 12, 8);
  const hull = mesh(new THREE.CylinderGeometry(0.55, 1.0, 0.16, n), flat(palette.alu, { metalness: 0.4 }));
  const rim = mesh(new THREE.CylinderGeometry(1.0, 0.55, 0.12, n), flat(palette.aluDark, { metalness: 0.4 }), 0, -0.14, 0);
  const dome = mesh(
    new THREE.SphereGeometry(0.36, n, seg(low, 5, 3), 0, Math.PI * 2, 0, Math.PI / 2),
    flat(0x7be0c8, { emissive: 0x1a5a4a, transparent: true, opacity: 0.9 }), 0, 0.08, 0,
  );
  g.add(hull, rim, dome);
  const lights = new THREE.Group();
  const count = seg(low, 8, 6);
  for (let i = 0; i < count; i++) {
    const a = (i / count) * Math.PI * 2;
    lights.add(mesh(new THREE.OctahedronGeometry(0.07, 0), glow(i % 2 ? palette.accent : palette.led), Math.cos(a) * 0.88, -0.07, Math.sin(a) * 0.88));
  }
  g.add(lights);
  for (let i = 0; i < 3; i++) {
    const a = (i / 3) * Math.PI * 2 + 0.5;
    g.add(mesh(new THREE.ConeGeometry(0.07, 0.22, 4), flat(palette.frame), Math.cos(a) * 0.4, -0.3, Math.sin(a) * 0.4));
  }
  g.userData = { tick: (t) => { lights.rotation.y = t * 2.5; } };
  return g;
}

// ---- Space whale: a long blue body with a pale belly, a tail with a fluke, two fins, one friendly eye.
function whale(low) {
  const g = new THREE.Group();
  const w = seg(low, 9, 6);
  const h = seg(low, 6, 4);
  const body = mesh(new THREE.SphereGeometry(0.62, w, h), flat(0x6a95d8), 0.2, 0, 0);
  body.scale.set(1.7, 0.85, 0.85);
  const belly = mesh(new THREE.SphereGeometry(0.62, w, h, 0, Math.PI * 2, Math.PI * 0.55, Math.PI * 0.45), flat(0xdfe8f2), 0.2, -0.02, 0);
  belly.scale.set(1.68, 0.84, 0.84);
  const tailBase = mesh(new THREE.ConeGeometry(0.36, 1.0, seg(low, 6, 4)), flat(0x6a95d8), -1.15, 0.05, 0);
  tailBase.rotation.z = Math.PI / 2; // narrow end towards -x
  g.add(body, belly, tailBase);
  const tail = new THREE.Group();
  tail.position.set(-1.6, 0.08, 0);
  const fluke = flatShape([[0, 0], [-0.1, 0.45], [-0.4, 0.55], [-0.28, 0.08], [-0.4, -0.55], [-0.1, -0.45]], twoSided(0x4f78b8));
  fluke.rotation.x = Math.PI / 2; // flat, like a real whale's tail: spread sideways
  tail.add(fluke);
  g.add(tail);
  for (const s of [-1, 1]) {
    const fin = flatShape([[0, 0], [-0.3, -0.1], [-0.55, -0.5], [0.05, -0.3]], twoSided(0x4f78b8));
    fin.position.set(0.45, -0.3, s * 0.45);
    fin.rotation.x = s * 0.5;
    g.add(fin);
    g.add(mesh(new THREE.OctahedronGeometry(0.07, 0), glow(0xffffff), 1.05, 0.12, s * 0.42));
  }
  // A few stars on its back.
  for (let i = 0; i < 3; i++) g.add(mesh(new THREE.OctahedronGeometry(0.05, 0), glow(palette.star), 0.4 - i * 0.5, 0.52 - Math.abs(i - 1) * 0.06, 0));
  g.userData = { tick: (t) => { tail.rotation.z = Math.sin(t * 5) * 0.25; g.rotation.z = Math.sin(t * 2.5) * 0.05; } };
  return g;
}

// ---- Dragon: a green serpent body of shrinking links, a horned head, two bat wings that flap, a tail spike.
function dragon(low) {
  const g = new THREE.Group();
  const links = seg(low, 7, 5);
  const bodyMat = flat(0x3f8f4a);
  const body = new THREE.Group();
  for (let i = 0; i < links; i++) {
    const r = 0.34 - i * (0.22 / links);
    body.add(mesh(new THREE.IcosahedronGeometry(r, 0), i % 2 ? flat(0x2f7a3c) : bodyMat, -i * (low ? 0.36 : 0.28), Math.sin(i * 0.9) * 0.08, 0));
  }
  g.add(body);
  const headMat = flat(0x4fa35a);
  const head = new THREE.Group();
  head.position.set(0.5, 0.1, 0);
  head.add(mesh(new THREE.OctahedronGeometry(0.3, 0), headMat));
  const snout = mesh(new THREE.ConeGeometry(0.17, 0.4, 4), headMat, 0.3, -0.04, 0);
  snout.rotation.z = -Math.PI / 2;
  head.add(snout);
  for (const s of [-1, 1]) {
    const horn = mesh(new THREE.ConeGeometry(0.06, 0.32, 3), flat(palette.desert), -0.08, 0.3, s * 0.14);
    horn.rotation.z = 0.5;
    head.add(horn);
    head.add(mesh(new THREE.OctahedronGeometry(0.05, 0), glow(palette.highlight), 0.16, 0.1, s * 0.15));
  }
  g.add(head);
  const wings = [];
  for (const s of [-1, 1]) {
    const pivot = new THREE.Group();
    pivot.position.set(0.1, 0.25, s * 0.2);
    const wing = flatShape([[0, 0], [-0.1, 0.62], [-0.35, 0.4], [-0.5, 0.58], [-0.7, 0.0]], twoSided(palette.highlight, { roughness: 0.9 }));
    wing.rotation.x = s > 0 ? -Math.PI / 2 + 0.45 : Math.PI / 2 - 0.45; // fanned out sideways, tilted up
    pivot.add(wing);
    g.add(pivot);
    wings.push([pivot, s]);
  }
  const spike = mesh(new THREE.ConeGeometry(0.1, 0.34, 4), flat(palette.highlight), -links * (low ? 0.36 : 0.28) + 0.1, 0, 0);
  spike.rotation.z = Math.PI / 2;
  g.add(spike);
  g.position.x = 0.5;
  g.userData = { tick: (t) => { for (const [p, s] of wings) p.rotation.x = s * Math.sin(t * 7) * 0.45; body.rotation.y = Math.sin(t * 3) * 0.12; } };
  return g;
}

// ---- Inspector: the rival agency's satellite. Plain grey bus, two panels, a dish, and a big magnifying glass
// held out in front of it; a clipboard on top. No flag, no emblem.
function inspector(low) {
  const g = new THREE.Group();
  const bus = mesh(new THREE.BoxGeometry(0.7, 0.55, 0.55), flat(0x9aa5b4, { metalness: 0.3 }), -0.1, 0, 0);
  const band = mesh(new THREE.BoxGeometry(0.72, 0.1, 0.57), flat(palette.frame), -0.1, 0.12, 0);
  g.add(bus, band);
  for (const s of [-1, 1]) {
    g.add(mesh(new THREE.BoxGeometry(0.06, 0.04, 0.5), flat(palette.frame), -0.1, 0, s * 0.52));
    g.add(mesh(new THREE.BoxGeometry(0.6, 0.03, 0.62), flat(palette.cell, { metalness: 0.4 }), -0.1, 0, s * 0.88));
  }
  const dish = mesh(new THREE.ConeGeometry(0.22, 0.14, seg(low, 7, 5), 1, true), twoSided(palette.alu), -0.1, 0.42, 0);
  g.add(dish, mesh(new THREE.CylinderGeometry(0.02, 0.02, 0.22, 4), flat(palette.frame), -0.1, 0.3, 0));
  // The magnifying glass: a ring, a pale lens, a handle, out in front (+x).
  const glass = new THREE.Group();
  glass.position.set(0.8, 0.02, 0);
  glass.add(mesh(new THREE.TorusGeometry(0.34, 0.05, 4, seg(low, 10, 7)), flat(palette.copper, { metalness: 0.5 })));
  glass.add(mesh(new THREE.CircleGeometry(0.31, seg(low, 10, 7)), new THREE.MeshBasicMaterial({ color: 0xbfe3f2, transparent: true, opacity: 0.35, side: THREE.DoubleSide })));
  const handle = mesh(new THREE.CylinderGeometry(0.045, 0.055, 0.42, 4), flat(palette.frame), 0, -0.55, 0);
  glass.add(handle);
  g.add(glass);
  g.add(mesh(new THREE.CylinderGeometry(0.03, 0.03, 0.4, 4), flat(palette.frame), 0.55, -0.2, 0));
  g.children[g.children.length - 1].rotation.z = Math.PI / 2 - 0.5;
  const lamp = mesh(new THREE.OctahedronGeometry(0.06, 0), glow(0xff5a5a), -0.1, 0.32, 0.3);
  g.add(lamp);
  g.userData = { tick: (t) => { lamp.visible = Math.sin(t * 9) > -0.3; glass.rotation.z = Math.sin(t * 3) * 0.15; } };
  return g;
}

/** Star models by name. An unknown name is the generic shape (event_effects.js); the catalog rejects one at load. */
// Drawn at half size inside a wrapper, so the longest (the whale) is about the size of the generic visitor's span.
const scaled = (make) => (low) => {
  const inner = make(low);
  inner.scale.setScalar(0.5);
  const outer = new THREE.Group();
  outer.add(inner);
  outer.userData = inner.userData;
  return outer;
};

export const MODELS = { ufo: scaled(ufo), whale: scaled(whale), dragon: scaled(dragon), inspector: scaled(inspector) };
