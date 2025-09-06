import { test, expect } from "@playwright/test";

// Helper function to retry page navigation with connection issues
async function navigateWithRetry(page: any, maxRetries = 3) {
  for (let attempt = 1; attempt <= maxRetries; attempt++) {
    try {
      await page.goto("/", { waitUntil: "domcontentloaded", timeout: 10000 });
      return; // Success
    } catch (error: any) {
      if (attempt === maxRetries) {
        throw error; // Last attempt failed
      }
      if (
        error.message.includes("Could not connect to the server") ||
        error.message.includes("net::ERR_CONNECTION_REFUSED")
      ) {
        console.log(`Connection attempt ${attempt} failed, retrying...`);
        await page.waitForTimeout(1000 * attempt); // Exponential backoff
      } else {
        throw error; // Non-connection error, don't retry
      }
    }
  }
}

test.describe("Cross-Browser Metadata Compatibility", () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to our test page with retry logic for connection issues
    await navigateWithRetry(page);
  });

  test.describe("Chromium (Chrome/Edge)", () => {
    test("should generate metadata correctly in Chromium", async ({ page }) => {
      // Test basic metadata
      const title = await page.title();
      expect(title).toBe("Welcome to My Site");

      const description = await page
        .locator('meta[name="description"]')
        .getAttribute("content");
      expect(description).toBe(
        "A blazing fast Leptos application with comprehensive metadata management",
      );

      // Test OpenGraph metadata
      const ogTitle = await page
        .locator('meta[property="og:title"]')
        .getAttribute("content");
      expect(ogTitle).toBe("Welcome to My Site");

      // Test Twitter metadata
      const twitterCard = await page
        .locator('meta[name="twitter:card"]')
        .getAttribute("content");
      expect(twitterCard).toBe("summary_large_image");
    });

    test("should handle metadata rendering in Chromium", async ({ page }) => {
      // Test that metadata is properly rendered
      const metaTags = await page.locator("meta").count();
      expect(metaTags).toBeGreaterThan(10); // Should have many meta tags

      // Test that all required tags are present
      const description = await page.locator('meta[name="description"]');
      await expect(description).toHaveCount(1);

      const ogTitle = await page.locator('meta[property="og:title"]');
      await expect(ogTitle).toHaveCount(1);
    });
  });

  test.describe("Firefox", () => {
    test("should generate metadata correctly in Firefox", async ({ page }) => {
      // Test basic metadata
      const title = await page.title();
      expect(title).toBe("Welcome to My Site");

      const description = await page
        .locator('meta[name="description"]')
        .getAttribute("content");
      expect(description).toBe(
        "A blazing fast Leptos application with comprehensive metadata management",
      );

      // Test OpenGraph metadata
      const ogTitle = await page
        .locator('meta[property="og:title"]')
        .getAttribute("content");
      expect(ogTitle).toBe("Welcome to My Site");

      // Test Twitter metadata
      const twitterCard = await page
        .locator('meta[name="twitter:card"]')
        .getAttribute("content");
      expect(twitterCard).toBe("summary_large_image");
    });

    test("should handle metadata rendering in Firefox", async ({ page }) => {
      // Test that metadata is properly rendered
      const metaTags = await page.locator("meta").count();
      expect(metaTags).toBeGreaterThan(10);

      // Test that all required tags are present
      const description = await page.locator('meta[name="description"]');
      await expect(description).toHaveCount(1);

      const ogTitle = await page.locator('meta[property="og:title"]');
      await expect(ogTitle).toHaveCount(1);
    });
  });

  test.describe("WebKit (Safari)", () => {
    test("should generate metadata correctly in WebKit", async ({ page }) => {
      // Test basic metadata
      const title = await page.title();
      expect(title).toBe("Welcome to My Site");

      const description = await page
        .locator('meta[name="description"]')
        .getAttribute("content");
      expect(description).toBe(
        "A blazing fast Leptos application with comprehensive metadata management",
      );

      // Test OpenGraph metadata
      const ogTitle = await page
        .locator('meta[property="og:title"]')
        .getAttribute("content");
      expect(ogTitle).toBe("Welcome to My Site");

      // Test Twitter metadata
      const twitterCard = await page
        .locator('meta[name="twitter:card"]')
        .getAttribute("content");
      expect(twitterCard).toBe("summary_large_image");
    });

    test("should handle metadata rendering in WebKit", async ({ page }) => {
      // Test that metadata is properly rendered
      const metaTags = await page.locator("meta").count();
      expect(metaTags).toBeGreaterThan(10);

      // Test that all required tags are present
      const description = await page.locator('meta[name="description"]');
      await expect(description).toHaveCount(1);

      const ogTitle = await page.locator('meta[property="og:title"]');
      await expect(ogTitle).toHaveCount(1);
    });
  });

  test.describe("Mobile Chrome", () => {
    test("should generate metadata correctly on mobile Chrome", async ({
      page,
    }) => {
      // Set mobile viewport
      await page.setViewportSize({ width: 375, height: 667 });

      // Test basic metadata
      const title = await page.title();
      expect(title).toBe("Welcome to My Site");

      const description = await page
        .locator('meta[name="description"]')
        .getAttribute("content");
      expect(description).toBe(
        "A blazing fast Leptos application with comprehensive metadata management",
      );

      // Test OpenGraph metadata
      const ogTitle = await page
        .locator('meta[property="og:title"]')
        .getAttribute("content");
      expect(ogTitle).toBe("Welcome to My Site");
    });

    test("should handle mobile viewport correctly", async ({ page }) => {
      await page.setViewportSize({ width: 375, height: 667 });

      // Test viewport meta tag
      const viewport = await page
        .locator('meta[name="viewport"]')
        .getAttribute("content");
      expect(viewport).toBe("width=device-width, initial-scale=1");

      // Test that page content is accessible on mobile
      const h1 = await page.locator("h1").textContent();
      expect(h1).toBe("Welcome to My Site");
    });
  });

  test.describe("Mobile Safari", () => {
    test("should generate metadata correctly on mobile Safari", async ({
      page,
    }) => {
      // Set mobile viewport
      await page.setViewportSize({ width: 375, height: 667 });

      // Test basic metadata
      const title = await page.title();
      expect(title).toBe("Welcome to My Site");

      const description = await page
        .locator('meta[name="description"]')
        .getAttribute("content");
      expect(description).toBe(
        "A blazing fast Leptos application with comprehensive metadata management",
      );

      // Test OpenGraph metadata
      const ogTitle = await page
        .locator('meta[property="og:title"]')
        .getAttribute("content");
      expect(ogTitle).toBe("Welcome to My Site");
    });

    test("should handle mobile interactions correctly", async ({ page }) => {
      await page.setViewportSize({ width: 375, height: 667 });

      // Test touch interactions if any
      const title = await page.title();
      expect(title).toBe("Welcome to My Site");

      // Test that metadata is still properly generated on mobile
      const description = await page
        .locator('meta[name="description"]')
        .getAttribute("content");
      expect(description).toBe(
        "A blazing fast Leptos application with comprehensive metadata management",
      );
    });
  });

  test.describe("Cross-Browser Consistency", () => {
    test("should generate identical metadata across all browsers", async ({
      page,
    }) => {
      // Test that core metadata is consistent
      const title = await page.title();
      expect(title).toBe("Welcome to My Site");

      const description = await page
        .locator('meta[name="description"]')
        .getAttribute("content");
      expect(description).toBe(
        "A blazing fast Leptos application with comprehensive metadata management",
      );

      const keywords = await page
        .locator('meta[name="keywords"]')
        .getAttribute("content");
      expect(keywords).toBe("leptos, metadata, rust, web, seo");

      const canonical = await page
        .locator('link[rel="canonical"]')
        .getAttribute("href");
      expect(canonical).toBe("https://example.com");
    });

    test("should have consistent OpenGraph metadata across browsers", async ({
      page,
    }) => {
      // Test OpenGraph consistency
      const ogTitle = await page
        .locator('meta[property="og:title"]')
        .getAttribute("content");
      expect(ogTitle).toBe("Welcome to My Site");

      const ogDescription = await page
        .locator('meta[property="og:description"]')
        .getAttribute("content");
      expect(ogDescription).toBe(
        "A blazing fast Leptos application with comprehensive metadata management",
      );

      const ogType = await page
        .locator('meta[property="og:type"]')
        .getAttribute("content");
      expect(ogType).toBe("website");

      const ogUrl = await page
        .locator('meta[property="og:url"]')
        .getAttribute("content");
      expect(ogUrl).toBe("https://example.com");
    });

    test("should have consistent Twitter metadata across browsers", async ({
      page,
    }) => {
      // Test Twitter consistency
      const twitterCard = await page
        .locator('meta[name="twitter:card"]')
        .getAttribute("content");
      expect(twitterCard).toBe("summary_large_image");

      const twitterTitle = await page
        .locator('meta[name="twitter:title"]')
        .getAttribute("content");
      expect(twitterTitle).toBe("Welcome to My Site");

      const twitterDescription = await page
        .locator('meta[name="twitter:description"]')
        .getAttribute("content");
      expect(twitterDescription).toBe(
        "A blazing fast Leptos application with comprehensive metadata management",
      );
    });
  });

  test.describe("Browser-Specific Features", () => {
    test("should handle browser-specific meta tag rendering", async ({
      page,
    }) => {
      // Test that all browsers can render the same meta tags
      const metaSelectors = [
        'meta[name="description"]',
        'meta[name="keywords"]',
        'meta[name="author"]',
        'meta[property="og:title"]',
        'meta[property="og:description"]',
        'meta[property="og:type"]',
        'meta[property="og:url"]',
        'meta[property="og:site_name"]',
        'meta[property="og:image"]',
        'meta[name="twitter:card"]',
        'meta[name="twitter:site"]',
        'meta[name="twitter:title"]',
        'meta[name="twitter:description"]',
        'meta[name="twitter:image"]',
      ];

      for (const selector of metaSelectors) {
        const element = page.locator(selector);
        await expect(element).toHaveCount(1);
        const content = await element.getAttribute("content");
        expect(content).toBeTruthy();
      }
    });

    test("should handle JSON-LD consistently across browsers", async ({
      page,
    }) => {
      // Test JSON-LD rendering
      const jsonLdScript = await page
        .locator('script[type="application/ld+json"]')
        .innerHTML();
      expect(jsonLdScript).toBeTruthy();

      // Parse and validate JSON-LD
      const jsonLd = JSON.parse(jsonLdScript);
      expect(jsonLd["@context"]).toBe("https://schema.org");
      expect(jsonLd["@type"]).toBe("WebPage");
      expect(jsonLd.name).toBe("Welcome to My Site");
    });
  });
});
