# API Contracts Component Design

## Overview
Design specification for robust API contract validation with real OpenAPI schema support.

## Current Problems  
- ContractValidator has only 15/171 lines tested (8.8% coverage)
- No real OpenAPI schema traversal - only hard-coded heuristics
- Missing integration tests and middleware examples
- No validation rule composition or extensibility
- Placeholder validation logic insufficient for production

## Design Goals
- **Real OpenAPI Support**: Full schema validation against OpenAPI 3.x specs
- **Extensible Rules**: Pluggable validation rule system
- **Framework Integration**: Ready-to-use middleware for Axum, Tower
- **Performance**: Sub-10ms validation for typical payloads
- **Comprehensive Testing**: >90% coverage with integration tests

## Architecture

### 1. Core Validator (`validator.rs`)
**Responsibility**: Schema-driven validation engine  
**Line Target**: 280 lines

```rust
//! OpenAPI contract validation engine

use openapiv3::{OpenAPI, Schema, ReferenceOr};
use serde_json::Value;
use std::collections::HashMap;
use crate::api::contracts::{rules::*, config::*};

pub struct ContractValidator {
    spec: OpenAPI,
    rules: Vec<Box<dyn ValidationRule>>,
    config: ValidatorConfig,
    schema_cache: HashMap<String, Schema>,
}

impl ContractValidator {
    /// Create validator from OpenAPI specification
    pub fn new(spec: OpenAPI) -> Result<Self, ValidationError> {
        let mut validator = Self {
            spec,
            rules: Vec::new(),
            config: ValidatorConfig::default(),
            schema_cache: HashMap::new(),
        };
        
        // Add default validation rules
        validator.add_rule(Box::new(SchemaComplianceRule));
        validator.add_rule(Box::new(RequiredFieldsRule));  
        validator.add_rule(Box::new(TypeConstraintsRule));
        validator.add_rule(Box::new(FormatValidationRule));
        
        Ok(validator)
    }
    
    /// Add custom validation rule
    pub fn add_rule(&mut self, rule: Box<dyn ValidationRule>) {
        self.rules.push(rule);
    }
    
    /// Validate request against OpenAPI spec
    pub fn validate_request(
        &self,
        method: &str,
        path: &str,
        headers: &HashMap<String, String>,
        body: Option<&Value>,
    ) -> ValidationResult {
        let mut result = ValidationResult::new();
        
        // Find matching operation
        let operation = match self.find_operation(method, path) {
            Some(op) => op,
            None => {
                result.add_error(ValidationError::PathNotFound { 
                    path: path.to_string() 
                });
                return result;
            }
        };
        
        // Validate request body if present
        if let (Some(body), Some(request_body)) = (body, &operation.request_body) {
            if let Err(error) = self.validate_request_body(body, request_body) {
                result.add_error(error);
            }
        }
        
        // Apply custom validation rules  
        for rule in &self.rules {
            if let Err(error) = rule.validate_request(method, path, headers, body) {
                result.add_error(error);
            }
        }
        
        result
    }
    
    /// Validate response against OpenAPI spec
    pub fn validate_response(
        &self,
        method: &str,
        path: &str,
        status_code: u16,
        headers: &HashMap<String, String>,
        body: Option<&Value>,
    ) -> ValidationResult {
        let mut result = ValidationResult::new();
        
        let operation = match self.find_operation(method, path) {
            Some(op) => op,
            None => return result, // Skip response validation for unknown endpoints
        };
        
        // Validate response schema
        if let Some(response_spec) = operation.responses.responses.get(&status_code.to_string()) {
            if let (Some(body), Some(content)) = (body, &response_spec.content) {
                if let Err(error) = self.validate_response_body(body, content) {
                    result.add_error(error);
                }
            }
        }
        
        result
    }
    
    fn find_operation(&self, method: &str, path: &str) -> Option<&openapiv3::Operation> {
        // Find matching path item
        for (spec_path, path_item) in &self.spec.paths.paths {
            if self.path_matches(spec_path, path) {
                return match method.to_uppercase().as_str() {
                    "GET" => path_item.get.as_ref(),
                    "POST" => path_item.post.as_ref(),  
                    "PUT" => path_item.put.as_ref(),
                    "DELETE" => path_item.delete.as_ref(),
                    "PATCH" => path_item.patch.as_ref(),
                    _ => None,
                };
            }
        }
        None
    }
    
    fn validate_request_body(
        &self, 
        body: &Value, 
        request_body: &ReferenceOr<openapiv3::RequestBody>
    ) -> Result<(), ValidationError> {
        // Resolve request body schema
        let request_body = match request_body {
            ReferenceOr::Item(rb) => rb,
            ReferenceOr::Reference { reference } => {
                return Err(ValidationError::UnresolvedReference { 
                    reference: reference.clone() 
                });
            }
        };
        
        // Get JSON schema from content
        if let Some(json_content) = request_body.content.get("application/json") {
            if let Some(schema) = &json_content.schema {
                return self.validate_against_schema(body, schema);
            }
        }
        
        Ok(())
    }
    
    fn validate_against_schema(
        &self, 
        value: &Value, 
        schema: &ReferenceOr<Schema>
    ) -> Result<(), ValidationError> {
        let schema = match schema {
            ReferenceOr::Item(s) => s,
            ReferenceOr::Reference { reference } => {
                // Resolve schema reference
                return self.resolve_and_validate(value, reference);
            }
        };
        
        match &schema.schema_kind {
            openapiv3::SchemaKind::Type(type_schema) => {
                self.validate_type_schema(value, type_schema)
            }
            openapiv3::SchemaKind::OneOf { one_of } => {
                self.validate_one_of(value, one_of)
            }
            openapiv3::SchemaKind::AllOf { all_of } => {
                self.validate_all_of(value, all_of)  
            }
            openapiv3::SchemaKind::AnyOf { any_of } => {
                self.validate_any_of(value, any_of)
            }
            _ => Ok(()), // Skip unknown schema types
        }
    }
}
```

