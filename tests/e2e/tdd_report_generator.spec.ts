import { test, expect } from '@playwright/test';

// TDD Phase 2: Comprehensive Report Generator Test
// This test validates that our report generator can actually run tests and generate reports

test.describe('TDD Report Generator Validation', () => {
  test('should generate a basic test report', async ({ page }) => {
    // Given: A test page is accessible
    await page.goto('/', { waitUntil: 'domcontentloaded', timeout: 10000 });
    
    // When: We run a simple test
    const title = await page.title();
    const metaCount = await page.locator('meta').count();
    
    // Then: Basic metadata should be present
    expect(title).toBe('Welcome to My Site');
    expect(metaCount).toBeGreaterThan(5);
  });

  test('should validate OpenGraph metadata structure', async ({ page }) => {
    // Given: A test page is loaded
    await page.goto('/', { waitUntil: 'domcontentloaded', timeout: 10000 });
    
    // When: We check OpenGraph metadata
    const ogTitle = await page.locator('meta[property="og:title"]').getAttribute('content');
    const ogDescription = await page.locator('meta[property="og:description"]').getAttribute('content');
    const ogType = await page.locator('meta[property="og:type"]').getAttribute('content');
    
    // Then: OpenGraph metadata should be properly structured
    expect(ogTitle).toBe('Welcome to My Site');
    expect(ogDescription).toBe('A blazing fast Leptos application with comprehensive metadata management');
    expect(ogType).toBe('website');
  });

  test('should validate Twitter Card metadata structure', async ({ page }) => {
    // Given: A test page is loaded
    await page.goto('/', { waitUntil: 'domcontentloaded', timeout: 10000 });
    
    // When: We check Twitter Card metadata
    const twitterCard = await page.locator('meta[name="twitter:card"]').getAttribute('content');
    const twitterTitle = await page.locator('meta[name="twitter:title"]').getAttribute('content');
    const twitterDescription = await page.locator('meta[name="twitter:description"]').getAttribute('content');
    
    // Then: Twitter Card metadata should be properly structured
    expect(twitterCard).toBe('summary_large_image');
    expect(twitterTitle).toBe('Welcome to My Site');
    expect(twitterDescription).toBe('A blazing fast Leptos application with comprehensive metadata management');
  });

  test('should validate JSON-LD structured data', async ({ page }) => {
    // Given: A test page is loaded
    await page.goto('/', { waitUntil: 'domcontentloaded', timeout: 10000 });
    
    // When: We check JSON-LD data
    const jsonLdScript = await page.locator('script[type="application/ld+json"]').innerHTML();
    
    // Then: JSON-LD should be valid and contain required fields
    expect(jsonLdScript).toBeTruthy();
    
    // Parse and validate JSON-LD structure
    const jsonLd = JSON.parse(jsonLdScript);
    expect(jsonLd['@context']).toBe('https://schema.org');
    expect(jsonLd['@type']).toBe('WebPage');
    expect(jsonLd.name).toBe('Welcome to My Site');
  });

  test('should validate SEO requirements', async ({ page }) => {
    // Given: A test page is loaded
    await page.goto('/', { waitUntil: 'domcontentloaded', timeout: 10000 });
    
    // When: We check SEO elements
    const title = await page.title();
    const description = await page.locator('meta[name="description"]').getAttribute('content');
    const canonical = await page.locator('link[rel="canonical"]').getAttribute('href');
    
    // Then: SEO requirements should be met
    expect(title.length).toBeGreaterThanOrEqual(10);
    expect(title.length).toBeLessThanOrEqual(60);
    expect(description?.length).toBeGreaterThanOrEqual(50);
    expect(description?.length).toBeLessThanOrEqual(160);
    expect(canonical).toMatch(/^https?:\/\//);
  });
});
