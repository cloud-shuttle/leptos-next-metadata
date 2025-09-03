use playwright::Playwright;
use image_compare::{Algorithm, Metric, Similarity};
use std::path::Path;
use tokio_test;

#[tokio::test]
async fn test_og_image_visual_regression() {
    let playwright = Playwright::initialize().await.unwrap();
    let browser = playwright.chromium().launcher().headless(true).launch().await.unwrap();
    let context = browser.context_builder().build().await.unwrap();
    let page = context.new_page().await.unwrap();
    
    page.goto("http://localhost:3000/test-og-visual").await.unwrap();
    page.wait_for_load_state(playwright::api::LoadState::NetworkIdle).await.unwrap();
    
    // Get the OG image URL
    let og_image_url = page
        .query_selector("meta[property='og:image']").await.unwrap()
        .unwrap()
        .get_attribute("content").await.unwrap()
        .unwrap();
    
    // Download the generated image
    let response = page.request().get(&og_image_url).await.unwrap();
    let image_data = response.body().await.unwrap();
    
    // Save current image
    let current_path = "tests/fixtures/visual/og-image-current.png";
    std::fs::create_dir_all(Path::new(current_path).parent().unwrap()).unwrap();
    std::fs::write(current_path, &image_data).unwrap();
    
    // Load expected image
    let expected_path = "tests/fixtures/visual/og-image-expected.png";
    
    if Path::new(expected_path).exists() {
        let expected = image::open(expected_path).unwrap();
        let current = image::load_from_memory(&image_data).unwrap();
        
        // Compare images
        let result = image_compare::rgba_hybrid_compare(
            &expected.to_rgba8(),
            &current.to_rgba8(),
        ).unwrap();
        
        assert!(
            result.score > 0.95,
            "OG image visual regression detected: similarity score {}. \
             Check tests/fixtures/visual/og-image-current.png vs og-image-expected.png",
            result.score
        );
    } else {
        // First run - save as expected
        std::fs::copy(current_path, expected_path).unwrap();
        println!("Saved baseline image: {}", expected_path);
    }
    
    browser.close().await.unwrap();
}

#[tokio::test]
async fn test_different_template_visual_regression() {
    let playwright = Playwright::initialize().await.unwrap();
    let browser = playwright.chromium().launcher().headless(true).launch().await.unwrap();
    let context = browser.context_builder().build().await.unwrap();
    
    let templates = vec![
        ("default", "default template"),
        ("blog_post", "blog post template"),
        ("article", "article template"),
        ("simple", "simple template"),
    ];
    
    for (template_name, description) in templates {
        let page = context.new_page().await.unwrap();
        
        page.goto(&format!("http://localhost:3000/test-template/{}", template_name)).await.unwrap();
        page.wait_for_load_state(playwright::api::LoadState::NetworkIdle).await.unwrap();
        
        let og_image_url = page
            .query_selector("meta[property='og:image']").await.unwrap()
            .unwrap()
            .get_attribute("content").await.unwrap()
            .unwrap();
        
        let response = page.request().get(&og_image_url).await.unwrap();
        let image_data = response.body().await.unwrap();
        
        let current_path = format!("tests/fixtures/visual/og-{}-current.png", template_name);
        std::fs::write(&current_path, &image_data).unwrap();
        
        let expected_path = format!("tests/fixtures/visual/og-{}-expected.png", template_name);
        
        if Path::new(&expected_path).exists() {
            let expected = image::open(&expected_path).unwrap();
            let current = image::load_from_memory(&image_data).unwrap();
            
            let result = image_compare::rgba_hybrid_compare(
                &expected.to_rgba8(),
                &current.to_rgba8(),
            ).unwrap();
            
            assert!(
                result.score > 0.95,
                "Template {} visual regression: score {} ({})",
                template_name, result.score, description
            );
        } else {
            std::fs::copy(&current_path, &expected_path).unwrap();
            println!("Saved baseline for template {}: {}", template_name, expected_path);
        }
        
        page.close().await.unwrap();
    }
    
    browser.close().await.unwrap();
}

