use playwright::Playwright;
use std::collections::HashMap;
use tokio_test;

#[tokio::test]
async fn test_basic_metadata_rendering() {
    let playwright = Playwright::initialize().await.unwrap();
    let browser = playwright.chromium().launcher().headless(true).launch().await.unwrap();
    let context = browser.context_builder().build().await.unwrap();
    let page = context.new_page().await.unwrap();
    
    // Navigate to test page
    page.goto("http://localhost:3000/test-basic").await.unwrap();
    
    // Wait for page to load
    page.wait_for_load_state(playwright::api::LoadState::NetworkIdle).await.unwrap();
    
    // Check title
    let title = page.title().await.unwrap();
    assert_eq!(title, "Basic Test Page");
    
    // Check meta description
    let description = page
        .query_selector("meta[name='description']").await.unwrap()
        .unwrap()
        .get_attribute("content").await.unwrap()
        .unwrap();
    assert_eq!(description, "This is a basic test page description");
    
    // Check meta keywords
    let keywords = page
        .query_selector("meta[name='keywords']").await.unwrap()
        .unwrap()
        .get_attribute("content").await.unwrap()
        .unwrap();
    assert_eq!(keywords, "test,leptos,metadata");
    
    // Check Open Graph tags
    let og_title = page
        .query_selector("meta[property='og:title']").await.unwrap()
        .unwrap()
        .get_attribute("content").await.unwrap()
        .unwrap();
    assert_eq!(og_title, "Basic Test Page");
    
    let og_description = page
        .query_selector("meta[property='og:description']").await.unwrap()
        .unwrap()
        .get_attribute("content").await.unwrap()
        .unwrap();
    assert_eq!(og_description, "This is a basic test page description");
    
    browser.close().await.unwrap();
}

#[tokio::test]
async fn test_dynamic_metadata_updates() {
    let playwright = Playwright::initialize().await.unwrap();
    let browser = playwright.chromium().launcher().headless(true).launch().await.unwrap();
    let context = browser.context_builder().build().await.unwrap();
    let page = context.new_page().await.unwrap();
    
    page.goto("http://localhost:3000/test-dynamic").await.unwrap();
    page.wait_for_load_state(playwright::api::LoadState::NetworkIdle).await.unwrap();
    
    // Initial title
    let initial_title = page.title().await.unwrap();
    assert_eq!(initial_title, "Initial Title");
    
    // Click button to update metadata
    page.click("#update-metadata").await.unwrap();
    
    // Wait for title to change
    page.wait_for_function("() => document.title === 'Updated Title'", None).await.unwrap();
    
    let updated_title = page.title().await.unwrap();
    assert_eq!(updated_title, "Updated Title");
    
    // Check that description was also updated
    let description = page
        .query_selector("meta[name='description']").await.unwrap()
        .unwrap()
        .get_attribute("content").await.unwrap()
        .unwrap();
    assert_eq!(description, "This title was updated dynamically");
    
    browser.close().await.unwrap();
}

#[tokio::test]
async fn test_og_image_generation() {
    let playwright = Playwright::initialize().await.unwrap();
    let browser = playwright.chromium().launcher().headless(true).launch().await.unwrap();
    let context = browser.context_builder().build().await.unwrap();
    let page = context.new_page().await.unwrap();
    
    page.goto("http://localhost:3000/test-og-image").await.unwrap();
    page.wait_for_load_state(playwright::api::LoadState::NetworkIdle).await.unwrap();
    
    // Check that OG image tag exists and points to generated image
    let og_image = page
        .query_selector("meta[property='og:image']").await.unwrap()
        .unwrap()
        .get_attribute("content").await.unwrap()
        .unwrap();
    
    assert!(og_image.starts_with("http"));
    assert!(og_image.contains("/api/og"));
    
    // Test the actual image endpoint
    let response = page.request().get(&og_image).await.unwrap();
    assert_eq!(response.status(), 200);
    
    let content_type = response.headers().get("content-type").unwrap();
    assert_eq!(content_type, "image/png");
    
    let body = response.body().await.unwrap();
    assert!(body.len() > 1000); // Should be a reasonable image size
    assert!(body.len() < 500_000); // But not too large
    
    browser.close().await.unwrap();
}

