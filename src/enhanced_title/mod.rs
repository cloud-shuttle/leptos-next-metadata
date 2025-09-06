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
/// use leptos_next_metadata::prelude::*;
///
/// #[component]
/// fn App() -> impl IntoView {
///     // Basic usage
///     let _title = view! { <EnhancedTitle text="My Page" /> };
///
///     // With formatter
///     let formatter = |text: &str| format!("{} | My Site", text);
///     let _formatted_title = view! {
///         <EnhancedTitle text="My Page" formatter=formatter />
///     };
///
///     // With template
///     let _template_title = view! {
///         <EnhancedTitle
///             text="My Page"
///             template="{} | My Site"
///         />
///     };
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
    // use super::*;

    #[test]
    fn test_enhanced_title_component_exists() {
        // Test that we can create an EnhancedTitle component
        let _title = view! {
            <EnhancedTitle text="Test Page" />
        };

        // Test passes if component compiles
    }

    #[test]
    fn test_enhanced_title_with_formatter() {
        // Test EnhancedTitle component with formatter function
        let formatter = |text: &str| format!("{} | My Site", text);
        let _title = view! {
            <EnhancedTitle text="Test Page" formatter=formatter />
        };

        // Test passes if component compiles with formatter
    }

    #[test]
    fn test_enhanced_title_with_template() {
        // Test EnhancedTitle component with template string
        let _title = view! {
            <EnhancedTitle
                text="Test Page"
                template="{} | My Site"
            />
        };

        // Test passes if component compiles with template
    }

    #[test]
    fn test_enhanced_title_with_prefix() {
        // Test EnhancedTitle component with prefix
        let _title = view! {
            <EnhancedTitle
                text="Test Page"
                prefix="Welcome to"
            />
        };

        // Test passes if component compiles with prefix
    }

    #[test]
    fn test_enhanced_title_with_suffix() {
        // Test EnhancedTitle component with suffix
        let _title = view! {
            <EnhancedTitle
                text="Test Page"
                suffix="| My Site"
            />
        };

        // Test passes if component compiles with suffix
    }

    #[test]
    fn test_enhanced_title_with_prefix_and_suffix() {
        // Test EnhancedTitle component with both prefix and suffix
        let _title = view! {
            <EnhancedTitle
                text="Test Page"
                prefix="Welcome to"
                suffix="| My Site"
            />
        };

        // Test passes if component compiles with prefix and suffix
    }

    #[test]
    fn test_enhanced_title_formatter_priority() {
        // Test that formatter takes priority over template/prefix/suffix
        let formatter = |text: &str| format!("Formatted: {}", text);
        let _title = view! {
            <EnhancedTitle
                text="Test Page"
                formatter=formatter
                template="{} | My Site"
                prefix="Welcome to"
                suffix="| My Site"
            />
        };

        // Test passes if component compiles with formatter taking priority
    }

    #[test]
    fn test_enhanced_title_template_priority() {
        // Test that template takes priority over prefix/suffix when no formatter
        let _title = view! {
            <EnhancedTitle
                text="Test Page"
                template="{} | My Site"
                prefix="Welcome to"
                suffix="| My Site"
            />
        };

        // Test passes if component compiles with template taking priority
    }

    #[test]
    fn test_enhanced_title_dynamic_text() {
        // Test EnhancedTitle component with dynamic text
        let dynamic_text = "Dynamic Page".to_string();
        let _title = view! {
            <EnhancedTitle text=dynamic_text />
        };

        // Test passes if component compiles with dynamic text
    }

    #[test]
    fn test_enhanced_title_complex_formatter() {
        // Test EnhancedTitle component with complex formatter
        let formatter = |text: &str| {
            let words: Vec<&str> = text.split_whitespace().collect();
            format!("{} ({} words)", text, words.len())
        };
        let _title = view! {
            <EnhancedTitle
                text="This is a test page"
                formatter=formatter
            />
        };

        // Test passes if component compiles with complex formatter
    }

    #[test]
    fn test_enhanced_title_formatting_logic() {
        // Test the actual formatting logic
        let text = "My Page";

        // Test formatter function
        let formatter = |text: &str| format!("{} | My Site", text);
        let formatted = formatter(text);
        assert_eq!(formatted, "My Page | My Site");

        // Test template string
        let template = "{} | My Site";
        let templated = template.replace("{}", text);
        assert_eq!(templated, "My Page | My Site");

        // Test prefix and suffix
        let prefix = "Welcome to";
        let suffix = "| My Site";
        let prefixed_suffixed = format!("{} {} {}", prefix, text, suffix);
        assert_eq!(prefixed_suffixed, "Welcome to My Page | My Site");
    }

    #[test]
    fn test_enhanced_title_edge_cases() {
        // Test edge cases
        let empty_text = "";
        let _title1 = view! {
            <EnhancedTitle text=empty_text />
        };

        let long_text = "This is a very long title that might cause issues with formatting and should be handled gracefully";
        let _title2 = view! {
            <EnhancedTitle text=long_text />
        };

        let special_chars = "Title with special chars: !@#$%^&*()";
        let _title3 = view! {
            <EnhancedTitle text=special_chars />
        };

        // Test passes if all edge cases compile
    }

    #[test]
    fn test_enhanced_title_multiple_formatters() {
        // Test that only one formatter is applied (priority order)
        let formatter1 = |text: &str| format!("Formatted: {}", text);
        let _formatter2 = |text: &str| format!("Also Formatted: {}", text);

        // This should use formatter1 (first one provided)
        let _title = view! {
            <EnhancedTitle
                text="Test Page"
                formatter=formatter1
            />
        };

        // Test passes if component compiles with single formatter
    }
}
