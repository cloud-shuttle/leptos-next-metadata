use leptos::prelude::*;

/// A component to set metadata on the document's `<html>` element from within the application.
///
/// This component takes no props, but can take any number of spread attributes
/// following the `{..}` operator. It allows you to set attributes like `lang`, `dir`, and `data-*`
/// attributes on the html element, which is essential for proper HTML structure and accessibility.
///
/// # Example
///
/// ```rust
/// use leptos_next_metadata::prelude::*;
///
/// #[component]
/// fn App() -> impl IntoView {
///     view! {
///         <main>
///             <Html
///                 {..}
///                 lang="en"
///                 dir="ltr"
///                 data-theme="light"
///             />
///         </main>
///     }
/// }
/// ```
#[component]
pub fn Html() -> impl IntoView {
    // Use the Html component from leptos_meta
    leptos_meta::Html()
}

mod tests {
    // use super::*;

    #[test]
    fn test_html_component_exists() {
        // Test that we can create an Html component
        let _html = view! {
            <Html />
        };

        // Test passes if component compiles
    }

    #[test]
    fn test_html_component_with_lang() {
        // Test Html component with lang attribute using spread syntax
        let _html = view! {
            <Html {..} lang="en" />
        };

        // Test passes if component compiles with lang
    }

    #[test]
    fn test_html_component_with_dir() {
        // Test Html component with dir attribute using spread syntax
        let _html = view! {
            <Html {..} dir="ltr" />
        };

        // Test passes if component compiles with dir
    }

    #[test]
    fn test_html_component_with_data_attributes() {
        // Test Html component with data attributes using spread syntax
        let _html = view! {
            <Html {..} data-theme="dark" data-color-scheme="dark" />
        };

        // Test passes if component compiles with data attributes
    }

    #[test]
    fn test_html_component_with_all_attributes() {
        // Test Html component with all attributes using spread syntax
        let _html = view! {
            <Html {..} lang="en" dir="ltr" data-theme="light" />
        };

        // Test passes if component compiles with all attributes
    }

    #[test]
    fn test_html_component_with_rtl() {
        // Test Html component with RTL (right-to-left) attributes
        let _html = view! {
            <Html {..} lang="he" dir="rtl" />
        };

        // Test passes if component compiles with RTL attributes
    }
}
