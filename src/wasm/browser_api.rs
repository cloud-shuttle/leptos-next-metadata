//! Browser API integration for WASM
//!
//! Provides safe wrappers around browser APIs for metadata management

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::{Document, Element, HtmlElement, HtmlLinkElement, HtmlMetaElement, Storage, Window};

/// Browser API wrapper for metadata operations
#[derive(Debug, Clone)]
pub struct BrowserApi {
    window: Option<Window>,
    document: Option<Document>,
}

impl Default for BrowserApi {
    fn default() -> Self {
        Self::new()
    }
}

impl BrowserApi {
    /// Create a new browser API wrapper
    pub fn new() -> Self {
        let window = web_sys::window();
        let document = window.as_ref().and_then(|w| w.document());

        Self { window, document }
    }

    /// Get the window object
    pub fn window(&self) -> Option<&Window> {
        self.window.as_ref()
    }

    /// Get the document object
    pub fn document(&self) -> Option<&Document> {
        self.document.as_ref()
    }

    /// Set document title
    pub fn set_title(&self, title: &str) -> Result<(), JsValue> {
        if let Some(document) = &self.document {
            document.set_title(title);
            Ok(())
        } else {
            Err(JsValue::from_str("No document available"))
        }
    }

    /// Get document title
    pub fn get_title(&self) -> Result<String, JsValue> {
        if let Some(document) = &self.document {
            Ok(document.title())
        } else {
            Err(JsValue::from_str("No document available"))
        }
    }

    /// Set meta tag content
    pub fn set_meta_content(&self, name: &str, content: &str) -> Result<(), JsValue> {
        if let Some(document) = &self.document {
            let selector = format!("meta[name=\"{}\"]", name);
            let element = document.query_selector(&selector)?;

            if let Some(element) = element {
                // Update existing meta tag
                element.set_attribute("content", content)?;
            } else {
                // Create new meta tag
                let meta = document.create_element("meta")?;
                meta.set_attribute("name", name)?;
                meta.set_attribute("content", content)?;
                document
                    .head()
                    .ok_or("No head element")?
                    .append_child(&meta)?;
            }
            Ok(())
        } else {
            Err(JsValue::from_str("No document available"))
        }
    }

    /// Get meta tag content
    pub fn get_meta_content(&self, name: &str) -> Result<Option<String>, JsValue> {
        if let Some(document) = &self.document {
            let selector = format!("meta[name=\"{}\"]", name);
            let element = document.query_selector(&selector)?;

            if let Some(element) = element {
                let content = element.get_attribute("content");
                Ok(content)
            } else {
                Ok(None)
            }
        } else {
            Err(JsValue::from_str("No document available"))
        }
    }

    /// Set link tag href
    pub fn set_link_href(&self, rel: &str, href: &str) -> Result<(), JsValue> {
        if let Some(document) = &self.document {
            let selector = format!("link[rel=\"{}\"]", rel);
            let element = document.query_selector(&selector)?;

            if let Some(element) = element {
                // Update existing link tag
                element.set_attribute("href", href)?;
            } else {
                // Create new link tag
                let link = document.create_element("link")?;
                link.set_attribute("rel", rel)?;
                link.set_attribute("href", href)?;
                document
                    .head()
                    .ok_or("No head element")?
                    .append_child(&link)?;
            }
            Ok(())
        } else {
            Err(JsValue::from_str("No document available"))
        }
    }

    /// Get link tag href
    pub fn get_link_href(&self, rel: &str) -> Result<Option<String>, JsValue> {
        if let Some(document) = &self.document {
            let selector = format!("link[rel=\"{}\"]", rel);
            let element = document.query_selector(&selector)?;

            if let Some(element) = element {
                let href = element.get_attribute("href");
                Ok(href)
            } else {
                Ok(None)
            }
        } else {
            Err(JsValue::from_str("No document available"))
        }
    }

    /// Get local storage
    pub fn get_local_storage(&self) -> Result<Option<Storage>, JsValue> {
        if let Some(window) = &self.window {
            window.local_storage().map_err(|e| e.into())
        } else {
            Err(JsValue::from_str("No window available"))
        }
    }

    /// Get session storage
    pub fn get_session_storage(&self) -> Result<Option<Storage>, JsValue> {
        if let Some(window) = &self.window {
            window.session_storage().map_err(|e| e.into())
        } else {
            Err(JsValue::from_str("No window available"))
        }
    }

    /// Store data in local storage
    pub fn store_local(&self, key: &str, value: &str) -> Result<(), JsValue> {
        if let Some(storage) = self.get_local_storage()? {
            storage.set_item(key, value)?;
            Ok(())
        } else {
            Err(JsValue::from_str("Local storage not available"))
        }
    }

    /// Retrieve data from local storage
    pub fn retrieve_local(&self, key: &str) -> Result<Option<String>, JsValue> {
        if let Some(storage) = self.get_local_storage()? {
            Ok(storage.get_item(key)?)
        } else {
            Err(JsValue::from_str("Local storage not available"))
        }
    }

    /// Store data in session storage
    pub fn store_session(&self, key: &str, value: &str) -> Result<(), JsValue> {
        if let Some(storage) = self.get_session_storage()? {
            storage.set_item(key, value)?;
            Ok(())
        } else {
            Err(JsValue::from_str("Session storage not available"))
        }
    }

