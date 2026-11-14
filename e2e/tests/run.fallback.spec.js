// The Run without WebGL (low-poly ticket 03): today's SVG orbit view plays as before, laid out as a page.
import { test, expect } from '@playwright/test';
import { collectErrors, CROP_WATCH_SHARE } from './helpers.js';

test('without WebGL the Run draws the SVG orbit view and plays on', async ({ page }) => {
  const errors = collectErrors(page);
  await page.goto(`./#share=${CROP_WATCH_SHARE}`);
  await page.getByRole('button', { name: 'Launch Review' }).click();
  await page.getByRole('button', { name: 'Launch', exact: true }).click();
  await expect(page.locator('.scene-layer')).toHaveAttribute('data-scene', 'fallback');
  const craft = page.locator('.orbit-svg .spacecraft');
  await expect(page.locator('.orbit-svg')).toBeVisible();
  await expect(page.locator('.run-stage')).toBeHidden();
  // The SVG spacecraft moves with the playback (unless a question pauses it first).
  const before = await craft.getAttribute('transform');
  await expect
    .poll(async () => (await page.locator('.decision-prompt').isVisible()) || (await craft.getAttribute('transform')) !== before)
    .toBe(true);
  expect(errors).toEqual([]);
});
