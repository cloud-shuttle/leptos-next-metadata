//! Security utilities for WASM environments
//!
//! Provides security measures, validation, and secure defaults for WASM applications

use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use web_sys::{Document, HtmlElement, Window};

/// Security configuration for WASM applications
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// Content Security Policy settings
    pub csp: CspConfig,
    /// Input validation settings
    pub validation: ValidationConfig,
    /// Secure defaults
    pub secure_defaults: SecureDefaults,
    /// Allowed origins
    pub allowed_origins: Vec<String>,
    /// Security headers
    pub security_headers: HashMap<String, String>,
}

/// Content Security Policy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CspConfig {
    /// Default source policy
    pub default_src: Vec<String>,
    /// Script source policy
    pub script_src: Vec<String>,
    /// Style source policy
    pub style_src: Vec<String>,
    /// Image source policy
    pub img_src: Vec<String>,
    /// Font source policy
    pub font_src: Vec<String>,
    /// Connect source policy
    pub connect_src: Vec<String>,
    /// Frame source policy
    pub frame_src: Vec<String>,
    /// Object source policy
    pub object_src: Vec<String>,
    /// Media source policy
    pub media_src: Vec<String>,
    /// Manifest source policy
    pub manifest_src: Vec<String>,
    /// Worker source policy
    pub worker_src: Vec<String>,
    /// Child source policy
    pub child_src: Vec<String>,
    /// Form action policy
    pub form_action: Vec<String>,
    /// Frame ancestors policy
    pub frame_ancestors: Vec<String>,
    /// Base URI policy
    pub base_uri: Vec<String>,
    /// Upgrade insecure requests
    pub upgrade_insecure_requests: bool,
    /// Block all mixed content
    pub block_all_mixed_content: bool,
    /// Require SRI for scripts
    pub require_sri_for: Vec<String>,
}

/// Input validation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationConfig {
    /// Maximum string length
    pub max_string_length: usize,
    /// Maximum array length
    pub max_array_length: usize,
    /// Maximum object depth
    pub max_object_depth: usize,
    /// Allowed HTML tags
    pub allowed_html_tags: Vec<String>,
    /// Allowed HTML attributes
    pub allowed_html_attributes: Vec<String>,
    /// Allowed protocols
    pub allowed_protocols: Vec<String>,
    /// Sanitize HTML
    pub sanitize_html: bool,
    /// Validate URLs
    pub validate_urls: bool,
    /// Validate emails
    pub validate_emails: bool,
}

/// Secure defaults configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecureDefaults {
    /// Use HTTPS only
    pub https_only: bool,
    /// Disable eval
    pub disable_eval: bool,
    /// Disable inline scripts
    pub disable_inline_scripts: bool,
    /// Disable inline styles
    pub disable_inline_styles: bool,
    /// Disable data URIs
    pub disable_data_uris: bool,
    /// Disable blob URIs
    pub disable_blob_uris: bool,
    /// Disable javascript URIs
    pub disable_javascript_uris: bool,
    /// Require SRI
    pub require_sri: bool,
    /// Enable HSTS
    pub enable_hsts: bool,
    /// Enable XSS protection
    pub enable_xss_protection: bool,
    /// Enable content type sniffing protection
    pub enable_content_type_sniffing_protection: bool,
}

/// Security validator for WASM applications
#[derive(Debug)]
pub struct SecurityValidator {
    /// Security configuration
    config: SecurityConfig,
    /// Compiled regex patterns
    patterns: ValidationPatterns,
}

/// Compiled validation patterns
#[derive(Debug)]
struct ValidationPatterns {
    /// URL validation pattern
    url_pattern: Regex,
    /// Email validation pattern
    email_pattern: Regex,
    /// HTML tag pattern
    html_tag_pattern: Regex,
    /// HTML attribute pattern
    html_attribute_pattern: Regex,
    /// Script pattern
    script_pattern: Regex,
    /// Style pattern
    style_pattern: Regex,
}

/// Security audit result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityAudit {
    /// Overall security score (0-100)
    pub security_score: u8,
    /// Security issues found
    pub issues: Vec<SecurityIssue>,
    /// Recommendations
    pub recommendations: Vec<String>,
    /// Compliance status
    pub compliance: ComplianceStatus,
}

