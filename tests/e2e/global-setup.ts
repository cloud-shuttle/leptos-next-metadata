import { chromium, FullConfig } from "@playwright/test";

async function globalSetup(config: FullConfig) {
  const { baseURL } = config.projects[0].use;

  console.log("🚀 Setting up Playwright test environment...");
  console.log(`📍 Base URL: ${baseURL}`);

  // Verify that browsers are available
  const browser = await chromium.launch();
  const page = await browser.newPage();

  try {
    // Test basic browser functionality
    await page.goto("data:text/html,<html><body><h1>Test</h1></body></html>");
    const title = await page.title();
    console.log("✅ Browser test passed");

    // Test if we can access the test server
    if (baseURL && baseURL !== "http://localhost:3000") {
      try {
        await page.goto(baseURL);
        console.log(`✅ Test server accessible at ${baseURL}`);
      } catch (error) {
        console.log(
          `⚠️  Test server not accessible at ${baseURL} (this is expected if not running)`,
        );
      }
    }
  } catch (error) {
    console.error("❌ Browser test failed:", error);
    throw error;
  } finally {
    await browser.close();
  }

  console.log("🎭 Playwright environment setup complete!");
}

export default globalSetup;
