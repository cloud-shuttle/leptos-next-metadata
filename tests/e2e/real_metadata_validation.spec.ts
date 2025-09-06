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

test.describe("Real Metadata Macro Validation", () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to our test page with retry logic for connection issues
    await navigateWithRetry(page);
  });

  test.describe("Static metadata! Macro Output", () => {
    test("should generate correct page title", async ({ page }) => {
      const title = await page.title();
      expect(title).toBe("Welcome to My Site");
    });

    test("should generate correct meta description", async ({ page }) => {
      const description = await page
        .locator('meta[name="description"]')
        .getAttribute("content");
      expect(description).toBe(
        "A blazing fast Leptos application with comprehensive metadata management",
      );
    });

    test("should generate correct keywords", async ({ page }) => {
      const keywords = await page
        .locator('meta[name="keywords"]')
        .getAttribute("content");
      expect(keywords).toBe("leptos, metadata, rust, web, seo");
    });

    test("should generate correct author", async ({ page }) => {
      const author = await page
        .locator('meta[name="author"]')
        .getAttribute("content");
      expect(author).toBe("Peter Hanssens");
    });

    test("should generate correct canonical URL", async ({ page }) => {
      const canonical = await page
        .locator('link[rel="canonical"]')
        .getAttribute("href");
      expect(canonical).toBe("https://example.com");
    });
  });

  test.describe("OpenGraph Metadata Output", () => {
    test("should generate correct OpenGraph title", async ({ page }) => {
      const ogTitle = await page
        .locator('meta[property="og:title"]')
        .getAttribute("content");
      expect(ogTitle).toBe("Welcome to My Site");
    });

    test("should generate correct OpenGraph description", async ({ page }) => {
      const ogDescription = await page
        .locator('meta[property="og:description"]')
        .getAttribute("content");
      expect(ogDescription).toBe(
        "A blazing fast Leptos application with comprehensive metadata management",
      );
    });

    test("should generate correct OpenGraph type", async ({ page }) => {
      const ogType = await page
        .locator('meta[property="og:type"]')
        .getAttribute("content");
      expect(ogType).toBe("website");
    });

    test("should generate correct OpenGraph URL", async ({ page }) => {
      const ogUrl = await page
        .locator('meta[property="og:url"]')
        .getAttribute("content");
      expect(ogUrl).toBe("https://example.com");
    });

    test("should generate correct OpenGraph site name", async ({ page }) => {
      const ogSiteName = await page
        .locator('meta[property="og:site_name"]')
        .getAttribute("content");
      expect(ogSiteName).toBe("My Site");
    });

    test("should generate correct OpenGraph image", async ({ page }) => {
      const ogImage = await page
        .locator('meta[property="og:image"]')
        .getAttribute("content");
      expect(ogImage).toBe("https://example.com/og-image.jpg");
    });
  });

  test.describe("Twitter Card Metadata Output", () => {
    test("should generate correct Twitter card type", async ({ page }) => {
      const twitterCard = await page
        .locator('meta[name="twitter:card"]')
        .getAttribute("content");
      expect(twitterCard).toBe("summary_large_image");
    });

    test("should generate correct Twitter site", async ({ page }) => {
      const twitterSite = await page
        .locator('meta[name="twitter:site"]')
        .getAttribute("content");
      expect(twitterSite).toBe("@mysite");
    });

    test("should generate correct Twitter title", async ({ page }) => {
      const twitterTitle = await page
        .locator('meta[name="twitter:title"]')
        .getAttribute("content");
      expect(twitterTitle).toBe("Welcome to My Site");
    });

    test("should generate correct Twitter description", async ({ page }) => {
      const twitterDescription = await page
        .locator('meta[name="twitter:description"]')
        .getAttribute("content");
      expect(twitterDescription).toBe(
        "A blazing fast Leptos application with comprehensive metadata management",
      );
    });

    test("should generate correct Twitter image", async ({ page }) => {
      const twitterImage = await page
        .locator('meta[name="twitter:image"]')
        .getAttribute("content");
      expect(twitterImage).toBe("https://example.com/twitter-image.jpg");
    });
  });

  test.describe("JSON-LD Structured Data Output", () => {
    test("should generate JSON-LD script tag", async ({ page }) => {
      const jsonLdScript = await page
        .locator('script[type="application/ld+json"]')
        .innerHTML();
      expect(jsonLdScript).toBeTruthy();
    });

    test("should have valid JSON-LD structure", async ({ page }) => {
      const jsonLdScript = await page
        .locator('script[type="application/ld+json"]')
        .innerHTML();

      // Parse JSON-LD and validate structure
      const jsonLd = JSON.parse(jsonLdScript);
      expect(jsonLd["@context"]).toBe("https://schema.org");
      expect(jsonLd["@type"]).toBe("WebPage");
      expect(jsonLd.name).toBe("Welcome to My Site");
      expect(jsonLd.description).toBe(
        "A blazing fast Leptos application with comprehensive metadata management",
      );
      expect(jsonLd.url).toBe("https://example.com");
    });
  });

  test.describe("HTML Structure Validation", () => {
    test("should have proper HTML structure", async ({ page }) => {
      // Check HTML lang attribute
      const htmlLang = await page.locator("html").getAttribute("lang");
      expect(htmlLang).toBe("en");

      // Check charset
      const charset = await page
        .locator("meta[charset]")
        .getAttribute("charset");
      expect(charset?.toLowerCase()).toBe("utf-8");

      // Check viewport
      const viewport = await page
        .locator('meta[name="viewport"]')
        .getAttribute("content");
      expect(viewport).toBe("width=device-width, initial-scale=1");
    });

    test("should have proper page content", async ({ page }) => {
      // Check page heading
      const h1 = await page.locator("h1").textContent();
      expect(h1).toBe("Welcome to My Site");

      // Check page paragraphs
      const paragraphs = await page.locator("p").allTextContents();
      expect(paragraphs).toContain(
        "This is a test page for metadata macro testing.",
      );
      expect(paragraphs).toContain(
        "Check the page source to see the generated metadata tags.",
      );
    });
  });

  test.describe("Metadata Quality Validation", () => {
    test("should meet SEO requirements", async ({ page }) => {
      const title = await page.title();
      const description = await page
        .locator('meta[name="description"]')
        .getAttribute("content");

      // Title length validation (10-60 characters)
      expect(title.length).toBeGreaterThanOrEqual(10);
      expect(title.length).toBeLessThanOrEqual(60);

      // Description length validation (50-160 characters)
      expect(description?.length).toBeGreaterThanOrEqual(50);
      expect(description?.length).toBeLessThanOrEqual(160);
    });

    test("should have all required meta tags", async ({ page }) => {
      // Check essential meta tags exist
      const requiredTags = [
        'meta[name="description"]',
        'meta[name="keywords"]',
        'meta[name="author"]',
        'meta[property="og:title"]',
        'meta[property="og:description"]',
        'meta[property="og:type"]',
        'meta[name="twitter:card"]',
        'meta[name="twitter:title"]',
        'link[rel="canonical"]',
        'script[type="application/ld+json"]',
      ];

      for (const selector of requiredTags) {
        const element = page.locator(selector);
        await expect(element).toHaveCount(1);
      }
    });

    test("should have proper URL formats", async ({ page }) => {
      // Check canonical URL format
      const canonical = await page
        .locator('link[rel="canonical"]')
        .getAttribute("href");
      expect(canonical).toMatch(/^https?:\/\//);

      // Check OpenGraph URL format
      const ogUrl = await page
        .locator('meta[property="og:url"]')
        .getAttribute("content");
      expect(ogUrl).toMatch(/^https?:\/\//);

      // Check image URL formats
      const ogImage = await page
        .locator('meta[property="og:image"]')
        .getAttribute("content");
      expect(ogImage).toMatch(/^https?:\/\//);

      const twitterImage = await page
        .locator('meta[name="twitter:image"]')
        .getAttribute("content");
      expect(twitterImage).toMatch(/^https?:\/\//);
    });
  });
});
