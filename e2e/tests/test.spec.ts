import { test, expect } from '@playwright/test';

const URL = process.env.URL ?? "https://www.playsnake.no";

test('snake canvas appears', async ({ page }) => {
  await page.goto(URL);
  const canvas = page.locator('css=#phone canvas');
  await canvas.waitFor();
  expect(canvas).toBeAttached();
});

test('highscores load successfully', async ({page}) => {
  await page.goto(URL);
  // Initially no highscores appear
  expect(page.getByText('Loading highscores...')).toHaveCount(2);

  // Wait for highscores to be fetched
  await page.waitForResponse(response => response.url().includes('topten'));
  // Wait for a little bit longer
  await page.waitForTimeout(500);

  // Now highscore table should be rendered
  expect(page.getByText('Loading highscores...')).toHaveCount(0);
});
