// Design screen over the 3D hangar: the spacecraft is assembled from the Design, follows every change, and turns
// when dragged on the stage (and only there). The canvas exposes data-shot / data-parts / data-yaw for these checks.
import { test, expect } from '@playwright/test';
import { collectErrors, chooseReducedMotion, CROP_WATCH_SHARE } from './helpers.js';

const CROP_WATCH_PARTS = [
  'cubesat-6u', 'solar-deployable', 'comms-s-band', 'reaction-wheels', 'battery-m', 'computer-obc', 'multispectral-imager',
];

async function partIds(page) {
  return ((await page.locator('canvas.scene-canvas').getAttribute('data-parts')) ?? '').split(' ').filter(Boolean);
}

test('a share link lands on the Design with the spacecraft assembled in the hangar', async ({ page }) => {
  const errors = collectErrors(page);
  await page.goto(`./#share=${CROP_WATCH_SHARE}`);
  await expect(page.getByRole('tab', { name: /Science/ })).toBeVisible();
  const canvas = page.locator('canvas.scene-canvas');
  await expect(page.locator('.scene-layer')).toHaveAttribute('data-scene', 'live');
  await expect(canvas).toHaveAttribute('data-shot', 'hangar', { timeout: 20_000 });
  await expect.poll(() => partIds(page)).toEqual(expect.arrayContaining(CROP_WATCH_PARTS));
  await expect(page.locator('.scene-layer')).not.toHaveClass(/dimmed/);
  expect(errors).toEqual([]);
});

test('adding and removing an Instrument adds and removes its part', async ({ page }) => {
  const errors = collectErrors(page);
  await page.goto(`./#share=${CROP_WATCH_SHARE}`);
  await expect(page.locator('canvas.scene-canvas')).toHaveAttribute('data-shot', 'hangar', { timeout: 20_000 });
  const card = page.getByRole('button', { name: /Wide-field camera/ });
  await card.click();
  await expect.poll(() => partIds(page)).toContain('wide-camera');
  await card.click();
  await expect.poll(() => partIds(page)).not.toContain('wide-camera');
  expect(errors).toEqual([]);
});

test('dragging on the stage turns the model; the panels stay clickable', async ({ page }) => {
  await chooseReducedMotion(page); // no idle turn, so only the drag moves it
  await page.goto(`./#share=${CROP_WATCH_SHARE}`);
  const canvas = page.locator('canvas.scene-canvas');
  await expect(canvas).toHaveAttribute('data-shot', 'hangar', { timeout: 20_000 });
  const yaw = async () => Number(await canvas.getAttribute('data-yaw'));
  await expect.poll(yaw).not.toBeNaN();
  const before = await yaw();

  const stage = await page.locator('.hangar-stage').boundingBox();
  const cx = stage.x + stage.width / 2;
  const cy = stage.y + stage.height / 2;
  await page.mouse.move(cx, cy);
  await page.mouse.down();
  await page.mouse.move(cx + 120, cy, { steps: 6 });
  await page.mouse.up();
  await expect.poll(yaw).toBeGreaterThan(before + 0.5);

  // A drag that starts on a panel does not turn the model, and the panel's controls still work.
  const turned = await yaw();
  const panel = await page.locator('.role-panel').boundingBox();
  await page.mouse.move(panel.x + 40, panel.y + panel.height - 20);
  await page.mouse.down();
  await page.mouse.move(panel.x + 200, panel.y + panel.height - 20, { steps: 6 });
  await page.mouse.up();
  expect(await yaw()).toBeCloseTo(turned, 2);
  await page.getByRole('tab', { name: /Launch & Orbit/ }).click();
  await expect(page.getByRole('tab', { name: /Launch & Orbit/ })).toHaveAttribute('aria-selected', 'true');
  await expect(page.locator('.design-screen')).toHaveAttribute('data-panel-side', 'right');
});

test('the Design layout keeps panels, stage and gauges usable at tablet width', async ({ page }) => {
  await page.setViewportSize({ width: 820, height: 1180 });
  await page.goto(`./#share=${CROP_WATCH_SHARE}`);
  for (const label of ['Budget', 'Mass', 'Power', 'Data', 'Lifetime']) {
    await expect(page.locator('.gauge-strip .gauge-label', { hasText: label })).toBeVisible();
  }
  const stage = await page.locator('.hangar-stage').boundingBox();
  expect(stage.height).toBeGreaterThan(200);
  await page.getByRole('tab', { name: /Spacecraft/ }).click();
  await page.getByRole('button', { name: /CubeSat 12U/ }).click();
  await expect.poll(() => partIds(page)).toContain('cubesat-12u');
  const width = await page.evaluate(() => document.documentElement.scrollWidth);
  expect(width).toBeLessThanOrEqual(820);
});
