import { test, expect } from '@playwright/test';

test.describe('Metadata Validation', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to the home page before each test
    await page.goto('/');
  });

  test('should have correct page title', async ({ page }) => {
    const title = await page.title();
    expect(title).toBe('Welcome to My Site');
  });

  test('should have correct meta description', async ({ page }) => {
    const description = await page.locator('meta[name="description"]').getAttribute('content');
    expect(description).toBe('A blazing fast Leptos application with comprehensive metadata management');
  });

  test('should have correct Open Graph metadata', async ({ page }) => {
    // Check Open Graph title
    const ogTitle = await page.locator('meta[property="og:title"]').getAttribute('content');
    expect(ogTitle).toBe('Welcome to My Site');

    // Check Open Graph description
    const ogDescription = await page.locator('meta[property="og:description"]').getAttribute('content');
    expect(ogDescription).toBe('A blazing fast Leptos application with comprehensive metadata management');

    // Check Open Graph type
    const ogType = await page.locator('meta[property="og:type"]').getAttribute('content');
    expect(ogType).toBe('website');

    // Check Open Graph site name
    const ogSiteName = await page.locator('meta[property="og:site_name"]').getAttribute('content');
    expect(ogSiteName).toBe('My Site');

    // Check Open Graph images
    const ogImage = await page.locator('meta[property="og:image"]').getAttribute('content');
    expect(ogImage).toBe('/og-home.png');
  });

  test('should have correct Twitter Card metadata', async ({ page }) => {
    // Check Twitter Card type
    const twitterCard = await page.locator('meta[name="twitter:card"]').getAttribute('content');
    expect(twitterCard).toBe('summary_large_image');

    // Check Twitter site
    const twitterSite = await page.locator('meta[name="twitter:site"]').getAttribute('content');
    expect(twitterSite).toBe('@mysite');
  });

  test('should have correct keywords', async ({ page }) => {
    const keywords = await page.locator('meta[name="keywords"]').getAttribute('content');
    expect(keywords).toContain('leptos');
    expect(keywords).toContain('metadata');
    expect(keywords).toContain('rust');
    expect(keywords).toContain('web');
    expect(keywords).toContain('seo');
  });

  test('should have JSON-LD structured data', async ({ page }) => {
    const jsonLdScript = await page.locator('script[type="application/ld+json"]').innerHTML();
    expect(jsonLdScript).toBeTruthy();
    
    // Parse JSON-LD and validate structure
    const jsonLd = JSON.parse(jsonLdScript);
    expect(jsonLd['@type']).toBe('WebPage');
    expect(jsonLd.name).toBe('Welcome to My Site');
    expect(jsonLd.description).toBe('A blazing fast Leptos application with comprehensive metadata management');
  });

  test('should have correct canonical URL', async ({ page }) => {
    const canonical = await page.locator('link[rel="canonical"]').getAttribute('href');
    expect(canonical).toBe('http://localhost:3000/');
  });

  test('should have correct viewport meta tag', async ({ page }) => {
    const viewport = await page.locator('meta[name="viewport"]').getAttribute('content');
    expect(viewport).toBe('width=device-width, initial-scale=1');
  });
});

test.describe('Blog Post Metadata', () => {
  test('should have correct blog post metadata', async ({ page }) => {
    await page.goto('/blog/first-post');

    // Check page title
    const title = await page.title();
    expect(title).toBe('Blog Post | My Blog');

    // Check meta description
    const description = await page.locator('meta[name="description"]').getAttribute('content');
    expect(description).toBe('This is a sample blog post with comprehensive metadata');

    // Check Open Graph type for article
    const ogType = await page.locator('meta[property="og:type"]').getAttribute('content');
    expect(ogType).toBe('article');

    // Check Twitter creator
    const twitterCreator = await page.locator('meta[name="twitter:creator"]').getAttribute('content');
    expect(twitterCreator).toBe('@johndoe');
  });
});

test.describe('Product Page Metadata', () => {
  test('should have correct product metadata', async ({ page }) => {
    await page.goto('/product/1');

    // Check page title
    const title = await page.title();
    expect(title).toBe('Product | My Store');

    // Check meta description
    const description = await page.locator('meta[name="description"]').getAttribute('content');
    expect(description).toBe('This is a sample product with comprehensive metadata');

    // Check Open Graph type for product
    const ogType = await page.locator('meta[property="og:type"]').getAttribute('content');
    expect(ogType).toBe('product');
  });
});

test.describe('SEO Best Practices', () => {
  test('should have proper heading hierarchy', async ({ page }) => {
    await page.goto('/');

    // Check that there's only one h1
    const h1Count = await page.locator('h1').count();
    expect(h1Count).toBe(1);

    // Check that h1 contains the main title
    const h1Text = await page.locator('h1').textContent();
    expect(h1Text).toBe('Welcome to My Site');
  });

  test('should have proper navigation structure', async ({ page }) => {
    await page.goto('/');

    // Check navigation links
    const navLinks = page.locator('nav a');
    await expect(navLinks).toHaveCount(2);

    // Check first link
    const firstLink = navLinks.first();
    await expect(firstLink).toHaveAttribute('href', '/blog/first-post');
    await expect(firstLink).toHaveText('Read our first blog post');

    // Check second link
    const secondLink = navLinks.nth(1);
    await expect(secondLink).toHaveAttribute('href', '/product/1');
    await expect(secondLink).toHaveText('Check out our products');
  });

  test('should have proper page structure', async ({ page }) => {
    await page.goto('/');

    // Check that page has main content
    const mainContent = page.locator('div');
    await expect(mainContent).toBeVisible();

    // Check that page has descriptive text
    const description = page.locator('p').first();
    await expect(description).toContainText('blazing fast Leptos application');
  });
});

test.describe('Performance and Accessibility', () => {
  test('should load quickly', async ({ page }) => {
    const startTime = Date.now();
    await page.goto('/');
    const loadTime = Date.now() - startTime;
    
    // Page should load in under 3 seconds
    expect(loadTime).toBeLessThan(3000);
  });

  test('should have proper alt text for images', async ({ page }) => {
    await page.goto('/');
    
    // Check all images have alt text
    const images = page.locator('img');
    const imageCount = await images.count();
    
    if (imageCount > 0) {
      for (let i = 0; i < imageCount; i++) {
        const alt = await images.nth(i).getAttribute('alt');
        expect(alt).toBeTruthy();
      }
    }
  });
});
