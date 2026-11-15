// Scenario select and Brief over the 3D scene: markers, fly-to, skipping the flight, reduced motion.
import { test, expect } from '@playwright/test';
import { collectErrors, chooseReducedMotion, catalogScenarios } from './helpers.js';

async function openScenarioSelect(page) {
  await page.goto('./');
  await page.getByRole('button', { name: 'Start' }).click();
  await page.getByRole('button', { name: 'Choose a Scenario' }).click();
  await expect(page.getByRole('heading', { name: 'Choose a Scenario' })).toBeVisible();
}

test('Scenario select shows every card with its illustration over the clear scene', async ({ page }) => {
  const errors = collectErrors(page);
  await openScenarioSelect(page);
  const catalog = catalogScenarios();
  await expect(page.locator('.scenario-card')).toHaveCount(catalog.length);
  await expect(page.locator('.scenario-card .illustration-slot img')).toHaveCount(catalog.length);
  for (const { name, country } of catalog) {
    await expect(page.locator('.scenario-card', { hasText: name }).locator('.scenario-country')).toContainText(country);
  }
  for (const img of await page.locator('.scenario-card img').all()) {
    await expect.poll(() => img.evaluate((i) => i.complete && i.naturalWidth > 0)).toBe(true);
  }
  for (const name of ['Crop Watch', 'Connect the Villages', 'Space Weather Sentinel']) {
    const card = page.locator('.scenario-card', { hasText: name });
    await expect(card.locator('.difficulty')).toBeVisible();
    await expect(card.locator('.constraints li').first()).toBeVisible();
    await expect(card).toContainText('Budget');
  }
  await expect(page.locator('.scene-layer')).toHaveAttribute('data-scene', 'live');
  await expect(page.locator('.scene-layer')).not.toHaveClass(/dimmed/);
  await page.waitForTimeout(2500);
  await page.screenshot({ path: 'test-results/scenario-select.png' });
  expect(errors).toEqual([]);
});

test('hovering or focusing a card turns the globe without errors', async ({ page }) => {
  const errors = collectErrors(page);
  await openScenarioSelect(page);
  const cards = page.locator('.scenario-card');
  for (let i = 0; i < await cards.count(); i++) {
    await cards.nth(i).hover();
    await page.waitForTimeout(150);
  }
  await page.mouse.move(5, 5);
  await cards.first().getByRole('button', { name: 'Select' }).focus();
  await page.waitForTimeout(150);
  await expect(page.locator('.scene-layer')).toHaveAttribute('data-scene', 'live');
  expect(errors).toEqual([]);
});

test('hovering the Brazil card turns the globe to its pin and highlights Brazil only', async ({ page }) => {
  const errors = collectErrors(page);
  await openScenarioSelect(page);
  const canvas = page.locator('canvas.scene-canvas');
  await expect(canvas).toHaveAttribute('data-highlight', 'angola');
  await page.locator('.scenario-card', { hasText: 'Amazon Watch' }).hover();
  await expect(canvas).toHaveAttribute('data-highlight', 'brazil');
  await expect(canvas).toHaveAttribute('data-shot-at', '-7,-55.4');
  await expect(canvas).toHaveAttribute('data-flying', 'false', { timeout: 15000 });
  await page.waitForTimeout(500);
  await page.screenshot({ path: 'test-results/scenario-select-brazil.png' });
  await page.locator('.scenario-card', { hasText: 'Amazon Watch' }).getByRole('button', { name: 'Select' }).click();
  await expect(page.getByRole('heading', { name: 'Amazon Watch' })).toBeVisible();
  await expect(canvas).toHaveAttribute('data-highlight', 'brazil');
  await expect(canvas).toHaveAttribute('data-flying', 'false', { timeout: 15000 });
  await page.waitForTimeout(500);
  await page.screenshot({ path: 'test-results/brief-brazil.png' });
  expect(errors).toEqual([]);
});

