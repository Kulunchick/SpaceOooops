// Procedural low-poly Earth: an icosphere whose faces are coloured from the coarse land mask in land.js,
// a sparse cloud shell, and a star field. Flat shading, palette colours only, no textures.
import * as THREE from '../../vendor/three/three.module.js';
import { palette } from './palette.js';
import { countryAt, countryParts, inCountry, mainlandAt, surfaceAt } from './land.js';

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
    case 'ice': return palette.ice;
    case 'land': return landColour(lat, lon, jitter);
    default: return jitter > 0.7 ? palette.oceanDeep : palette.ocean;
  }
}

// Points along a country's coasts and borders, about half a degree apart, to find the facets an outline crosses.
function outlineSamples(parts) {
  const points = [];
  for (const part of parts) {
    part.forEach(([lon0, lat0], i) => {
      const [lon1, lat1] = part[(i + 1) % part.length];
      const steps = Math.max(1, Math.ceil(Math.hypot(lon1 - lon0, lat1 - lat0) / 0.5));
      for (let s = 0; s < steps; s++) {
        const t = s / steps;
        points.push(latLonToVector(lat0 + (lat1 - lat0) * t, lon0 + (lon1 - lon0) * t));
      }
    });
  }
  return points;
}

// Whether the unit vector `p` lies inside the spherical triangle a, b, c (seen from Earth's centre).
const edge = new THREE.Vector3();
function inTriangle(p, a, b, c) {
  const side = (u, v) => Math.sign(edge.crossVectors(u, v).dot(p));
  const s = side(a, b);
  return s === side(b, c) && s === side(c, a);
}

/**
 * The facets that show a Target Country: every facet whose centre is inside its outline, and also every facet off
 * the mainland (sea or island) that the outline touches: a corner inside it, or a stretch of its coast running
 * through it. The second rule lets a narrow country of islands (Japan) read as a country and keeps the facets under
 * its pin and stations; facets on another country's mainland are never taken, so no highlight bleeds into a neighbour.
 */
function countryFaces(id, faces, points) {
  const samples = outlineSamples(countryParts(id));
  const out = new Set();
  // The facet under each of the country's pins and stations (only those inside its outline: Angola's backup station
  // is in South Africa), even where the coarse globe puts the facet's centre over the border.
  for (const { lat, lon } of points) {
    if (!inCountry(id, lon, lat)) continue;
    const p = latLonToVector(lat, lon);
    const face = faces.findIndex((f) => f.corners[0].dot(p) > 0.9 && inTriangle(p, ...f.corners));
    if (face >= 0) out.add(face);
  }
  faces.forEach((f, index) => {
    if (f.country === id) {
      out.add(index);
      return;
    }
    if (f.country !== null || f.mainland) return;
    // Facets far from every sample cannot touch the outline (a facet is a few degrees across).
    if (!samples.some((p) => p.distanceToSquared(f.centroid) < 0.012)) return;
    const touches = f.corners.some((v) => {
      const at = vectorToLatLon(v);
      return inCountry(id, at.lon, at.lat);
    }) || samples.some((p) => p.dot(f.centroid) > 0.9 && inTriangle(p, ...f.corners));
    if (touches) out.add(index);
  });
  return out;
}

/**
 * Colours every triangle of a non-indexed geometry with `colourOf(centroid)`. Returns `highlight(countryId, points)`,
 * which paints only the facets that show that Target Country (see `countryFaces`; `points` are its pins and stations,
 * [{ lat, lon }]) in the highlight colour; every other facet keeps its own colour. An unknown id or null clears it.
 */
function colourFaces(geometry, colourOf) {
  const pos = geometry.getAttribute('position');
  const colours = new Float32Array(pos.count * 3);
  const faces = [];
  const colour = new THREE.Color();
  for (let i = 0; i < pos.count; i += 3) {
    const corners = [0, 1, 2].map((k) => new THREE.Vector3().fromBufferAttribute(pos, i + k).normalize());
    const centroid = corners[0].clone().add(corners[1]).add(corners[2]).divideScalar(3);
    const at = vectorToLatLon(centroid);
    faces.push({ corners, centroid: centroid.clone().normalize(), country: countryAt(at.lon, at.lat), mainland: mainlandAt(at.lon, at.lat) });
    colour.setHex(colourOf(centroid));
    for (let k = 0; k < 3; k++) colour.toArray(colours, (i + k) * 3);
  }
  const base = colours.slice();
  const attribute = new THREE.BufferAttribute(colours, 3);
  geometry.setAttribute('color', attribute);
  const lit = new THREE.Color(palette.highlight);
  const shown = new Map();
  let current;
  return (countryId, points = []) => {
    const key = `${countryId}|${points.map((p) => `${p.lat},${p.lon}`).join(';')}`;
    if (key === current) return;
    current = key;
    colours.set(base);
    if (countryId && !shown.has(key)) shown.set(key, countryFaces(countryId, faces, points));
    for (const face of shown.get(key) ?? []) {
      for (let k = 0; k < 3; k++) lit.toArray(colours, (face * 3 + k) * 3);
    }
    attribute.needsUpdate = true;
  };
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
  const highlight = colourFaces(globeGeometry, faceColour);
  const globe = new THREE.Mesh(
    globeGeometry,
    new THREE.MeshStandardMaterial({ vertexColors: true, flatShading: true, roughness: 0.95, metalness: 0 }),
  );
  const clouds = cloudShell(quality === 'low' ? 6 : 9);
  const earth = new THREE.Group();
  earth.add(globe, clouds);
  return { earth, clouds, highlight, stars: starField(quality === 'low' ? 400 : 900) };
}
