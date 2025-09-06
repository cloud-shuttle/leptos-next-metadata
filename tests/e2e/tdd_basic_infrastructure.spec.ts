import { test, expect } from "@playwright/test";

// TDD Phase 1: Basic Infrastructure Test
// This test validates that our core setup is working before we build complex features

test.describe("TDD Basic Infrastructure Validation", () => {
  test("should connect to test server successfully", async ({ page }) => {
    // Given: A test server is running
    // When: We navigate to the test page
    await page.goto("/", { waitUntil: "domcontentloaded", timeout: 10000 });

    // Then: The page should load successfully
    const title = await page.title();
    expect(title).toBe("Welcome to My Site");
  });

  test("should have basic HTML structure", async ({ page }) => {
    // Given: A test page is loaded
    await page.goto("/", { waitUntil: "domcontentloaded", timeout: 10000 });

    // When: We check the HTML structure
    const html = await page.locator("html").getAttribute("lang");
    const head = await page.locator("head").count();
    const body = await page.locator("body").count();

    // Then: Basic HTML elements should exist
    expect(html).toBe("en");
    expect(head).toBe(1);
    expect(body).toBe(1);
  });

  test("should have at least one meta tag", async ({ page }) => {
    // Given: A test page is loaded
    await page.goto("/", { waitUntil: "domcontentloaded", timeout: 10000 });

    // When: We count meta tags
    const metaCount = await page.locator("meta").count();

    // Then: Should have at least one meta tag
    expect(metaCount).toBeGreaterThan(0);
  });

  test("should have a title tag", async ({ page }) => {
    // Given: A test page is loaded
    await page.goto("/", { waitUntil: "domcontentloaded", timeout: 10000 });

    // When: We check for title
    const title = await page.title();

    // Then: Should have a non-empty title
    expect(title).toBeTruthy();
    expect(title.length).toBeGreaterThan(0);
  });
});
