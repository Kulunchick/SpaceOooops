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
