// Scenario select and Brief without WebGL: the same information over the static background.
import { test, expect } from '@playwright/test';
import { collectErrors } from './helpers.js';

test('Scenario select and Brief work over the static background', async ({ page }) => {
  const errors = collectErrors(page);
  await page.goto('./');
  await page.getByRole('button', { name: 'Start' }).click();
  await page.getByRole('button', { name: 'Choose a Scenario' }).click();
  await expect(page.locator('.scenario-card')).toHaveCount(3);
  await expect(page.locator('.scene-layer')).toHaveAttribute('data-scene', 'fallback');
  await expect(page.locator('.scene-fallback')).toBeVisible();
  for (const img of await page.locator('.scenario-card img').all()) {
    await expect.poll(() => img.evaluate((i) => i.complete && i.naturalWidth > 0)).toBe(true);
  }
  await page.locator('.scenario-card', { hasText: 'Space Weather Sentinel' }).getByRole('button', { name: 'Select' }).click();
  await expect(page.getByRole('heading', { name: 'Space Weather Sentinel' })).toBeVisible();
  await expect(page.getByText('Key constraints')).toBeVisible();
  await page.screenshot({ path: 'test-results/brief-fallback.png' });
  await page.getByRole('button', { name: 'Start Design' }).click();
  await expect(page.getByRole('button', { name: 'Launch Review' })).toBeVisible();
  expect(errors).toEqual([]);
});
