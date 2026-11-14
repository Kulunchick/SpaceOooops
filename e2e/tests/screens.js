// History, team setup and Sources (ticket 09). Registered by screens.webgl.spec.js and screens.fallback.spec.js,
// so the same checks run with and without WebGL.
import { test, expect } from '@playwright/test';
import { readFileSync } from 'node:fs';
import { collectErrors, CROP_WATCH_SHARE, flyCropWatch } from './helpers.js';

/** Names of every [[asset]] in data/credits.toml. */
function creditedNames() {
  const text = readFileSync(new URL('../../data/credits.toml', import.meta.url), 'utf8');
  return [...text.matchAll(/^name = "(.*)"$/gm)].map((m) => m[1]);
}

export function registerScreenTests(sceneMode) {
  test('Sources is a skinned panel listing every credited asset and the AI statement', async ({ page }) => {
    const errors = collectErrors(page);
    await page.goto('./');
    await page.getByRole('button', { name: 'Sources' }).click();
    const panel = page.locator('.sources-screen');
    await expect(panel).toBeVisible();
    await expect(panel).toHaveClass(/lp-panel/);
    await expect(page.getByRole('heading', { name: 'Sources and credits' })).toBeVisible();
    const names = creditedNames();
    expect(names.length).toBeGreaterThan(2);
    for (const name of names) await expect(panel.locator('.credit-name', { hasText: name }).first()).toBeVisible();
    await expect(page.locator('.credit')).toHaveCount(names.length);
    await expect(page.locator('.ai-assets')).toContainText('No image, music or sound effect in this game was made with AI tools.');
    await expect(page.locator('.scene-layer')).toHaveAttribute('data-scene', sceneMode);
    await expect(page.locator('.scene-layer')).not.toHaveClass(/dimmed/);
    await page.screenshot({ path: `test-results/sources-${sceneMode}.png`, fullPage: true });
    await page.getByRole('button', { name: 'Back' }).click();
    await expect(page.getByRole('heading', { name: 'SpaceOooops' })).toBeVisible();
    expect(errors).toEqual([]);
  });

  test('team setup is a skinned panel and still works', async ({ page }) => {
    const errors = collectErrors(page);
    await page.goto('./');
    await page.getByRole('button', { name: 'Start' }).click();
    const panel = page.locator('.team-setup');
    await expect(panel).toHaveClass(/lp-panel/);
    await page.getByRole('button', { name: /^Team/ }).click();
    await expect(page.getByLabel('Number of players')).toBeVisible();
    await page.screenshot({ path: `test-results/team-${sceneMode}.png`, fullPage: true });
    await page.getByRole('button', { name: 'Choose a Scenario' }).click();
    await expect(page.locator('.scene-layer')).toHaveAttribute('data-scene', sceneMode);
    expect(errors).toEqual([]);
  });

  test('History lists a Run and "Load this Design" opens it on the Design screen', async ({ page }) => {
    const errors = collectErrors(page);
    await flyCropWatch(page);
    await page.goto('./');
    await page.reload();
    await page.getByRole('button', { name: 'History' }).click();
    const panel = page.locator('.history-screen');
    await expect(panel).toHaveClass(/lp-panel/);
    const item = page.locator('.history-item').first();
    await expect(item).toContainText('Crop Watch');
    await page.screenshot({ path: `test-results/history-${sceneMode}.png`, fullPage: true });
    await item.getByRole('button', { name: 'Load this Design' }).click();
    await expect(page.getByRole('button', { name: 'Launch Review' })).toBeVisible();
    await expect(page.getByRole('button', { name: 'Share design' })).toBeVisible();
    expect(errors).toEqual([]);
  });

  test('History can be cleared', async ({ page }) => {
    await flyCropWatch(page);
    await page.goto('./');
    await page.reload();
    await page.getByRole('button', { name: 'History' }).click();
    await page.getByRole('button', { name: 'Clear history' }).click();
    await expect(page.getByText('No Runs yet.')).toBeVisible();
  });

  test('a share link opens the Design with its Scenario', async ({ page }) => {
    const errors = collectErrors(page);
    await page.goto(`./#share=${CROP_WATCH_SHARE}`);
    await expect(page.getByRole('button', { name: 'Launch Review' })).toBeVisible();
    await expect(page.getByRole('button', { name: 'Share design' })).toBeVisible();
    expect(errors).toEqual([]);
  });
}
