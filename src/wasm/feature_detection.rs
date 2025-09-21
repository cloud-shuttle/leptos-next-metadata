//! Feature detection for WASM environments
//!
//! Provides runtime detection of browser capabilities and features

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::{Document, HtmlCanvasElement, Window};

/// Browser feature detection results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureDetection {
    /// Web Storage support
    pub web_storage: WebStorageSupport,
    /// Canvas support
    pub canvas: CanvasSupport,
    /// WebGL support
    pub webgl: WebGLSupport,
    /// Fetch API support
    pub fetch: bool,
    /// Web Workers support
    pub web_workers: bool,
    /// Service Workers support
    pub service_workers: bool,
    /// WebAssembly support
    pub webassembly: bool,
    /// ES6 modules support
    pub es6_modules: bool,
    /// Intersection Observer support
    pub intersection_observer: bool,
    /// Resize Observer support
    pub resize_observer: bool,
    /// Performance API support
    pub performance_api: bool,
    /// Geolocation API support
    pub geolocation: bool,
    /// Device orientation support
    pub device_orientation: bool,
    /// Touch events support
    pub touch_events: bool,
    /// Pointer events support
    pub pointer_events: bool,
    /// CSS Grid support
    pub css_grid: bool,
    /// CSS Flexbox support
    pub css_flexbox: bool,
    /// CSS Custom Properties support
    pub css_custom_properties: bool,
}

/// Web Storage support details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebStorageSupport {
    /// Local Storage available
    pub local_storage: bool,
    /// Session Storage available
    pub session_storage: bool,
    /// IndexedDB available
    pub indexed_db: bool,
    /// WebSQL available (deprecated)
    pub web_sql: bool,
}

/// Canvas support details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanvasSupport {
    /// Basic Canvas 2D support
    pub canvas_2d: bool,
    /// Canvas text support
    pub canvas_text: bool,
    /// Canvas image data support
    pub canvas_image_data: bool,
    /// Canvas path support
    pub canvas_path: bool,
    /// Canvas transform support
    pub canvas_transform: bool,
}

/// WebGL support details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebGLSupport {
    /// WebGL 1.0 support
    pub webgl_1: bool,
    /// WebGL 2.0 support
    pub webgl_2: bool,
    /// WebGL extensions available
    pub extensions: Vec<String>,
}

impl Default for FeatureDetection {
    fn default() -> Self {
        Self::detect()
    }
}

impl FeatureDetection {
    /// Detect all browser features
    pub fn detect() -> Self {
        let window = web_sys::window().unwrap_or_else(|| {
            // Fallback for testing
            return JsValue::NULL.into();
        });

        Self {
            web_storage: Self::detect_web_storage(&window),
            canvas: Self::detect_canvas(&window),
            webgl: Self::detect_webgl(&window),
            fetch: Self::detect_fetch(&window),
            web_workers: Self::detect_web_workers(&window),
            service_workers: Self::detect_service_workers(&window),
            webassembly: Self::detect_webassembly(&window),
            es6_modules: Self::detect_es6_modules(&window),
            intersection_observer: Self::detect_intersection_observer(&window),
            resize_observer: Self::detect_resize_observer(&window),
            performance_api: Self::detect_performance_api(&window),
            geolocation: Self::detect_geolocation(&window),
            device_orientation: Self::detect_device_orientation(&window),
            touch_events: Self::detect_touch_events(&window),
            pointer_events: Self::detect_pointer_events(&window),
            css_grid: Self::detect_css_grid(&window),
            css_flexbox: Self::detect_css_flexbox(&window),
            css_custom_properties: Self::detect_css_custom_properties(&window),
        }
    }

    /// Detect Web Storage support
    fn detect_web_storage(window: &Window) -> WebStorageSupport {
        WebStorageSupport {
            local_storage: window.local_storage().is_ok(),
            session_storage: window.session_storage().is_ok(),
            indexed_db: js_sys::Reflect::has(window, &JsValue::from_str("indexedDB"))
                .unwrap_or(false),
            web_sql: js_sys::Reflect::has(window, &JsValue::from_str("openDatabase"))
                .unwrap_or(false),
        }
    }

    /// Detect Canvas support
    fn detect_canvas(window: &Window) -> CanvasSupport {
        let document = window.document().unwrap_or_else(|| {
            return JsValue::NULL.into();
        });

        let canvas_support = if let Ok(canvas) = document.create_element("canvas") {
            if let Ok(canvas) = canvas.dyn_into::<HtmlCanvasElement>() {
                let context = canvas.get_context("2d").ok().flatten();
                context.is_some()
            } else {
                false
            }
        } else {
            false
        };

        CanvasSupport {
            canvas_2d: canvas_support,
            canvas_text: canvas_support, // Simplified - in real implementation, test specific features
            canvas_image_data: canvas_support,
            canvas_path: canvas_support,
            canvas_transform: canvas_support,
        }
    }

