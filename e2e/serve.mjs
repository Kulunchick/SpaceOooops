// Serves ../dist under /SpaceOooops/, the same sub-path as GitHub Pages, for the Playwright tests.
// Usage: node serve.mjs [port]
import { createServer } from 'node:http';
import { readFile, stat } from 'node:fs/promises';
import { extname, join, normalize, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const root = resolve(fileURLToPath(new URL('../dist', import.meta.url)));
const base = '/SpaceOooops/';
const port = Number(process.argv[2] ?? 4173);
const types = {
  '.html': 'text/html; charset=utf-8',
  '.js': 'text/javascript; charset=utf-8',
  '.mjs': 'text/javascript; charset=utf-8',
  '.css': 'text/css; charset=utf-8',
  '.wasm': 'application/wasm',
  '.svg': 'image/svg+xml',
  '.woff2': 'font/woff2',
  '.json': 'application/json',
  '.png': 'image/png',
  '.ogg': 'audio/ogg',
  '.mp3': 'audio/mpeg',
  '.glb': 'model/gltf-binary',
};

createServer(async (req, res) => {
  const path = decodeURIComponent(new URL(req.url, 'http://localhost').pathname);
  if (path === '/' || path === '/SpaceOooops') {
    res.writeHead(302, { location: base }).end();
    return;
  }
  if (!path.startsWith(base)) {
    res.writeHead(404).end('not found');
    return;
  }
  let file = normalize(join(root, path.slice(base.length)));
  if (!file.startsWith(root)) {
    res.writeHead(403).end();
    return;
  }
  try {
    if ((await stat(file)).isDirectory()) file = join(file, 'index.html');
    const body = await readFile(file);
    res.writeHead(200, { 'content-type': types[extname(file)] ?? 'application/octet-stream' }).end(body);
  } catch {
    res.writeHead(404).end('not found');
  }
}).listen(port, () => console.log(`serving ${root} at http://localhost:${port}${base}`));
