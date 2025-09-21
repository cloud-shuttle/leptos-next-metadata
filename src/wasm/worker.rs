//! Web Worker entry point for background processing
//!
//! This module contains the logic that runs inside Web Workers to handle
//! heavy metadata processing tasks without blocking the main thread.

use serde_json;
use serde_wasm_bindgen;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

use super::worker_manager::{WorkerResult, WorkerTask};
use crate::canvas_types::{CanvasOgParams, OgImageTemplate};
use crate::error::{ErrorKind, MetadataError};

/// Entry point for Web Worker messages
/// This function will be called from JavaScript in the worker
#[wasm_bindgen]
pub async fn handle_worker_message(
    task_type: JsValue,
    payload: JsValue,
) -> Result<JsValue, JsValue> {
    let task: WorkerTask = serde_wasm_bindgen::from_value(task_type)
        .map_err(|e| JsValue::from_str(&format!("Failed to deserialize worker task: {}", e)))?;

    let result = match task {
        WorkerTask::GenerateOgImage { params } => {
            // Generate OG image in worker
            match generate_og_image_in_worker(&params).await {
                Ok(data_url) => WorkerResult::OgImageGenerated { data_url },
                Err(e) => WorkerResult::Error {
                    message: e.message,
                    kind: format!("{:?}", e.kind),
                },
            }
        }
        WorkerTask::GenerateOgImageFromTemplate { template, data } => {
            // Generate OG image from template in worker
            match generate_og_image_from_template_in_worker(&template, &data).await {
                Ok(data_url) => WorkerResult::OgImageGenerated { data_url },
                Err(e) => WorkerResult::Error {
                    message: e.message,
                    kind: format!("{:?}", e.kind),
                },
            }
        }
        WorkerTask::ProcessMetadata { metadata } => {
            // Process metadata in worker
            match process_metadata_in_worker(&metadata).await {
                Ok(result) => WorkerResult::MetadataProcessed { result },
                Err(e) => WorkerResult::Error {
                    message: e.message,
                    kind: format!("{:?}", e.kind),
                },
            }
        }
        WorkerTask::ValidateMetadata { metadata } => {
            // Validate metadata in worker
            match validate_metadata_in_worker(&metadata).await {
                Ok((is_valid, errors)) => WorkerResult::MetadataValidated { is_valid, errors },
                Err(e) => WorkerResult::Error {
                    message: e.message,
                    kind: format!("{:?}", e.kind),
                },
            }
        }
    };

    serde_wasm_bindgen::to_value(&result)
        .map_err(|e| JsValue::from_str(&format!("Failed to serialize worker result: {}", e)))
}

/// Generate OG image in worker context
async fn generate_og_image_in_worker(params: &CanvasOgParams) -> Result<String, MetadataError> {
    // In a real implementation, we would create a CanvasOgGenerator in the worker
    // For now, we'll simulate the generation process

    // Simulate some processing time
    let promise = js_sys::Promise::resolve(&JsValue::undefined());
    let _ = wasm_bindgen_futures::JsFuture::from(promise).await;

    // For demonstration, return a placeholder data URL
    // In a real implementation, this would use the CanvasOgGenerator
    let placeholder_data_url = format!(
        "data:image/svg+xml;base64,{}",
        base64::encode(format!(
            r#"<svg width="{}" height="{}" xmlns="http://www.w3.org/2000/svg">
                <rect width="100%" height="100%" fill="{}"/>
                <text x="50%" y="50%" text-anchor="middle" dy=".3em" font-family="Arial" font-size="48" fill="{}">
                    {}
                </text>
            </svg>"#,
            params.width.unwrap_or(1200),
            params.height.unwrap_or(630),
            params.background_color.as_deref().unwrap_or("#4f46e5"),
            params.text_color.as_deref().unwrap_or("#ffffff"),
            params.title
        ))
    );

    Ok(placeholder_data_url)
}

/// Generate OG image from template in worker context
async fn generate_og_image_from_template_in_worker(
    template: &OgImageTemplate,
    data: &HashMap<String, String>,
) -> Result<String, MetadataError> {
    // Apply data to template
    let mut params = template.default_params.clone();

    for (key, value) in data {
        match key.as_str() {
            "title" => params.title = value.clone(),
            "description" => params.description = Some(value.clone()),
            _ => {
                // Handle custom template variables
                if let Some(ref mut layers) = params.layers {
                    for layer in layers {
                        match layer {
                            crate::canvas_types::OgImageLayer::Text(text_layer) => {
                                if text_layer.content.contains(&format!("{{{}}}", key)) {
                                    text_layer.content =
                                        text_layer.content.replace(&format!("{{{}}}", key), value);
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    // Generate the image with the modified parameters
    generate_og_image_in_worker(&params).await
}

/// Process metadata in worker context
async fn process_metadata_in_worker(
    metadata: &serde_json::Value,
) -> Result<serde_json::Value, MetadataError> {
    // Simulate metadata processing
    let promise = js_sys::Promise::resolve(&JsValue::undefined());
    let _ = wasm_bindgen_futures::JsFuture::from(promise).await;

    // For now, just return the metadata with a processed flag
    let mut result = metadata.clone();
    if let Some(obj) = result.as_object_mut() {
        obj.insert("processed".to_string(), serde_json::Value::Bool(true));
        obj.insert(
            "processed_at".to_string(),
            serde_json::Value::String(
                js_sys::Date::new_0()
                    .to_iso_string()
                    .as_string()
                    .unwrap_or_default(),
            ),
        );
    }

    Ok(result)
}

/// Validate metadata in worker context
async fn validate_metadata_in_worker(
    metadata: &serde_json::Value,
) -> Result<(bool, Vec<String>), MetadataError> {
    // Simulate metadata validation
    let promise = js_sys::Promise::resolve(&JsValue::undefined());
    let _ = wasm_bindgen_futures::JsFuture::from(promise).await;

    let mut errors = Vec::new();
    let mut is_valid = true;

    // Basic validation logic
    if let Some(obj) = metadata.as_object() {
        // Check for required fields
        if !obj.contains_key("title") {
            errors.push("Title is required".to_string());
            is_valid = false;
        }

        if let Some(title) = obj.get("title") {
            if let Some(title_str) = title.as_str() {
                if title_str.is_empty() {
                    errors.push("Title cannot be empty".to_string());
                    is_valid = false;
                }
                if title_str.len() > 60 {
                    errors.push("Title should be 60 characters or less".to_string());
                    is_valid = false;
                }
            }
        }

        // Check description length
        if let Some(description) = obj.get("description") {
            if let Some(desc_str) = description.as_str() {
                if desc_str.len() > 160 {
                    errors.push("Description should be 160 characters or less".to_string());
                    is_valid = false;
                }
            }
        }
    } else {
        errors.push("Metadata must be an object".to_string());
        is_valid = false;
    }

    Ok((is_valid, errors))
}
