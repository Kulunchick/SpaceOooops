// Launch Review beside the launch pad and the launch sequence (low-poly ticket 07). The canvas exposes
// data-shot / data-pad-phase / data-pad-votes / data-pad-rocket / data-pad-craft; audio state is on <html data-audio-*>.
import { test, expect } from '@playwright/test';
import { collectErrors, CROP_WATCH_SHARE } from './helpers.js';

const canvas = (page) => page.locator('canvas.scene-canvas');
const html = (page) => page.locator('html');

async function openLaunchReview(page, share = CROP_WATCH_SHARE) {
  await page.goto(`./#share=${share}`);
  await page.getByRole('button', { name: 'Launch Review' }).click();
  await expect(page.getByRole('heading', { name: 'Launch Review' })).toBeVisible();
  await expect(canvas(page)).toHaveAttribute('data-shot', 'pad', { timeout: 20_000 });
}

test('the pad shows the chosen Launch Vehicle with the spacecraft on top, beside the review', async ({ page }) => {
  const errors = collectErrors(page);
  await openLaunchReview(page);
  await expect(canvas(page)).toHaveAttribute('data-pad-rocket', 'medium');
  await expect(canvas(page)).toHaveAttribute('data-pad-craft', '1');
  await expect(page.locator('.scene-layer')).not.toHaveClass(/dimmed/);
  // The review's explanations are all still there.
  await expect(page.getByText('Hard constraints broken: 0')).toBeVisible();
  await expect(page.getByRole('button', { name: 'Launch', exact: true })).toBeEnabled();
  await page.waitForTimeout(2500);
  await page.screenshot({ path: 'test-results/pad-medium.png' });
  expect(errors).toEqual([]);
});

for (const [name, vehicle, cls] of [['small', 'small-launcher', 'small'], ['heavy', 'geo-rideshare', 'heavy']]) {
  test(`the ${name} Launch Vehicle stands on the pad`, async ({ page }) => {
    const errors = collectErrors(page);
    await openLaunchReview(page, CROP_WATCH_SHARE.replace('cubesat-rideshare', vehicle));
    await expect(canvas(page)).toHaveAttribute('data-pad-rocket', cls);
    await expect(canvas(page)).toHaveAttribute('data-pad-craft', '1');
    await page.waitForTimeout(2500);
    await page.screenshot({ path: `test-results/pad-${name}.png` });
    expect(errors).toEqual([]);
  });
}

test('a blocked review still explains why it is blocked, beside the pad', async ({ page }) => {
  await page.goto('./#share=v1.crop-watch.multispectral-imager.cubesat-6u.solar-deployable~f.battery-m~f.reaction-wheels~f.comms-s-band~f.f.sso-600.geo-rideshare');
  await page.getByRole('button', { name: 'Launch Review' }).click();
  await expect(canvas(page)).toHaveAttribute('data-shot', 'pad', { timeout: 20_000 });
  await expect(page.locator('.launch-blocked')).toContainText(/Launch is blocked/);
  await expect(page.getByRole('button', { name: 'Launch', exact: true })).toBeDisabled();
  await page.screenshot({ path: 'test-results/pad-blocked.png' });
});

