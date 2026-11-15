// Random Events and Decisions in the 3D Run scene (low-poly ticket 05). The effects follow the Run playback only:
// nothing is in the scene before its day, a day with a Random Event plays slowly enough to be seen, and a skip
// leaves nothing behind. The canvas exposes data-effects (active kinds), data-flare (wave progress, 0..1),
// data-feather, data-dark-parts, data-stations (`id:on|dark`) and data-contact (the station a beam goes to).
// Set EFFECT_SHOTS to a folder to save a screenshot of each effect mid-way (for checking the look by eye).
import { test, expect } from '@playwright/test';
import { collectErrors, pressLaunch, CROP_WATCH_SHARE } from './helpers.js';

const BASIC_SHARE = CROP_WATCH_SHARE.replaceAll('~f', '~b').replace('.f.sso', '.b.sso');

/**
 * Fixes the Run seed: a constant k / 2^32 from Math.random gives seed (k << 32) | k (see run.webgl.spec.js).
 * Also lets a test hold the playback where it is: the playback driver ticks on setInterval, so a held interval
 * is a paused Run, while the renderer keeps drawing frames.
 */
async function seedRuns(page, k) {
  await page.addInitScript((value) => {
    Math.random = () => value;
    const every = window.setInterval.bind(window);
    window.__holdPlayback = false;
    window.setInterval = (fn, ms, ...args) => every(() => window.__holdPlayback || fn(...args), ms);
  }, (k + 0.5) / 4294967296);
}

const hold = (page, on) => page.evaluate((v) => (window.__holdPlayback = v), on);

async function launch(page, share = CROP_WATCH_SHARE) {
  await page.goto(`./#share=${share}`);
  await page.getByRole('button', { name: 'Launch Review' }).click();
  await pressLaunch(page);
  await expect(page.locator('canvas.scene-canvas')).toHaveAttribute('data-shot', 'orbit', { timeout: 20_000 });
}

async function shot(page, name) {
  if (!process.env.EFFECT_SHOTS) return;
  // A close-up of the spacecraft first (it moves on quickly), where the craft's own changes show.
  const xy = await page.locator('canvas.scene-canvas').getAttribute('data-craft-xy');
  const [x, y] = (xy ?? '640 400').split(' ').map(Number);
  const clip = { x: Math.max(0, x - 120), y: Math.max(0, y - 120), width: 240, height: 240 };
  await page.screenshot({ path: `${process.env.EFFECT_SHOTS}/${name}-craft.png`, clip });
  await page.screenshot({ path: `${process.env.EFFECT_SHOTS}/${name}.png` });
}

const canvas = (page) => page.locator('canvas.scene-canvas');
const attr = (page, name) => canvas(page).getAttribute(name);

test('a solar flare sends a wave from the Sun to the spacecraft, and safe mode feathers its panels', async ({ page }) => {
  const errors = collectErrors(page);
  // Crop Watch, k = 28: a flare warning before day 3.
  await seedRuns(page, 28);
  await launch(page);
  await expect(page.locator('.decision-prompt')).toBeVisible({ timeout: 20_000 });
  // Paused on the warning: the flare has not happened yet, and nothing moves on.
  await page.waitForTimeout(600);
  expect(await attr(page, 'data-effects')).not.toMatch(/flare|safeMode/);
  expect(await attr(page, 'data-flare')).toBe('');
  expect(await attr(page, 'data-feather')).toBe('0.00');
  await page.getByRole('button', { name: /Enter safe mode/ }).click();
  // The flare day plays slowly: the wave is seen in flight before it reaches the spacecraft (at 0.6 of the day).
  await expect.poll(async () => Number((await attr(page, 'data-flare')) || -1), { intervals: [20] }).toBeGreaterThan(0.3);
  // Paused mid-flight, the wave stays exactly where it is.
  await hold(page, true);
  await page.waitForTimeout(200);
  const during = await canvas(page).evaluate((c) => ({ ...c.dataset }));
  await shot(page, 'flare-and-safe-mode');
  expect(await attr(page, 'data-flare')).toBe(during.flare);
  expect(Number(during.flare)).toBeLessThan(0.6);
  // Safe mode has already feathered the panels by the time the wave arrives.
  expect(during.effects).toMatch(/flare/);
  expect(during.effects).toMatch(/safeMode/);
  expect(Number(during.feather)).toBeGreaterThan(0.9);
  await hold(page, false);
  await expect(page.locator('html')).toHaveAttribute('data-audio-last-cue', 'flare');
  // Two days of safe mode, then the panels open again and the effects are gone.
  await expect(canvas(page)).not.toHaveAttribute('data-effects', /safeMode|flare/, { timeout: 10_000 });
  await expect.poll(async () => Number(await attr(page, 'data-feather'))).toBeLessThan(0.05);
  // The event log keeps its captions word for word.
  await expect(page.locator('.event-log')).toContainText('A solar flare hit. The spacecraft switched off its Instruments and waited in safe mode for 2 days.');
  expect(errors).toEqual([]);
});

test('a ground station outage turns the main beacon dark until contact returns', async ({ page }) => {
  const errors = collectErrors(page);
  // Crop Watch, k = 10: the ground station goes offline on day 8 (a passive new event on day 4 comes before it).
  await seedRuns(page, 10);
  await launch(page);
  await expect(page.locator('.decision-prompt')).toBeVisible({ timeout: 20_000 });
  expect(await attr(page, 'data-stations')).toBe('primary:on backup:on');
  await page.getByRole('button', { name: /Wait it out/ }).click();
  await expect(canvas(page)).toHaveAttribute('data-stations', 'primary:dark backup:on');
  await hold(page, true);
  await expect(page.locator('html')).toHaveAttribute('data-audio-last-cue', 'outage');
  // No Data comes down while waiting, so no beam.
  expect(await attr(page, 'data-contact')).toBe('');
  await shot(page, 'outage');
  await hold(page, false);
  await expect(canvas(page)).toHaveAttribute('data-stations', 'primary:on backup:on', { timeout: 10_000 });
  expect(errors).toEqual([]);
});