### 2. Validation Rules (`rules.rs`)
**Responsibility**: Pluggable validation rule implementations
**Line Target**: 250 lines

```rust
//! Validation rule implementations

use serde_json::Value;
use std::collections::HashMap;
use crate::api::contracts::ValidationError;

/// Trait for custom validation rules
pub trait ValidationRule: Send + Sync {
    fn name(&self) -> &str;
    
    fn validate_request(
        &self,
        method: &str,
        path: &str, 
        headers: &HashMap<String, String>,
        body: Option<&Value>,
    ) -> Result<(), ValidationError>;
    
    fn validate_response(
        &self,
        method: &str,
        path: &str,
        status_code: u16,
        headers: &HashMap<String, String>, 
        body: Option<&Value>,
    ) -> Result<(), ValidationError> {
        // Default: no response validation
        Ok(())
    }
}

/// Validates basic schema compliance
pub struct SchemaComplianceRule;

impl ValidationRule for SchemaComplianceRule {
    fn name(&self) -> &str { "schema-compliance" }
    
    fn validate_request(
        &self,
        method: &str,
        path: &str,
        headers: &HashMap<String, String>,
        body: Option<&Value>,
    ) -> Result<(), ValidationError> {
        // Validate Content-Type header for requests with body
        if body.is_some() && method != "GET" {
            if !headers.contains_key("content-type") {
                return Err(ValidationError::MissingHeader {
                    header: "content-type".to_string(),
                });
            }
        }
        
        Ok(())
    }
}

/// Validates required fields are present
pub struct RequiredFieldsRule;

impl ValidationRule for RequiredFieldsRule {
    fn name(&self) -> &str { "required-fields" }
    
    fn validate_request(
        &self,
        _method: &str,
        _path: &str,
        _headers: &HashMap<String, String>,
        body: Option<&Value>,
    ) -> Result<(), ValidationError> {
        if let Some(Value::Object(obj)) = body {
            // Check for common required fields in metadata
            if !obj.contains_key("title") && !obj.contains_key("description") {
                return Err(ValidationError::MissingRequiredField {
                    field: "title or description".to_string(),
                });
            }
        }
        
        Ok(())
    }
}

/// Validates data types match schema constraints
pub struct TypeConstraintsRule;

impl ValidationRule for TypeConstraintsRule {
    fn name(&self) -> &str { "type-constraints" }
    
    fn validate_request(
        &self,
        _method: &str,
        _path: &str,
        _headers: &HashMap<String, String>,
        body: Option<&Value>,
    ) -> Result<(), ValidationError> {
        if let Some(Value::Object(obj)) = body {
            // Validate string fields
            for (key, value) in obj {
                if key.ends_with("_url") {
                    if let Value::String(url) = value {
                        if !self.is_valid_url(url) {
                            return Err(ValidationError::InvalidFormat {
                                field: key.clone(),
                                expected: "valid URL".to_string(),
                                actual: url.clone(),
                            });
                        }
                    }
                }
            }
        }
        
        Ok(())
    }
}

impl TypeConstraintsRule {
    fn is_valid_url(&self, url: &str) -> bool {
        url::Url::parse(url).is_ok()
    }
}

/// Validates string formats (email, url, date, etc.)
pub struct FormatValidationRule;

impl ValidationRule for FormatValidationRule {
    fn name(&self) -> &str { "format-validation" }
    
    fn validate_request(
        &self,
        _method: &str,
        _path: &str,
        _headers: &HashMap<String, String>,
        body: Option<&Value>,
    ) -> Result<(), ValidationError> {
        if let Some(Value::Object(obj)) = body {
            for (key, value) in obj {
                if let Value::String(s) = value {
                    if key.contains("email") {
                        if !self.is_valid_email(s) {
                            return Err(ValidationError::InvalidFormat {
                                field: key.clone(),
                                expected: "valid email".to_string(), 
                                actual: s.clone(),
                            });
                        }
                    }
                }
            }
        }
        
        Ok(())
    }
}

impl FormatValidationRule {
    fn is_valid_email(&self, email: &str) -> bool {
        // Simple email validation
        email.contains('@') && email.contains('.') && email.len() > 5
    }
}
```

