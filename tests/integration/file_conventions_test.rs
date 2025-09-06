use leptos_next_metadata::conventions::*;
use tempfile::TempDir;
use std::fs;
use std::path::Path;
use pretty_assertions::assert_eq;

#[test]
fn test_favicon_detection() {
    let temp_dir = TempDir::new().unwrap();
    let app_dir = temp_dir.path().join("app");
    fs::create_dir(&app_dir).unwrap();

    // Create favicon.ico
    fs::write(app_dir.join("favicon.ico"), b"fake ico data").unwrap();

    let scanner = ConventionScanner::new(app_dir);
    let results = scanner.scan().unwrap();

    assert_eq!(results.len(), 1);
    match &results[0] {
        ConventionMetadata::Icon {
            path,
            rel,
            sizes,
            media_type
        } => {
            assert!(path.ends_with("favicon.ico"));
            assert_eq!(rel, "icon");
            assert_eq!(sizes, &None);
            assert_eq!(media_type.as_deref(), Some("image/x-icon"));
        }
        _ => panic!("Expected Icon, got {:?}", results[0]),
    }
}

#[test]
fn test_icon_detection_priority() {
    let temp_dir = TempDir::new().unwrap();
    let app_dir = temp_dir.path().join("app");
    fs::create_dir_all(&app_dir).unwrap();

    // Create multiple icons in order of decreasing priority
    fs::write(app_dir.join("apple-icon.png"), b"apple png").unwrap();
    fs::write(app_dir.join("icon.png"), b"icon png").unwrap();
    fs::write(app_dir.join("icon.svg"), b"<svg></svg>").unwrap();
    fs::write(app_dir.join("favicon.ico"), b"ico data").unwrap();

    let scanner = ConventionScanner::new(app_dir);
    let results = scanner.scan().unwrap();

    // Should be sorted by priority: apple-touch-icon > icon > favicon
    assert_eq!(results.len(), 4);

    let types: Vec<_> = results.iter().map(|r| match r {
        ConventionMetadata::Icon { rel, .. } => rel.as_str(),
        _ => panic!("Expected Icon"),
    }).collect();

    // Apple touch icon should come first (highest priority)
    assert_eq!(types[0], "apple-touch-icon");
    // Then regular icons
    assert!(types[1..].contains(&"icon"));
    assert!(types[1..].contains(&"icon"));
    assert!(types[1..].contains(&"icon")); // favicon.ico gets rel="icon"
}

#[test]
fn test_icon_with_sizes() {
    let temp_dir = TempDir::new().unwrap();
    let app_dir = temp_dir.path().join("app");
    fs::create_dir_all(&app_dir).unwrap();

    // Create icons with size indicators in filename
    fs::write(app_dir.join("icon-16x16.png"), b"16px icon").unwrap();
    fs::write(app_dir.join("icon-32x32.png"), b"32px icon").unwrap();
    fs::write(app_dir.join("apple-icon-180x180.png"), b"apple icon").unwrap();

    let scanner = ConventionScanner::new(app_dir);
    let results = scanner.scan().unwrap();

    assert_eq!(results.len(), 3);

    // Find the 32x32 icon
    let icon_32 = results.iter().find(|r| match r {
        ConventionMetadata::Icon { path, .. } => path.contains("32x32"),
        _ => false,
    }).unwrap();

    match icon_32 {
        ConventionMetadata::Icon { sizes, .. } => {
            assert_eq!(sizes.as_deref(), Some("32x32"));
        }
        _ => panic!("Expected Icon"),
    }

    // Find the apple icon
    let apple_icon = results.iter().find(|r| match r {
        ConventionMetadata::Icon { rel, .. } => rel == "apple-touch-icon",
        _ => false,
    }).unwrap();

    match apple_icon {
        ConventionMetadata::Icon { sizes, .. } => {
            assert_eq!(sizes.as_deref(), Some("180x180"));
        }
        _ => panic!("Expected Icon"),
    }
}