test('ground contact passes show as a beam between the spacecraft and the station', async ({ page }) => {
  const errors = collectErrors(page);
  // Crop Watch, k = 3052: no question before day 203.
  await seedRuns(page, 3052);
  await launch(page);
  await expect(canvas(page)).toHaveAttribute('data-contact', 'primary', { timeout: 60_000 });
  await hold(page, true);
  await page.waitForTimeout(300);
  await shot(page, 'contact');
  await hold(page, false);
  expect(errors).toEqual([]);
});

test('a failed Component sparks and goes dark on the model, and a skip leaves no effect behind', async ({ page }) => {
  const errors = collectErrors(page);
  // All-Basic Crop Watch, k = 10: an outage on day 8, then the deployable panels fail on day 39 and end the Mission
  // (no new event asks a question before then).
  await seedRuns(page, 10);
  await launch(page, BASIC_SHARE);
  await expect(page.locator('.decision-prompt')).toBeVisible({ timeout: 20_000 });
  await page.getByRole('button', { name: /Wait it out/ }).click();
  // A 39-day Run takes the same 30 s as any other at 1x: speed up to day 30, then watch the failure at 1x.
  await page.getByRole('button', { name: '4×' }).click();
  const day = async () => Number((await page.locator('.dashboards').innerText()).match(/(\d+) \/ 365/)?.[1] ?? 0);
  await expect.poll(day, { intervals: [50], timeout: 20_000 }).toBeGreaterThanOrEqual(30);
  await page.getByRole('button', { name: '1×' }).click();
  await expect(canvas(page)).toHaveAttribute('data-dark-parts', /solar-deployable/, { timeout: 20_000 });
  await hold(page, true);
  await page.waitForTimeout(200);
  await shot(page, 'failure');
  await hold(page, false);
  await expect(page.locator('html')).toHaveAttribute('data-audio-last-cue', 'failure');
  // The Mission ends that day; skip (or let the playback hold run out) to the Debrief.
  await page.getByRole('button', { name: /Skip/ }).click({ timeout: 1000 }).catch(() => {});
  await expect(page.locator('.verdict-title')).toBeVisible();
  await expect(canvas(page)).toHaveAttribute('data-effects', '');
  await expect(canvas(page)).toHaveAttribute('data-dark-parts', '');
  expect(errors).toEqual([]);
});

test('a new Random Event shows as a link-interference effect with its Cue, and is gone after its day', async ({ page }) => {
  const errors = collectErrors(page);
  // Crop Watch, k = 42: the first new event is a link interference on day 45 (the control-room key went on
  // holiday), with no question before it.
  await seedRuns(page, 42);
  await launch(page);
  await expect(canvas(page)).toHaveAttribute('data-effects', /linkInterference/, { timeout: 40_000 });
  await hold(page, true);
  await page.waitForTimeout(200);
  expect(await attr(page, 'data-event-fx')).toMatch(/^linkInterference:/);
  await shot(page, 'link-interference');
  // Paused, the effect stays exactly where it is.
  const during = await attr(page, 'data-event-fx');
  await page.waitForTimeout(200);
  expect(await attr(page, 'data-event-fx')).toBe(during);
  await expect(page.locator('html')).toHaveAttribute('data-audio-last-cue', 'interference');
  await hold(page, false);
  await expect(canvas(page)).not.toHaveAttribute('data-effects', /linkInterference/, { timeout: 10_000 });
  expect(await attr(page, 'data-event-fx')).toBe('');
  await expect(page.locator('.event-log')).toContainText('The only key to the control room went on holiday');
  expect(errors).toEqual([]);
});

// Each new scene effect kind on a seed whose Run reaches it before any question (Crop Watch's known-good Design).
const KINDS = [
  { kind: 'flyBy', cue: 'fly-by', k: 42, what: 'the debris of an anti-satellite test flies by on day 89' },
  { kind: 'panelShading', cue: 'shading', k: 6, what: 'the spacecraft is scared of its own shadow on day 34' },
  { kind: 'bonusGlow', cue: 'bonus', k: 48, what: 'a rare sight lands in frame on day 22' },
  { kind: 'visitor', cue: 'visitor', k: 133, what: 'aliens borrow the satellite on day 38' },
  { kind: 'hit', cue: 'hit', k: 14, what: 'a static discharge hits the hull on day 8' },
];

for (const { kind, cue, k, what } of KINDS) {
  test(`a ${kind} effect is drawn at the spacecraft and plays its Cue (${what})`, async ({ page }) => {
    const errors = collectErrors(page);
    await seedRuns(page, k);
    await launch(page);
    await expect(canvas(page)).toHaveAttribute('data-effects', new RegExp(kind), { timeout: 40_000 });
    await hold(page, true);
    await page.waitForTimeout(200);
    expect(await attr(page, 'data-event-fx')).toMatch(new RegExp(`(^| )${kind}:`));
    await shot(page, kind);
    await expect(page.locator('html')).toHaveAttribute('data-audio-last-cue', cue);
    await hold(page, false);
    expect(errors).toEqual([]);
  });
}
