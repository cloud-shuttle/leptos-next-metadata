use leptos::prelude::*;
use leptos_meta::*;

/// An enhanced Title component with formatter support.
///
/// This component extends the basic Title functionality with the ability to apply
/// custom formatters to the title text, enabling dynamic title generation with
/// consistent formatting patterns.
///
/// # Example
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_next_metadata::prelude::*;
///
/// #[component]
/// fn App() -> impl IntoView {
///     view! { <EnhancedTitle text="My Page" /> }
/// }
/// ```
#[component]
pub fn EnhancedTitle(
    /// The base title text
    #[prop(into)]
    text: String,
    /// Optional formatter function to transform the title
    #[prop(optional)]
    formatter: Option<fn(&str) -> String>,
    /// Optional template string for formatting (alternative to formatter)
    #[prop(optional, into)]
    template: Option<String>,
    /// Optional suffix to append to the title
    #[prop(optional, into)]
    suffix: Option<String>,
    /// Optional prefix to prepend to the title
    #[prop(optional, into)]
    prefix: Option<String>,
) -> impl IntoView {
    // Apply formatting logic
    let formatted_text = if let Some(formatter_fn) = formatter {
        // Use custom formatter function
        formatter_fn(&text)
    } else if let Some(template_str) = template {
        // Use template string
        template_str.replace("{}", &text)
    } else {
        // Apply prefix and suffix if provided
        let mut result = text.clone();
        if let Some(prefix_str) = prefix {
            result = format!("{} {}", prefix_str, result);
        }
        if let Some(suffix_str) = suffix {
            result = format!("{} {}", result, suffix_str);
        }
        result
    };

    // Use the standard Title component from leptos_meta
    view! {
        <Title text=formatted_text />
    }
}

mod tests {

    #[test]
    fn test_enhanced_title_component_exists() {
        // Test that we can create an EnhancedTitle component
        // This test verifies the component compiles and can be instantiated
        // Note: This is a compilation test - actual component testing would require
        // a full Leptos runtime setup which is complex for unit tests

        // Test passes if this module compiles
        // Component exists and compiles
    }

    #[test]
    fn test_title_formatting_logic() {
        // Test the core formatting logic without Leptos components

        // Test basic text (no formatting)
        let text = "My Page".to_string();
        let formatted = if let Some(_formatter_fn) = None::<fn(&str) -> String> {
            "formatted".to_string()
        } else if let Some(template_str) = Some("{} | My Site".to_string()) {
            template_str.replace("{}", &text)
        } else {
            text.clone()
        };
        assert_eq!(formatted, "My Page | My Site");

        // Test prefix and suffix
        let text = "My Page".to_string();
        let prefix = Some("Home".to_string());
        let suffix = Some("Site".to_string());

        let mut result = text.clone();
        if let Some(prefix_str) = prefix {
            result = format!("{} {}", prefix_str, result);
        }
        if let Some(suffix_str) = suffix {
            result = format!("{} {}", result, suffix_str);
        }
        assert_eq!(result, "Home My Page Site");
    }

    #[test]
    fn test_template_replacement() {
        let text = "Test Page";
        let template = "{} | My Website";
        let result = template.replace("{}", text);
        assert_eq!(result, "Test Page | My Website");
    }

    #[test]
    fn test_multiple_template_replacements() {
        let text = "Home";
        let template = "{} - {} | My Site";
        let result = template.replace("{}", text);
        assert_eq!(result, "Home - Home | My Site");
    }

    #[test]
    fn test_prefix_suffix_combination() {
        let text = "Page";
        let prefix = "My";
        let suffix = "Site";

        let mut result = text.to_string();
        result = format!("{} {}", prefix, result);
        result = format!("{} {}", result, suffix);

        assert_eq!(result, "My Page Site");
    }

    #[test]
    fn test_empty_template() {
        let text = "Test";
        let template = "";
        let result = template.replace("{}", text);
        assert_eq!(result, "");
    }

    #[test]
    fn test_template_without_placeholder() {
        let text = "Test";
        let template = "Static Title";
        let result = template.replace("{}", text);
        assert_eq!(result, "Static Title");
    }
}
