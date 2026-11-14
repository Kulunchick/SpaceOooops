// Without WebGL, Launch Review and Launch work as before the overhaul: the plain review page, and Launch goes
// straight to the Run with no launch sequence.
import { test, expect } from '@playwright/test';
import { collectErrors, CROP_WATCH_SHARE } from './helpers.js';

test('Launch Review and Launch work without WebGL', async ({ page }) => {
  const errors = collectErrors(page);
  await page.goto(`./#share=${CROP_WATCH_SHARE}`);
  await page.getByRole('button', { name: 'Launch Review' }).click();
  await expect(page.locator('.scene-layer')).toHaveAttribute('data-scene', 'fallback');
  await expect(page.getByText('Hard constraints broken: 0')).toBeVisible();
  await page.screenshot({ path: 'test-results/pad-fallback.png' });
  await page.getByRole('button', { name: 'Launch', exact: true }).click();
  await expect(page.getByRole('button', { name: /Skip/ }).first()).toBeVisible();
  await expect(page.getByRole('button', { name: 'Skip launch' })).toBeHidden();
  expect(errors).toEqual([]);
});