/// Security issue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityIssue {
    /// Issue severity
    pub severity: SecuritySeverity,
    /// Issue category
    pub category: SecurityCategory,
    /// Issue description
    pub description: String,
    /// Issue location
    pub location: Option<String>,
    /// Fix suggestion
    pub fix_suggestion: Option<String>,
}

/// Security severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecuritySeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Security issue categories
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityCategory {
    Csp,
    Validation,
    Headers,
    Content,
    Network,
    Storage,
    Dom,
}

/// Compliance status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceStatus {
    /// OWASP compliance
    pub owasp: bool,
    /// CSP compliance
    pub csp: bool,
    /// HTTPS compliance
    pub https: bool,
    /// Input validation compliance
    pub input_validation: bool,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            csp: CspConfig::default(),
            validation: ValidationConfig::default(),
            secure_defaults: SecureDefaults::default(),
            allowed_origins: vec!["self".to_string()],
            security_headers: HashMap::new(),
        }
    }
}

impl Default for CspConfig {
    fn default() -> Self {
        Self {
            default_src: vec!["'self'".to_string()],
            script_src: vec!["'self'".to_string()],
            style_src: vec!["'self'".to_string(), "'unsafe-inline'".to_string()],
            img_src: vec!["'self'".to_string(), "data:".to_string()],
            font_src: vec!["'self'".to_string()],
            connect_src: vec!["'self'".to_string()],
            frame_src: vec!["'none'".to_string()],
            object_src: vec!["'none'".to_string()],
            media_src: vec!["'self'".to_string()],
            manifest_src: vec!["'self'".to_string()],
            worker_src: vec!["'self'".to_string()],
            child_src: vec!["'self'".to_string()],
            form_action: vec!["'self'".to_string()],
            frame_ancestors: vec!["'none'".to_string()],
            base_uri: vec!["'self'".to_string()],
            upgrade_insecure_requests: true,
            block_all_mixed_content: true,
            require_sri_for: vec!["script".to_string(), "style".to_string()],
        }
    }
}

impl Default for ValidationConfig {
    fn default() -> Self {
        Self {
            max_string_length: 10000,
            max_array_length: 1000,
            max_object_depth: 10,
            allowed_html_tags: vec![
                "p".to_string(),
                "br".to_string(),
                "strong".to_string(),
                "em".to_string(),
                "u".to_string(),
                "i".to_string(),
                "b".to_string(),
                "span".to_string(),
                "div".to_string(),
                "h1".to_string(),
                "h2".to_string(),
                "h3".to_string(),
                "h4".to_string(),
                "h5".to_string(),
                "h6".to_string(),
                "ul".to_string(),
                "ol".to_string(),
                "li".to_string(),
                "a".to_string(),
                "img".to_string(),
            ],
            allowed_html_attributes: vec![
                "class".to_string(),
                "id".to_string(),
                "href".to_string(),
                "src".to_string(),
                "alt".to_string(),
                "title".to_string(),
                "target".to_string(),
                "rel".to_string(),
            ],
            allowed_protocols: vec![
                "http".to_string(),
                "https".to_string(),
                "mailto".to_string(),
            ],
            sanitize_html: true,
            validate_urls: true,
            validate_emails: true,
        }
    }
}

impl Default for SecureDefaults {
    fn default() -> Self {
        Self {
            https_only: true,
            disable_eval: true,
            disable_inline_scripts: true,
            disable_inline_styles: false, // Allow inline styles for WASM
            disable_data_uris: false,     // Allow data URIs for WASM
            disable_blob_uris: false,     // Allow blob URIs for WASM
            disable_javascript_uris: true,
            require_sri: true,
            enable_hsts: true,
            enable_xss_protection: true,
            enable_content_type_sniffing_protection: true,
        }
    }
}

impl SecurityValidator {
    /// Create a new security validator
    pub fn new() -> Result<Self, JsValue> {
        let config = SecurityConfig::default();
        let patterns = ValidationPatterns::new()?;
        Ok(Self { config, patterns })
    }

    /// Create with custom configuration
    pub fn with_config(config: SecurityConfig) -> Result<Self, JsValue> {
        let patterns = ValidationPatterns::new()?;
        Ok(Self { config, patterns })
    }