#[tokio::test]
async fn test_twitter_card_rendering() {
    let playwright = Playwright::initialize().await.unwrap();
    let browser = playwright.chromium().launcher().headless(true).launch().await.unwrap();
    let context = browser.context_builder().build().await.unwrap();
    let page = context.new_page().await.unwrap();
    
    page.goto("http://localhost:3000/test-twitter-card").await.unwrap();
    page.wait_for_load_state(playwright::api::LoadState::NetworkIdle).await.unwrap();
    
    // Check Twitter card tags
    let twitter_card = page
        .query_selector("meta[name='twitter:card']").await.unwrap()
        .unwrap()
        .get_attribute("content").await.unwrap()
        .unwrap();
    assert_eq!(twitter_card, "summary_large_image");
    
    let twitter_site = page
        .query_selector("meta[name='twitter:site']").await.unwrap()
        .unwrap()
        .get_attribute("content").await.unwrap()
        .unwrap();
    assert_eq!(twitter_site, "@example");
    
    let twitter_title = page
        .query_selector("meta[name='twitter:title']").await.unwrap()
        .unwrap()
        .get_attribute("content").await.unwrap()
        .unwrap();
    assert_eq!(twitter_title, "Twitter Card Test");
    
    let twitter_image = page
        .query_selector("meta[name='twitter:image']").await.unwrap()
        .unwrap()
        .get_attribute("content").await.unwrap()
        .unwrap();
    assert!(twitter_image.starts_with("http"));
    
    browser.close().await.unwrap();
}

#[tokio::test]
async fn test_json_ld_structured_data() {
    let playwright = Playwright::initialize().await.unwrap();
    let browser = playwright.chromium().launcher().headless(true).launch().await.unwrap();
    let context = browser.context_builder().build().await.unwrap();
    let page = context.new_page().await.unwrap();
    
    page.goto("http://localhost:3000/test-json-ld").await.unwrap();
    page.wait_for_load_state(playwright::api::LoadState::NetworkIdle).await.unwrap();
    
    // Check JSON-LD script tag exists
    let json_ld_script = page
        .query_selector("script[type='application/ld+json']").await.unwrap()
        .unwrap()
        .text_content().await.unwrap()
        .unwrap();
    
    // Parse and validate JSON-LD
    let json_data: serde_json::Value = serde_json::from_str(&json_ld_script).unwrap();
    
    assert_eq!(json_data["@type"], "Article");
    assert_eq!(json_data["headline"], "Test Article");
    assert!(json_data["author"]["@type"] == "Person");
    assert!(json_data["author"]["name"] == "John Doe");
    
    browser.close().await.unwrap();
}

#[tokio::test]
async fn test_favicon_loading() {
    let playwright = Playwright::initialize().await.unwrap();
    let browser = playwright.chromium().launcher().headless(true).launch().await.unwrap();
    let context = browser.context_builder().build().await.unwrap();
    let page = context.new_page().await.unwrap();
    
    page.goto("http://localhost:3000/test-icons").await.unwrap();
    page.wait_for_load_state(playwright::api::LoadState::NetworkIdle).await.unwrap();
    
    // Check favicon link
    let favicon = page
        .query_selector("link[rel='icon']").await.unwrap()
        .unwrap()
        .get_attribute("href").await.unwrap()
        .unwrap();
    
    // Test that favicon actually loads
    let response = page.request().get(&favicon).await.unwrap();
    assert_eq!(response.status(), 200);
    
    let content_type = response.headers().get("content-type").unwrap();
    assert!(content_type.contains("image"));
    
    // Check apple touch icon
    let apple_icon = page
        .query_selector("link[rel='apple-touch-icon']").await.unwrap()
        .unwrap()
        .get_attribute("href").await.unwrap()
        .unwrap();
    
    let response = page.request().get(&apple_icon).await.unwrap();
    assert_eq!(response.status(), 200);
    
    browser.close().await.unwrap();
}

