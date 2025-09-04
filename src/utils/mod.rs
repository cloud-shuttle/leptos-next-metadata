//! Utility functions and helpers for leptos-next-metadata


/// Common utility functions for metadata operations
pub mod common {
    
    
    /// Truncate a string to a maximum length, adding ellipsis if needed
    pub fn truncate_string(s: &str, max_len: usize) -> String {
        if s.len() <= max_len {
            s.to_string()
        } else {
            format!("{}...", &s[..max_len.saturating_sub(3)])
        }
    }
    
    /// Validate if a string is a valid URL
    pub fn is_valid_url(url: &str) -> bool {
        url::Url::parse(url).is_ok()
    }
    
    /// Clean and normalize a string for metadata use
    pub fn clean_string(s: &str) -> String {
        s.trim()
            .replace('\n', " ")
            .replace('\r', " ")
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
    }
}

/// Image utility functions
pub mod image {
    
    
    /// Calculate aspect ratio from dimensions
    pub fn aspect_ratio(width: u32, height: u32) -> f64 {
        if height == 0 {
            0.0
        } else {
            width as f64 / height as f64
        }
    }
    
    /// Check if dimensions are valid for social media
    pub fn is_valid_social_dimensions(width: u32, height: u32) -> bool {
        width >= 200 && height >= 200 && width <= 4096 && height <= 4096
    }
}

/// SEO utility functions
pub mod seo {
    
    
    /// Generate a slug from a title
    pub fn title_to_slug(title: &str) -> String {
        title
            .to_lowercase()
            .chars()
            .filter(|c| c.is_alphanumeric() || c.is_whitespace())
            .collect::<String>()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join("-")
    }
    
    /// Check if a string contains common spam words
    pub fn contains_spam_words(text: &str) -> bool {
        let spam_words = [
            "buy now", "click here", "free", "limited time", "act now",
            "best price", "cheap", "discount", "offer", "sale"
        ];
        
        let text_lower = text.to_lowercase();
        spam_words.iter().any(|word| text_lower.contains(word))
    }
}

/// Cache utility functions
pub mod cache {
    
    
    /// Generate a cache key from metadata
    pub fn generate_cache_key(metadata: &crate::metadata::Metadata) -> String {
        // For now, use a simple string-based cache key
        // In a full implementation, this could use a more sophisticated approach
        let title = metadata.title.as_ref().map(|t| match t {
            crate::metadata::Title::Static(s) => s.as_str(),
            crate::metadata::Title::Template { default, .. } => default.as_str(),
        }).unwrap_or("no-title");
        
        let description = metadata.description.as_deref().unwrap_or("no-description");
        format!("{}-{}", title, description)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::metadata::{Metadata, Title};
    
            mod common {
            use super::*;
            
            #[test]
            fn test_truncate_string() {
                let short = "Hello";
                let long = "This is a very long string that needs to be truncated";
                
                assert_eq!(crate::utils::common::truncate_string(short, 10), "Hello");
                assert_eq!(crate::utils::common::truncate_string(long, 20), "This is a very lo...");
                assert_eq!(crate::utils::common::truncate_string(long, 5), "Th...");
                assert_eq!(crate::utils::common::truncate_string(long, 0), "...");
            }
            
            #[test]
            fn test_is_valid_url() {
                assert!(crate::utils::common::is_valid_url("https://example.com"));
                assert!(crate::utils::common::is_valid_url("http://localhost:3000"));
                assert!(crate::utils::common::is_valid_url("ftp://files.example.com"));
                assert!(!crate::utils::common::is_valid_url("not-a-url"));
                assert!(!crate::utils::common::is_valid_url(""));
            }
            
            #[test]
            fn test_clean_string() {
                let dirty = "  Hello\n  World\r\n  !  ";
                let clean = crate::utils::common::clean_string(dirty);
                assert_eq!(clean, "Hello World !");
                
                let single_line = "Single line";
                assert_eq!(crate::utils::common::clean_string(single_line), "Single line");
            }
        }
    
            mod image {
            use super::*;
            
            #[test]
            fn test_aspect_ratio() {
                assert_eq!(crate::utils::image::aspect_ratio(16, 9), 16.0 / 9.0);
                assert_eq!(crate::utils::image::aspect_ratio(4, 3), 4.0 / 3.0);
                assert_eq!(crate::utils::image::aspect_ratio(1, 1), 1.0);
                assert_eq!(crate::utils::image::aspect_ratio(0, 5), 0.0);
                assert_eq!(crate::utils::image::aspect_ratio(10, 0), 0.0);
            }
            
            #[test]
            fn test_is_valid_social_dimensions() {
                // Valid dimensions
                assert!(crate::utils::image::is_valid_social_dimensions(1200, 630));
                assert!(crate::utils::image::is_valid_social_dimensions(200, 200));
                assert!(crate::utils::image::is_valid_social_dimensions(4096, 4096));
                
                // Invalid dimensions
                assert!(!crate::utils::image::is_valid_social_dimensions(199, 200));
                assert!(!crate::utils::image::is_valid_social_dimensions(200, 199));
                assert!(!crate::utils::image::is_valid_social_dimensions(4097, 4096));
                assert!(!crate::utils::image::is_valid_social_dimensions(4096, 4097));
            }
        }
    
            mod seo {
            use super::*;
            
            #[test]
            fn test_title_to_slug() {
                assert_eq!(crate::utils::seo::title_to_slug("Hello World"), "hello-world");
                assert_eq!(crate::utils::seo::title_to_slug("Hello, World!"), "hello-world");
                assert_eq!(crate::utils::seo::title_to_slug("Multiple   Spaces"), "multiple-spaces");
                assert_eq!(crate::utils::seo::title_to_slug("Special@#$%^&*()Chars"), "specialchars");
                assert_eq!(crate::utils::seo::title_to_slug(""), "");
            }
            
            #[test]
            fn test_contains_spam_words() {
                assert!(crate::utils::seo::contains_spam_words("Buy now and get the best price!"));
                assert!(crate::utils::seo::contains_spam_words("Limited time offer - act now!"));
                assert!(crate::utils::seo::contains_spam_words("Click here for free stuff"));
                assert!(!crate::utils::seo::contains_spam_words("This is a legitimate article"));
                assert!(!crate::utils::seo::contains_spam_words(""));
            }
        }
    
            mod cache {
            use super::*;
            
            #[test]
            fn test_generate_cache_key() {
                let metadata = Metadata::with_title("Test Title")
                    .description("Test Description");
                
                let cache_key = crate::utils::cache::generate_cache_key(&metadata);
                assert_eq!(cache_key, "Test Title-Test Description");
            }
            
            #[test]
            fn test_generate_cache_key_no_title() {
                let metadata = Metadata::default()
                    .description("Test Description");
                
                let cache_key = crate::utils::cache::generate_cache_key(&metadata);
                assert_eq!(cache_key, "no-title-Test Description");
            }
            
            #[test]
            fn test_generate_cache_key_no_description() {
                let metadata = Metadata::with_title("Test Title");
                
                let cache_key = crate::utils::cache::generate_cache_key(&metadata);
                assert_eq!(cache_key, "Test Title-no-description");
            }
            
            #[test]
            fn test_generate_cache_key_template_title() {
                let metadata = Metadata {
                    title: Some(Title::Template {
                        template: "%s | Site".to_string(),
                        default: "Default Title".to_string(),
                    }),
                    description: Some("Test Description".to_string()),
                    ..Default::default()
                };
                
                let cache_key = crate::utils::cache::generate_cache_key(&metadata);
                assert_eq!(cache_key, "Default Title-Test Description");
            }
        }
}
