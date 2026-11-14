// Without WebGL the Debrief works as it did before the overhaul (low-poly ticket 08): the plain page with every
// section, the verdict, and "Modify design" back to the Design with the choices intact.
import { test, expect } from '@playwright/test';
import { collectErrors, flyCropWatch } from './helpers.js';

test('the Debrief keeps every section and Modify design returns to the Design without WebGL', async ({ page }) => {
  const errors = collectErrors(page);
  await flyCropWatch(page);
  await expect(page.locator('.scene-layer')).toHaveAttribute('data-scene', 'fallback');
  const panel = page.locator('.screen.debrief');
  for (const heading of ['Why it went this way', 'Score', 'Key numbers']) {
    await expect(panel.getByRole('heading', { name: heading })).toBeVisible();
  }
  // The page is the plain document: no inner scroll box.
  expect(await panel.evaluate((el) => getComputedStyle(el).overflowY)).toBe('visible');
  await panel.getByRole('button', { name: 'Modify design' }).click();
  await expect(page.getByRole('button', { name: 'Launch Review' })).toBeVisible();
  expect(errors).toEqual([]);
});
