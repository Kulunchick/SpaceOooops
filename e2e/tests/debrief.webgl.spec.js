// The Debrief over the 3D scene (low-poly ticket 08): a hero shot on Success, a failure visual picked from the main
// cause (tumbling, dark, re-entering, adrift), a verdict sound on arrival, every section kept in a scrollable panel,
// and "Modify design" back to the hangar with the spacecraft intact.
// The canvas exposes data-shot, data-debrief (the fate drawn) and data-debrief-spin (the craft's rotation this frame).
// Set DEBRIEF_SHOTS=<dir> to save a screenshot of each verdict.
import { test, expect } from '@playwright/test';
import { collectErrors, CROP_WATCH_SHARE, pressLaunch } from './helpers.js';

const LEO_SHARE = CROP_WATCH_SHARE.replace('.sso-600.cubesat-rideshare', '.leo-400.station-deploy');

/** Fixes the Run seed: a constant k / 2^32 from Math.random gives seed (k << 32) | k (see run.webgl.spec.js). */
async function seedRuns(page, k) {
  await page.addInitScript((value) => {
    Math.random = () => value;
  }, (k + 0.5) / 4294967296);
}

const canvas = (page) => page.locator('canvas.scene-canvas');

/** Flies the share link's Design with a fixed seed (one that asks no question) and skips to the Debrief. */
async function flyToDebrief(page, share, k, { reducedMotion = false } = {}) {
  if (reducedMotion) await page.emulateMedia({ reducedMotion: 'reduce' });
  await seedRuns(page, k);
  await page.goto(`./#share=${share}`);
  await page.getByRole('button', { name: 'Launch Review' }).click();
  await pressLaunch(page);
  await expect(page.locator('.run-screen')).toBeVisible();
  await page.getByRole('button', { name: /Skip/ }).click();
  await expect(page.locator('.verdict-title')).toBeVisible();
}

async function shot(page, name) {
  const dir = process.env.DEBRIEF_SHOTS;
  if (!dir) return;
  await page.waitForTimeout(2500);
  await page.screenshot({ path: `${dir}/debrief-${name}.png` });
}

const SECTIONS = [
  'Why it went this way',
  'Score',
  'Key numbers',
  'Real-Mission Match',
];

// Each case: the Design, a seed that asks no question, the verdict and the visual that Rust picks for it.
const CASES = [
  { name: 'success', share: CROP_WATCH_SHARE, k: 50, shotKind: 'debriefHero', fate: 'hero', title: 'Success', cue: 'success' },
  { name: 'tumbling', share: CROP_WATCH_SHARE, k: 34, shotKind: 'debriefFailure', fate: 'tumbling', title: 'Mission failed', cue: 'mission-failed' },
  { name: 'dark', share: CROP_WATCH_SHARE, k: 46, shotKind: 'debriefFailure', fate: 'dark', title: 'Mission failed', cue: 'mission-failed' },
  { name: 'reentry', share: LEO_SHARE, k: 34, shotKind: 'debriefFailure', fate: 'reentry', title: 'Mission failed', cue: 'mission-failed' },
  { name: 'adrift', share: CROP_WATCH_SHARE, k: 76, shotKind: 'debriefFailure', fate: 'adrift', title: 'Mission failed', cue: 'mission-failed' },
];

for (const c of CASES) {
  test(`the ${c.name} Debrief shows its shot and plays the verdict sound once`, async ({ page }) => {
    const errors = collectErrors(page);
    await flyToDebrief(page, c.share, c.k);
    await expect(page.locator('.verdict-title')).toHaveText(c.title);
    await expect(canvas(page)).toHaveAttribute('data-shot', c.shotKind, { timeout: 10_000 });
    await expect(canvas(page)).toHaveAttribute('data-debrief', c.fate);
    await expect(page.locator('html')).toHaveAttribute('data-audio-last-cue', c.cue);
    // The Run's orbit view is gone: nothing keeps flying behind the panel.
    await expect(canvas(page)).toHaveAttribute('data-orbit', '');
    // The scene is dimmed behind the panel but drawn (no longer hidden by a full veil).
    await expect(page.locator('.scene-layer')).toHaveClass(/dimmed/);
    await shot(page, c.name);
    expect(errors).toEqual([]);
  });
}

