// Procedural low-poly Earth: an icosphere whose faces are coloured from the coarse land mask in land.js,
// a sparse cloud shell, and a star field. Flat shading, palette colours only, no textures.
import * as THREE from '../../vendor/three/three.module.js';
import { palette } from './palette.js';
import { surfaceAt } from './land.js';

export const EARTH_RADIUS = 1;
const DEG = Math.PI / 180;

/** Unit vector for a latitude/longitude in degrees. Longitude 0, latitude 0 points along +z. */
export function latLonToVector(lat, lon, radius = 1) {
  return new THREE.Vector3(
    radius * Math.cos(lat * DEG) * Math.sin(lon * DEG),
    radius * Math.sin(lat * DEG),
    radius * Math.cos(lat * DEG) * Math.cos(lon * DEG),
  );
}

function vectorToLatLon(v) {
  const n = v.clone().normalize();
  return { lat: Math.asin(n.y) / DEG, lon: Math.atan2(n.x, n.z) / DEG };
}

// Small deterministic hash so the look is identical on every load.
function hash3(x, y, z) {
  const s = Math.sin(x * 127.1 + y * 311.7 + z * 74.7) * 43758.5453;
  return s - Math.floor(s);
}

function landColour(lat, lon, jitter) {
  const a = Math.abs(lat);
  if (a > 60) return palette.ice;
  if (a < 10) return jitter > 0.25 ? palette.forest : palette.grass;
  if (a < 16) return jitter > 0.5 ? palette.savanna : palette.grass;
  if (a < 32) return jitter > 0.2 ? palette.desert : palette.savanna;
  return jitter > 0.4 ? palette.grass : palette.forest;
}

function faceColour(centroid) {
  const { lat, lon } = vectorToLatLon(centroid);
  const jitter = hash3(centroid.x, centroid.y, centroid.z);
  switch (surfaceAt(lon, lat)) {
    case 'angola': return palette.angola;
    case 'ice': return palette.ice;
    case 'land': return landColour(lat, lon, jitter);
    default: return jitter > 0.7 ? palette.oceanDeep : palette.ocean;
  }
}

/** Colours every triangle of a non-indexed geometry with `colourOf(centroid)`. */
function colourFaces(geometry, colourOf) {
  const pos = geometry.getAttribute('position');
  const colours = new Float32Array(pos.count * 3);
  const a = new THREE.Vector3(), b = new THREE.Vector3(), c = new THREE.Vector3();
  const colour = new THREE.Color();
  for (let i = 0; i < pos.count; i += 3) {
    a.fromBufferAttribute(pos, i);
    b.fromBufferAttribute(pos, i + 1);
    c.fromBufferAttribute(pos, i + 2);
    colour.setHex(colourOf(a.add(b).add(c).divideScalar(3)));
    for (let k = 0; k < 3; k++) colour.toArray(colours, (i + k) * 3);
  }
  geometry.setAttribute('color', new THREE.BufferAttribute(colours, 3));
}

function cloudShell(detail) {
  const sphere = new THREE.IcosahedronGeometry(EARTH_RADIUS * 1.025, detail);
  const pos = sphere.getAttribute('position');
  const kept = [];
  const v = new THREE.Vector3();
  for (let i = 0; i < pos.count; i += 3) {
    v.fromBufferAttribute(pos, i);
    const { lat, lon } = vectorToLatLon(v);
    // Keep the sky over Angola clear so the title shot always shows it.
    if (lat > -22 && lat < -2 && lon > 6 && lon < 28) continue;
    // Clouds cluster in bands: more near the equator and in the mid-latitudes.
    const band = Math.abs(Math.sin(lat * DEG * 3));
    if (hash3(Math.round(v.x * 5), Math.round(v.y * 5), Math.round(v.z * 5)) * (0.5 + band) > 1.05) {
      for (let k = 0; k < 9; k++) kept.push(pos.array[i * 3 + k]);
    }
  }
  const geometry = new THREE.BufferGeometry();
  geometry.setAttribute('position', new THREE.Float32BufferAttribute(kept, 3));
  geometry.computeVertexNormals();
  const material = new THREE.MeshStandardMaterial({
    color: palette.cloud, flatShading: true, transparent: true, opacity: 0.45, roughness: 1, depthWrite: false,
  });
  return new THREE.Mesh(geometry, material);
}

function starField(count) {
  const positions = new Float32Array(count * 3);
  for (let i = 0; i < count; i++) {
    const u = hash3(i, 1.3, 7.1) * 2 - 1;
    const t = hash3(i, 9.2, 3.3) * Math.PI * 2;
    const r = 40 + hash3(i, 4.4, 2.2) * 20;
    const s = Math.sqrt(1 - u * u);
    positions.set([r * s * Math.cos(t), r * u, r * s * Math.sin(t)], i * 3);
  }
  const geometry = new THREE.BufferGeometry();
  geometry.setAttribute('position', new THREE.BufferAttribute(positions, 3));
  return new THREE.Points(geometry, new THREE.PointsMaterial({ color: palette.star, size: 0.12, sizeAttenuation: true }));
}

/** Builds the Earth group (globe + clouds) and the stars. `quality` is 'low' or 'high'. */
export function createEarth(quality) {
  const detail = quality === 'low' ? 14 : 20;
  const globeGeometry = new THREE.IcosahedronGeometry(EARTH_RADIUS, detail);
  colourFaces(globeGeometry, faceColour);
  const globe = new THREE.Mesh(
    globeGeometry,
    new THREE.MeshStandardMaterial({ vertexColors: true, flatShading: true, roughness: 0.95, metalness: 0 }),
  );
  const clouds = cloudShell(quality === 'low' ? 6 : 9);
  const earth = new THREE.Group();
  earth.add(globe, clouds);
  return { earth, clouds, stars: starField(quality === 'low' ? 400 : 900) };
}