    /// Detect WebGL support
    fn detect_webgl(window: &Window) -> WebGLSupport {
        let document = window.document().unwrap_or_else(|| {
            return JsValue::NULL.into();
        });

        let (webgl_1, webgl_2) = if let Ok(canvas) = document.create_element("canvas") {
            if let Ok(canvas) = canvas.dyn_into::<HtmlCanvasElement>() {
                let webgl_1 = canvas.get_context("webgl").is_ok()
                    || canvas.get_context("experimental-webgl").is_ok();
                let webgl_2 = canvas.get_context("webgl2").is_ok();
                (webgl_1, webgl_2)
            } else {
                (false, false)
            }
        } else {
            (false, false)
        };

        WebGLSupport {
            webgl_1,
            webgl_2,
            extensions: Vec::new(), // Simplified - in real implementation, query actual extensions
        }
    }

    /// Detect Fetch API support
    fn detect_fetch(window: &Window) -> bool {
        js_sys::Reflect::has(window, &JsValue::from_str("fetch")).unwrap_or(false)
    }

    /// Detect Web Workers support
    fn detect_web_workers(window: &Window) -> bool {
        js_sys::Reflect::has(window, &JsValue::from_str("Worker")).unwrap_or(false)
    }

    /// Detect Service Workers support
    fn detect_service_workers(window: &Window) -> bool {
        js_sys::Reflect::has(window, &JsValue::from_str("serviceWorker")).unwrap_or(false)
    }

    /// Detect WebAssembly support
    fn detect_webassembly(window: &Window) -> bool {
        js_sys::Reflect::has(window, &JsValue::from_str("WebAssembly")).unwrap_or(false)
    }

    /// Detect ES6 modules support
    fn detect_es6_modules(window: &Window) -> bool {
        // Check if script type="module" is supported
        let document = window.document().unwrap_or_else(|| {
            return JsValue::NULL.into();
        });

        if let Ok(script) = document.create_element("script") {
            script.set_attribute("type", "module").is_ok()
        } else {
            false
        }
    }

    /// Detect Intersection Observer support
    fn detect_intersection_observer(window: &Window) -> bool {
        js_sys::Reflect::has(window, &JsValue::from_str("IntersectionObserver")).unwrap_or(false)
    }

    /// Detect Resize Observer support
    fn detect_resize_observer(window: &Window) -> bool {
        js_sys::Reflect::has(window, &JsValue::from_str("ResizeObserver")).unwrap_or(false)
    }

    /// Detect Performance API support
    fn detect_performance_api(window: &Window) -> bool {
        js_sys::Reflect::has(window, &JsValue::from_str("performance")).unwrap_or(false)
    }

    /// Detect Geolocation API support
    fn detect_geolocation(window: &Window) -> bool {
        if let Ok(geolocation) = window.navigator().geolocation() {
            !geolocation.is_null()
        } else {
            false
        }
    }

    /// Detect Device Orientation support
    fn detect_device_orientation(window: &Window) -> bool {
        js_sys::Reflect::has(window, &JsValue::from_str("DeviceOrientationEvent")).unwrap_or(false)
    }

    /// Detect Touch Events support
    fn detect_touch_events(window: &Window) -> bool {
        js_sys::Reflect::has(window, &JsValue::from_str("TouchEvent")).unwrap_or(false)
    }

    /// Detect Pointer Events support
    fn detect_pointer_events(window: &Window) -> bool {
        js_sys::Reflect::has(window, &JsValue::from_str("PointerEvent")).unwrap_or(false)
    }

    /// Detect CSS Grid support
    fn detect_css_grid(window: &Window) -> bool {
        let document = window.document().unwrap_or_else(|| {
            return JsValue::NULL.into();
        });

        if let Ok(element) = document.create_element("div") {
            if let Ok(element) = element.dyn_into::<web_sys::HtmlElement>() {
                let style = element.style();
                style.set_property("display", "grid").is_ok()
            } else {
                false
            }
        } else {
            false
        }
    }

    /// Detect CSS Flexbox support
    fn detect_css_flexbox(window: &Window) -> bool {
        let document = window.document().unwrap_or_else(|| {
            return JsValue::NULL.into();
        });

        if let Ok(element) = document.create_element("div") {
            if let Ok(element) = element.dyn_into::<web_sys::HtmlElement>() {
                let style = element.style();
                style.set_property("display", "flex").is_ok()
            } else {
                false
            }
        } else {
            false
        }
    }

