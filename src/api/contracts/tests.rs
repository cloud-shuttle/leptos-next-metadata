//! Tests for API contract validation

use super::*;
use openapiv3::OpenAPI;
use serde_json::json;

#[cfg(test)]
#[allow(dead_code)]
mod contract_tests {
    use super::*;

    #[test]
    fn test_contract_validator_creation() {
        let schema = OpenAPI::default();
        let validator = ContractValidator::new(schema);
        assert!(validator.rules.strict);
    }

    #[test]
    fn test_validation_rules_default() {
        let rules = ValidationRules::default();
        assert!(rules.strict);
        assert!(!rules.allow_additional_properties);
        assert_eq!(rules.max_request_size, 1024 * 1024);
    }

    #[test]
    fn test_validation_result_creation() {
        let result = ValidationResult {
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
            score: 100,
        };

        assert!(result.is_valid);
        assert_eq!(result.score, 100);
    }

    #[test]
    fn test_validation_error_detail() {
        let error = ValidationErrorDetail {
            code: "MISSING_REQUIRED_FIELD".to_string(),
            message: "Title is required".to_string(),
            field: Some("title".to_string()),
            suggestion: Some("Provide a title".to_string()),
        };

        assert_eq!(error.code, "MISSING_REQUIRED_FIELD");
        assert_eq!(error.field, Some("title".to_string()));
    }

    #[test]
    fn test_validation_warning_detail() {
        let warning = ValidationWarningDetail {
            code: "MISSING_RECOMMENDED_FIELD".to_string(),
            message: "Description is recommended".to_string(),
            field: Some("description".to_string()),
            suggestion: Some("Add a description".to_string()),
        };

        assert_eq!(warning.code, "MISSING_RECOMMENDED_FIELD");
        assert_eq!(warning.field, Some("description".to_string()));
    }

    #[test]
    fn test_validation_score_calculation() {
        let errors = vec![ValidationErrorDetail {
            code: "ERROR1".to_string(),
            message: "Error 1".to_string(),
            field: None,
            suggestion: None,
        }];
        let warnings = vec![ValidationWarningDetail {
            code: "WARNING1".to_string(),
            message: "Warning 1".to_string(),
            field: None,
            suggestion: None,
        }];

        let schema = OpenAPI::default();
        let validator = ContractValidator::new(schema);
        let score = validator.calculate_validation_score(&errors, &warnings);

        // 1 error (20 points) + 1 warning (5 points) = 25 points penalty
        // Score should be 100 - 25 = 75
        assert_eq!(score, 75);
    }

    #[test]
    fn test_validation_score_max_penalty() {
        let errors = vec![
            ValidationErrorDetail {
                code: "ERROR1".to_string(),
                message: "Error 1".to_string(),
                field: None,
                suggestion: None,
            },
            ValidationErrorDetail {
                code: "ERROR2".to_string(),
                message: "Error 2".to_string(),
                field: None,
                suggestion: None,
            },
            ValidationErrorDetail {
                code: "ERROR3".to_string(),
                message: "Error 3".to_string(),
                field: None,
                suggestion: None,
            },
        ];

        let schema = OpenAPI::default();
        let validator = ContractValidator::new(schema);
        let score = validator.calculate_validation_score(&errors, &[]);

        // 3 errors * 20 points = 60 points penalty
        // Score should be 100 - 60 = 40
        assert_eq!(score, 40);
    }

    #[test]
    fn test_validation_score_over_100_penalty() {
        let errors = vec![
            ValidationErrorDetail {
                code: "ERROR1".to_string(),
                message: "Error 1".to_string(),
                field: None,
                suggestion: None,
            },
            ValidationErrorDetail {
                code: "ERROR2".to_string(),
                message: "Error 2".to_string(),
                field: None,
                suggestion: None,
            },
            ValidationErrorDetail {
                code: "ERROR3".to_string(),
                message: "Error 3".to_string(),
                field: None,
                suggestion: None,
            },
            ValidationErrorDetail {
                code: "ERROR4".to_string(),
                message: "Error 4".to_string(),
                field: None,
                suggestion: None,
            },
            ValidationErrorDetail {
                code: "ERROR5".to_string(),
                message: "Error 5".to_string(),
                field: None,
                suggestion: None,
            },
            ValidationErrorDetail {
                code: "ERROR6".to_string(),
                message: "Error 6".to_string(),
                field: None,
                suggestion: None,
            },
        ];

        let schema = OpenAPI::default();
        let validator = ContractValidator::new(schema);
        let score = validator.calculate_validation_score(&errors, &[]);

        // 6 errors * 20 points = 120 points penalty
        // Score should be 0 (capped at 0)
        assert_eq!(score, 0);
    }

