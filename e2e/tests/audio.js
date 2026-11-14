// Shared audio checks (ticket 04), registered by audio.webgl.spec.js and audio.fallback.spec.js.
// Nobody can listen in CI, so the bridge mirrors its state to <html data-audio-*>: whether the context is
// unlocked, the current and wanted music track, the effective gains, the last Cue and how many sources started.
import { expect } from '@playwright/test';
import { collectErrors, CROP_WATCH_SHARE, pressLaunch } from './helpers.js';

const html = (page) => page.locator('html');
const attr = (page, name) => page.evaluate((n) => document.documentElement.dataset[n], name);

async function unlock(page) {
  await page.locator('body').click({ position: { x: 5, y: 5 } });
  await expect(html(page)).toHaveAttribute('data-audio-unlocked', 'true');
}

async function openSettings(page) {
  if (!(await page.locator('#settings-panel').isVisible())) await page.getByRole('button', { name: 'Sound and graphics settings' }).click();
  await expect(page.locator('#settings-panel')).toBeVisible();
}

export function registerAudioTests(test) {
  test('nothing plays before the first click, then the menu music starts', async ({ page }) => {
    const errors = collectErrors(page);
    await page.goto('./');
    await expect(html(page)).toHaveAttribute('data-audio-wanted', 'menu');
    await expect(html(page)).toHaveAttribute('data-audio-unlocked', 'false');
    await expect(html(page)).toHaveAttribute('data-audio-track', '');
    await expect(html(page)).toHaveAttribute('data-audio-sources', '0');
    await unlock(page);
    await expect(html(page)).toHaveAttribute('data-audio-track', 'menu');
    expect(errors).toEqual([]);
  });

  test('the Run plays the Run track and switching crossfades', async ({ page }) => {
    await page.goto(`./#share=${CROP_WATCH_SHARE}`);
    await unlock(page);
    await expect(html(page)).toHaveAttribute('data-audio-track', 'menu');
    await page.getByRole('button', { name: 'Launch Review' }).click();
    await pressLaunch(page);
    await expect(html(page)).toHaveAttribute('data-audio-wanted', 'run');
    await expect(html(page)).toHaveAttribute('data-audio-fading', '1'); // the menu track fading out
    await expect(html(page)).toHaveAttribute('data-audio-track', 'run');
    await expect(html(page)).toHaveAttribute('data-audio-fading', '0', { timeout: 6000 });
  });

  test('adding a part snaps, loading a whole design does not', async ({ page }) => {
    await page.goto('./');
    await unlock(page);
    // A share link arriving while the game runs assembles the whole spacecraft at once: no snap burst.
    await page.evaluate((code) => { location.hash = `#share=${code}`; }, CROP_WATCH_SHARE);
    await expect(page.getByRole('button', { name: 'Launch Review' })).toBeVisible();
    await page.waitForTimeout(300);
    expect(await attr(page, 'audioLastCue')).not.toBe('snap');

    const card = page.getByRole('button', { name: /Wide-field camera/ });
    await card.click();
    await expect(card).toHaveAttribute('aria-pressed', 'true');
    await expect(html(page)).toHaveAttribute('data-audio-last-cue', 'snap');
  });

  test('volumes and mute are remembered across a reload', async ({ page }) => {
    await page.goto('./');
    await openSettings(page);
    await page.locator('#settings-music').fill('40');
    await page.locator('#settings-effects').fill('55');
    await page.locator('#settings-mute').check();
    await page.reload();
    await openSettings(page);
    await expect(page.locator('#settings-music')).toHaveValue('40');
    await expect(page.locator('#settings-effects')).toHaveValue('55');
    await expect(page.locator('#settings-mute')).toBeChecked();
    await expect(html(page)).toHaveAttribute('data-audio-muted', 'true');
    await page.locator('#settings-mute').uncheck();
    await expect(html(page)).toHaveAttribute('data-audio-music-gain', '0.4');
    await expect(html(page)).toHaveAttribute('data-audio-sfx-gain', '0.55');
  });

  test('mute is fully silent: gains are exactly 0 and no sound source starts', async ({ page }) => {
    await page.goto('./');
    await unlock(page);
    await expect(html(page)).toHaveAttribute('data-audio-track', 'menu');
    await openSettings(page);
    await page.locator('#settings-mute').check();
    await expect(html(page)).toHaveAttribute('data-audio-music-gain', '0');
    await expect(html(page)).toHaveAttribute('data-audio-sfx-gain', '0');
    await expect(html(page)).toHaveAttribute('data-audio-track', '');
    const before = await attr(page, 'audioSources');
    await page.getByRole('button', { name: 'Sound and graphics settings' }).click(); // a UI click that would normally click
    await page.getByRole('button', { name: 'Sound and graphics settings' }).click();
    await page.waitForTimeout(400);
    expect(await attr(page, 'audioSources')).toBe(before);
    // Unmuting brings the music back.
    await page.locator('#settings-mute').uncheck();
    await expect(html(page)).toHaveAttribute('data-audio-track', 'menu');
  });
}
