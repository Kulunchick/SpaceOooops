// Scenario markers: low-poly pins standing on the globe, plus the camera poses for the Scenario select and Brief
// Shots. The Scene Spec says where (lat/lon from the Scenario data) and which pin is selected.
import * as THREE from '../../vendor/three/three.module.js';
import { palette } from './palette.js';
import { EARTH_RADIUS, latLonToVector } from './earth.js';
import { registerShot } from './camera.js';

// Scenario select: Earth with Angola centred, far enough back that the whole region and its pins are in view.
registerShot('globe', (shot) => ({
  position: latLonToVector(shot.lat - 3, shot.lon, 3.6),
  target: latLonToVector(shot.lat, shot.lon, 0.25),
}));

// Brief: the camera holds close over the chosen place, tilted a little so the pin stands out.
registerShot('region', (shot) => ({
  position: latLonToVector(shot.lat - 7, shot.lon, 2.3),
  target: latLonToVector(shot.lat, shot.lon, EARTH_RADIUS * 0.97),
}));

const PIN_HEIGHT = 0.085;
const UP = new THREE.Vector3(0, 1, 0);

function createPin() {
  const group = new THREE.Group();
  const material = new THREE.MeshBasicMaterial({ color: palette.ice });
  // A faceted cone pointing away from the globe, with a gem on top, standing on a flat ring.
  const cone = new THREE.Mesh(new THREE.ConeGeometry(0.022, PIN_HEIGHT, 5), material);
  cone.rotation.x = Math.PI; // tip down, wide end up: a map pin
  cone.position.y = PIN_HEIGHT / 2;
  const gem = new THREE.Mesh(new THREE.OctahedronGeometry(0.03, 0), material);
  gem.position.y = PIN_HEIGHT + 0.02;
  const ring = new THREE.Mesh(
    new THREE.RingGeometry(0.035, 0.05, 6),
    new THREE.MeshBasicMaterial({ color: palette.space, side: THREE.DoubleSide, transparent: true, opacity: 0.9 }),
  );
  ring.rotation.x = -Math.PI / 2;
  ring.position.y = 0.004;
  group.add(cone, gem, ring);
  return { group, gem, ring, material };
}

/** Builds the pin set. `set(markers)` replaces the pins; `update` bobs the selected one. */
export function createMarkers(scene) {
  const root = new THREE.Group();
  scene.add(root);
  let pins = [];
  let key = '';

  function clear() {
    for (const pin of pins) {
      root.remove(pin.group);
      pin.group.traverse((o) => {
        o.geometry?.dispose();
        o.material?.dispose();
      });
    }
    pins = [];
  }

  return {
    set(markers) {
      const next = JSON.stringify(markers ?? []);
      if (next === key) return;
      key = next;
      clear();
      for (const m of markers ?? []) {
        const pin = createPin();
        pin.selected = m.selected;
        pin.group.position.copy(latLonToVector(m.lat, m.lon, EARTH_RADIUS));
        pin.group.quaternion.setFromUnitVectors(UP, pin.group.position.clone().normalize());
        pin.group.scale.setScalar(m.selected ? 1.5 : 1);
        if (m.selected) pin.material.color.setHex(palette.craftGold);
        root.add(pin.group);
        pins.push(pin);
      }
    },
    update(seconds, reducedMotion) {
      for (const pin of pins) {
        const pulse = reducedMotion ? 1 : 1 + Math.sin(seconds * 3) * 0.12;
        pin.ring.scale.setScalar(pin.selected ? pulse * 1.3 : 1);
        pin.gem.rotation.y = reducedMotion ? 0 : seconds * 1.2;
      }
    },
    dispose() {
      clear();
      scene.remove(root);
    },
  };
}
