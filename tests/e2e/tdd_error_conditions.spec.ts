import { test, expect } from "@playwright/test";

// TDD Phase 3: Error Conditions & Failure Scenarios Test
// This test validates that our library handles errors gracefully and fails predictably

test.describe("TDD Error Conditions & Failure Scenarios", () => {
  test("should handle malformed HTML gracefully", async ({ page }) => {
    // Given: A test page is loaded
    await page.goto("/", { waitUntil: "domcontentloaded", timeout: 10000 });

    // When: We check for malformed HTML patterns
    const html = await page.content();

    // Then: HTML should be well-formed
    expect(html).toContain("<!DOCTYPE html>");
    expect(html).toContain("<html");
    expect(html).toContain("</html>");
    expect(html).toContain("<head>");
    expect(html).toContain("</head>");
    expect(html).toContain("<body>");
    expect(html).toContain("</body>");
  });

  test("should validate required metadata presence", async ({ page }) => {
    // Given: A test page is loaded
    await page.goto("/", { waitUntil: "domcontentloaded", timeout: 10000 });

    // When: We check for required metadata
    const requiredMeta = [
      'meta[name="description"]',
      'meta[property="og:title"]',
      'meta[property="og:description"]',
      'meta[property="og:type"]',
      'meta[name="twitter:card"]',
      'meta[name="twitter:title"]',
      'meta[name="twitter:description"]',
    ];

    // Then: All required metadata should be present
    for (const selector of requiredMeta) {
      const count = await page.locator(selector).count();
      expect(count).toBe(1);
    }
  });

  test("should detect duplicate metadata entries", async ({ page }) => {
    // Given: A test page is loaded
    await page.goto("/", { waitUntil: "domcontentloaded", timeout: 10000 });

    // When: We check for duplicate metadata
    const allMeta = await page.locator("meta").all();
    const metaMap = new Map<string, number>();

    for (const meta of allMeta) {
      const name = await meta.getAttribute("name");
      const property = await meta.getAttribute("property");
      const key = name || property || "unknown";
      metaMap.set(key, (metaMap.get(key) || 0) + 1);
    }

    // Then: No critical metadata should be duplicated
    const duplicates = Array.from(metaMap.entries()).filter(
      ([_, count]) => count > 1,
    );

    // Allow some duplicates for different contexts (e.g., og:title and twitter:title)
    const allowedDuplicates = ["title", "description"];
    const criticalDuplicates = duplicates.filter(
      ([key, _]) => !allowedDuplicates.includes(key),
    );

    if (criticalDuplicates.length > 0) {
      console.log(
        "⚠️  Warning: Duplicate metadata detected:",
        criticalDuplicates,
      );
    }

    // Critical metadata should not be duplicated
    expect(criticalDuplicates.length).toBe(0);
  });

  test("should validate metadata content quality", async ({ page }) => {
    // Given: A test page is loaded
    await page.goto("/", { waitUntil: "domcontentloaded", timeout: 10000 });

    // When: We check metadata content quality
    const title = await page.title();
    const description = await page
      .locator('meta[name="description"]')
      .getAttribute("content");

    // Then: Content should meet quality standards
    expect(title).not.toBe("");
    expect(title).not.toBe("Untitled");
    expect(title).not.toBe("Page Title");

    if (description) {
      expect(description).not.toBe("");
      expect(description).not.toBe("Page description");
      expect(description).not.toBe("Enter your description here");

      // Should not be placeholder text
      const placeholderPatterns = [
        /^[A-Z\s]+$/, // ALL CAPS
        /^[a-z\s]+$/, // all lowercase
        /^[0-9\s]+$/, // numbers only
        /^[^\w\s]+$/, // symbols only
      ];

      for (const pattern of placeholderPatterns) {
        expect(description).not.toMatch(pattern);
      }
    }
  });

  test("should handle invalid URL formats", async ({ page }) => {
    // Given: A test page is loaded
    await page.goto("/", { waitUntil: "domcontentloaded", timeout: 10000 });

    // When: We check URL metadata
    const canonical = await page
      .locator('link[rel="canonical"]')
      .getAttribute("href");
    const ogUrl = await page
      .locator('meta[property="og:url"]')
      .getAttribute("content");

    // Then: URLs should be valid
    if (canonical) {
      try {
        new URL(canonical);
        expect(true).toBe(true); // URL is valid
      } catch {
        throw new Error(`Invalid canonical URL: ${canonical}`);
      }
    }

    if (ogUrl) {
      try {
        new URL(ogUrl);
        expect(true).toBe(true); // URL is valid
      } catch {
        throw new Error(`Invalid og:url: ${ogUrl}`);
      }
    }
  });

  test("should validate JSON-LD schema compliance", async ({ page }) => {
    // Given: A test page is loaded
    await page.goto("/", { waitUntil: "domcontentloaded", timeout: 10000 });

    // When: We check JSON-LD schema
    const jsonLdScripts = await page
      .locator('script[type="application/ld+json"]')
      .count();

    if (jsonLdScripts > 0) {
      const jsonLdScript = await page
        .locator('script[type="application/ld+json"]')
        .first()
        .innerHTML();

      try {
        const jsonLd = JSON.parse(jsonLdScript);

        // Then: Should have required schema.org properties
        expect(jsonLd["@context"]).toBe("https://schema.org");
        expect(jsonLd["@type"]).toBeTruthy();

        // Type-specific validation
        if (jsonLd["@type"] === "WebPage") {
          expect(jsonLd.name).toBeTruthy();
          expect(jsonLd.url).toBeTruthy();
        }

        // Should not have empty required fields
        if (jsonLd.name) {
          expect(jsonLd.name.trim().length).toBeGreaterThan(0);
        }
      } catch (error) {
        throw new Error(`JSON-LD validation failed: ${error}`);
      }
    }
  });

  test("should detect accessibility issues in metadata", async ({ page }) => {
    // Given: A test page is loaded
    await page.goto("/", { waitUntil: "domcontentloaded", timeout: 10000 });

    // When: We check for accessibility-related metadata
    const viewport = await page
      .locator('meta[name="viewport"]')
      .getAttribute("content");
    const lang = await page.locator("html").getAttribute("lang");

    // Then: Should have accessibility metadata
    expect(lang).toBeTruthy();
    expect(lang.length).toBeGreaterThan(0);

    if (viewport) {
      // Should include responsive viewport settings
      expect(viewport).toContain("width=device-width");
    }
  });

  test("should handle missing critical metadata gracefully", async ({
    page,
  }) => {
    // Given: A test page is loaded
    await page.goto("/", { waitUntil: "domcontentloaded", timeout: 10000 });

    // When: We check for critical metadata
    const criticalMeta = [
      "title",
      'meta[name="description"]',
      'meta[property="og:title"]',
      'meta[property="og:description"]',
    ];

    // Then: All critical metadata should be present
    for (const selector of criticalMeta) {
      if (selector === "title") {
        const title = await page.title();
        expect(title).toBeTruthy();
        expect(title.length).toBeGreaterThan(0);
      } else {
        const count = await page.locator(selector).count();
        expect(count).toBe(1);
      }
    }
  });

  test("should validate metadata consistency across platforms", async ({
    page,
  }) => {
    // Given: A test page is loaded
    await page.goto("/", { waitUntil: "domcontentloaded", timeout: 10000 });

    // When: We check metadata consistency
    const title = await page.title();
    const ogTitle = await page
      .locator('meta[property="og:title"]')
      .getAttribute("content");
    const twitterTitle = await page
      .locator('meta[name="twitter:title"]')
      .getAttribute("content");

    // Then: Titles should be consistent across platforms
    expect(ogTitle).toBe(title);
    expect(twitterTitle).toBe(title);

    const description = await page
      .locator('meta[name="description"]')
      .getAttribute("content");
    const ogDescription = await page
      .locator('meta[property="og:description"]')
      .getAttribute("content");
    const twitterDescription = await page
      .locator('meta[name="twitter:description"]')
      .getAttribute("content");

    if (description && ogDescription && twitterDescription) {
      expect(ogDescription).toBe(description);
      expect(twitterDescription).toBe(description);
    }
  });
});
