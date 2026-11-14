// Procedural low-poly rockets for the Launch Vehicle classes in the Scene Spec ("small", "medium", "heavy").
// createRocket(cls, height) returns a group standing on y = 0 with its nose at y = height. The fairing is a child
// named 'fairing' and the payload bay top is at `group.userData.payloadY`, so a later Shot (the pad, ticket 07) can
// open the fairing or stack the spacecraft on top.
import * as THREE from '../../vendor/three/three.module.js';
import { palette } from './palette.js';

const flat = (color, metal = 0.2, rough = 0.6) =>
  new THREE.MeshStandardMaterial({ color, flatShading: true, metalness: metal, roughness: rough });

function cylinder(r, h, material, y, segments = 8, rTop = r) {
  const m = new THREE.Mesh(new THREE.CylinderGeometry(rTop, r, h, segments), material);
  m.position.y = y + h / 2;
  return m;
}

/** An ogive nose: a lathe from radius r at the base to a point `h` above. */
function nose(r, h, material, segments = 8) {
  const pts = [];
  for (let i = 0; i <= 5; i++) {
    const t = i / 5;
    pts.push(new THREE.Vector2(Math.max(r * Math.sqrt(1 - t * t), 0.0005), h * t));
  }
  pts.reverse();
  return new THREE.Mesh(new THREE.LatheGeometry(pts, segments), material);
}

/** One core stage stack (engines, first stage, interstage, second stage, fairing) of radius r and height h. */
function core(r, h, body, band, fairingMat) {
  const g = new THREE.Group();
  const engine = flat(palette.rocketDark, 0.6, 0.4);
  // Engine bells.
  const bell = cylinder(r * 0.55, h * 0.03, engine, 0, 8, r * 0.3);
  g.add(bell);
  const first = h * 0.58;
  g.add(cylinder(r, first, body, h * 0.03));
  g.add(cylinder(r * 1.01, h * 0.035, band, h * 0.03 + first));
  const secondY = h * 0.03 + first + h * 0.035;
  const second = h * 0.15;
  g.add(cylinder(r, second, body, secondY));
  const fairingY = secondY + second;
  const fairing = new THREE.Group();
  fairing.name = 'fairing';
  const fr = r * 1.12;
  fairing.add(cylinder(fr, h * 0.09, fairingMat, fairingY, 8, fr));
  const tip = nose(fr, h - (fairingY + h * 0.09), fairingMat);
  tip.position.y = fairingY + h * 0.09;
  fairing.add(tip);
  g.add(fairing);
  g.userData.payloadY = fairingY;
  // Grid fins / legs at the base.
  for (let i = 0; i < 4; i++) {
    const a = (i / 4) * Math.PI * 2 + Math.PI / 4;
    const fin = new THREE.Mesh(new THREE.BoxGeometry(r * 0.12, h * 0.08, r * 0.9), band);
    fin.position.set(Math.cos(a) * r * 1.2, h * 0.07, Math.sin(a) * r * 1.2);
    fin.rotation.y = -a;
    g.add(fin);
  }
  return g;
}

export function createRocket(cls = 'medium', height = 2) {
  const group = new THREE.Group();
  group.name = `rocket-${cls}`;
  const white = flat(palette.rocketWhite, 0.15, 0.55);
  const dark = flat(palette.rocketDark, 0.3, 0.5);
  const accent = flat(palette.accent, 0.2, 0.5);
  if (cls === 'small') {
    // A slim dark single-core launcher with a white fairing.
    const c = core(height * 0.036, height, dark, accent, white);
    group.add(c);
    group.userData.payloadY = c.userData.payloadY;
  } else if (cls === 'heavy') {
    // A wide core with two strap-on boosters.
    const r = height * 0.05;
    const c = core(r, height, white, dark, white);
    group.add(c);
    for (const s of [-1, 1]) {
      const bh = height * 0.55;
      const booster = new THREE.Group();
      booster.add(cylinder(r * 0.55, bh * 0.85, white, 0));
      const tip = nose(r * 0.55, bh * 0.15, accent);
      tip.position.y = bh * 0.85;
      booster.add(tip);
      booster.position.x = s * r * 1.6;
      group.add(booster);
    }
    group.userData.payloadY = c.userData.payloadY;
  } else {
    const c = core(height * 0.03, height, white, dark, white);
    group.add(c);
    group.userData.payloadY = c.userData.payloadY;
  }
  return group;
}
