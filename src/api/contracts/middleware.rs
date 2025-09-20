//! HTTP middleware implementations
//!
//! This module provides ready-to-use middleware for Axum and Tower frameworks
//! for API contract validation.

use super::types::*;
use super::validator::ContractValidator;
use crate::{Error, Result};
#[cfg(feature = "api-contracts")]
use axum::{
    body::Body,
    extract::Request,
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};
use std::collections::HashMap;
#[cfg(feature = "api-contracts")]
use tower::{Layer, Service};

/// Axum middleware for request/response validation
pub struct ContractMiddleware {
    validator: ContractValidator,
    #[allow(dead_code)]
    config: MiddlewareConfig,
}

impl ContractMiddleware {
    /// Create new contract middleware
    pub fn new(validator: ContractValidator) -> Self {
        Self {
            validator,
            config: MiddlewareConfig::default(),
        }
    }

    /// Create middleware with custom configuration
    pub fn with_config(validator: ContractValidator, config: MiddlewareConfig) -> Self {
        Self { validator, config }
    }

    /// Validate request middleware
    pub fn validate_request(
        &self,
        method: &str,
        path: &str,
        headers: &HashMap<String, String>,
        body: Option<&serde_json::Value>,
    ) -> ValidationResult {
        self.validator.validate_request(method, path, headers, body)
    }

    /// Validate response middleware
    pub fn validate_response(
        &self,
        method: &str,
        path: &str,
        status_code: u16,
        headers: &HashMap<String, String>,
        body: Option<&serde_json::Value>,
    ) -> ValidationResult {
        self.validator
            .validate_response(method, path, status_code, headers, body)
    }
}

/// Axum middleware function
#[cfg(feature = "api-contracts")]
pub async fn validate_contracts(
    request: Request,
    next: Next,
) -> std::result::Result<Response, StatusCode> {
    // Extract request details
    let method = request.method().to_string();
    let path = request.uri().path().to_string();
    let headers = convert_header_map(request.headers());

    // Read request body
    let (parts, body) = request.into_parts();
    let body_bytes = axum::body::to_bytes(body, usize::MAX)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let body_json = if !body_bytes.is_empty() {
        Some(serde_json::from_slice(&body_bytes).map_err(|_| StatusCode::BAD_REQUEST)?)
    } else {
        None
    };

    // Get validator from request extensions
    let validator = parts
        .extensions
        .get::<ContractValidator>()
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

    // Validate request
    let validation_result =
        validator.validate_request(&method, &path, &headers, body_json.as_ref());

    if !validation_result.is_valid() {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Reconstruct request and continue
    let request = Request::from_parts(parts, Body::from(body_bytes));
    let response = next.run(request).await;

    // TODO: Add response validation if needed

    Ok(response)
}

/// Convert HeaderMap to HashMap<String, String>
#[cfg(feature = "api-contracts")]
fn convert_header_map(headers: &HeaderMap) -> HashMap<String, String> {
    headers
        .iter()
        .map(|(name, value)| (name.to_string(), value.to_str().unwrap_or("").to_string()))
        .collect()
}

/// Tower layer implementation
#[cfg(feature = "api-contracts")]
#[derive(Clone)]
pub struct ContractValidationLayer {
    validator: ContractValidator,
    config: MiddlewareConfig,
}

#[cfg(feature = "api-contracts")]
impl ContractValidationLayer {
    /// Create new validation layer
    pub fn new(validator: ContractValidator) -> Self {
        Self {
            validator,
            config: MiddlewareConfig::default(),
        }
    }

    /// Create validation layer with custom configuration
    pub fn with_config(validator: ContractValidator, config: MiddlewareConfig) -> Self {
        Self { validator, config }
    }
}

#[cfg(feature = "api-contracts")]
impl<S> Layer<S> for ContractValidationLayer {
    type Service = ContractValidationService<S>;

    fn layer(&self, service: S) -> Self::Service {
        ContractValidationService {
            inner: service,
            validator: self.validator.clone(),
            config: self.config.clone(),
        }
    }
}

#[cfg(feature = "api-contracts")]
#[derive(Clone)]
pub struct ContractValidationService<S> {
    inner: S,
    validator: ContractValidator,
    config: MiddlewareConfig,
}

#[cfg(feature = "api-contracts")]
impl<S> Service<Request> for ContractValidationService<S>
where
    S: Service<Request, Response = Response> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = std::pin::Pin<
        Box<
            dyn std::future::Future<Output = std::result::Result<Self::Response, Self::Error>>
                + Send,
        >,
    >;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::result::Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut request: Request) -> Self::Future {
        let validator = self.validator.clone();
        let _config = self.config.clone();
        let mut inner = self.inner.clone();

        Box::pin(async move {
            // Add validator to request extensions
            request.extensions_mut().insert(validator);

            // Call inner service
            inner.call(request).await
        })
    }
}

/// Middleware configuration
#[derive(Debug, Clone)]
pub struct MiddlewareConfig {
    /// Whether to validate requests
    pub validate_requests: bool,
    /// Whether to validate responses
    pub validate_responses: bool,
    /// Strict mode - fail on any validation error
    pub strict_mode: bool,
    /// Include error response body with details
    pub error_response_body: bool,
    /// Maximum request size in bytes
    pub max_request_size: usize,
    /// Maximum response size in bytes
    pub max_response_size: usize,
}

impl Default for MiddlewareConfig {
    fn default() -> Self {
        Self {
            validate_requests: true,
            validate_responses: false, // Response validation is expensive
            strict_mode: false,
            error_response_body: true,
            max_request_size: 1024 * 1024,       // 1MB
            max_response_size: 10 * 1024 * 1024, // 10MB
        }
    }
}

/// Load OpenAPI schema from file
pub fn load_openapi_schema(path: &str) -> Result<openapiv3::OpenAPI> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| Error::ValidationError(format!("Failed to read schema file: {}", e)))?;

    let schema: openapiv3::OpenAPI = serde_yaml::from_str(&content)
        .map_err(|e| Error::ValidationError(format!("Failed to parse OpenAPI schema: {}", e)))?;

    Ok(schema)
}

/// Create default contract validator
pub fn create_default_validator() -> Result<ContractValidator> {
    let schema_path = "docs/api/openapi.yaml";
    let schema = load_openapi_schema(schema_path)?;
    ContractValidator::new(schema)
}

/// Create minimal OpenAPI spec for testing
pub fn create_minimal_spec() -> openapiv3::OpenAPI {
    openapiv3::OpenAPI {
        openapi: "3.0.0".to_string(),
        info: openapiv3::Info {
            title: "Default API".to_string(),
            version: "1.0.0".to_string(),
            ..Default::default()
        },
        ..Default::default()
    }
}

/// Create validator with minimal spec for testing
pub fn create_test_validator() -> Result<ContractValidator> {
    let spec = create_minimal_spec();
    ContractValidator::new(spec)
}

/// Convenience function to create validation layer
pub fn create_validation_layer(schema_path: &str) -> Result<ContractValidationLayer> {
    let schema = load_openapi_schema(schema_path)?;
    let validator = ContractValidator::new(schema)?;
    Ok(ContractValidationLayer::new(validator))
}

/// Convenience function to create validation layer with config
pub fn create_validation_layer_with_config(
    schema_path: &str,
    config: MiddlewareConfig,
) -> Result<ContractValidationLayer> {
    let schema = load_openapi_schema(schema_path)?;
    let validator = ContractValidator::new(schema)?;
    Ok(ContractValidationLayer::with_config(validator, config))
}