#[tokio::test]
async fn test_responsive_image_sizes() {
    let playwright = Playwright::initialize().await.unwrap();
    let browser = playwright.chromium().launcher().headless(true).launch().await.unwrap();
    let context = browser.context_builder().build().await.unwrap();
    let page = context.new_page().await.unwrap();
    
    let sizes = vec![
        (1200, 630, "og"),
        (1200, 600, "twitter-large"),
        (600, 314, "twitter-summary"),
        (800, 600, "custom"),
    ];
    
    for (width, height, size_name) in sizes {
        page.goto(&format!(
            "http://localhost:3000/test-responsive?w={}&h={}",
            width, height
        )).await.unwrap();
        page.wait_for_load_state(playwright::api::LoadState::NetworkIdle).await.unwrap();
        
        let og_image_url = page
            .query_selector("meta[property='og:image']").await.unwrap()
            .unwrap()
            .get_attribute("content").await.unwrap()
            .unwrap();
        
        let response = page.request().get(&og_image_url).await.unwrap();
        let image_data = response.body().await.unwrap();
        
        // Verify image dimensions
        let img = image::load_from_memory(&image_data).unwrap();
        assert_eq!(img.width(), width as u32);
        assert_eq!(img.height(), height as u32);
        
        // Save for visual comparison
        let current_path = format!("tests/fixtures/visual/size-{}-current.png", size_name);
        std::fs::write(&current_path, &image_data).unwrap();
        
        let expected_path = format!("tests/fixtures/visual/size-{}-expected.png", size_name);
        
        if Path::new(&expected_path).exists() {
            let expected = image::open(&expected_path).unwrap();
            let current = image::load_from_memory(&image_data).unwrap();
            
            let result = image_compare::rgba_hybrid_compare(
                &expected.to_rgba8(),
                &current.to_rgba8(),
            ).unwrap();
            
            assert!(
                result.score > 0.95,
                "Size {} visual regression: score {}",
                size_name, result.score
            );
        } else {
            std::fs::copy(&current_path, &expected_path).unwrap();
        }
    }
    
    browser.close().await.unwrap();
}

#[tokio::test]
async fn test_gradient_backgrounds() {
    let playwright = Playwright::initialize().await.unwrap();
    let browser = playwright.chromium().launcher().headless(true).launch().await.unwrap();
    let context = browser.context_builder().build().await.unwrap();
    let page = context.new_page().await.unwrap();
    
    let gradients = vec![
        ("blue-purple", "#667eea", "#764ba2"),
        ("red-orange", "#ff7b7b", "#ff8c42"),
        ("green-blue", "#56ccf2", "#2f80ed"),
        ("purple-pink", "#a8edea", "#fed6e3"),
    ];
    
    for (gradient_name, color1, color2) in gradients {
        page.goto(&format!(
            "http://localhost:3000/test-gradient?name={}&c1={}&c2={}",
            gradient_name,
            color1.trim_start_matches('#'),
            color2.trim_start_matches('#')
        )).await.unwrap();
        page.wait_for_load_state(playwright::api::LoadState::NetworkIdle).await.unwrap();
        
        let og_image_url = page
            .query_selector("meta[property='og:image']").await.unwrap()
            .unwrap()
            .get_attribute("content").await.unwrap()
            .unwrap();
        
        let response = page.request().get(&og_image_url).await.unwrap();
        let image_data = response.body().await.unwrap();
        
        let current_path = format!("tests/fixtures/visual/gradient-{}-current.png", gradient_name);
        std::fs::write(&current_path, &image_data).unwrap();
        
        let expected_path = format!("tests/fixtures/visual/gradient-{}-expected.png", gradient_name);
        
        if Path::new(&expected_path).exists() {
            let expected = image::open(&expected_path).unwrap();
            let current = image::load_from_memory(&image_data).unwrap();
            
            let result = image_compare::rgba_hybrid_compare(
                &expected.to_rgba8(),
                &current.to_rgba8(),
            ).unwrap();
            
            assert!(
                result.score > 0.90, // Slightly lower threshold for gradients
                "Gradient {} visual regression: score {}",
                gradient_name, result.score
            );
        } else {
            std::fs::copy(&current_path, &expected_path).unwrap();
        }
    }
    
    browser.close().await.unwrap();
}