### 3. HTTP Middleware (`middleware.rs`)
**Responsibility**: Framework integration adapters
**Line Target**: 200 lines

```rust
//! HTTP middleware implementations

use axum::{
    body::{Body, Bytes},
    extract::Request,
    http::{HeaderMap, Method, StatusCode},
    middleware::Next,
    response::Response,
};
use std::collections::HashMap;
use tower::{Layer, Service};
use crate::api::contracts::{ContractValidator, ValidationResult};

/// Axum middleware for request/response validation
pub struct ContractMiddleware {
    validator: ContractValidator,
    config: MiddlewareConfig,
}

impl ContractMiddleware {
    pub fn new(validator: ContractValidator) -> Self {
        Self {
            validator,
            config: MiddlewareConfig::default(),
        }
    }
    
    pub fn with_config(mut self, config: MiddlewareConfig) -> Self {
        self.config = config;
        self
    }
}

/// Axum middleware function
pub async fn validate_contracts(
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Extract request details
    let method = request.method().to_string();
    let path = request.uri().path().to_string();
    let headers = convert_header_map(request.headers());
    
    // Read request body
    let (mut parts, body) = request.into_parts();
    let body_bytes = axum::body::to_bytes(body, usize::MAX).await
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    let body_json = if !body_bytes.is_empty() {
        Some(serde_json::from_slice(&body_bytes)
            .map_err(|_| StatusCode::BAD_REQUEST)?)
    } else {
        None
    };
    
    // Get validator from request extensions
    let validator = parts.extensions.get::<ContractValidator>()
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // Validate request
    let validation_result = validator.validate_request(
        &method,
        &path,
        &headers,
        body_json.as_ref(),
    );
    
    if !validation_result.is_valid() {
        return Err(StatusCode::BAD_REQUEST);
    }
    
    // Reconstruct request and continue
    let request = Request::from_parts(parts, Body::from(body_bytes));
    let response = next.run(request).await;
    
    // TODO: Add response validation if needed
    
    Ok(response)
}

fn convert_header_map(headers: &HeaderMap) -> HashMap<String, String> {
    headers.iter()
        .map(|(name, value)| {
            (
                name.to_string(),
                value.to_str().unwrap_or("").to_string(),
            )
        })
        .collect()
}

/// Tower layer implementation  
#[derive(Clone)]
pub struct ContractValidationLayer {
    validator: ContractValidator,
}

impl ContractValidationLayer {
    pub fn new(validator: ContractValidator) -> Self {
        Self { validator }
    }
}

impl<S> Layer<S> for ContractValidationLayer {
    type Service = ContractValidationService<S>;
    
    fn layer(&self, service: S) -> Self::Service {
        ContractValidationService {
            inner: service,
            validator: self.validator.clone(),
        }
    }
}

#[derive(Clone)]
pub struct ContractValidationService<S> {
    inner: S,
    validator: ContractValidator,
}

impl<S> Service<Request> for ContractValidationService<S>
where
    S: Service<Request, Response = Response> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = std::pin::Pin<Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>> + Send>>;
    
    fn poll_ready(&mut self, cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }
    
    fn call(&mut self, mut request: Request) -> Self::Future {
        let validator = self.validator.clone();
        let mut inner = self.inner.clone();
        
        Box::pin(async move {
            // Add validator to request extensions
            request.extensions_mut().insert(validator);
            
            // Call inner service
            inner.call(request).await
        })
    }
}

#[derive(Debug, Clone)]
pub struct MiddlewareConfig {
    pub validate_requests: bool,
    pub validate_responses: bool,
    pub strict_mode: bool,
    pub error_response_body: bool,
}

impl Default for MiddlewareConfig {
    fn default() -> Self {
        Self {
            validate_requests: true,
            validate_responses: false, // Response validation is expensive
            strict_mode: false,
            error_response_body: true,
        }
    }
}
```

