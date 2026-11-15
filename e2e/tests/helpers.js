// Shared helpers for the smoke tests.
import { readFileSync } from 'node:fs';
import { expect } from '@playwright/test';

/** The Scenarios in data/scenarios.toml (name and Target Country name), so checks follow the catalog. */
export function catalogScenarios() {
  const text = readFileSync(new URL('../../data/scenarios.toml', import.meta.url), 'utf8');
  return text
    .split(/^\[\[scenarios\]\]/m)
    .slice(1)
    .map((block) => ({
      name: block.match(/^name = "(.*)"/m)[1],
      country: block.match(/^target_country = \{[^}]*name = "([^"]*)"/m)[1],
    }));
}

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

/** Starts every page load with Motion set to Reduced in the stored settings (the OS preference is not followed). */
export async function chooseReducedMotion(page) {
  await page.addInitScript(() => localStorage.setItem('spaceooops.settings.v1', 'motion=reduced'));
}

/** Presses Launch and, when the 3D launch sequence starts, skips it, so the Run begins at once. */
export async function pressLaunch(page) {
  await page.getByRole('button', { name: 'Launch', exact: true }).click();
  // With reduced motion there is no launch sequence to skip.
  if (await page.evaluate(() => document.documentElement.dataset.motion === 'reduced')) return;
  if (await page.locator('.scene-layer[data-scene="live"]').count()) {
    const skip = page.getByRole('button', { name: 'Skip launch' });
    if (await skip.waitFor({ state: 'visible', timeout: 5000 }).then(() => true, () => false)) await skip.click();
  }
}

/**
 * Crop Watch with the seed of k = 19 (see run.webgl.spec.js): on day 6 a dragon takes the satellite for something
 * shiny, a fantasy new Random Event with a question. Answers it and checks the event log tells the answer, marked
 * Unexplained.
 */
export async function answerTheDragon(page) {
  const prompt = page.locator('.decision-prompt');
  await expect(prompt).toContainText('A dragon is circling the spacecraft, attracted by the shine. Dim the lights?', { timeout: 20_000 });
  await expect(prompt).toContainText('A gamble: about 7 in 10 it goes well.');
  await page.getByRole('button', { name: /Dim the lights/ }).click();
  const entry = page.locator('.event-log li.event-fantasy').filter({ hasText: 'Day 6' });
  await expect(entry).toContainText('The spacecraft dimmed its lights until the dragon lost interest. The Instruments were off for 2 days.');
  await expect(entry.locator('.event-mark')).toHaveText('Unexplained');
  await expect(prompt).toBeHidden();
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
