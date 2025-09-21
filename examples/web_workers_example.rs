//! Web Workers Example
//!
//! This example demonstrates how to use the Web Workers functionality
//! for background OG image generation and metadata processing.

#[cfg(target_arch = "wasm32")]
use leptos_next_metadata::prelude::*;
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub async fn example_web_workers_usage() -> Result<JsValue, JsValue> {
    // Create a WASM metadata context
    let context = WasmMetadataContext::new();

    // Check if workers are supported
    if !context.are_workers_supported() {
        return Err(JsValue::from_str(
            "Web Workers are not supported in this environment",
        ));
    }

    // Create a worker manager
    let mut worker_manager = context
        .create_worker_manager()
        .map_err(|e| JsValue::from_str(&format!("Failed to create worker manager: {:?}", e)))?;

    // Example 1: Generate OG image in background
    let og_params = CanvasOgParams {
        title: "Background Generated Image".to_string(),
        description: Some("This image was generated in a Web Worker".to_string()),
        width: Some(1200),
        height: Some(630),
        background_color: Some("#4f46e5".to_string()),
        text_color: Some("#ffffff".to_string()),
        font_family: Some("Arial".to_string()),
        title_font_size: Some(48),
        description_font_size: Some(24),
        logo_url: None,
        font_urls: None,
        default_font_family: None,
        layers: None,
        background_image_url: None,
        background_image_opacity: None,
        text_gradient: None,
        text_shadow: None,
        text_outline: None,
        logo_position: None,
        text_align: None,
        padding: None,
    };

    // Generate image in worker (non-blocking)
    let data_url = worker_manager
        .generate_og_image_in_worker(og_params)
        .await
        .map_err(|e| JsValue::from_str(&format!("Failed to generate OG image: {}", e.message)))?;

    // Example 2: Process metadata in background
    let metadata = serde_json::json!({
        "title": "Processed in Worker",
        "description": "This metadata was processed in a Web Worker",
        "keywords": ["web workers", "background processing", "performance"]
    });

    let processed_metadata = worker_manager
        .process_metadata_in_worker(metadata)
        .await
        .map_err(|e| JsValue::from_str(&format!("Failed to process metadata: {}", e.message)))?;

    // Example 3: Validate metadata in background
    let validation_result = worker_manager
        .validate_metadata_in_worker(processed_metadata.clone())
        .await
        .map_err(|e| JsValue::from_str(&format!("Failed to validate metadata: {}", e.message)))?;

    // Return results
    let results = serde_json::json!({
        "og_image_data_url": data_url,
        "processed_metadata": processed_metadata,
        "validation_result": validation_result,
        "message": "All operations completed successfully in Web Workers!"
    });

    Ok(serde_wasm_bindgen::to_value(&results)?)
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    println!("Web Workers example is only available in WASM environments");
    println!("To use this example:");
    println!("1. Build the project with wasm-pack");
    println!("2. Load the generated WASM module in a browser");
    println!("3. Call the example_web_workers_usage() function");
}

#[cfg(target_arch = "wasm32")]
fn main() {
    // This will be called when the WASM module is loaded
    web_sys::console::log_1(&"Web Workers example loaded successfully!".into());
}