    /// Validate input string
    pub fn validate_string(&self, input: &str) -> Result<ValidationResult, JsValue> {
        let mut issues = Vec::new();

        // Check length
        if input.len() > self.config.validation.max_string_length {
            issues.push(ValidationIssue {
                field: "length".to_string(),
                message: format!(
                    "String too long: {} > {}",
                    input.len(),
                    self.config.validation.max_string_length
                ),
                severity: ValidationSeverity::High,
            });
        }

        // Check for script injection
        if self.patterns.script_pattern.is_match(input) {
            issues.push(ValidationIssue {
                field: "content".to_string(),
                message: "Potential script injection detected".to_string(),
                severity: ValidationSeverity::Critical,
            });
        }

        // Check for style injection
        if self.patterns.style_pattern.is_match(input) {
            issues.push(ValidationIssue {
                field: "content".to_string(),
                message: "Potential style injection detected".to_string(),
                severity: ValidationSeverity::High,
            });
        }

        Ok(ValidationResult {
            is_valid: issues.is_empty(),
            issues,
        })
    }

    /// Validate URL
    pub fn validate_url(&self, url: &str) -> Result<ValidationResult, JsValue> {
        let mut issues = Vec::new();

        // Check URL format
        if !self.patterns.url_pattern.is_match(url) {
            issues.push(ValidationIssue {
                field: "url".to_string(),
                message: "Invalid URL format".to_string(),
                severity: ValidationSeverity::High,
            });
        }

        // Check protocol
        if let Some(protocol) = url.split(':').next() {
            if !self
                .config
                .validation
                .allowed_protocols
                .contains(&protocol.to_string())
            {
                issues.push(ValidationIssue {
                    field: "protocol".to_string(),
                    message: format!("Protocol not allowed: {}", protocol),
                    severity: ValidationSeverity::High,
                });
            }
        }

        Ok(ValidationResult {
            is_valid: issues.is_empty(),
            issues,
        })
    }

    /// Validate email
    pub fn validate_email(&self, email: &str) -> Result<ValidationResult, JsValue> {
        let mut issues = Vec::new();

        if !self.patterns.email_pattern.is_match(email) {
            issues.push(ValidationIssue {
                field: "email".to_string(),
                message: "Invalid email format".to_string(),
                severity: ValidationSeverity::Medium,
            });
        }

        Ok(ValidationResult {
            is_valid: issues.is_empty(),
            issues,
        })
    }

