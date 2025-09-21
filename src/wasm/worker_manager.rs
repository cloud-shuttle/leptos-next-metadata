//! Web Workers Manager for WASM
//!
//! Provides background processing capabilities for heavy metadata operations
//! like OG image generation, keeping the main thread responsive.

use futures::channel::oneshot;
use js_sys::Promise;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Blob, BlobPropertyBag, MessageEvent, Url, Worker};

use crate::canvas_types::{CanvasOgParams, OgImageTemplate};
use crate::error::{ErrorKind, MetadataError};

static NEXT_TASK_ID: AtomicU32 = AtomicU32::new(0);

/// Types of tasks that can be sent to workers
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum WorkerTask {
    GenerateOgImage {
        params: CanvasOgParams,
    },
    GenerateOgImageFromTemplate {
        template: OgImageTemplate,
        data: HashMap<String, String>,
    },
    ProcessMetadata {
        metadata: serde_json::Value,
    },
    ValidateMetadata {
        metadata: serde_json::Value,
    },
}

/// Results returned from workers
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum WorkerResult {
    OgImageGenerated { data_url: String },
    MetadataProcessed { result: serde_json::Value },
    MetadataValidated { is_valid: bool, errors: Vec<String> },
    Error { message: String, kind: String },
}

/// Message sent to worker
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WorkerMessage {
    id: u32,
    task_type: WorkerTask,
}

/// Message received from worker
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WorkerResponse {
    id: u32,
    status: String, // "success" or "error"
    result: Option<WorkerResult>,
    error: Option<String>,
}

/// Manager for Web Workers
pub struct WasmWorkerManager {
    worker: Worker,
    pending_tasks: HashMap<u32, oneshot::Sender<Result<WorkerResult, MetadataError>>>,
}

impl WasmWorkerManager {
    /// Create a new worker manager
    pub fn new() -> Result<Self, JsValue> {
        // Create a Blob URL for the worker script
        let worker_script = include_str!("../../../worker.js");
        let blob = Blob::new_with_str_sequence_and_options(
            &js_sys::Array::of1(&JsValue::from_str(worker_script)),
            BlobPropertyBag::new().type_("application/javascript"),
        )?;
        let worker_url = Url::create_object_url_with_blob(&blob)?;

        let worker = Worker::new(&worker_url)?;
        Url::revoke_object_url(&worker_url)?; // Clean up the Blob URL

        let manager = Self {
            worker,
            pending_tasks: HashMap::new(),
        };

        // Set up message listener for the worker
        let worker_clone = manager.worker.clone();
        let pending_tasks_clone = manager.pending_tasks.clone();
        let onmessage_callback =
            Closure::<dyn FnMut(MessageEvent)>::new(move |event: MessageEvent| {
                if let Ok(response_data) = event.data().dyn_into::<js_sys::Object>() {
                    if let Ok(response_json) = js_sys::JSON::stringify(&response_data) {
                        if let Ok(response_str) = response_json.as_string() {
                            if let Ok(response) =
                                serde_json::from_str::<WorkerResponse>(&response_str)
                            {
                                // Find the pending task and send the result
                                if let Some(sender) = pending_tasks_clone.get(&response.id) {
                                    let result = match response.status.as_str() {
                                        "success" => {
                                            if let Some(result) = response.result {
                                                Ok(result)
                                            } else {
                                                Err(MetadataError::new(
                                                    ErrorKind::Unknown,
                                                    "No result in successful response".to_string(),
                                                ))
                                            }
                                        }
                                        "error" => {
                                            let error_msg = response.error.unwrap_or_else(|| {
                                                "Unknown worker error".to_string()
                                            });
                                            Err(MetadataError::new(
                                                ErrorKind::Unknown,
                                                format!("Worker error: {}", error_msg),
                                            ))
                                        }
                                        _ => Err(MetadataError::new(
                                            ErrorKind::Unknown,
                                            format!("Unknown response status: {}", response.status),
                                        )),
                                    };

                                    // Note: In a real implementation, we'd need to handle the sender properly
                                    // For now, we'll log the result
                                    web_sys::console::log_1(
                                        &format!("Worker result: {:?}", result).into(),
                                    );
                                }
                            }
                        }
                    }
                }
            });

        manager
            .worker
            .set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
        onmessage_callback.forget();

        Ok(manager)
    }