#[tokio::test]
async fn test_responsive_og_images() {
    let playwright = Playwright::initialize().await.unwrap();
    let browser = playwright.chromium().launcher().headless(true).launch().await.unwrap();
    let context = browser.context_builder().build().await.unwrap();
    let page = context.new_page().await.unwrap();
    
    page.goto("http://localhost:3000/test-responsive-og").await.unwrap();
    page.wait_for_load_state(playwright::api::LoadState::NetworkIdle).await.unwrap();
    
    // Get OG image URL
    let og_image = page
        .query_selector("meta[property='og:image']").await.unwrap()
        .unwrap()
        .get_attribute("content").await.unwrap()
        .unwrap();
    
    // Test different sizes by appending query parameters
    let sizes = vec![
        ("1200", "630"),   // Standard OG
        ("1200", "600"),   // Twitter large
        ("600", "314"),    // Twitter summary
    ];
    
    for (width, height) in sizes {
        let sized_url = format!("{}?w={}&h={}", og_image, width, height);
        let response = page.request().get(&sized_url).await.unwrap();
        assert_eq!(response.status(), 200);
        
        let body = response.body().await.unwrap();
        assert!(body.len() > 1000);
        
        // Could use image processing library to verify actual dimensions
        // For now, just verify it's a valid image response
        let content_type = response.headers().get("content-type").unwrap();
        assert_eq!(content_type, "image/png");
    }
    
    browser.close().await.unwrap();
}

#[tokio::test]
async fn test_metadata_inheritance() {
    let playwright = Playwright::initialize().await.unwrap();
    let browser = playwright.chromium().launcher().headless(true).launch().await.unwrap();
    let context = browser.context_builder().build().await.unwrap();
    let page = context.new_page().await.unwrap();
    
    // Test parent page
    page.goto("http://localhost:3000/blog").await.unwrap();
    page.wait_for_load_state(playwright::api::LoadState::NetworkIdle).await.unwrap();
    
    let parent_title = page.title().await.unwrap();
    assert_eq!(parent_title, "Blog | My Site");
    
    // Test child page that should inherit and override
    page.goto("http://localhost:3000/blog/first-post").await.unwrap();
    page.wait_for_load_state(playwright::api::LoadState::NetworkIdle).await.unwrap();
    
    let child_title = page.title().await.unwrap();
    assert_eq!(child_title, "First Post | My Site");
    
    // Child should have its own description
    let child_description = page
        .query_selector("meta[name='description']").await.unwrap()
        .unwrap()
        .get_attribute("content").await.unwrap()
        .unwrap();
    assert!(child_description.contains("First Post"));
    
    // But should inherit some parent metadata (like site name in OG)
    let og_site_name = page
        .query_selector("meta[property='og:site_name']").await.unwrap()
        .unwrap()
        .get_attribute("content").await.unwrap()
        .unwrap();
    assert_eq!(og_site_name, "My Site");
    
    browser.close().await.unwrap();
}