#[test]
fn test_manifest_detection() {
    let temp_dir = TempDir::new().unwrap();
    let app_dir = temp_dir.path().join("app");
    fs::create_dir_all(&app_dir).unwrap();

    let manifest_content = r#"{
        "name": "Test App",
        "short_name": "Test",
        "start_url": "/",
        "display": "standalone"
    }"#;

    fs::write(app_dir.join("manifest.json"), manifest_content).unwrap();

    let scanner = ConventionScanner::new(app_dir);
    let results = scanner.scan().unwrap();

    assert_eq!(results.len(), 1);
    match &results[0] {
        ConventionMetadata::Manifest { path } => {
            assert!(path.ends_with("manifest.json"));
        }
        _ => panic!("Expected Manifest, got {:?}", results[0]),
    }
}

#[test]
fn test_sitemap_detection() {
    let temp_dir = TempDir::new().unwrap();
    let app_dir = temp_dir.path().join("app");
    fs::create_dir_all(&app_dir).unwrap();

    let sitemap_content = r#"<?xml version="1.0" encoding="UTF-8"?>
    <urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
        <url><loc>https://example.com/</loc></url>
    </urlset>"#;

    fs::write(app_dir.join("sitemap.xml"), sitemap_content).unwrap();

    let scanner = ConventionScanner::new(app_dir);
    let results = scanner.scan().unwrap();

    assert_eq!(results.len(), 1);
    match &results[0] {
        ConventionMetadata::Sitemap { path } => {
            assert!(path.ends_with("sitemap.xml"));
        }
        _ => panic!("Expected Sitemap, got {:?}", results[0]),
    }
}

#[test]
fn test_robots_txt_detection() {
    let temp_dir = TempDir::new().unwrap();
    let app_dir = temp_dir.path().join("app");
    fs::create_dir_all(&app_dir).unwrap();

    let robots_content = r#"User-agent: *
Allow: /
Sitemap: https://example.com/sitemap.xml"#;

    fs::write(app_dir.join("robots.txt"), robots_content).unwrap();

    let scanner = ConventionScanner::new(app_dir);
    let results = scanner.scan().unwrap();

    assert_eq!(results.len(), 1);
    match &results[0] {
        ConventionMetadata::Robots { path } => {
            assert!(path.ends_with("robots.txt"));
        }
        _ => panic!("Expected Robots, got {:?}", results[0]),
    }
}

#[test]
fn test_opengraph_image_detection() {
    let temp_dir = TempDir::new().unwrap();
    let app_dir = temp_dir.path().join("app");
    fs::create_dir_all(&app_dir).unwrap();

    // Create OG images
    fs::write(app_dir.join("opengraph-image.png"), b"og image").unwrap();
    fs::write(app_dir.join("opengraph-image.jpg"), b"og image jpg").unwrap();

    let scanner = ConventionScanner::new(app_dir);
    let results = scanner.scan().unwrap();

    assert_eq!(results.len(), 2);

    let og_images: Vec<_> = results.iter().filter_map(|r| match r {
        ConventionMetadata::OpenGraphImage { path, media_type } => Some((path, media_type)),
        _ => None,
    }).collect();

    assert_eq!(og_images.len(), 2);

    // PNG should come first (higher priority)
    assert!(og_images[0].0.ends_with(".png"));
    assert_eq!(og_images[0].1.as_deref(), Some("image/png"));

    assert!(og_images[1].0.ends_with(".jpg"));
    assert_eq!(og_images[1].1.as_deref(), Some("image/jpeg"));
}

#[test]
fn test_twitter_image_detection() {
    let temp_dir = TempDir::new().unwrap();
    let app_dir = temp_dir.path().join("app");
    fs::create_dir_all(&app_dir).unwrap();

    fs::write(app_dir.join("twitter-image.png"), b"twitter image").unwrap();

    let scanner = ConventionScanner::new(app_dir);
    let results = scanner.scan().unwrap();

    assert_eq!(results.len(), 1);
    match &results[0] {
        ConventionMetadata::TwitterImage { path, media_type } => {
            assert!(path.ends_with("twitter-image.png"));
            assert_eq!(media_type.as_deref(), Some("image/png"));
        }
        _ => panic!("Expected TwitterImage, got {:?}", results[0]),
    }
}

