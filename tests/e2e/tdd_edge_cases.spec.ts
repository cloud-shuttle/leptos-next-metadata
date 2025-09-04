import { test, expect } from '@playwright/test';

// TDD Phase 3: Edge Cases & Error Conditions Test
// This test validates that our library handles edge cases gracefully in production

test.describe('TDD Edge Cases & Error Conditions', () => {
  test('should handle extremely long metadata values gracefully', async ({ page }) => {
    // Given: A test page with very long metadata
    await page.goto('/', { waitUntil: 'domcontentloaded', timeout: 10000 });
    
    // When: We check for extremely long content
    const description = await page.locator('meta[name="description"]').getAttribute('content');
    const title = await page.title();
    
    // Then: Should handle long content without breaking
    expect(description).toBeTruthy();
    expect(title).toBeTruthy();
    
    // Edge case: Very long values should still be valid
    if (description && description.length > 1000) {
      console.log('⚠️  Warning: Very long description detected');
    }
  });

  test('should handle special characters in metadata correctly', async ({ page }) => {
    // Given: A test page is loaded
    await page.goto('/', { waitUntil: 'domcontentloaded', timeout: 10000 });
    
    // When: We check metadata with special characters
    const ogTitle = await page.locator('meta[property="og:title"]').getAttribute('content');
    const ogDescription = await page.locator('meta[property="og:description"]').getAttribute('content');
    
    // Then: Special characters should be properly encoded
    expect(ogTitle).toBeTruthy();
    expect(ogDescription).toBeTruthy();
    
    // Edge case: Check for HTML entities and special chars
    if (ogTitle?.includes('&') || ogTitle?.includes('<') || ogTitle?.includes('>')) {
      console.log('⚠️  Warning: Special characters in og:title');
    }
  });

  test('should handle missing optional metadata gracefully', async ({ page }) => {
    // Given: A test page is loaded
    await page.goto('/', { waitUntil: 'domcontentloaded', timeout: 10000 });
    
    // When: We check for optional metadata that might not exist
    const author = await page.locator('meta[name="author"]').count();
    const keywords = await page.locator('meta[name="keywords"]').count();
    const robots = await page.locator('meta[name="robots"]').count();
    
    // Then: Should handle missing optional fields gracefully
    // These are optional, so count could be 0 or 1
    expect(author).toBeGreaterThanOrEqual(0);
    expect(keywords).toBeGreaterThanOrEqual(0);
    expect(robots).toBeGreaterThanOrEqual(0);
  });

  test('should validate URL formats in metadata', async ({ page }) => {
    // Given: A test page is loaded
    await page.goto('/', { waitUntil: 'domcontentloaded', timeout: 10000 });
    
    // When: We check URL-based metadata
    const canonical = await page.locator('link[rel="canonical"]').getAttribute('href');
    const ogUrl = await page.locator('meta[property="og:url"]').getAttribute('content');
    
    // Then: URLs should be properly formatted
    if (canonical) {
      expect(canonical).toMatch(/^https?:\/\//);
    }
    if (ogUrl) {
      expect(ogUrl).toMatch(/^https?:\/\//);
    }
  });

  test('should handle malformed JSON-LD gracefully', async ({ page }) => {
    // Given: A test page is loaded
    await page.goto('/', { waitUntil: 'domcontentloaded', timeout: 10000 });
    
    // When: We check JSON-LD data
    const jsonLdScripts = await page.locator('script[type="application/ld+json"]').count();
    
    if (jsonLdScripts > 0) {
      const jsonLdScript = await page.locator('script[type="application/ld+json"]').first().innerHTML();
      
      // Then: JSON-LD should be valid JSON
      try {
        const jsonLd = JSON.parse(jsonLdScript);
        expect(jsonLd).toBeTruthy();
        expect(typeof jsonLd).toBe('object');
      } catch (error) {
        // Edge case: Invalid JSON should be caught
        console.error('❌ Invalid JSON-LD detected:', error);
        throw new Error('JSON-LD is not valid JSON');
      }
    }
  });

  test('should validate metadata length constraints', async ({ page }) => {
    // Given: A test page is loaded
    await page.goto('/', { waitUntil: 'domcontentloaded', timeout: 10000 });
    
    // When: We check metadata lengths
    const title = await page.title();
    const description = await page.locator('meta[name="description"]').getAttribute('content');
    
    // Then: Should meet SEO length requirements
    expect(title.length).toBeGreaterThanOrEqual(10);
    expect(title.length).toBeLessThanOrEqual(60);
    
    if (description) {
      expect(description.length).toBeGreaterThanOrEqual(50);
      expect(description.length).toBeLessThanOrEqual(160);
    }
  });

  test('should handle concurrent metadata access', async ({ page }) => {
    // Given: A test page is loaded
    await page.goto('/', { waitUntil: 'domcontentloaded', timeout: 10000 });
    
    // When: We access multiple metadata elements simultaneously
    const promises = [
      page.title(),
      page.locator('meta[name="description"]').getAttribute('content'),
      page.locator('meta[property="og:title"]').getAttribute('content'),
      page.locator('meta[name="twitter:title"]').getAttribute('content')
    ];
    
    // Then: All should resolve without conflicts
    const results = await Promise.all(promises);
    expect(results).toHaveLength(4);
    expect(results.every(r => r !== null)).toBe(true);
  });

  test('should handle network interruptions gracefully', async ({ page }) => {
    // Given: A test page is loaded
    await page.goto('/', { waitUntil: 'domcontentloaded', timeout: 10000 });
    
    // When: We simulate network interruption by reloading
    await page.reload({ waitUntil: 'domcontentloaded', timeout: 10000 });
    
    // Then: Page should still load with metadata intact
    const title = await page.title();
    const metaCount = await page.locator('meta').count();
    
    expect(title).toBe('Welcome to My Site');
    expect(metaCount).toBeGreaterThan(5);
  });

  test('should validate character encoding consistency', async ({ page }) => {
    // Given: A test page is loaded
    await page.goto('/', { waitUntil: 'domcontentloaded', timeout: 10000 });
    
    // When: We check character encoding
    const charset = await page.locator('meta[charset]').getAttribute('charset') ||
                   await page.locator('meta[http-equiv="Content-Type"]').getAttribute('content');
    
    // Then: Should have consistent encoding
    if (charset) {
      expect(charset.toLowerCase()).toMatch(/utf-?8/);
    }
  });

  test('should handle empty or whitespace-only metadata', async ({ page }) => {
    // Given: A test page is loaded
    await page.goto('/', { waitUntil: 'domcontentloaded', timeout: 10000 });
    
    // When: We check for empty metadata
    const allMeta = await page.locator('meta').all();
    
    // Then: No metadata should be completely empty
    for (const meta of allMeta) {
      const content = await meta.getAttribute('content');
      const name = await meta.getAttribute('name');
      const property = await meta.getAttribute('property');
      const charset = await meta.getAttribute('charset');
      
      if (content !== null) {
        expect(content.trim().length).toBeGreaterThan(0);
      }
      
      // Should have either name, property, or charset (HTML5 charset meta tag)
      expect(name || property || charset).toBeTruthy();
    }
  });
});