    /// Retrieve data from session storage
    pub fn retrieve_session(&self, key: &str) -> Result<Option<String>, JsValue> {
        if let Some(storage) = self.get_session_storage()? {
            Ok(storage.get_item(key)?)
        } else {
            Err(JsValue::from_str("Session storage not available"))
        }
    }

    /// Clear local storage
    pub fn clear_local_storage(&self) -> Result<(), JsValue> {
        if let Some(storage) = self.get_local_storage()? {
            storage.clear()?;
            Ok(())
        } else {
            Err(JsValue::from_str("Local storage not available"))
        }
    }

    /// Clear session storage
    pub fn clear_session_storage(&self) -> Result<(), JsValue> {
        if let Some(storage) = self.get_session_storage()? {
            storage.clear()?;
            Ok(())
        } else {
            Err(JsValue::from_str("Session storage not available"))
        }
    }

    /// Get current URL
    pub fn get_current_url(&self) -> Result<String, JsValue> {
        if let Some(window) = &self.window {
            Ok(window.location().href()?)
        } else {
            Err(JsValue::from_str("No window available"))
        }
    }

    /// Get current pathname
    pub fn get_current_pathname(&self) -> Result<String, JsValue> {
        if let Some(window) = &self.window {
            Ok(window.location().pathname()?)
        } else {
            Err(JsValue::from_str("No window available"))
        }
    }

    /// Get current origin
    pub fn get_current_origin(&self) -> Result<String, JsValue> {
        if let Some(window) = &self.window {
            Ok(window.location().origin()?)
        } else {
            Err(JsValue::from_str("No window available"))
        }
    }

    /// Navigate to URL
    pub fn navigate_to(&self, url: &str) -> Result<(), JsValue> {
        if let Some(window) = &self.window {
            window.location().set_href(url)?;
            Ok(())
        } else {
            Err(JsValue::from_str("No window available"))
        }
    }

    /// Reload current page
    pub fn reload(&self) -> Result<(), JsValue> {
        if let Some(window) = &self.window {
            window.location().reload()?;
            Ok(())
        } else {
            Err(JsValue::from_str("No window available"))
        }
    }

    /// Check if feature is supported
    pub fn is_feature_supported(&self, feature: &str) -> bool {
        if let Some(window) = &self.window {
            match feature {
                "localStorage" => window.local_storage().is_ok(),
                "sessionStorage" => window.session_storage().is_ok(),
                "fetch" => {
                    js_sys::Reflect::has(window, &JsValue::from_str("fetch")).unwrap_or(false)
                }
                "webgl" => self.check_webgl_support(),
                "canvas" => self.check_canvas_support(),
                "webworkers" => {
                    js_sys::Reflect::has(window, &JsValue::from_str("Worker")).unwrap_or(false)
                }
                _ => false,
            }
        } else {
            false
        }
    }

    /// Check WebGL support
    fn check_webgl_support(&self) -> bool {
        if let Some(document) = &self.document {
            if let Ok(canvas) = document.create_element("canvas") {
                if let Ok(canvas) = canvas.dyn_into::<web_sys::HtmlCanvasElement>() {
                    return canvas.get_context("webgl").is_ok()
                        || canvas.get_context("experimental-webgl").is_ok();
                }
            }
        }
        false
    }

    /// Check Canvas support
    fn check_canvas_support(&self) -> bool {
        if let Some(document) = &self.document {
            if let Ok(canvas) = document.create_element("canvas") {
                return canvas.dyn_into::<web_sys::HtmlCanvasElement>().is_ok();
            }
        }
        false
    }

    /// Get user agent
    pub fn get_user_agent(&self) -> Result<String, JsValue> {
        if let Some(window) = &self.window {
            Ok(window.navigator().user_agent()?)
        } else {
            Err(JsValue::from_str("No window available"))
        }
    }

    /// Get language
    pub fn get_language(&self) -> Result<String, JsValue> {
        if let Some(window) = &self.window {
            Ok(window
                .navigator()
                .language()
                .unwrap_or_else(|| "en".to_string()))
        } else {
            Err(JsValue::from_str("No window available"))
        }
    }

    /// Get platform
    pub fn get_platform(&self) -> Result<String, JsValue> {
        if let Some(window) = &self.window {
            Ok(window.navigator().platform()?)
        } else {
            Err(JsValue::from_str("No window available"))
        }
    }

    /// Get screen dimensions
    pub fn get_screen_dimensions(&self) -> Result<(u32, u32), JsValue> {
        if let Some(window) = &self.window {
            let screen = window.screen()?;
            Ok((screen.width()? as u32, screen.height()? as u32))
        } else {
            Err(JsValue::from_str("No window available"))
        }
    }

    /// Get viewport dimensions
    pub fn get_viewport_dimensions(&self) -> Result<(u32, u32), JsValue> {
        if let Some(window) = &self.window {
            Ok((
                window.inner_width()?.as_f64().unwrap_or(0.0) as u32,
                window.inner_height()?.as_f64().unwrap_or(0.0) as u32,
            ))
        } else {
            Err(JsValue::from_str("No window available"))
        }
    }
}