    /// Sanitize HTML content
    pub fn sanitize_html(&self, html: &str) -> Result<String, JsValue> {
        if !self.config.validation.sanitize_html {
            return Ok(html.to_string());
        }

        // Simple HTML sanitization (in a real implementation, you'd use a proper HTML sanitizer)
        let mut sanitized = html.to_string();

        // Remove script tags
        sanitized = self
            .patterns
            .script_pattern
            .replace_all(&sanitized, "")
            .to_string();

        // Remove style tags
        sanitized = self
            .patterns
            .style_pattern
            .replace_all(&sanitized, "")
            .to_string();

        // Remove dangerous attributes
        for attr in &[
            "onclick",
            "onload",
            "onerror",
            "onmouseover",
            "onfocus",
            "onblur",
        ] {
            let pattern = format!(r#"{}\s*=\s*"[^"]*""#, attr);
            if let Ok(regex) = Regex::new(&pattern) {
                sanitized = regex.replace_all(&sanitized, "").to_string();
            }
        }

        Ok(sanitized)
    }

    /// Generate CSP header
    pub fn generate_csp_header(&self) -> String {
        let mut csp = String::new();

        // Add directives
        csp.push_str(&format!(
            "default-src {}",
            self.config.csp.default_src.join(" ")
        ));
        csp.push_str(&format!(
            "; script-src {}",
            self.config.csp.script_src.join(" ")
        ));
        csp.push_str(&format!(
            "; style-src {}",
            self.config.csp.style_src.join(" ")
        ));
        csp.push_str(&format!("; img-src {}", self.config.csp.img_src.join(" ")));
        csp.push_str(&format!(
            "; font-src {}",
            self.config.csp.font_src.join(" ")
        ));
        csp.push_str(&format!(
            "; connect-src {}",
            self.config.csp.connect_src.join(" ")
        ));
        csp.push_str(&format!(
            "; frame-src {}",
            self.config.csp.frame_src.join(" ")
        ));
        csp.push_str(&format!(
            "; object-src {}",
            self.config.csp.object_src.join(" ")
        ));
        csp.push_str(&format!(
            "; media-src {}",
            self.config.csp.media_src.join(" ")
        ));
        csp.push_str(&format!(
            "; manifest-src {}",
            self.config.csp.manifest_src.join(" ")
        ));
        csp.push_str(&format!(
            "; worker-src {}",
            self.config.csp.worker_src.join(" ")
        ));
        csp.push_str(&format!(
            "; child-src {}",
            self.config.csp.child_src.join(" ")
        ));
        csp.push_str(&format!(
            "; form-action {}",
            self.config.csp.form_action.join(" ")
        ));
        csp.push_str(&format!(
            "; frame-ancestors {}",
            self.config.csp.frame_ancestors.join(" ")
        ));
        csp.push_str(&format!(
            "; base-uri {}",
            self.config.csp.base_uri.join(" ")
        ));

        // Add boolean directives
        if self.config.csp.upgrade_insecure_requests {
            csp.push_str("; upgrade-insecure-requests");
        }
        if self.config.csp.block_all_mixed_content {
            csp.push_str("; block-all-mixed-content");
        }
        if !self.config.csp.require_sri_for.is_empty() {
            csp.push_str(&format!(
                "; require-sri-for {}",
                self.config.csp.require_sri_for.join(" ")
            ));
        }

        csp
    }

    /// Perform security audit
    pub fn perform_audit(&self) -> Result<SecurityAudit, JsValue> {
        let mut issues = Vec::new();
        let mut recommendations = Vec::new();

        // Check CSP configuration
        if self
            .config
            .csp
            .script_src
            .contains(&"'unsafe-eval'".to_string())
        {
            issues.push(SecurityIssue {
                severity: SecuritySeverity::Critical,
                category: SecurityCategory::Csp,
                description: "CSP allows unsafe-eval".to_string(),
                location: Some("CSP script-src".to_string()),
                fix_suggestion: Some("Remove 'unsafe-eval' from script-src".to_string()),
            });
        }

        if self
            .config
            .csp
            .script_src
            .contains(&"'unsafe-inline'".to_string())
        {
            issues.push(SecurityIssue {
                severity: SecuritySeverity::High,
                category: SecurityCategory::Csp,
                description: "CSP allows unsafe-inline scripts".to_string(),
                location: Some("CSP script-src".to_string()),
                fix_suggestion: Some("Remove 'unsafe-inline' from script-src".to_string()),
            });
        }

        // Check validation configuration
        if self.config.validation.max_string_length > 50000 {
            issues.push(SecurityIssue {
                severity: SecuritySeverity::Medium,
                category: SecurityCategory::Validation,
                description: "Maximum string length is very high".to_string(),
                location: Some("ValidationConfig.max_string_length".to_string()),
                fix_suggestion: Some("Consider reducing max_string_length".to_string()),
            });
        }

        // Check secure defaults
        if !self.config.secure_defaults.disable_eval {
            issues.push(SecurityIssue {
                severity: SecuritySeverity::Critical,
                category: SecurityCategory::Content,
                description: "Eval is not disabled".to_string(),
                location: Some("SecureDefaults.disable_eval".to_string()),
                fix_suggestion: Some("Enable disable_eval".to_string()),
            });
        }

        if !self.config.secure_defaults.https_only {
            issues.push(SecurityIssue {
                severity: SecuritySeverity::High,
                category: SecurityCategory::Network,
                description: "HTTPS only is not enabled".to_string(),
                location: Some("SecureDefaults.https_only".to_string()),
                fix_suggestion: Some("Enable https_only".to_string()),
            });
        }

        // Generate recommendations
        if issues.is_empty() {
            recommendations.push("Security configuration looks good!".to_string());
        } else {
            recommendations.push("Review and fix the security issues above".to_string());
        }

        // Calculate security score
        let critical_issues = issues
            .iter()
            .filter(|i| matches!(i.severity, SecuritySeverity::Critical))
            .count();
        let high_issues = issues
            .iter()
            .filter(|i| matches!(i.severity, SecuritySeverity::High))
            .count();
        let medium_issues = issues
            .iter()
            .filter(|i| matches!(i.severity, SecuritySeverity::Medium))
            .count();
        let low_issues = issues
            .iter()
            .filter(|i| matches!(i.severity, SecuritySeverity::Low))
            .count();

        let security_score =
            100 - (critical_issues * 25 + high_issues * 15 + medium_issues * 10 + low_issues * 5);
        let security_score = security_score.max(0) as u8;

        Ok(SecurityAudit {
            security_score,
            issues,
            recommendations,
            compliance: ComplianceStatus {
                owasp: security_score >= 80,
                csp: !self
                    .config
                    .csp
                    .script_src
                    .contains(&"'unsafe-eval'".to_string()),
                https: self.config.secure_defaults.https_only,
                input_validation: self.config.validation.sanitize_html,
            },
        })
    }
}

impl ValidationPatterns {
    fn new() -> Result<Self, JsValue> {
        Ok(Self {
            url_pattern: Regex::new(r"^https?://[^\s/$.?#].[^\s]*$")
                .map_err(|e| JsValue::from_str(&format!("URL regex error: {}", e)))?,
            email_pattern: Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$")
                .map_err(|e| JsValue::from_str(&format!("Email regex error: {}", e)))?,
            html_tag_pattern: Regex::new(r"<[^>]+>")
                .map_err(|e| JsValue::from_str(&format!("HTML tag regex error: {}", e)))?,
            html_attribute_pattern: Regex::new(r#"\s+[a-zA-Z-]+="[^"]*""#)
                .map_err(|e| JsValue::from_str(&format!("HTML attribute regex error: {}", e)))?,
            script_pattern: Regex::new(r"(?i)<script[^>]*>.*?</script>")
                .map_err(|e| JsValue::from_str(&format!("Script regex error: {}", e)))?,
            style_pattern: Regex::new(r"(?i)<style[^>]*>.*?</style>")
                .map_err(|e| JsValue::from_str(&format!("Style regex error: {}", e)))?,
        })
    }
}

/// Validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    /// Whether the input is valid
    pub is_valid: bool,
    /// Validation issues found
    pub issues: Vec<ValidationIssue>,
}

/// Validation issue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationIssue {
    /// Field that failed validation
    pub field: String,
    /// Issue message
    pub message: String,
    /// Issue severity
    pub severity: ValidationSeverity,
}