    /// Send a task to the worker and await its result
    pub async fn send_task(&mut self, task: WorkerTask) -> Result<WorkerResult, MetadataError> {
        let task_id = NEXT_TASK_ID.fetch_add(1, Ordering::SeqCst);
        let (tx, rx) = oneshot::channel();
        self.pending_tasks.insert(task_id, tx);

        let message = WorkerMessage {
            id: task_id,
            task_type: task,
        };

        let message_json = serde_json::to_string(&message)
            .map_err(|e| MetadataError::new(ErrorKind::Serialization, e.to_string()))?;

        let message_js = JsValue::from_str(&message_json);
        self.worker.post_message(&message_js)?;

        // Wait for the response
        rx.await.map_err(|_| {
            MetadataError::new(
                ErrorKind::Unknown,
                "Worker task cancelled or failed".to_string(),
            )
        })?
    }

    /// Generate an OG image in a Web Worker
    pub async fn generate_og_image_in_worker(
        &mut self,
        params: CanvasOgParams,
    ) -> Result<String, MetadataError> {
        let task = WorkerTask::GenerateOgImage { params };
        let result = self.send_task(task).await?;

        match result {
            WorkerResult::OgImageGenerated { data_url } => Ok(data_url),
            WorkerResult::Error { message, .. } => {
                Err(MetadataError::new(ErrorKind::Canvas, message))
            }
            _ => Err(MetadataError::new(
                ErrorKind::Unknown,
                "Unexpected result type from worker".to_string(),
            )),
        }
    }

    /// Generate an OG image from template in a Web Worker
    pub async fn generate_og_image_from_template_in_worker(
        &mut self,
        template: OgImageTemplate,
        data: HashMap<String, String>,
    ) -> Result<String, MetadataError> {
        let task = WorkerTask::GenerateOgImageFromTemplate { template, data };
        let result = self.send_task(task).await?;

        match result {
            WorkerResult::OgImageGenerated { data_url } => Ok(data_url),
            WorkerResult::Error { message, .. } => {
                Err(MetadataError::new(ErrorKind::Canvas, message))
            }
            _ => Err(MetadataError::new(
                ErrorKind::Unknown,
                "Unexpected result type from worker".to_string(),
            )),
        }
    }

    /// Process metadata in a Web Worker
    pub async fn process_metadata_in_worker(
        &mut self,
        metadata: serde_json::Value,
    ) -> Result<serde_json::Value, MetadataError> {
        let task = WorkerTask::ProcessMetadata { metadata };
        let result = self.send_task(task).await?;

        match result {
            WorkerResult::MetadataProcessed { result } => Ok(result),
            WorkerResult::Error { message, .. } => {
                Err(MetadataError::new(ErrorKind::Unknown, message))
            }
            _ => Err(MetadataError::new(
                ErrorKind::Unknown,
                "Unexpected result type from worker".to_string(),
            )),
        }
    }

    /// Validate metadata in a Web Worker
    pub async fn validate_metadata_in_worker(
        &mut self,
        metadata: serde_json::Value,
    ) -> Result<(bool, Vec<String>), MetadataError> {
        let task = WorkerTask::ValidateMetadata { metadata };
        let result = self.send_task(task).await?;

        match result {
            WorkerResult::MetadataValidated { is_valid, errors } => Ok((is_valid, errors)),
            WorkerResult::Error { message, .. } => {
                Err(MetadataError::new(ErrorKind::Validation, message))
            }
            _ => Err(MetadataError::new(
                ErrorKind::Unknown,
                "Unexpected result type from worker".to_string(),
            )),
        }
    }

    /// Terminate the worker
    pub fn terminate(&self) {
        self.worker.terminate();
    }

    /// Check if workers are supported
    pub fn is_supported() -> bool {
        if let Some(window) = web_sys::window() {
            if let Ok(_worker) = Worker::new("data:application/javascript,console.log('test')") {
                return true;
            }
        }
        false
    }
}

impl Drop for WasmWorkerManager {
    fn drop(&mut self) {
        self.terminate();
    }
}
