// Ticket 10, with WebGL: quality toggle and its default, reduced motion, and the first panel before the scene.
// The canvas exposes data-quality and data-reduced-motion; the settings panel is #settings-panel.
import { test, expect } from '@playwright/test';
import { collectErrors, flyCropWatch, CROP_WATCH_SHARE } from './helpers.js';

const canvas = (page) => page.locator('canvas.scene-canvas');
const SETTINGS_KEY = 'spaceooops.settings.v1';

async function openSettings(page) {
  if (!(await page.locator('#settings-panel').isVisible())) await page.getByRole('button', { name: 'Sound settings' }).click();
  await expect(page.locator('#settings-panel')).toBeVisible();
}

/** Starts every page with the given stored settings text (before the app reads it). */
async function storeSettings(page, text) {
  await page.addInitScript(([key, value]) => {
    if (!sessionStorage.getItem('seeded')) {
      localStorage.setItem(key, value);
      sessionStorage.setItem('seeded', '1');
    }
  }, [SETTINGS_KEY, text]);
}

test('the first panel is on screen before the 3D scene has loaded, then the scene fades in', async ({ page }) => {
  // Hold the renderer's code back so the "scene still loading" state lasts long enough to look at.
  await page.route('**/js/bridge/index.js', async (route) => {
    await new Promise((r) => setTimeout(r, 2500));
    await route.continue();
  });
  await page.goto('./');
  await expect(page.getByRole('heading', { name: 'SpaceOooops' })).toBeVisible();
  await expect(page.getByRole('button', { name: 'Start' })).toBeVisible();
  await expect(page.locator('.scene-layer')).toHaveAttribute('data-scene', 'loading');
  expect(await canvas(page).evaluate((c) => getComputedStyle(c).opacity)).toBe('0');
  await expect(page.locator('.scene-layer')).toHaveAttribute('data-scene', 'live', { timeout: 20_000 });
  await expect.poll(() => canvas(page).evaluate((c) => getComputedStyle(c).opacity)).toBe('1');
});

test('the quality toggle switches the scene and the choice survives a reload', async ({ page }) => {
  const errors = collectErrors(page);
  await storeSettings(page, 'quality=high');
  await page.goto('./');
  await expect(canvas(page)).toHaveAttribute('data-quality', 'high');
  await openSettings(page);
  await expect(page.locator('#settings-quality-high')).toHaveAttribute('aria-pressed', 'true');
  await page.locator('#settings-quality-low').click();
  await expect(page.locator('#settings-quality-low')).toHaveAttribute('aria-pressed', 'true');
  await expect(canvas(page)).toHaveAttribute('data-quality', 'low');
  // The scene still works after the rebuild: the camera can fly to the next Shot.
  await page.getByRole('button', { name: 'Start' }).click();
  await expect(page.getByRole('button', { name: 'Choose a Scenario' })).toBeVisible();
  await page.reload();
  await expect(canvas(page)).toHaveAttribute('data-quality', 'low');
  await openSettings(page);
  await expect(page.locator('#settings-quality-low')).toHaveAttribute('aria-pressed', 'true');
  await page.locator('#settings-quality-high').click();
  await expect(canvas(page)).toHaveAttribute('data-quality', 'high');
  await page.reload();
  await expect(canvas(page)).toHaveAttribute('data-quality', 'high');
  expect(errors).toEqual([]);
});

test('a weak device starts on Low, but a saved choice is never overridden', async ({ page, context }) => {
  await context.addInitScript(() => Object.defineProperty(navigator, 'deviceMemory', { get: () => 1 }));
  await page.goto('./');
  await expect(canvas(page)).toHaveAttribute('data-quality', 'low');
  await openSettings(page);
  await expect(page.locator('#settings-quality-low')).toHaveAttribute('aria-pressed', 'true');
  // Choosing High on this device is remembered and wins over the rule on the next visit.
  await page.locator('#settings-quality-high').click();
  await expect(canvas(page)).toHaveAttribute('data-quality', 'high');
  await page.reload();
  await expect(canvas(page)).toHaveAttribute('data-quality', 'high');
});

test('a quality change during the launch animation waits for it, and nothing breaks', async ({ page }) => {
  const errors = collectErrors(page);
  await storeSettings(page, 'quality=high');
  await page.goto(`./#share=${CROP_WATCH_SHARE}`);
  await page.getByRole('button', { name: 'Launch Review' }).click();
  await expect(canvas(page)).toHaveAttribute('data-shot', 'pad', { timeout: 20_000 });
  await page.getByRole('button', { name: 'Launch', exact: true }).click();
  await expect(canvas(page)).toHaveAttribute('data-pad-phase', 'countdown');
  // A real click would end the launch (a click anywhere does), so press the control from script: the change
  // arrives while the sequence is playing.
  await page.locator('#settings-quality-low').evaluate((el) => el.click());
  // The rebuild is held back while the sequence plays: the launch is not restarted or cut.
  await expect(page.locator('#settings-quality-low')).toHaveAttribute('aria-pressed', 'true');
  await page.waitForTimeout(500);
  await expect(canvas(page)).toHaveAttribute('data-quality', 'high');
  await expect(canvas(page)).not.toHaveAttribute('data-pad-phase', 'idle');
  await page.getByRole('button', { name: 'Skip launch' }).click();
  await expect(page.getByRole('heading', { name: 'Launch Review' })).toHaveCount(0, { timeout: 5000 });
  await expect(canvas(page)).toHaveAttribute('data-quality', 'low');
  await expect(canvas(page)).toHaveAttribute('data-orbit', /./, { timeout: 10_000 });
  expect(errors).toEqual([]);
});

test('reduced motion: the scene knows, the camera cuts, and a full Mission completes', async ({ page }) => {
  const errors = collectErrors(page);
  await page.emulateMedia({ reducedMotion: 'reduce' });
  await page.goto('./');
  await expect(canvas(page)).toHaveAttribute('data-reduced-motion', 'true');
  await page.getByRole('button', { name: 'Start' }).click();
  await expect(page.getByRole('button', { name: 'Choose a Scenario' })).toBeVisible();
  await expect(canvas(page)).toHaveAttribute('data-flying', 'false');
  await flyCropWatch(page);
  await expect(page.locator('.scene-layer')).toHaveAttribute('data-scene', 'live');
  expect(errors).toEqual([]);
});

test('switching reduced motion on while the game is open reaches the scene', async ({ page }) => {
  await page.goto('./');
  await expect(canvas(page)).toHaveAttribute('data-reduced-motion', 'false');
  await page.emulateMedia({ reducedMotion: 'reduce' });
  await expect(canvas(page)).toHaveAttribute('data-reduced-motion', 'true');
});

test('a quality change by clicking during the launch ends it and the Run carries on at the new level', async ({ page }) => {
  const errors = collectErrors(page);
  await storeSettings(page, 'quality=high');
  await page.goto(`./#share=${CROP_WATCH_SHARE}`);
  await page.getByRole('button', { name: 'Launch Review' }).click();
  await expect(canvas(page)).toHaveAttribute('data-shot', 'pad', { timeout: 20_000 });
  await page.getByRole('button', { name: 'Launch', exact: true }).click();
  await expect(canvas(page)).toHaveAttribute('data-pad-armed', 'true');
  await openSettings(page);
  await page.locator('#settings-quality-low').click();
  await expect(canvas(page)).toHaveAttribute('data-quality', 'low');
  await expect(canvas(page)).toHaveAttribute('data-orbit', /./, { timeout: 15_000 });
  expect(errors).toEqual([]);
});
