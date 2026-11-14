// Without WebGL: a static background replaces the 3D scene and the game stays fully playable.
import { test, expect } from '@playwright/test';
import { collectErrors, flyCropWatch } from './helpers.js';

test('the title shows the static background instead of the canvas', async ({ page }) => {
  const errors = collectErrors(page);
  await page.goto('./');
  await expect(page.getByRole('heading', { name: 'SpaceOooops' })).toBeVisible();
  await expect(page.locator('.scene-layer')).toHaveAttribute('data-scene', 'fallback');
  await expect(page.locator('.scene-fallback')).toBeVisible();
  await expect(page.locator('canvas.scene-canvas')).toBeHidden();
  expect(errors).toEqual([]);
});

test('Start leads on to team setup', async ({ page }) => {
  await page.goto('./');
  await page.getByRole('button', { name: 'Start' }).click();
  await expect(page.getByRole('button', { name: 'Choose a Scenario' })).toBeVisible();
});

test('a full Crop Watch Mission completes without WebGL', async ({ page }) => {
  const errors = collectErrors(page);
  await flyCropWatch(page);
  await expect(page.locator('.scene-layer')).toHaveAttribute('data-scene', 'fallback');
  expect(errors).toEqual([]);
});
