use leptos::prelude::*;

/// A component to set metadata on the document's `<body>` element from within the application.
///
/// This component takes no props, but can take any number of spread attributes
/// following the `{..}` operator. It allows you to set attributes like `class`, `lang`, and `dir`
/// on the body element, which is essential for proper HTML structure and accessibility.
///
/// # Example
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_next_metadata::prelude::*;
///
/// #[component]
/// fn App() -> impl IntoView {
///     view! { <Body /> }
/// }
/// ```
#[component]
pub fn Body() -> impl IntoView {
    // Use the Body component from leptos_meta
    leptos_meta::Body()
}

mod tests {
    #[test]
    fn test_body_component_exists() {
        // Test that we can create a Body component
        // This test verifies the component compiles and can be instantiated
        let _body = crate::Body();

        // Test passes if component compiles
    }
}