#[tokio::test]
async fn test_text_rendering() {
    let playwright = Playwright::initialize().await.unwrap();
    let browser = playwright.chromium().launcher().headless(true).launch().await.unwrap();
    let context = browser.context_builder().build().await.unwrap();
    let page = context.new_page().await.unwrap();
    
    let test_cases = vec![
        ("short-title", "Short", "Brief description"),
        ("long-title", "This is a very long title that might wrap to multiple lines", "Standard description"),
        ("unicode", "Título en Español 🚀", "Description with émojis and åccents"),
        ("special-chars", "Title with \"quotes\" & symbols", "Description with <tags> & entities"),
    ];
    
    for (test_name, title, description) in test_cases {
        page.goto(&format!(
            "http://localhost:3000/test-text?title={}&description={}",
            urlencoding::encode(title),
            urlencoding::encode(description)
        )).await.unwrap();
        page.wait_for_load_state(playwright::api::LoadState::NetworkIdle).await.unwrap();
        
        let og_image_url = page
            .query_selector("meta[property='og:image']").await.unwrap()
            .unwrap()
            .get_attribute("content").await.unwrap()
            .unwrap();
        
        let response = page.request().get(&og_image_url).await.unwrap();
        let image_data = response.body().await.unwrap();
        
        let current_path = format!("tests/fixtures/visual/text-{}-current.png", test_name);
        std::fs::write(&current_path, &image_data).unwrap();
        
        let expected_path = format!("tests/fixtures/visual/text-{}-expected.png", test_name);
        
        if Path::new(&expected_path).exists() {
            let expected = image::open(&expected_path).unwrap();
            let current = image::load_from_memory(&image_data).unwrap();
            
            let result = image_compare::rgba_hybrid_compare(
                &expected.to_rgba8(),
                &current.to_rgba8(),
            ).unwrap();
            
            assert!(
                result.score > 0.95,
                "Text rendering regression for {}: score {}",
                test_name, result.score
            );
        } else {
            std::fs::copy(&current_path, &expected_path).unwrap();
        }
    }
    
    browser.close().await.unwrap();
}

#[tokio::test]
async fn test_logo_rendering() {
    let playwright = Playwright::initialize().await.unwrap();
    let browser = playwright.chromium().launcher().headless(true).launch().await.unwrap();
    let context = browser.context_builder().build().await.unwrap();
    let page = context.new_page().await.unwrap();
    
    page.goto("http://localhost:3000/test-logo").await.unwrap();
    page.wait_for_load_state(playwright::api::LoadState::NetworkIdle).await.unwrap();
    
    let og_image_url = page
        .query_selector("meta[property='og:image']").await.unwrap()
        .unwrap()
        .get_attribute("content").await.unwrap()
        .unwrap();
    
    let response = page.request().get(&og_image_url).await.unwrap();
    let image_data = response.body().await.unwrap();
    
    let current_path = "tests/fixtures/visual/logo-current.png";
    std::fs::write(current_path, &image_data).unwrap();
    
    let expected_path = "tests/fixtures/visual/logo-expected.png";
    
    if Path::new(expected_path).exists() {
        let expected = image::open(expected_path).unwrap();
        let current = image::load_from_memory(&image_data).unwrap();
        
        let result = image_compare::rgba_hybrid_compare(
            &expected.to_rgba8(),
            &current.to_rgba8(),
        ).unwrap();
        
        assert!(
            result.score > 0.95,
            "Logo rendering regression: score {}",
            result.score
        );
    } else {
        std::fs::copy(current_path, expected_path).unwrap();
    }
    
    browser.close().await.unwrap();
}

#[tokio::test]
async fn test_cross_browser_visual_consistency() {
    let playwright = Playwright::initialize().await.unwrap();
    
    let browsers = vec![
        ("chromium", playwright.chromium().launcher().headless(true).launch().await.unwrap()),
        ("firefox", playwright.firefox().launcher().headless(true).launch().await.unwrap()),
        ("webkit", playwright.webkit().launcher().headless(true).launch().await.unwrap()),
    ];
    
    let test_url = "http://localhost:3000/test-cross-browser-visual";
    let mut browser_images = Vec::new();
    
    for (browser_name, browser) in browsers {
        let context = browser.context_builder().build().await.unwrap();
        let page = context.new_page().await.unwrap();
        
        page.goto(test_url).await.unwrap();
        page.wait_for_load_state(playwright::api::LoadState::NetworkIdle).await.unwrap();
        
        let og_image_url = page
            .query_selector("meta[property='og:image']").await.unwrap()
            .unwrap()
            .get_attribute("content").await.unwrap()
            .unwrap();
        
        let response = page.request().get(&og_image_url).await.unwrap();
        let image_data = response.body().await.unwrap();
        
        let path = format!("tests/fixtures/visual/cross-browser-{}.png", browser_name);
        std::fs::write(&path, &image_data).unwrap();
        
        browser_images.push((browser_name, image_data));
        browser.close().await.unwrap();
    }
    
    // Compare images between browsers - they should be very similar
    if browser_images.len() >= 2 {
        let chrome_img = image::load_from_memory(&browser_images[0].1).unwrap();
        let firefox_img = image::load_from_memory(&browser_images[1].1).unwrap();
        
        let result = image_compare::rgba_hybrid_compare(
            &chrome_img.to_rgba8(),
            &firefox_img.to_rgba8(),
        ).unwrap();
        
        assert!(
            result.score > 0.98, // Very high threshold for cross-browser consistency
            "Cross-browser visual inconsistency: Chrome vs Firefox score {}",
            result.score
        );
    }
}