test('the launch plays a countdown, lift-off and ascent with sound, then the Run starts', async ({ page }) => {
  const errors = collectErrors(page);
  await openLaunchReview(page);
  await page.locator('body').click({ position: { x: 5, y: 5 } }); // unlock audio
  await expect(html(page)).toHaveAttribute('data-audio-unlocked', 'true');
  await page.getByRole('button', { name: 'Launch', exact: true }).click();
  await expect(page.getByRole('button', { name: 'Skip launch' })).toBeVisible();
  await expect(canvas(page)).toHaveAttribute('data-pad-phase', 'countdown');
  await expect(page.locator('#launch-count')).toHaveText('3');
  await expect(html(page)).toHaveAttribute('data-audio-last-cue', 'countdown');
  await expect(canvas(page)).toHaveAttribute('data-pad-phase', 'ignition', { timeout: 6000 });
  await expect(html(page)).toHaveAttribute('data-audio-last-cue', 'engine');
  await page.screenshot({ path: 'test-results/launch-ignition.png' });
  await expect(canvas(page)).toHaveAttribute('data-pad-phase', 'liftoff', { timeout: 6000 });
  await page.waitForTimeout(2200);
  await page.screenshot({ path: 'test-results/launch-liftoff.png' });
  await expect(canvas(page)).toHaveAttribute('data-pad-phase', 'ascent', { timeout: 6000 });
  await page.screenshot({ path: 'test-results/launch-ascent.png' });
  // Nothing was recorded yet: History records the Run only when it starts.
  await expect(page.locator('.run-screen, .run')).toHaveCount(0);
  await expect(page.getByRole('button', { name: /Skip/ }).first()).toBeVisible({ timeout: 12_000 });
  await expect(html(page)).toHaveAttribute('data-audio-wanted', 'run');
  expect(errors).toEqual([]);
});

test('the Skip button ends the launch at once and starts the Run', async ({ page }) => {
  await openLaunchReview(page);
  await page.getByRole('button', { name: 'Launch', exact: true }).click();
  await page.getByRole('button', { name: 'Skip launch' }).click();
  await expect(html(page)).toHaveAttribute('data-audio-wanted', 'run');
  await expect(page.getByRole('heading', { name: 'Launch Review' })).toHaveCount(0);
});

test('a click anywhere during the launch ends it', async ({ page }) => {
  await openLaunchReview(page);
  await page.getByRole('button', { name: 'Launch', exact: true }).click();
  await expect(canvas(page)).toHaveAttribute('data-pad-phase', 'countdown');
  await page.waitForTimeout(600);
  await page.mouse.click(640, 400);
  await expect(page.getByRole('heading', { name: 'Launch Review' })).toHaveCount(0, { timeout: 3000 });
});

test('with reduced motion Launch goes straight to the Run', async ({ page }) => {
  await page.emulateMedia({ reducedMotion: 'reduce' });
  await openLaunchReview(page);
  await page.getByRole('button', { name: 'Launch', exact: true }).click();
  await expect(page.getByRole('heading', { name: 'Launch Review' })).toHaveCount(0, { timeout: 3000 });
  await expect(page.getByRole('button', { name: 'Skip launch' })).toBeHidden();
});

test('Go and No-Go votes light the lamps on the tower and cue a sound', async ({ page }) => {
  const errors = collectErrors(page);
  await page.goto('./');
  await page.getByRole('button', { name: 'Start' }).click();
  await page.getByRole('button', { name: /^Team/ }).click();
  await page.evaluate((code) => { location.hash = `#share=${code}`; }, CROP_WATCH_SHARE);
  await page.getByRole('button', { name: 'Launch Review' }).click();
  await expect(canvas(page)).toHaveAttribute('data-shot', 'pad', { timeout: 20_000 });
  await page.locator('body').click({ position: { x: 5, y: 5 } });
  await expect(html(page)).toHaveAttribute('data-audio-unlocked', 'true');
  await expect(canvas(page)).toHaveAttribute('data-pad-votes', 'pending pending pending pending');
  const row = (n) => page.locator('.vote-row').nth(n);
  await row(0).getByRole('button', { name: 'Go', exact: true }).click();
  await expect(canvas(page)).toHaveAttribute('data-pad-votes', 'go pending pending pending');
  await expect(html(page)).toHaveAttribute('data-audio-last-cue', 'go');
  await row(1).getByRole('button', { name: 'No-Go' }).click();
  await expect(canvas(page)).toHaveAttribute('data-pad-votes', 'go nogo pending pending');
  await expect(html(page)).toHaveAttribute('data-audio-last-cue', 'nogo');
  await page.screenshot({ path: 'test-results/pad-votes.png' });
  expect(errors).toEqual([]);
});
