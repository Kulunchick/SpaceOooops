// Random Events without WebGL (low-poly ticket 05): the SVG orbit view keeps today's event flash and caption.
import { test, expect } from '@playwright/test';
import { collectErrors, CROP_WATCH_SHARE } from './helpers.js';

test('without WebGL a solar flare still flashes on the SVG orbit view with its caption', async ({ page }) => {
  const errors = collectErrors(page);
  // Crop Watch, k = 28: a flare warning before day 3 (see effects.webgl.spec.js).
  await page.addInitScript((value) => {
    Math.random = () => value;
  }, 28.5 / 4294967296);
  await page.goto(`./#share=${CROP_WATCH_SHARE}`);
  await page.getByRole('button', { name: 'Launch Review' }).click();
  await page.getByRole('button', { name: 'Launch', exact: true }).click();
  await expect(page.locator('.scene-layer')).toHaveAttribute('data-scene', 'fallback');
  await page.getByRole('button', { name: /Enter safe mode/ }).click();
  await expect(page.locator('.orbit-svg .event-flash')).toBeVisible();
  await expect(page.locator('.orbit-event.orbit-event-live')).toContainText(
    'Day 3: ☀ A solar flare hit. The spacecraft switched off its Instruments and waited in safe mode for 2 days.',
  );
  expect(errors).toEqual([]);
});

test('without WebGL a new Random Event is text only: the caption shows and no scene effect is drawn', async ({ page }) => {
  const errors = collectErrors(page);
  // Crop Watch, k = 50: three link-interference events and no question (day 113 is the first).
  await page.addInitScript((value) => {
    Math.random = () => value;
  }, 50.5 / 4294967296);
  await page.goto(`./#share=${CROP_WATCH_SHARE}`);
  await page.getByRole('button', { name: 'Launch Review' }).click();
  await page.getByRole('button', { name: 'Launch', exact: true }).click();
  await expect(page.locator('.scene-layer')).toHaveAttribute('data-scene', 'fallback');
  await page.getByRole('button', { name: '4×' }).click();
  await expect(page.locator('.orbit-event.orbit-event-live')).toContainText('Day 113:', { timeout: 30_000 });
  await expect(page.locator('canvas.scene-canvas')).not.toHaveAttribute('data-effects', /linkInterference/);
  await expect(page.locator('canvas.scene-canvas')).not.toHaveAttribute('data-event-fx', /./);
  expect(errors).toEqual([]);
});