#[tokio::test]
async fn test_dynamic_content_visual_stability() {
    let playwright = Playwright::initialize().await.unwrap();
    let browser = playwright.chromium().launcher().headless(true).launch().await.unwrap();
    let context = browser.context_builder().build().await.unwrap();
    let page = context.new_page().await.unwrap();
    
    // Test that dynamic content generates consistent images
    let iterations = 5;
    let mut images = Vec::new();
    
    for i in 0..iterations {
        page.goto(&format!(
            "http://localhost:3000/test-dynamic-visual?seed={}",
            i  // Same seed should produce same content
        )).await.unwrap();
        page.wait_for_load_state(playwright::api::LoadState::NetworkIdle).await.unwrap();
        
        let og_image_url = page
            .query_selector("meta[property='og:image']").await.unwrap()
            .unwrap()
            .get_attribute("content").await.unwrap()
            .unwrap();
        
        let response = page.request().get(&og_image_url).await.unwrap();
        let image_data = response.body().await.unwrap();
        
        images.push(image_data);
    }
    
    // All images should be identical (for same seed)
    let reference = image::load_from_memory(&images[0]).unwrap();
    
    for (i, img_data) in images.iter().enumerate().skip(1) {
        let current = image::load_from_memory(img_data).unwrap();
        
        let result = image_compare::rgba_hybrid_compare(
            &reference.to_rgba8(),
            &current.to_rgba8(),
        ).unwrap();
        
        assert_eq!(
            result.score, 1.0,
            "Dynamic content not stable: iteration {} score {}",
            i, result.score
        );
    }
    
    browser.close().await.unwrap();
}

#[tokio::test]
async fn test_edge_case_visual_handling() {
    let playwright = Playwright::initialize().await.unwrap();
    let browser = playwright.chromium().launcher().headless(true).launch().await.unwrap();
    let context = browser.context_builder().build().await.unwrap();
    
    let edge_cases = vec![
        ("empty-title", "", "Normal description"),
        ("empty-description", "Normal title", ""),
        ("both-empty", "", ""),
        ("very-long", "x".repeat(200), "y".repeat(500)),
        ("newlines", "Title\nWith\nNewlines", "Description\nWith\nBreaks"),
        ("html-entities", "Title with &lt;tags&gt;", "Description &amp; symbols"),
    ];
    
    for (case_name, title, description) in edge_cases {
        let page = context.new_page().await.unwrap();
        
        page.goto(&format!(
            "http://localhost:3000/test-edge-cases?title={}&description={}",
            urlencoding::encode(title),
            urlencoding::encode(description)
        )).await.unwrap();
        page.wait_for_load_state(playwright::api::LoadState::NetworkIdle).await.unwrap();
        
        let og_image_url = page
            .query_selector("meta[property='og:image']").await.unwrap()
            .unwrap()
            .get_attribute("content").await.unwrap()
            .unwrap();
        
        let response = page.request().get(&og_image_url).await.unwrap();
        assert_eq!(response.status(), 200);
        
        let image_data = response.body().await.unwrap();
        assert!(image_data.len() > 1000); // Should still generate valid image
        
        // Verify it's a valid image
        let img = image::load_from_memory(&image_data);
        assert!(img.is_ok(), "Edge case {} produced invalid image", case_name);
        
        let current_path = format!("tests/fixtures/visual/edge-{}-current.png", case_name);
        std::fs::write(&current_path, &image_data).unwrap();
        
        page.close().await.unwrap();
    }
    
    browser.close().await.unwrap();
}