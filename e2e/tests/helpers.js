// Shared helpers for the smoke tests.
import { expect } from '@playwright/test';

/** Collects console errors and uncaught page errors; assert it is empty at the end of a test. */
export function collectErrors(page) {
  const errors = [];
  page.on('console', (msg) => {
    if (msg.type() === 'error') errors.push(msg.text());
  });
  page.on('pageerror', (err) => errors.push(String(err)));
  return errors;
}

/** The Crop Watch known-good Design from tests/common (passes Launch Review with no warnings). */
export const CROP_WATCH_SHARE =
  'v1.crop-watch.multispectral-imager.cubesat-6u.solar-deployable~f.battery-m~f.reaction-wheels~f.comms-s-band~f.f.sso-600.cubesat-rideshare';

/** Presses Launch and, when the 3D launch sequence starts, skips it, so the Run begins at once. */
export async function pressLaunch(page) {
  await page.getByRole('button', { name: 'Launch', exact: true }).click();
  // With reduced motion there is no launch sequence to skip.
  if (await page.evaluate(() => matchMedia('(prefers-reduced-motion: reduce)').matches)) return;
  if (await page.locator('.scene-layer[data-scene="live"]').count()) {
    const skip = page.getByRole('button', { name: 'Skip launch' });
    if (await skip.waitFor({ state: 'visible', timeout: 5000 }).then(() => true, () => false)) await skip.click();
  }
}

/** Opens the share link, launches, skips through the Run (answering any decision) and opens the Debrief. */
export async function flyCropWatch(page) {
  await page.goto(`./#share=${CROP_WATCH_SHARE}`);
  await page.getByRole('button', { name: 'Launch Review' }).click();
  await pressLaunch(page);
  const verdict = page.locator('.verdict-title');
  const debrief = page.getByRole('button', { name: 'Debrief' });
  const option = page.locator('.decision-option').first();
  const skip = page.getByRole('button', { name: /Skip/ });
  for (let i = 0; i < 30 && !(await verdict.isVisible()); i++) {
    if (await option.isVisible()) await option.click();
    else if ((await debrief.isVisible()) && (await debrief.isEnabled())) await debrief.click();
    else if (await skip.isVisible()) await skip.click();
    else await page.waitForTimeout(200);
  }
  await expect(verdict).toBeVisible();
}