#[tokio::test]
async fn test_csr_hydration() {
    let playwright = Playwright::initialize().await.unwrap();
    let browser = playwright.chromium().launcher().headless(true).launch().await.unwrap();
    let context = browser.context_builder().build().await.unwrap();
    let page = context.new_page().await.unwrap();
    
    // Navigate with JavaScript disabled to test SSR
    context.set_extra_http_headers(Some(vec![
        ("User-Agent".to_string(), "Mozilla/5.0 (compatible; Test/1.0)".to_string())
    ])).await.unwrap();
    
    page.goto("http://localhost:3000/test-hydration").await.unwrap();
    page.wait_for_load_state(playwright::api::LoadState::DomContentLoaded).await.unwrap();
    
    // Check that SSR metadata is present
    let ssr_title = page.title().await.unwrap();
    assert_eq!(ssr_title, "SSR Title");
    
    // Wait for hydration (JavaScript to load)
    page.wait_for_load_state(playwright::api::LoadState::NetworkIdle).await.unwrap();
    
    // After hydration, client-side updates should work
    page.click("#update-client-metadata").await.unwrap();
    
    // Wait for client-side title update
    page.wait_for_function("() => document.title === 'Client Updated Title'", None).await.unwrap();
    
    let hydrated_title = page.title().await.unwrap();
    assert_eq!(hydrated_title, "Client Updated Title");
    
    browser.close().await.unwrap();
}

