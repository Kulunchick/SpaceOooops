// Spacecraft models. Ticket 01 only needs a generic small craft for the title shot; ticket 02 replaces this
// with the live assembly built from the Scene Spec's parts.
import * as THREE from '../../vendor/three/three.module.js';
import { palette } from './palette.js';

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
