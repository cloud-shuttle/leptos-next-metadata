// Web Worker script for leptos-next-metadata
// This script loads the WASM module and handles message passing

import init, { handle_worker_message } from "./pkg/leptos_next_metadata.js";

let wasmInitialized = false;

async function initializeWasm() {
  if (!wasmInitialized) {
    await init();
    wasmInitialized = true;
  }
}

// Handle messages from the main thread
self.onmessage = async (event) => {
  try {
    // Ensure WASM is initialized
    await initializeWasm();

    const { id, taskType, payload } = event.data;

    // Call into Rust WASM function to handle the task
    const result = await handle_worker_message(taskType, payload);

    // Send success response back to main thread
    self.postMessage({
      id,
      status: "success",
      result: result,
    });
  } catch (error) {
    // Send error response back to main thread
    self.postMessage({
      id: event.data.id || 0,
      status: "error",
      error: error.toString(),
    });
  }
};

// Handle errors
self.onerror = (error) => {
  console.error("Worker error:", error);
  self.postMessage({
    id: 0,
    status: "error",
    error: error.toString(),
  });
};

// Handle unhandled promise rejections
self.onunhandledrejection = (event) => {
  console.error("Worker unhandled rejection:", event.reason);
  self.postMessage({
    id: 0,
    status: "error",
    error: event.reason.toString(),
  });
};