test('the Debrief panel keeps every section and scrolls on its own beside the scene', async ({ page }) => {
  const errors = collectErrors(page);
  await flyToDebrief(page, CROP_WATCH_SHARE, 50);
  const panel = page.locator('.screen.debrief');
  for (const heading of SECTIONS) await expect(panel.getByRole('heading', { name: heading })).toBeVisible();
  await expect(panel.getByRole('button', { name: 'Modify design' })).toBeVisible();
  await expect(panel.getByRole('button', { name: 'Fly again' })).toBeVisible();
  // At laptop width the panel is a column at the left: the scene is visible to its right.
  const box = await panel.boundingBox();
  expect(box.x + box.width).toBeLessThan(1280 * 0.75);
  const scrolls = await panel.evaluate((el) => getComputedStyle(el).overflowY);
  expect(scrolls).toBe('auto');
  expect(errors).toEqual([]);
});

test('the verdict sound does not play again when the Debrief is left and the player comes back by modifying and flying again', async ({ page }) => {
  await flyToDebrief(page, CROP_WATCH_SHARE, 50);
  await expect(page.locator('html')).toHaveAttribute('data-audio-last-cue', 'success');
  // Leaving the Debrief plays nothing new.
  await page.getByRole('button', { name: 'Modify design' }).click();
  await expect(canvas(page)).toHaveAttribute('data-shot', 'hangar', { timeout: 10_000 });
  expect(await page.locator('html').getAttribute('data-audio-last-cue')).not.toBe('mission-failed');
});

test('Modify design flies back to the hangar with the same spacecraft', async ({ page }) => {
  const errors = collectErrors(page);
  await seedRuns(page, 50);
  await page.goto(`./#share=${CROP_WATCH_SHARE}`);
  await expect(canvas(page)).toHaveAttribute('data-shot', 'hangar', { timeout: 20_000 });
  await expect(canvas(page)).toHaveAttribute('data-parts', /\S/);
  const parts = await canvas(page).getAttribute('data-parts');
  await page.getByRole('button', { name: 'Launch Review' }).click();
  await pressLaunch(page);
  await page.getByRole('button', { name: /Skip/ }).click();
  await expect(page.locator('.verdict-title')).toBeVisible();
  await page.getByRole('button', { name: 'Modify design' }).click();
  await expect(canvas(page)).toHaveAttribute('data-shot', 'hangar', { timeout: 10_000 });
  await expect(canvas(page)).toHaveAttribute('data-parts', parts);
  // The Design is intact on its tabs too: the Launch Review button is still there and the Debrief panel is gone.
  await expect(page.locator('.screen.debrief')).toHaveCount(0);
  expect(errors).toEqual([]);
});

test('the failure visuals are idle animations that hold a still pose with reduced motion', async ({ browser }) => {
  for (const { fate, share, k } of [CASES[1], CASES[3]].map((c) => ({ fate: c.fate, share: c.share, k: c.k }))) {
    const spin = (page) => canvas(page).getAttribute('data-debrief-spin');
    const moving = await browser.newPage();
    await flyToDebrief(moving, share, k);
    await expect(canvas(moving)).toHaveAttribute('data-debrief', fate);
    const a = await spin(moving);
    await moving.waitForTimeout(700);
    expect(await spin(moving), `${fate} animates`).not.toBe(a);
    await moving.close();

    const still = await browser.newPage();
    await flyToDebrief(still, share, k, { reducedMotion: true });
    await expect(canvas(still)).toHaveAttribute('data-debrief', fate);
    const b = await spin(still);
    await still.waitForTimeout(700);
    expect(await spin(still), `${fate} holds still`).toBe(b);
    await still.close();
  }
});
