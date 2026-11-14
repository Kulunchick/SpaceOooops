// With WebGL: the 3D scene starts behind the title panel, with no console errors.
import { test, expect } from '@playwright/test';
import { collectErrors, flyCropWatch } from './helpers.js';

test('title loads over a running 3D scene with no console errors', async ({ page }) => {
  const errors = collectErrors(page);
  await page.goto('./');
  await expect(page.getByRole('heading', { name: 'SpaceOooops' })).toBeVisible();
  await expect(page.locator('.scene-layer')).toHaveAttribute('data-scene', 'live');
  const canvas = page.locator('canvas.scene-canvas');
  await expect(canvas).toBeVisible();
  const size = await canvas.evaluate((c) => ({ w: c.width, h: c.height }));
  expect(size.w).toBeGreaterThan(0);
  expect(size.h).toBeGreaterThan(0);
  expect(errors).toEqual([]);
});

test('a full Mission plays over the 3D scene with no console errors', async ({ page }) => {
  const errors = collectErrors(page);
  await flyCropWatch(page);
  await expect(page.locator('.scene-layer')).toHaveAttribute('data-scene', 'live');
  expect(errors).toEqual([]);
});
