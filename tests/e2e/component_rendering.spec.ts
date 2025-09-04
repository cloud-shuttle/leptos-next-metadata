import { test, expect } from '@playwright/test';

test.describe('Component Rendering E2E Tests', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to the test server
    await page.goto('http://localhost:3000');
  });

  test('should render EnhancedTitle component with formatter', async ({ page }) => {
    // Test EnhancedTitle component rendering
    await page.goto('http://localhost:3000/enhanced-title-test');
    
    // Check that the title is rendered correctly
    await expect(page).toHaveTitle(/Enhanced Title Test/);
    
    // Check that the title contains the formatted text
    const title = await page.title();
    expect(title).toContain('Enhanced Title Test');
  });

  test('should render MetaTags component for SSR', async ({ page }) => {
    // Test MetaTags component rendering
    await page.goto('http://localhost:3000/meta-tags-test');
    
    // Check that meta tags are present
    const metaDescription = await page.locator('meta[name="description"]').getAttribute('content');
    expect(metaDescription).toBeTruthy();
    
    const metaKeywords = await page.locator('meta[name="keywords"]').getAttribute('content');
    expect(metaKeywords).toBeTruthy();
  });

  test('should render Html component with attributes', async ({ page }) => {
    // Test Html component rendering
    await page.goto('http://localhost:3000/html-component-test');
    
    // Check that html element has correct attributes
    const htmlElement = await page.locator('html');
    await expect(htmlElement).toHaveAttribute('lang', 'en');
    await expect(htmlElement).toHaveAttribute('dir', 'ltr');
  });

  test('should render Body component with attributes', async ({ page }) => {
    // Test Body component rendering
    await page.goto('http://localhost:3000/body-component-test');
    
    // Check that body element has correct attributes
    const bodyElement = await page.locator('body');
    await expect(bodyElement).toHaveAttribute('class', 'test-body');
    await expect(bodyElement).toHaveAttribute('lang', 'en');
  });

  test('should render HashedStylesheet component', async ({ page }) => {
    // Test HashedStylesheet component rendering
    await page.goto('http://localhost:3000/hashed-stylesheet-test');
    
    // Check that stylesheet link is present
    const stylesheetLink = await page.locator('link[rel="stylesheet"]');
    await expect(stylesheetLink).toBeVisible();
    
    // Check that the href contains the expected path
    const href = await stylesheetLink.getAttribute('href');
    expect(href).toContain('.css');
  });

  test('should render all components together', async ({ page }) => {
    // Test all components working together
    await page.goto('http://localhost:3000/all-components-test');
    
    // Check title
    await expect(page).toHaveTitle(/All Components Test/);
    
    // Check html attributes
    const htmlElement = await page.locator('html');
    await expect(htmlElement).toHaveAttribute('lang', 'en');
    
    // Check body attributes
    const bodyElement = await page.locator('body');
    await expect(bodyElement).toHaveAttribute('class', 'all-components');
    
    // Check meta tags
    const metaDescription = await page.locator('meta[name="description"]').getAttribute('content');
    expect(metaDescription).toBeTruthy();
    
    // Check stylesheet
    const stylesheetLink = await page.locator('link[rel="stylesheet"]');
    await expect(stylesheetLink).toBeVisible();
  });

  test('should handle EnhancedTitle formatter priority', async ({ page }) => {
    // Test EnhancedTitle formatter priority
    await page.goto('http://localhost:3000/title-formatter-priority-test');
    
    // Check that formatter takes priority over template/prefix/suffix
    const title = await page.title();
    expect(title).toContain('Formatted:');
    expect(title).not.toContain('Template:');
    expect(title).not.toContain('Prefix:');
    expect(title).not.toContain('Suffix:');
  });

  test('should handle EnhancedTitle template priority', async ({ page }) => {
    // Test EnhancedTitle template priority
    await page.goto('http://localhost:3000/title-template-priority-test');
    
    // Check that template takes priority over prefix/suffix when no formatter
    const title = await page.title();
    expect(title).toContain('Template:');
    expect(title).not.toContain('Prefix:');
    expect(title).not.toContain('Suffix:');
  });

  test('should handle EnhancedTitle prefix and suffix', async ({ page }) => {
    // Test EnhancedTitle prefix and suffix
    await page.goto('http://localhost:3000/title-prefix-suffix-test');
    
    // Check that prefix and suffix are applied
    const title = await page.title();
    expect(title).toContain('Welcome to');
    expect(title).toContain('| My Site');
  });

  test('should handle edge cases gracefully', async ({ page }) => {
    // Test edge cases
    await page.goto('http://localhost:3000/edge-cases-test');
    
    // Check that empty title is handled
    const title = await page.title();
    expect(title).toBeTruthy();
    
    // Check that long title is handled
    expect(title.length).toBeLessThan(100);
    
    // Check that special characters are handled
    expect(title).not.toContain('<');
    expect(title).not.toContain('>');
  });

  test('should handle dynamic title updates', async ({ page }) => {
    // Test dynamic title updates
    await page.goto('http://localhost:3000/dynamic-title-test');
    
    // Check initial title
    let title = await page.title();
    expect(title).toContain('Dynamic Title');
    
    // Simulate dynamic update (if applicable)
    await page.click('button[data-testid="update-title"]');
    
    // Check updated title
    title = await page.title();
    expect(title).toContain('Updated Title');
  });

  test('should handle component error states', async ({ page }) => {
    // Test component error states
    await page.goto('http://localhost:3000/error-states-test');
    
    // Check that page still loads even with errors
    await expect(page).toHaveTitle(/Error States Test/);
    
    // Check that error handling is graceful
    const bodyElement = await page.locator('body');
    await expect(bodyElement).toBeVisible();
  });

  test('should handle component performance', async ({ page }) => {
    // Test component performance
    const startTime = Date.now();
    
    await page.goto('http://localhost:3000/performance-test');
    
    const loadTime = Date.now() - startTime;
    
    // Check that page loads within reasonable time
    expect(loadTime).toBeLessThan(5000);
    
    // Check that all components are rendered
    await expect(page).toHaveTitle(/Performance Test/);
    
    const htmlElement = await page.locator('html');
    await expect(htmlElement).toHaveAttribute('lang', 'en');
    
    const bodyElement = await page.locator('body');
    await expect(bodyElement).toHaveAttribute('class', 'performance-test');
  });

  test('should handle component accessibility', async ({ page }) => {
    // Test component accessibility
    await page.goto('http://localhost:3000/accessibility-test');
    
    // Check that html has lang attribute
    const htmlElement = await page.locator('html');
    await expect(htmlElement).toHaveAttribute('lang', 'en');
    
    // Check that body has appropriate attributes
    const bodyElement = await page.locator('body');
    await expect(bodyElement).toHaveAttribute('lang', 'en');
    
    // Check that title is descriptive
    const title = await page.title();
    expect(title).toBeTruthy();
    expect(title.length).toBeGreaterThan(5);
  });

  test('should handle component SEO', async ({ page }) => {
    // Test component SEO
    await page.goto('http://localhost:3000/seo-test');
    
    // Check that title is SEO-friendly
    const title = await page.title();
    expect(title).toBeTruthy();
    expect(title.length).toBeGreaterThan(10);
    expect(title.length).toBeLessThan(60);
    
    // Check that meta description is present
    const metaDescription = await page.locator('meta[name="description"]').getAttribute('content');
    expect(metaDescription).toBeTruthy();
    expect(metaDescription.length).toBeGreaterThan(50);
    expect(metaDescription.length).toBeLessThan(160);
    
    // Check that meta keywords are present
    const metaKeywords = await page.locator('meta[name="keywords"]').getAttribute('content');
    expect(metaKeywords).toBeTruthy();
  });
});