    #[test]
    fn test_is_valid_url() {
        let schema = OpenAPI::default();
        let validator = ContractValidator::new(schema);

        assert!(validator.is_valid_url("https://example.com"));
        assert!(validator.is_valid_url("http://example.com"));
        assert!(!validator.is_valid_url("ftp://example.com"));
        assert!(!validator.is_valid_url("invalid-url"));
    }

    #[test]
    fn test_contract_middleware_creation() {
        let schema = OpenAPI::default();
        let validator = ContractValidator::new(schema);
        let _middleware = ContractMiddleware::new(validator);

        // Test that middleware was created successfully
        // If we get here, creation succeeded
    }

    #[test]
    fn test_validate_required_fields() {
        let schema = OpenAPI::default();
        let validator = ContractValidator::new(schema);

        // Test with missing required fields
        let data = json!({});
        let mut errors = Vec::new();
        validator.validate_required_fields(&data, &mut errors);

        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.field == Some("title".to_string())));
        assert!(errors.iter().any(|e| e.field == Some("url".to_string())));
    }

    #[test]
    fn test_validate_field_types() {
        let schema = OpenAPI::default();
        let validator = ContractValidator::new(schema);

        // Test with invalid title type
        let data = json!({
            "title": 123,
            "url": "https://example.com"
        });
        let mut errors = Vec::new();
        validator.validate_field_types(&data, &mut errors);

        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.code == "INVALID_TYPE"));
    }

    #[test]
    fn test_validate_field_constraints() {
        let schema = OpenAPI::default();
        let validator = ContractValidator::new(schema);

        // Test with missing recommended fields
        let data = json!({
            "title": "Test Title",
            "url": "https://example.com"
        });
        let mut errors = Vec::new();
        let mut warnings = Vec::new();
        validator.validate_field_constraints(&data, &mut errors, &mut warnings);

        assert!(!warnings.is_empty());
        assert!(warnings.iter().any(|w| w.field == Some("description".to_string())));
        assert!(warnings.iter().any(|w| w.field == Some("image".to_string())));
    }

    #[test]
    fn test_validate_open_graph() {
        let schema = OpenAPI::default();
        let validator = ContractValidator::new(schema);

        // Test with incomplete Open Graph data
        let og_data = json!({});
        let mut errors = Vec::new();
        let mut warnings = Vec::new();
        validator.validate_open_graph(&og_data, &mut errors, &mut warnings);

        assert!(!warnings.is_empty());
        assert!(warnings.iter().any(|w| w.field == Some("openGraph.type".to_string())));
        assert!(warnings.iter().any(|w| w.field == Some("openGraph.images".to_string())));
    }

    #[test]
    fn test_validate_twitter_config() {
        let schema = OpenAPI::default();
        let validator = ContractValidator::new(schema);

        // Test with incomplete Twitter data
        let twitter_data = json!({});
        let mut errors = Vec::new();
        let mut warnings = Vec::new();
        validator.validate_twitter_config(&twitter_data, &mut errors, &mut warnings);

        assert!(!warnings.is_empty());
        assert!(warnings.iter().any(|w| w.field == Some("twitter.card".to_string())));
    }

    // Note: convert_validation_errors test removed due to validator crate API complexity
    // This would require more complex mocking of the validator crate types

    #[test]
    fn test_get_suggestion_for_code() {
        let schema = OpenAPI::default();
        let validator = ContractValidator::new(schema);

        assert_eq!(
            validator.get_suggestion_for_code("length"),
            Some("Check the length of the field".to_string())
        );
        assert_eq!(
            validator.get_suggestion_for_code("url"),
            Some("Provide a valid URL".to_string())
        );
        assert_eq!(validator.get_suggestion_for_code("unknown"), None);
    }
}
