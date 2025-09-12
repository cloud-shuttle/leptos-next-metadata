use leptos_next_metadata::prelude::*;

fn main() {
    println!("leptos-next-metadata - Basic Example");

    // Create basic metadata
    let metadata = Metadata::new()
        .title("My Amazing Page")
        .description("This is a demonstration of leptos-next-metadata library")
        .keywords(vec![
            "rust".to_string(),
            "leptos".to_string(),
            "metadata".to_string(),
            "seo".to_string(),
        ]);

    println!("Created metadata:");
    println!("- Title: {:?}", metadata.title);
    println!("- Description: {:?}", metadata.description);
    println!("- Keywords: {:?}", metadata.keywords);

    // Test utility functions
    println!("\nTesting utility functions:");
    let long_text = "This is a very long text that should be truncated for demonstration purposes";
    let truncated = leptos_next_metadata::utils::common::truncate_string(long_text, 30);
    println!("- Truncated text: {}", truncated);

    let url_valid = leptos_next_metadata::utils::common::is_valid_url("https://example.com");
    let url_invalid = leptos_next_metadata::utils::common::is_valid_url("not-a-url");
    println!(
        "- URL validation: https://example.com = {}, not-a-url = {}",
        url_valid, url_invalid
    );

    println!("\nBasic functionality working! ✅");
}