/// Validation severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidationSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Security utilities
pub struct SecurityUtils;

impl SecurityUtils {
    /// Get security recommendations for WASM
    pub fn get_wasm_security_recommendations() -> Vec<String> {
        vec![
            "Use Content Security Policy (CSP) headers".to_string(),
            "Validate all user inputs on the client side".to_string(),
            "Sanitize HTML content before rendering".to_string(),
            "Use HTTPS for all communications".to_string(),
            "Disable eval and unsafe inline scripts".to_string(),
            "Implement proper error handling without exposing sensitive information".to_string(),
            "Use secure storage for sensitive data".to_string(),
            "Regularly audit dependencies for vulnerabilities".to_string(),
            "Implement rate limiting for API calls".to_string(),
            "Use secure random number generation".to_string(),
        ]
    }

    /// Check if current environment is secure
    pub fn is_secure_environment() -> bool {
        if let Some(window) = web_sys::window() {
            // Check if we're on HTTPS
            let location = window.location();
            if let Ok(protocol) = location.protocol() {
                return protocol == "https:";
            }
        }
        false
    }

    /// Get security headers for WASM applications
    pub fn get_security_headers() -> HashMap<String, String> {
        let mut headers = HashMap::new();

        headers.insert("X-Content-Type-Options".to_string(), "nosniff".to_string());
        headers.insert("X-Frame-Options".to_string(), "DENY".to_string());
        headers.insert("X-XSS-Protection".to_string(), "1; mode=block".to_string());
        headers.insert(
            "Referrer-Policy".to_string(),
            "strict-origin-when-cross-origin".to_string(),
        );
        headers.insert(
            "Permissions-Policy".to_string(),
            "geolocation=(), microphone=(), camera=()".to_string(),
        );

        headers
    }
}