### 4. Module Interface (`mod.rs`)  
**Responsibility**: Clean public API
**Line Target**: 50 lines

```rust
//! API contract validation module

mod validator;
mod rules;  
mod middleware;

pub use validator::ContractValidator;
pub use rules::{ValidationRule, ValidationResult, ValidationError};
pub use middleware::{ContractMiddleware, ContractValidationLayer, validate_contracts};

// Convenience functions
pub fn load_openapi_spec(path: &str) -> Result<openapiv3::OpenAPI, std::io::Error> {
    let content = std::fs::read_to_string(path)?;
    serde_yaml::from_str(&content)
        .or_else(|_| serde_json::from_str(&content))
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
}

pub fn create_default_validator() -> Result<ContractValidator, ValidationError> {
    // Load default OpenAPI spec or create minimal spec
    let spec = create_minimal_spec();
    ContractValidator::new(spec)
}

fn create_minimal_spec() -> openapiv3::OpenAPI {
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
```

## Testing Strategy

### 1. Schema Validation Tests
```rust
// tests/validator_test.rs
#[tokio::test]
async fn test_openapi_schema_validation() {
    let spec = load_test_schema("tests/fixtures/metadata-api.yaml").unwrap();
    let validator = ContractValidator::new(spec).unwrap();
    
    let invalid_request = serde_json::json!({
        "title": 123, // Should be string
        "description": "Valid description"
    });
    
    let result = validator.validate_request(
        "POST",
        "/api/metadata",
        &HashMap::new(),
        Some(&invalid_request),
    );
    
    assert!(!result.is_valid());
    assert_eq!(result.errors.len(), 1);
    assert!(result.errors[0].message.contains("type mismatch"));
}
```

### 2. Middleware Integration Tests  
```rust
#[tokio::test]
async fn test_axum_middleware_integration() {
    let spec = load_test_schema("tests/fixtures/metadata-api.yaml").unwrap();
    let validator = ContractValidator::new(spec).unwrap();
    
    let app = Router::new()
        .route("/api/metadata", post(create_metadata))
        .layer(ContractValidationLayer::new(validator));
        
    let invalid_request = Request::builder()
        .method("POST")
        .uri("/api/metadata")
        .header("content-type", "application/json")
        .body(r#"{"title": 123}"#) // Invalid type
        .unwrap();
        
    let response = app.oneshot(invalid_request).await.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}
```

## Migration Plan

### Phase 1: Core Validator (3 days)
1. Implement real OpenAPI schema parsing
2. Add proper schema traversal and validation
3. Create comprehensive error types
4. Add extensive unit tests

### Phase 2: Validation Rules (2 days)
1. Implement pluggable rule system  
2. Add common validation rules
3. Test rule composition and extension
4. Document custom rule creation

### Phase 3: Middleware Integration (2 days)
1. Create Axum middleware adapter
2. Add Tower layer implementation
3. Test framework integration
4. Add configuration options

### Phase 4: Testing & Documentation (2 days)
1. Add comprehensive test suite
2. Create integration test examples
3. Write middleware usage guides
4. Performance benchmarking

## Success Criteria
- [ ] Real OpenAPI 3.x schema support
- [ ] >90% test coverage with integration tests
- [ ] Ready-to-use Axum/Tower middleware
- [ ] Sub-10ms validation performance
- [ ] Comprehensive error reporting
- [ ] Pluggable validation rule system
- [ ] Production-ready error handling
