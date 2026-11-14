// Design screen without WebGL: the stage folds away, the Role panel takes the width, and every choice still works.
import { test, expect } from '@playwright/test';
import { collectErrors, CROP_WATCH_SHARE } from './helpers.js';

test('without WebGL the Design screen works as before', async ({ page }) => {
  const errors = collectErrors(page);
  await page.goto(`./#share=${CROP_WATCH_SHARE}`);
  await expect(page.locator('.scene-layer')).toHaveAttribute('data-scene', 'fallback');
  await expect(page.locator('.hangar-stage')).toBeHidden();
  const panel = page.locator('.role-panel');
  await expect(panel).toBeVisible();
  expect((await panel.boundingBox()).width).toBeGreaterThan(900);

  for (const label of ['Budget', 'Mass', 'Power', 'Data', 'Lifetime']) {
    await expect(page.locator('.gauge-strip .gauge-label', { hasText: label })).toBeVisible();
  }
  const card = page.getByRole('button', { name: /Wide-field camera/ });
  await card.click();
  await expect(card).toHaveAttribute('aria-pressed', 'true');
  await card.click();
  await expect(card).toHaveAttribute('aria-pressed', 'false');

  for (const role of ['Spacecraft', 'Launch & Orbit', 'Budget & Comms', 'Science']) {
    await page.getByRole('tab', { name: new RegExp(role) }).click();
    await expect(page.getByRole('tab', { name: new RegExp(role) })).toHaveAttribute('aria-selected', 'true');
  }
  await page.getByRole('button', { name: 'Launch Review' }).click();
  await expect(page.getByRole('button', { name: 'Launch', exact: true })).toBeVisible();
  expect(errors).toEqual([]);
});
