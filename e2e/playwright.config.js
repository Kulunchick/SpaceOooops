// Browser smoke tests against the Trunk-built site. Build first:
//   trunk build --release --public-url /SpaceOooops/
// then, in e2e/: npm ci && npx playwright install chromium && npx playwright test
import { defineConfig } from '@playwright/test';

const PORT = Number(process.env.E2E_PORT ?? 4173);

export default defineConfig({
  testDir: './tests',
  timeout: 90_000,
  // Software WebGL (SwiftShader) is CPU-bound: parallel workers slow each other's frames until timers and
  // timeouts fail. One worker is reliable here and on a CI runner; E2E_WORKERS=n tries more on a fast machine.
  workers: Number(process.env.E2E_WORKERS ?? 1),
  fullyParallel: false,
  expect: { timeout: 10_000 },
  retries: process.env.CI ? 1 : 0,
  reporter: process.env.CI ? 'github' : 'list',
  use: {
    baseURL: `http://localhost:${PORT}/SpaceOooops/`,
    viewport: { width: 1280, height: 800 },
  },
  webServer: {
    command: `node serve.mjs ${PORT}`,
    url: `http://localhost:${PORT}/SpaceOooops/`,
    reuseExistingServer: !process.env.CI,
  },
  projects: [
    {
      // WebGL through SwiftShader, so the 3D scene runs even on a GPU-less CI machine.
      name: 'webgl',
      testMatch: /webgl\.spec\.js/,
      use: {
        browserName: 'chromium',
        launchOptions: { args: ['--use-angle=swiftshader', '--enable-unsafe-swiftshader', '--ignore-gpu-blocklist'] },
      },
    },
    {
      // No WebGL at all: the game must fall back to the static background and stay playable.
      name: 'no-webgl',
      testMatch: /fallback\.spec\.js/,
      use: {
        browserName: 'chromium',
        launchOptions: { args: ['--disable-webgl', '--disable-webgl2', '--disable-3d-apis'] },
      },
    },
  ],
});
