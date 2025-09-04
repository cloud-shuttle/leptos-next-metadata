import { FullConfig } from '@playwright/test';

async function globalTeardown(config: FullConfig) {
  console.log('🧹 Cleaning up Playwright test environment...');
  
  // Add any cleanup logic here if needed
  // For example, cleaning up test data, stopping services, etc.
  
  console.log('✅ Playwright environment cleanup complete!');
}

export default globalTeardown;