test('hovering the India card turns the globe to its pin and highlights India only', async ({ page }) => {
  const errors = collectErrors(page);
  await openScenarioSelect(page);
  const canvas = page.locator('canvas.scene-canvas');
  await page.locator('.scenario-card', { hasText: 'Cyclone Watch' }).hover();
  await expect(canvas).toHaveAttribute('data-highlight', 'india');
  await expect(canvas).toHaveAttribute('data-shot-at', '20.2,85.7');
  await expect(canvas).toHaveAttribute('data-flying', 'false', { timeout: 15000 });
  await page.waitForTimeout(500);
  await page.screenshot({ path: 'test-results/scenario-select-india.png' });
  await page.locator('.scenario-card', { hasText: 'Cyclone Watch' }).getByRole('button', { name: 'Select' }).click();
  await expect(page.getByRole('heading', { name: 'Cyclone Watch' })).toBeVisible();
  await expect(canvas).toHaveAttribute('data-highlight', 'india');
  await expect(canvas).toHaveAttribute('data-flying', 'false', { timeout: 15000 });
  await page.waitForTimeout(500);
  await page.screenshot({ path: 'test-results/brief-india.png' });
test('hovering the Japan card turns the globe to its pin and highlights Japan only', async ({ page }) => {
  const errors = collectErrors(page);
  await openScenarioSelect(page);
  const canvas = page.locator('canvas.scene-canvas');
  await page.locator('.scenario-card', { hasText: 'Disaster Response' }).hover();
  await expect(canvas).toHaveAttribute('data-highlight', 'japan');
  await expect(canvas).toHaveAttribute('data-shot-at', '38.3,140.7');
  await expect(canvas).toHaveAttribute('data-flying', 'false', { timeout: 15000 });
  await page.waitForTimeout(500);
  await page.screenshot({ path: 'test-results/scenario-select-japan.png' });
  await page.locator('.scenario-card', { hasText: 'Disaster Response' }).getByRole('button', { name: 'Select' }).click();
  await expect(page.getByRole('heading', { name: 'Disaster Response' })).toBeVisible();
  await expect(canvas).toHaveAttribute('data-highlight', 'japan');
  await expect(canvas).toHaveAttribute('data-flying', 'false', { timeout: 15000 });
  await page.waitForTimeout(500);
  await page.screenshot({ path: 'test-results/brief-japan.png' });
  expect(errors).toEqual([]);
});

test('hovering the Argentina card turns the globe to far-south Patagonia and highlights Argentina only', async ({ page }) => {
  const errors = collectErrors(page);
  await openScenarioSelect(page);
  const canvas = page.locator('canvas.scene-canvas');
  await page.locator('.scenario-card', { hasText: 'Glacier Watch' }).hover();
  await expect(canvas).toHaveAttribute('data-highlight', 'argentina');
  await expect(canvas).toHaveAttribute('data-shot-at', '-50.5,-73');
  await expect(canvas).toHaveAttribute('data-flying', 'false', { timeout: 15000 });
  await page.waitForTimeout(500);
  await page.screenshot({ path: 'test-results/scenario-select-argentina.png' });
  await page.locator('.scenario-card', { hasText: 'Glacier Watch' }).getByRole('button', { name: 'Select' }).click();
  await expect(page.getByRole('heading', { name: 'Glacier Watch' })).toBeVisible();
  await expect(canvas).toHaveAttribute('data-highlight', 'argentina');
  await expect(canvas).toHaveAttribute('data-flying', 'false', { timeout: 15000 });
  await page.waitForTimeout(500);
  await page.screenshot({ path: 'test-results/brief-argentina.png' });
  expect(errors).toEqual([]);
});

test('choosing a Scenario flies the camera and a click skips the flight', async ({ page }) => {
  const errors = collectErrors(page);
  await openScenarioSelect(page);
  const canvas = page.locator('canvas.scene-canvas');
  await page.waitForTimeout(2500);
  await expect(canvas).toHaveAttribute('data-flying', 'false');
  await page.locator('.scenario-card', { hasText: 'Crop Watch' }).getByRole('button', { name: 'Select' }).click();
  await expect(page.getByRole('heading', { name: 'Crop Watch' })).toBeVisible();
  await expect(canvas).toHaveAttribute('data-flying', 'true');
  await page.waitForTimeout(400);
  await page.mouse.click(900, 400);
  await expect(canvas).toHaveAttribute('data-flying', 'false', { timeout: 2000 });
  await page.screenshot({ path: 'test-results/brief.png' });
  await page.getByRole('button', { name: 'Start Design' }).click();
  // The Design is the hangar over a clear scene (ticket 02).
  await expect(page.locator('canvas.scene-canvas')).toHaveAttribute('data-shot', 'hangar');
  await expect(page.locator('.scene-layer')).not.toHaveClass(/dimmed/);
  expect(errors).toEqual([]);
});

test('the flight ends by itself', async ({ page }) => {
  await openScenarioSelect(page);
  await page.locator('.scenario-card', { hasText: 'Space Weather Sentinel' }).getByRole('button', { name: 'Select' }).click();
  await expect(page.locator('canvas.scene-canvas')).toHaveAttribute('data-flying', 'false', { timeout: 15000 });
});

test('with reduced motion the camera cuts instead of flying', async ({ page }) => {
  await chooseReducedMotion(page);
  await openScenarioSelect(page);
  const canvas = page.locator('canvas.scene-canvas');
  await page.locator('.scenario-card', { hasText: 'Connect the Villages' }).getByRole('button', { name: 'Select' }).click();
  await expect(page.getByRole('heading', { name: 'Connect the Villages' })).toBeVisible();
  for (let i = 0; i < 10; i++) {
    await expect(canvas).toHaveAttribute('data-flying', 'false');
    await page.waitForTimeout(50);
  }
});
