// Phone width (390 x 844): every screen fits the width, its main buttons can be reached and are not covered by the
// corner settings button. Registered by phone.webgl.spec.js and phone.fallback.spec.js.
import { expect } from '@playwright/test';
import { collectErrors, pressLaunch, CROP_WATCH_SHARE } from './helpers.js';

/** No sideways scrolling, and (after scrolling to the end) no button hidden under the settings button. */
async function expectUsable(page, name) {
  const overflow = await page.evaluate(() => document.documentElement.scrollWidth - window.innerWidth);
  expect(overflow, `${name}: horizontal overflow`).toBeLessThanOrEqual(0);
  await page.evaluate(() => window.scrollTo(0, document.body.scrollHeight));
  const covered = await page.evaluate(() => {
    const corner = document.querySelector('.settings-toggle')?.getBoundingClientRect();
    if (!corner) return [];
    const hits = [];
    for (const b of document.querySelectorAll('.app button:not([disabled]), .app a')) {
      const r = b.getBoundingClientRect();
      if (r.width === 0 || r.height === 0 || r.bottom < 0 || r.top > window.innerHeight) continue;
      const overlap = r.left < corner.right && r.right > corner.left && r.top < corner.bottom && r.bottom > corner.top;
      if (overlap) hits.push((b.textContent || '').trim());
    }
    return hits;
  });
  expect(covered, `${name}: buttons under the settings button`).toEqual([]);
}

export function registerPhoneTests(test) {
  test.use({ viewport: { width: 390, height: 844 }, hasTouch: true });

  test('every screen of a Mission is usable at phone width', async ({ page }) => {
    test.setTimeout(150_000);
    const errors = collectErrors(page);
    await page.goto('./');
    await expect(page.getByRole('button', { name: 'Start' })).toBeVisible();
    await expectUsable(page, 'title');
    await page.getByRole('button', { name: 'Start' }).click();
    await expectUsable(page, 'team setup');
    await page.getByRole('button', { name: 'Choose a Scenario' }).click();
    await expect(page.locator('.scenario-card').first()).toBeVisible();
    await expectUsable(page, 'scenario select');
    await page.locator('.scenario-card', { hasText: 'Crop Watch' }).getByRole('button', { name: 'Select' }).click();
    await expect(page.getByRole('button', { name: 'Start Design' })).toBeVisible();
    await expectUsable(page, 'brief');
    await page.goto(`./#share=${CROP_WATCH_SHARE}`);
    const review = page.getByRole('button', { name: 'Launch Review' });
    await expect(review).toBeInViewport();
    const panel = await page.locator('.role-panel').boundingBox();
    expect(panel.height, 'the Role panel keeps room to work in').toBeGreaterThan(200);
    await expectUsable(page, 'design');
    await review.click();
    await expect(page.getByRole('button', { name: 'Launch', exact: true })).toBeAttached();
    await expectUsable(page, 'launch review');
    await page.getByRole('button', { name: 'Launch', exact: true }).scrollIntoViewIfNeeded();
    await pressLaunch(page);
    await expect(page.getByRole('button', { name: /Skip/ })).toBeVisible({ timeout: 15_000 });
    await expectUsable(page, 'run');
    await page.getByRole('button', { name: /Skip/ }).click();
    const option = page.locator('.decision-option').first();
    const skip = page.getByRole('button', { name: /Skip/ });
    for (let i = 0; i < 30 && !(await page.locator('.verdict-title').isVisible()); i++) {
      if (await option.isVisible()) await option.click();
      else if (await skip.isVisible()) await skip.click();
      else await page.waitForTimeout(200);
    }
    await expect(page.locator('.verdict-title')).toBeVisible();
    await expectUsable(page, 'debrief');
    expect(errors).toEqual([]);
  });
}
