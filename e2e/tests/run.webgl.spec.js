// The Run over the 3D orbit scene (low-poly ticket 03): the Design's spacecraft orbits Earth, its place comes only
// from the Run playback (pause, speed-up and skip stay in sync), and the panels keep the centre clear.
// The canvas exposes data-shot, data-orbit (Orbit id), data-orbit-angle (the angle drawn this frame),
// data-orbit-target (the latest Scene Spec's angle) and data-in-shadow.
import { test, expect } from '@playwright/test';
import { collectErrors, CROP_WATCH_SHARE } from './helpers.js';

const TAU = Math.PI * 2;

/**
 * Makes the Run's seed deterministic: the UI builds it from two Math.random() halves, so a constant k / 2^32
 * gives seed (k << 32) | k. Crop Watch with k = 28 asks its first question on day 3; with k = 34 it asks none.
 */
async function seedRuns(page, k) {
  await page.addInitScript((value) => {
    Math.random = () => value;
  }, (k + 0.5) / 4294967296);
}

async function launchCropWatch(page) {
  await page.goto(`./#share=${CROP_WATCH_SHARE}`);
  await page.getByRole('button', { name: 'Launch Review' }).click();
  await page.getByRole('button', { name: 'Launch', exact: true }).click();
  // The launch sequence on the pad (ticket 07) can be skipped.
  await page.getByRole('button', { name: 'Skip launch' }).click();
  await expect(page.locator('.run-screen')).toBeVisible();
  await expect(page.locator('canvas.scene-canvas')).toHaveAttribute('data-shot', 'orbit', { timeout: 20_000 });
}

async function angles(page) {
  const canvas = page.locator('canvas.scene-canvas');
  return {
    drawn: Number(await canvas.getAttribute('data-orbit-angle')),
    target: Number(await canvas.getAttribute('data-orbit-target')),
  };
}

/** Angle swept by the drawn spacecraft over `ms`, unwrapped from samples every 100 ms. */
async function sweep(page, ms) {
  let last = (await angles(page)).drawn;
  let total = 0;
  for (let t = 0; t < ms; t += 100) {
    await page.waitForTimeout(100);
    const now = (await angles(page)).drawn;
    total += (((now - last) % TAU) + TAU) % TAU;
    last = now;
  }
  return total;
}

function overlaps(a, b) {
  return a.x < b.x + b.width && b.x < a.x + a.width && a.y < b.y + b.height && b.y < a.y + a.height;
}

test('the Run shows the Design orbiting the 3D Earth instead of the SVG, with the panels at the edges', async ({ page }) => {
  const errors = collectErrors(page);
  await seedRuns(page, 34);
  await launchCropWatch(page);
  const canvas = page.locator('canvas.scene-canvas');
  await expect(canvas).toHaveAttribute('data-orbit', 'sso-600');
  await expect(page.locator('.scene-layer')).not.toHaveClass(/dimmed/);
  await expect(page.locator('.orbit-svg')).toBeHidden();
  // The caption with the Orbit's facts and the shadow state stays.
  await expect(page.locator('.orbit-caption')).toBeVisible();
  // The empty stage the orbit is framed in is not covered by the header, dashboards or event log.
  const stage = await page.locator('.run-stage').boundingBox();
  expect(stage.width).toBeGreaterThan(400);
  expect(stage.height).toBeGreaterThan(300);
  for (const panel of ['.run-head', '.dashboards', '.event-log']) {
    const box = await page.locator(`.run-screen ${panel}`).boundingBox();
    expect(overlaps(stage, box), `${panel} covers the stage`).toBe(false);
  }
  expect(errors).toEqual([]);
});

test('paused playback holds the spacecraft exactly where the playback is', async ({ page }) => {
  const errors = collectErrors(page);
  await seedRuns(page, 28);
  await launchCropWatch(page);
  // The playback waits before day 3 for the player's answer: playback 2 of 365 days.
  await expect(page.locator('.decision-prompt')).toBeVisible({ timeout: 20_000 });
  await page.waitForTimeout(500);
  const first = await angles(page);
  await page.waitForTimeout(1500);
  const later = await angles(page);
  expect(later.drawn).toBe(first.drawn);
  expect(later.drawn).toBe(later.target);
  // SSO: 14 * 90 / 96.7 visual turns over the whole Run, so 2 of 365 days is 0.449 rad (the SVG view's angle).
  expect(Math.abs(later.drawn - ((TAU * 14 * 90) / 96.7) * (2 / 365))).toBeLessThan(0.005);
  // Answering lets the playback, and the spacecraft, move on.
  await page.locator('.decision-option').first().click();
  await expect.poll(async () => (await angles(page)).drawn).not.toBe(later.drawn);
  expect(errors).toEqual([]);
});

test('speeding up the playback speeds up the spacecraft, and skipping leaves the orbit for the Debrief', async ({ page }) => {
  const errors = collectErrors(page);
  await seedRuns(page, 34);
  await launchCropWatch(page);
  await page.waitForTimeout(800);
  const slow = await sweep(page, 1500);
  await page.getByRole('button', { name: '4×' }).click();
  await page.waitForTimeout(300);
  const fast = await sweep(page, 1500);
  // About four times as fast (frame timing on a software renderer makes it rough).
  expect(fast / slow).toBeGreaterThan(2.5);
  expect(fast / slow).toBeLessThan(6);
  await page.getByRole('button', { name: /Skip/ }).click();
  await expect(page.locator('.verdict-title')).toBeVisible();
  // The Debrief has no orbit view: nothing keeps flying behind it.
  await expect(page.locator('canvas.scene-canvas')).toHaveAttribute('data-orbit', '');
  expect(errors).toEqual([]);
});
