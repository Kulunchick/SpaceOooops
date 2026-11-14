// Ticket 10, without WebGL: the settings control still works and a full Mission completes with reduced motion.
import { test, expect } from '@playwright/test';
import { collectErrors, flyCropWatch } from './helpers.js';

test('a full Crop Watch Mission completes with reduced motion and no WebGL', async ({ page }) => {
  const errors = collectErrors(page);
  await page.emulateMedia({ reducedMotion: 'reduce' });
  await flyCropWatch(page);
  await expect(page.locator('.scene-layer')).toHaveAttribute('data-scene', 'fallback');
  expect(errors).toEqual([]);
});

test('the quality choice is saved even without a 3D scene', async ({ page }) => {
  await page.goto('./');
  await page.getByRole('button', { name: 'Sound and graphics settings' }).click();
  await page.locator('#settings-quality-low').click();
  await expect(page.locator('#settings-quality-low')).toHaveAttribute('aria-pressed', 'true');
  await page.reload();
  await page.getByRole('button', { name: 'Sound and graphics settings' }).click();
  await expect(page.locator('#settings-quality-low')).toHaveAttribute('aria-pressed', 'true');
  expect(await page.evaluate(() => localStorage.getItem('spaceooops.settings.v1'))).toContain('quality=low');
});