#[test]
fn test_nested_directory_scanning() {
    let temp_dir = TempDir::new().unwrap();
    let app_dir = temp_dir.path().join("app");
    let blog_dir = app_dir.join("blog");
    let post_dir = blog_dir.join("first-post");
    fs::create_dir_all(&post_dir).unwrap();

    // Root level
    fs::write(app_dir.join("favicon.ico"), b"root favicon").unwrap();
    fs::write(app_dir.join("manifest.json"), r#"{"name": "App"}"#).unwrap();

    // Blog level
    fs::write(blog_dir.join("opengraph-image.png"), b"blog og").unwrap();

    // Post level
    fs::write(post_dir.join("opengraph-image.jpg"), b"post og").unwrap();
    fs::write(post_dir.join("twitter-image.png"), b"post twitter").unwrap();

    let scanner = ConventionScanner::new(app_dir);
    let results = scanner.scan().unwrap();

    // Should find all files
    assert_eq!(results.len(), 5);

    let paths: Vec<_> = results.iter().map(|r| r.path().to_string_lossy().to_string()).collect();
    assert!(paths.iter().any(|p| p.ends_with("favicon.ico")));
    assert!(paths.iter().any(|p| p.ends_with("manifest.json")));
    assert!(paths.iter().any(|p| p.contains("blog") && p.ends_with("opengraph-image.png")));
    assert!(paths.iter().any(|p| p.contains("first-post") && p.ends_with("opengraph-image.jpg")));
    assert!(paths.iter().any(|p| p.contains("first-post") && p.ends_with("twitter-image.png")));
}

#[test]
fn test_convention_priority_resolution() {
    let temp_dir = TempDir::new().unwrap();
    let app_dir = temp_dir.path().join("app");
    let nested_dir = app_dir.join("blog").join("post");
    fs::create_dir_all(&nested_dir).unwrap();

    // Create same-type files at different levels
    fs::write(app_dir.join("opengraph-image.png"), b"root og").unwrap();
    fs::write(nested_dir.join("opengraph-image.jpg"), b"nested og").unwrap();

    // Scan from nested directory - should prefer closer file
    let scanner = ConventionScanner::new(&nested_dir);
    let results = scanner.scan().unwrap();

    // Should find both, but prefer the closer one
    let og_images: Vec<_> = results.iter().filter_map(|r| match r {
        ConventionMetadata::OpenGraphImage { path, .. } => Some(path),
        _ => None,
    }).collect();

    assert!(!og_images.is_empty());
    // The first (highest priority) should be the nested one
    assert!(og_images[0].ends_with("opengraph-image.jpg"));
}

#[test]
fn test_ignored_files() {
    let temp_dir = TempDir::new().unwrap();
    let app_dir = temp_dir.path().join("app");
    fs::create_dir_all(&app_dir).unwrap();

    // Create files that should be ignored
    fs::write(app_dir.join(".DS_Store"), b"macos file").unwrap();
    fs::write(app_dir.join("Thumbs.db"), b"windows file").unwrap();
    fs::write(app_dir.join("desktop.ini"), b"windows file").unwrap();
    fs::write(app_dir.join(".gitignore"), b"git file").unwrap();

    // And one that should be detected
    fs::write(app_dir.join("favicon.ico"), b"favicon").unwrap();

    let scanner = ConventionScanner::new(app_dir);
    let results = scanner.scan().unwrap();

    // Should only find favicon, not system files
    assert_eq!(results.len(), 1);
    match &results[0] {
        ConventionMetadata::Icon { path, .. } => {
            assert!(path.ends_with("favicon.ico"));
        }
        _ => panic!("Expected Icon"),
    }
}

#[test]
fn test_media_type_detection() {
    let temp_dir = TempDir::new().unwrap();
    let app_dir = temp_dir.path().join("app");
    fs::create_dir_all(&app_dir).unwrap();

    // Create files with different extensions
    let files = vec![
        ("icon.png", "image/png"),
        ("icon.jpg", "image/jpeg"),
        ("icon.jpeg", "image/jpeg"),
        ("icon.gif", "image/gif"),
        ("icon.svg", "image/svg+xml"),
        ("icon.webp", "image/webp"),
        ("favicon.ico", "image/x-icon"),
        ("manifest.json", "application/manifest+json"),
        ("manifest.webmanifest", "application/manifest+json"),
    ];

    for (filename, expected_type) in &files {
        fs::write(app_dir.join(filename), b"test content").unwrap();
    }

    let scanner = ConventionScanner::new(app_dir);
    let results = scanner.scan().unwrap();

    for (filename, expected_type) in files {
        let found = results.iter().find(|r| {
            r.path().file_name().and_then(|n| n.to_str()) == Some(&filename)
        }).expect(&format!("File {} not found in results", filename));

        match found {
            ConventionMetadata::Icon { media_type, .. } |
            ConventionMetadata::OpenGraphImage { media_type, .. } |
            ConventionMetadata::TwitterImage { media_type, .. } => {
                assert_eq!(
                    media_type.as_deref(),
                    Some(expected_type),
                    "Wrong media type for {}",
                    filename
                );
            }
            ConventionMetadata::Manifest { .. } => {
                // Manifest detection doesn't store media type
                assert!(expected_type.contains("manifest"));
            }
            _ => {
                panic!("Unexpected metadata type for {}", filename);
            }
        }
    }
}

#[test]
fn test_custom_patterns() {
    let temp_dir = TempDir::new().unwrap();
    let app_dir = temp_dir.path().join("app");
    fs::create_dir_all(&app_dir).unwrap();

    // Create files with custom naming patterns
    fs::write(app_dir.join("icon-light.png"), b"light icon").unwrap();
    fs::write(app_dir.join("icon-dark.png"), b"dark icon").unwrap();
    fs::write(app_dir.join("apple-touch-icon-precomposed.png"), b"precomposed apple icon").unwrap();

    let mut scanner = ConventionScanner::new(app_dir);

    // Add custom pattern for themed icons
    scanner.add_pattern(r"icon-(light|dark)\.png$", ConventionType::Icon);

    let results = scanner.scan().unwrap();

    // Should find standard apple icon plus custom themed icons
    assert!(results.len() >= 3);

    let icon_paths: Vec<_> = results.iter().filter_map(|r| match r {
        ConventionMetadata::Icon { path, .. } => Some(path.to_string_lossy().to_string()),
        _ => None,
    }).collect();

    assert!(icon_paths.iter().any(|p| p.contains("light")));
    assert!(icon_paths.iter().any(|p| p.contains("dark")));
    assert!(icon_paths.iter().any(|p| p.contains("precomposed")));
}

#[test]
fn test_error_handling() {
    // Test with non-existent directory
    let scanner = ConventionScanner::new("/non/existent/path");
    let result = scanner.scan();
    assert!(result.is_err());

    // Test with file instead of directory
    let temp_file = tempfile::NamedTempFile::new().unwrap();
    let scanner = ConventionScanner::new(temp_file.path());
    let result = scanner.scan();
    assert!(result.is_err());
}

#[test]
fn test_performance_with_large_directory() {
    let temp_dir = TempDir::new().unwrap();
    let app_dir = temp_dir.path().join("app");
    fs::create_dir_all(&app_dir).unwrap();

    // Create many files to test performance
    for i in 0..1000 {
        fs::write(app_dir.join(format!("file-{}.txt", i)), b"content").unwrap();
    }

    // Add a few convention files
    fs::write(app_dir.join("favicon.ico"), b"favicon").unwrap();
    fs::write(app_dir.join("manifest.json"), r#"{"name": "test"}"#).unwrap();

    let scanner = ConventionScanner::new(app_dir);

    let start = std::time::Instant::now();
    let results = scanner.scan().unwrap();
    let duration = start.elapsed();

    // Should complete quickly even with many files
    assert!(duration.as_millis() < 1000, "Scanning took {}ms", duration.as_millis());

    // Should only find convention files, not all files
    assert_eq!(results.len(), 2);
}

#[test]
fn test_symbolic_links() {
    let temp_dir = TempDir::new().unwrap();
    let app_dir = temp_dir.path().join("app");
    let assets_dir = temp_dir.path().join("assets");
    fs::create_dir_all(&app_dir).unwrap();
    fs::create_dir_all(&assets_dir).unwrap();

    // Create original file
    fs::write(assets_dir.join("favicon.ico"), b"favicon content").unwrap();

    // Create symbolic link (if supported on platform)
    #[cfg(unix)]
    {
        use std::os::unix::fs::symlink;
        let _ = symlink(assets_dir.join("favicon.ico"), app_dir.join("favicon.ico"));

        let scanner = ConventionScanner::new(app_dir);
        let results = scanner.scan().unwrap();

        // Should follow symlink and detect the file
        assert!(!results.is_empty());
        match &results[0] {
            ConventionMetadata::Icon { path, .. } => {
                assert!(path.ends_with("favicon.ico"));
            }
            _ => panic!("Expected Icon"),
        }
    }
}