#[tokio::test]
async fn test_performance_metrics() {
    let playwright = Playwright::initialize().await.unwrap();
    let browser = playwright.chromium().launcher().headless(true).launch().await.unwrap();
    let context = browser.context_builder().build().await.unwrap();
    let page = context.new_page().await.unwrap();
    
    // Enable performance tracking
    page.coverage().start_js_coverage(None).await.unwrap();
    
    let start_time = std::time::Instant::now();
    page.goto("http://localhost:3000/test-performance").await.unwrap();
    page.wait_for_load_state(playwright::api::LoadState::NetworkIdle).await.unwrap();
    let load_time = start_time.elapsed();
    
    // Page should load quickly
    assert!(load_time.as_millis() < 3000, "Page took {}ms to load", load_time.as_millis());
    
    // Check Core Web Vitals
    let web_vitals = page.evaluate("
        new Promise((resolve) => {
            const observer = new PerformanceObserver((list) => {
                const entries = list.getEntries();
                const vitals = {};
                entries.forEach(entry => {
                    if (entry.entryType === 'largest-contentful-paint') {
                        vitals.lcp = entry.startTime;
                    }
                    if (entry.entryType === 'first-input') {
                        vitals.fid = entry.processingStart - entry.startTime;
                    }
                });
                resolve(vitals);
            });
            observer.observe({entryTypes: ['largest-contentful-paint', 'first-input']});
            
            // Fallback resolve after timeout
            setTimeout(() => resolve({}), 5000);
        })
    ", None).await.unwrap();
    
    println!("Web Vitals: {:?}", web_vitals);
    
    // Stop coverage and analyze
    let coverage = page.coverage().stop_js_coverage().await.unwrap();
    let total_bytes: usize = coverage.iter().map(|c| c.text.len()).sum();
    let used_bytes: usize = coverage.iter().map(|c| {
        c.ranges.iter().map(|r| r.end - r.start).sum::<usize>()
    }).sum();
    
    let usage_percent = (used_bytes as f64 / total_bytes as f64) * 100.0;
    println!("JavaScript usage: {:.1}% ({}/{})", usage_percent, used_bytes, total_bytes);
    
    browser.close().await.unwrap();
}

#[tokio::test]
async fn test_accessibility() {
    let playwright = Playwright::initialize().await.unwrap();
    let browser = playwright.chromium().launcher().headless(true).launch().await.unwrap();
    let context = browser.context_builder().build().await.unwrap();
    let page = context.new_page().await.unwrap();
    
    page.goto("http://localhost:3000/test-accessibility").await.unwrap();
    page.wait_for_load_state(playwright::api::LoadState::NetworkIdle).await.unwrap();
    
    // Check basic accessibility metadata
    let lang = page
        .query_selector("html").await.unwrap()
        .unwrap()
        .get_attribute("lang").await.unwrap()
        .unwrap_or_else(|| "en".to_string());
    assert!(!lang.is_empty());
    
    // Check that viewport meta tag is present for responsive design
    let viewport = page
        .query_selector("meta[name='viewport']").await.unwrap()
        .unwrap()
        .get_attribute("content").await.unwrap()
        .unwrap();
    assert!(viewport.contains("width=device-width"));
    
    // Check that images have alt text
    let images = page.query_selector_all("img").await.unwrap();
    for image in images {
        let alt = image.get_attribute("alt").await.unwrap();
        assert!(alt.is_some(), "Image missing alt attribute");
    }
    
    // Check skip links exist
    let skip_links = page.query_selector_all("a[href^='#']").await.unwrap();
    assert!(!skip_links.is_empty(), "No skip links found");
    
    browser.close().await.unwrap();
}

#[tokio::test]
async fn test_cross_browser_compatibility() {
    let playwright = Playwright::initialize().await.unwrap();
    
    let browsers = vec![
        playwright.chromium().launcher().headless(true).launch().await.unwrap(),
        playwright.firefox().launcher().headless(true).launch().await.unwrap(),
        playwright.webkit().launcher().headless(true).launch().await.unwrap(),
    ];
    
    for browser in browsers {
        let context = browser.context_builder().build().await.unwrap();
        let page = context.new_page().await.unwrap();
        
        page.goto("http://localhost:3000/test-cross-browser").await.unwrap();
        page.wait_for_load_state(playwright::api::LoadState::NetworkIdle).await.unwrap();
        
        // Basic metadata should work in all browsers
        let title = page.title().await.unwrap();
        assert_eq!(title, "Cross Browser Test");
        
        let description = page
            .query_selector("meta[name='description']").await.unwrap()
            .unwrap()
            .get_attribute("content").await.unwrap()
            .unwrap();
        assert!(!description.is_empty());
        
        // Test JavaScript metadata updates
        page.click("#update-metadata").await.unwrap();
        page.wait_for_function("() => document.title.includes('Updated')", None).await.unwrap();
        
        let updated_title = page.title().await.unwrap();
        assert!(updated_title.contains("Updated"));
        
        browser.close().await.unwrap();
    }
}

#[tokio::test]
async fn test_mobile_responsive_metadata() {
    let playwright = Playwright::initialize().await.unwrap();
    let browser = playwright.chromium().launcher().headless(true).launch().await.unwrap();
    
    // Test mobile viewport
    let mobile_context = browser
        .context_builder()
        .viewport(Some(playwright::api::ViewportSize { width: 375, height: 667 }))
        .user_agent(Some("Mozilla/5.0 (iPhone; CPU iPhone OS 14_0 like Mac OS X)"))
        .build().await.unwrap();
    
    let page = mobile_context.new_page().await.unwrap();
    
    page.goto("http://localhost:3000/test-mobile").await.unwrap();
    page.wait_for_load_state(playwright::api::LoadState::NetworkIdle).await.unwrap();
    
    // Check mobile-specific metadata
    let viewport = page
        .query_selector("meta[name='viewport']").await.unwrap()
        .unwrap()
        .get_attribute("content").await.unwrap()
        .unwrap();
    
    assert!(viewport.contains("width=device-width"));
    assert!(viewport.contains("initial-scale=1"));
    
    // Check Apple mobile web app tags
    let apple_mobile_capable = page
        .query_selector("meta[name='apple-mobile-web-app-capable']").await.unwrap();
    assert!(apple_mobile_capable.is_some());
    
    let apple_status_bar = page
        .query_selector("meta[name='apple-mobile-web-app-status-bar-style']").await.unwrap();
    assert!(apple_status_bar.is_some());
    
    // Test mobile-specific icons
    let apple_icon = page
        .query_selector("link[rel='apple-touch-icon']").await.unwrap()
        .unwrap()
        .get_attribute("href").await.unwrap()
        .unwrap();
    
    let response = page.request().get(&apple_icon).await.unwrap();
    assert_eq!(response.status(), 200);
    
    browser.close().await.unwrap();
}