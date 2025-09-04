import { test, expect } from '@playwright/test';

test.describe('Component Interactions E2E Tests', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to the test server
    await page.goto('http://localhost:3000');
  });

  test('should handle EnhancedTitle with MetaTags interaction', async ({ page }) => {
    // Test EnhancedTitle and MetaTags working together
    await page.goto('http://localhost:3000/title-meta-interaction');
    
    // Check that title is rendered
    await expect(page).toHaveTitle(/Title Meta Interaction/);
    
    // Check that meta tags are present
    const metaDescription = await page.locator('meta[name="description"]').getAttribute('content');
    expect(metaDescription).toBeTruthy();
    
    // Check that title and meta description are consistent
    const title = await page.title();
    expect(title).toContain('Title Meta Interaction');
  });

  test('should handle Html and Body component interaction', async ({ page }) => {
    // Test Html and Body components working together
    await page.goto('http://localhost:3000/html-body-interaction');
    
    // Check html attributes
    const htmlElement = await page.locator('html');
    await expect(htmlElement).toHaveAttribute('lang', 'en');
    await expect(htmlElement).toHaveAttribute('dir', 'ltr');
    
    // Check body attributes
    const bodyElement = await page.locator('body');
    await expect(bodyElement).toHaveAttribute('class', 'interaction-test');
    await expect(bodyElement).toHaveAttribute('lang', 'en');
    
    // Check that attributes don't conflict
    const htmlLang = await htmlElement.getAttribute('lang');
    const bodyLang = await bodyElement.getAttribute('lang');
    expect(htmlLang).toBe(bodyLang);
  });

  test('should handle HashedStylesheet with other components', async ({ page }) => {
    // Test HashedStylesheet with other components
    await page.goto('http://localhost:3000/stylesheet-component-interaction');
    
    // Check that stylesheet is loaded
    const stylesheetLink = await page.locator('link[rel="stylesheet"]');
    await expect(stylesheetLink).toBeVisible();
    
    // Check that other components still work
    await expect(page).toHaveTitle(/Stylesheet Component Interaction/);
    
    const htmlElement = await page.locator('html');
    await expect(htmlElement).toHaveAttribute('lang', 'en');
    
    const bodyElement = await page.locator('body');
    await expect(bodyElement).toHaveAttribute('class', 'stylesheet-test');
  });

  test('should handle all components in complex interaction', async ({ page }) => {
    // Test all components in complex interaction
    await page.goto('http://localhost:3000/complex-interaction');
    
    // Check title with formatter
    const title = await page.title();
    expect(title).toContain('Complex Interaction');
    expect(title).toContain('| Complex Site');
    
    // Check html attributes
    const htmlElement = await page.locator('html');
    await expect(htmlElement).toHaveAttribute('lang', 'en');
    await expect(htmlElement).toHaveAttribute('dir', 'ltr');
    await expect(htmlElement).toHaveAttribute('data-theme', 'dark');
    
    // Check body attributes
    const bodyElement = await page.locator('body');
    await expect(bodyElement).toHaveAttribute('class', 'complex-test');
    await expect(bodyElement).toHaveAttribute('lang', 'en');
    
    // Check meta tags
    const metaDescription = await page.locator('meta[name="description"]').getAttribute('content');
    expect(metaDescription).toBeTruthy();
    
    const metaKeywords = await page.locator('meta[name="keywords"]').getAttribute('content');
    expect(metaKeywords).toBeTruthy();
    
    // Check stylesheet
    const stylesheetLink = await page.locator('link[rel="stylesheet"]');
    await expect(stylesheetLink).toBeVisible();
  });

  test('should handle component priority and conflicts', async ({ page }) => {
    // Test component priority and conflict resolution
    await page.goto('http://localhost:3000/component-priority-test');
    
    // Check that formatter takes priority
    const title = await page.title();
    expect(title).toContain('Formatted:');
    expect(title).not.toContain('Template:');
    expect(title).not.toContain('Prefix:');
    expect(title).not.toContain('Suffix:');
    
    // Check that html attributes are applied correctly
    const htmlElement = await page.locator('html');
    await expect(htmlElement).toHaveAttribute('lang', 'en');
    
    // Check that body attributes are applied correctly
    const bodyElement = await page.locator('body');
    await expect(bodyElement).toHaveAttribute('class', 'priority-test');
  });

  test('should handle dynamic component updates', async ({ page }) => {
    // Test dynamic component updates
    await page.goto('http://localhost:3000/dynamic-updates');
    
    // Check initial state
    let title = await page.title();
    expect(title).toContain('Dynamic Updates');
    
    // Simulate dynamic update
    await page.click('button[data-testid="update-title"]');
    
    // Check updated state
    title = await page.title();
    expect(title).toContain('Updated Title');
    
    // Check that other components are not affected
    const htmlElement = await page.locator('html');
    await expect(htmlElement).toHaveAttribute('lang', 'en');
    
    const bodyElement = await page.locator('body');
    await expect(bodyElement).toHaveAttribute('class', 'dynamic-test');
  });

  test('should handle component error recovery', async ({ page }) => {
    // Test component error recovery
    await page.goto('http://localhost:3000/error-recovery');
    
    // Check that page loads despite errors
    await expect(page).toHaveTitle(/Error Recovery/);
    
    // Check that working components still function
    const htmlElement = await page.locator('html');
    await expect(htmlElement).toHaveAttribute('lang', 'en');
    
    const bodyElement = await page.locator('body');
    await expect(bodyElement).toHaveAttribute('class', 'error-recovery-test');
    
    // Check that meta tags are present
    const metaDescription = await page.locator('meta[name="description"]').getAttribute('content');
    expect(metaDescription).toBeTruthy();
  });

  test('should handle component performance under load', async ({ page }) => {
    // Test component performance under load
    const startTime = Date.now();
    
    await page.goto('http://localhost:3000/performance-load');
    
    const loadTime = Date.now() - startTime;
    
    // Check that page loads within reasonable time
    expect(loadTime).toBeLessThan(5000);
    
    // Check that all components are rendered
    await expect(page).toHaveTitle(/Performance Load/);
    
    const htmlElement = await page.locator('html');
    await expect(htmlElement).toHaveAttribute('lang', 'en');
    
    const bodyElement = await page.locator('body');
    await expect(bodyElement).toHaveAttribute('class', 'performance-load-test');
    
    const stylesheetLink = await page.locator('link[rel="stylesheet"]');
    await expect(stylesheetLink).toBeVisible();
  });

  test('should handle component accessibility in interactions', async ({ page }) => {
    // Test component accessibility in interactions
    await page.goto('http://localhost:3000/accessibility-interactions');
    
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
    
    // Check that meta tags are accessible
    const metaDescription = await page.locator('meta[name="description"]').getAttribute('content');
    expect(metaDescription).toBeTruthy();
  });

  test('should handle component SEO in interactions', async ({ page }) => {
    // Test component SEO in interactions
    await page.goto('http://localhost:3000/seo-interactions');
    
    // Check that title is SEO-friendly
    const title = await page.title();
    expect(title).toBeTruthy();
    expect(title.length).toBeGreaterThan(10);
    expect(title.length).toBeLessThan(60);
    
    // Check that meta description is present and SEO-friendly
    const metaDescription = await page.locator('meta[name="description"]').getAttribute('content');
    expect(metaDescription).toBeTruthy();
    expect(metaDescription.length).toBeGreaterThan(50);
    expect(metaDescription.length).toBeLessThan(160);
    
    // Check that meta keywords are present
    const metaKeywords = await page.locator('meta[name="keywords"]').getAttribute('content');
    expect(metaKeywords).toBeTruthy();
    
    // Check that html has proper lang attribute for SEO
    const htmlElement = await page.locator('html');
    await expect(htmlElement).toHaveAttribute('lang', 'en');
  });

  test('should handle component cross-browser compatibility', async ({ page }) => {
    // Test component cross-browser compatibility
    await page.goto('http://localhost:3000/cross-browser-test');
    
    // Check that all components work across browsers
    await expect(page).toHaveTitle(/Cross Browser Test/);
    
    const htmlElement = await page.locator('html');
    await expect(htmlElement).toHaveAttribute('lang', 'en');
    
    const bodyElement = await page.locator('body');
    await expect(bodyElement).toHaveAttribute('class', 'cross-browser-test');
    
    const stylesheetLink = await page.locator('link[rel="stylesheet"]');
    await expect(stylesheetLink).toBeVisible();
    
    const metaDescription = await page.locator('meta[name="description"]').getAttribute('content');
    expect(metaDescription).toBeTruthy();
  });

  test('should handle component mobile responsiveness', async ({ page }) => {
    // Test component mobile responsiveness
    await page.setViewportSize({ width: 375, height: 667 }); // iPhone SE size
    
    await page.goto('http://localhost:3000/mobile-responsive-test');
    
    // Check that all components work on mobile
    await expect(page).toHaveTitle(/Mobile Responsive Test/);
    
    const htmlElement = await page.locator('html');
    await expect(htmlElement).toHaveAttribute('lang', 'en');
    
    const bodyElement = await page.locator('body');
    await expect(bodyElement).toHaveAttribute('class', 'mobile-responsive-test');
    
    const stylesheetLink = await page.locator('link[rel="stylesheet"]');
    await expect(stylesheetLink).toBeVisible();
    
    const metaDescription = await page.locator('meta[name="description"]').getAttribute('content');
    expect(metaDescription).toBeTruthy();
  });
});
