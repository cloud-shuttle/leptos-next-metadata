import { test, expect } from '@playwright/test';

test.describe('Component Performance E2E Tests', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to the test server
    await page.goto('http://localhost:3000');
  });

  test('should load EnhancedTitle component quickly', async ({ page }) => {
    // Test EnhancedTitle component performance
    const startTime = Date.now();
    
    await page.goto('http://localhost:3000/enhanced-title-performance');
    
    const loadTime = Date.now() - startTime;
    
    // Check that page loads within 2 seconds
    expect(loadTime).toBeLessThan(2000);
    
    // Check that title is rendered
    await expect(page).toHaveTitle(/Enhanced Title Performance/);
  });

  test('should load MetaTags component quickly', async ({ page }) => {
    // Test MetaTags component performance
    const startTime = Date.now();
    
    await page.goto('http://localhost:3000/meta-tags-performance');
    
    const loadTime = Date.now() - startTime;
    
    // Check that page loads within 2 seconds
    expect(loadTime).toBeLessThan(2000);
    
    // Check that meta tags are present
    const metaDescription = await page.locator('meta[name="description"]').getAttribute('content');
    expect(metaDescription).toBeTruthy();
  });

  test('should load Html component quickly', async ({ page }) => {
    // Test Html component performance
    const startTime = Date.now();
    
    await page.goto('http://localhost:3000/html-component-performance');
    
    const loadTime = Date.now() - startTime;
    
    // Check that page loads within 2 seconds
    expect(loadTime).toBeLessThan(2000);
    
    // Check that html attributes are applied
    const htmlElement = await page.locator('html');
    await expect(htmlElement).toHaveAttribute('lang', 'en');
  });

  test('should load Body component quickly', async ({ page }) => {
    // Test Body component performance
    const startTime = Date.now();
    
    await page.goto('http://localhost:3000/body-component-performance');
    
    const loadTime = Date.now() - startTime;
    
    // Check that page loads within 2 seconds
    expect(loadTime).toBeLessThan(2000);
    
    // Check that body attributes are applied
    const bodyElement = await page.locator('body');
    await expect(bodyElement).toHaveAttribute('class', 'performance-test');
  });

  test('should load HashedStylesheet component quickly', async ({ page }) => {
    // Test HashedStylesheet component performance
    const startTime = Date.now();
    
    await page.goto('http://localhost:3000/hashed-stylesheet-performance');
    
    const loadTime = Date.now() - startTime;
    
    // Check that page loads within 2 seconds
    expect(loadTime).toBeLessThan(2000);
    
    // Check that stylesheet is loaded
    const stylesheetLink = await page.locator('link[rel="stylesheet"]');
    await expect(stylesheetLink).toBeVisible();
  });

  test('should load all components together quickly', async ({ page }) => {
    // Test all components performance together
    const startTime = Date.now();
    
    await page.goto('http://localhost:3000/all-components-performance');
    
    const loadTime = Date.now() - startTime;
    
    // Check that page loads within 3 seconds
    expect(loadTime).toBeLessThan(3000);
    
    // Check that all components are rendered
    await expect(page).toHaveTitle(/All Components Performance/);
    
    const htmlElement = await page.locator('html');
    await expect(htmlElement).toHaveAttribute('lang', 'en');
    
    const bodyElement = await page.locator('body');
    await expect(bodyElement).toHaveAttribute('class', 'all-components-performance');
    
    const stylesheetLink = await page.locator('link[rel="stylesheet"]');
    await expect(stylesheetLink).toBeVisible();
    
    const metaDescription = await page.locator('meta[name="description"]').getAttribute('content');
    expect(metaDescription).toBeTruthy();
  });

  test('should handle multiple EnhancedTitle formatters efficiently', async ({ page }) => {
    // Test multiple EnhancedTitle formatters performance
    const startTime = Date.now();
    
    await page.goto('http://localhost:3000/multiple-formatters-performance');
    
    const loadTime = Date.now() - startTime;
    
    // Check that page loads within 2 seconds
    expect(loadTime).toBeLessThan(2000);
    
    // Check that title is rendered correctly
    const title = await page.title();
    expect(title).toContain('Multiple Formatters Performance');
  });

  test('should handle complex component interactions efficiently', async ({ page }) => {
    // Test complex component interactions performance
    const startTime = Date.now();
    
    await page.goto('http://localhost:3000/complex-interactions-performance');
    
    const loadTime = Date.now() - startTime;
    
    // Check that page loads within 3 seconds
    expect(loadTime).toBeLessThan(3000);
    
    // Check that all components work together
    await expect(page).toHaveTitle(/Complex Interactions Performance/);
    
    const htmlElement = await page.locator('html');
    await expect(htmlElement).toHaveAttribute('lang', 'en');
    
    const bodyElement = await page.locator('body');
    await expect(bodyElement).toHaveAttribute('class', 'complex-interactions-performance');
    
    const stylesheetLink = await page.locator('link[rel="stylesheet"]');
    await expect(stylesheetLink).toBeVisible();
  });

  test('should handle dynamic updates efficiently', async ({ page }) => {
    // Test dynamic updates performance
    await page.goto('http://localhost:3000/dynamic-updates-performance');
    
    // Check initial load time
    const initialTitle = await page.title();
    expect(initialTitle).toContain('Dynamic Updates Performance');
    
    // Test dynamic update performance
    const updateStartTime = Date.now();
    
    await page.click('button[data-testid="update-title"]');
    
    const updateTime = Date.now() - updateStartTime;
    
    // Check that update happens within 1 second
    expect(updateTime).toBeLessThan(1000);
    
    // Check that title is updated
    const updatedTitle = await page.title();
    expect(updatedTitle).toContain('Updated Title');
  });

  test('should handle large datasets efficiently', async ({ page }) => {
    // Test large datasets performance
    const startTime = Date.now();
    
    await page.goto('http://localhost:3000/large-datasets-performance');
    
    const loadTime = Date.now() - startTime;
    
    // Check that page loads within 5 seconds
    expect(loadTime).toBeLessThan(5000);
    
    // Check that all components are rendered
    await expect(page).toHaveTitle(/Large Datasets Performance/);
    
    const htmlElement = await page.locator('html');
    await expect(htmlElement).toHaveAttribute('lang', 'en');
    
    const bodyElement = await page.locator('body');
    await expect(bodyElement).toHaveAttribute('class', 'large-datasets-performance');
  });

  test('should handle concurrent component rendering efficiently', async ({ page }) => {
    // Test concurrent component rendering performance
    const startTime = Date.now();
    
    await page.goto('http://localhost:3000/concurrent-rendering-performance');
    
    const loadTime = Date.now() - startTime;
    
    // Check that page loads within 3 seconds
    expect(loadTime).toBeLessThan(3000);
    
    // Check that all components are rendered
    await expect(page).toHaveTitle(/Concurrent Rendering Performance/);
    
    const htmlElement = await page.locator('html');
    await expect(htmlElement).toHaveAttribute('lang', 'en');
    
    const bodyElement = await page.locator('body');
    await expect(bodyElement).toHaveAttribute('class', 'concurrent-rendering-performance');
    
    const stylesheetLink = await page.locator('link[rel="stylesheet"]');
    await expect(stylesheetLink).toBeVisible();
  });

  test('should handle memory usage efficiently', async ({ page }) => {
    // Test memory usage performance
    const startTime = Date.now();
    
    await page.goto('http://localhost:3000/memory-usage-performance');
    
    const loadTime = Date.now() - startTime;
    
    // Check that page loads within 3 seconds
    expect(loadTime).toBeLessThan(3000);
    
    // Check that all components are rendered
    await expect(page).toHaveTitle(/Memory Usage Performance/);
    
    const htmlElement = await page.locator('html');
    await expect(htmlElement).toHaveAttribute('lang', 'en');
    
    const bodyElement = await page.locator('body');
    await expect(bodyElement).toHaveAttribute('class', 'memory-usage-performance');
    
    // Check that memory usage is reasonable (no memory leaks)
    const memoryUsage = await page.evaluate(() => {
      if (performance.memory) {
        return performance.memory.usedJSHeapSize;
      }
      return 0;
    });
    
    // Check that memory usage is reasonable (less than 50MB)
    expect(memoryUsage).toBeLessThan(50 * 1024 * 1024);
  });

  test('should handle network latency efficiently', async ({ page }) => {
    // Test network latency performance
    const startTime = Date.now();
    
    // Simulate slow network
    await page.route('**/*', route => {
      setTimeout(() => route.continue(), 100);
    });
    
    await page.goto('http://localhost:3000/network-latency-performance');
    
    const loadTime = Date.now() - startTime;
    
    // Check that page loads within 5 seconds even with network latency
    expect(loadTime).toBeLessThan(5000);
    
    // Check that all components are rendered
    await expect(page).toHaveTitle(/Network Latency Performance/);
    
    const htmlElement = await page.locator('html');
    await expect(htmlElement).toHaveAttribute('lang', 'en');
    
    const bodyElement = await page.locator('body');
    await expect(bodyElement).toHaveAttribute('class', 'network-latency-performance');
  });

  test('should handle browser compatibility efficiently', async ({ page }) => {
    // Test browser compatibility performance
    const startTime = Date.now();
    
    await page.goto('http://localhost:3000/browser-compatibility-performance');
    
    const loadTime = Date.now() - startTime;
    
    // Check that page loads within 3 seconds
    expect(loadTime).toBeLessThan(3000);
    
    // Check that all components are rendered
    await expect(page).toHaveTitle(/Browser Compatibility Performance/);
    
    const htmlElement = await page.locator('html');
    await expect(htmlElement).toHaveAttribute('lang', 'en');
    
    const bodyElement = await page.locator('body');
    await expect(bodyElement).toHaveAttribute('class', 'browser-compatibility-performance');
    
    const stylesheetLink = await page.locator('link[rel="stylesheet"]');
    await expect(stylesheetLink).toBeVisible();
  });
});