    /// Detect CSS Custom Properties support
    fn detect_css_custom_properties(window: &Window) -> bool {
        let document = window.document().unwrap_or_else(|| {
            return JsValue::NULL.into();
        });

        if let Ok(element) = document.create_element("div") {
            if let Ok(element) = element.dyn_into::<web_sys::HtmlElement>() {
                let style = element.style();
                style.set_property("--test-var", "1px").is_ok()
            } else {
                false
            }
        } else {
            false
        }
    }

    /// Check if a specific feature is supported
    pub fn supports(&self, feature: &str) -> bool {
        match feature {
            "localStorage" => self.web_storage.local_storage,
            "sessionStorage" => self.web_storage.session_storage,
            "indexedDB" => self.web_storage.indexed_db,
            "canvas" => self.canvas.canvas_2d,
            "webgl" => self.webgl.webgl_1,
            "webgl2" => self.webgl.webgl_2,
            "fetch" => self.fetch,
            "webworkers" => self.web_workers,
            "serviceworkers" => self.service_workers,
            "webassembly" => self.webassembly,
            "es6modules" => self.es6_modules,
            "intersectionobserver" => self.intersection_observer,
            "resizeobserver" => self.resize_observer,
            "performance" => self.performance_api,
            "geolocation" => self.geolocation,
            "deviceorientation" => self.device_orientation,
            "touchevents" => self.touch_events,
            "pointerevents" => self.pointer_events,
            "cssgrid" => self.css_grid,
            "cssflexbox" => self.css_flexbox,
            "csscustomproperties" => self.css_custom_properties,
            _ => false,
        }
    }

    /// Get all supported features
    pub fn get_supported_features(&self) -> Vec<String> {
        let features = [
            "localStorage",
            "sessionStorage",
            "indexedDB",
            "canvas",
            "webgl",
            "webgl2",
            "fetch",
            "webworkers",
            "serviceworkers",
            "webassembly",
            "es6modules",
            "intersectionobserver",
            "resizeobserver",
            "performance",
            "geolocation",
            "deviceorientation",
            "touchevents",
            "pointerevents",
            "cssgrid",
            "cssflexbox",
            "csscustomproperties",
        ];

        features
            .iter()
            .filter(|&&feature| self.supports(feature))
            .map(|&feature| feature.to_string())
            .collect()
    }

    /// Get all unsupported features
    pub fn get_unsupported_features(&self) -> Vec<String> {
        let features = [
            "localStorage",
            "sessionStorage",
            "indexedDB",
            "canvas",
            "webgl",
            "webgl2",
            "fetch",
            "webworkers",
            "serviceworkers",
            "webassembly",
            "es6modules",
            "intersectionobserver",
            "resizeobserver",
            "performance",
            "geolocation",
            "deviceorientation",
            "touchevents",
            "pointerevents",
            "cssgrid",
            "cssflexbox",
            "csscustomproperties",
        ];

        features
            .iter()
            .filter(|&&feature| !self.supports(feature))
            .map(|&feature| feature.to_string())
            .collect()
    }

    /// Check if essential features are supported
    pub fn has_essential_features(&self) -> bool {
        self.web_storage.local_storage || self.web_storage.session_storage
    }

    /// Get feature support score (0-100)
    pub fn get_support_score(&self) -> u8 {
        let total_features = 21; // Total number of features we check
        let supported_features = self.get_supported_features().len();
        ((supported_features as f32 / total_features as f32) * 100.0) as u8
    }
}

/// Feature detection utility functions
pub struct FeatureDetector;

impl FeatureDetector {
    /// Quick check for essential features
    pub fn has_essential_support() -> bool {
        let detection = FeatureDetection::detect();
        detection.has_essential_features()
    }

    /// Get feature support summary
    pub fn get_support_summary() -> String {
        let detection = FeatureDetection::detect();
        let score = detection.get_support_score();
        let supported = detection.get_supported_features().len();
        let total = 21;

        format!(
            "Browser support: {}/{} features ({}%)",
            supported, total, score
        )
    }

    /// Check if feature is supported (static method)
    pub fn is_supported(feature: &str) -> bool {
        let detection = FeatureDetection::detect();
        detection.supports(feature)
    }

    /// Get recommended storage backend
    pub fn get_recommended_storage() -> &'static str {
        let detection = FeatureDetection::detect();
        if detection.web_storage.local_storage {
            "local"
        } else if detection.web_storage.session_storage {
            "session"
        } else {
            "memory"
        }
    }

    /// Get recommended image generation method
    pub fn get_recommended_image_method() -> &'static str {
        let detection = FeatureDetection::detect();
        if detection.webgl.webgl_2 {
            "webgl2"
        } else if detection.webgl.webgl_1 {
            "webgl"
        } else if detection.canvas.canvas_2d {
            "canvas2d"
        } else {
            "none"
        }
    }
}
