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
    #[test]
    fn test_html_component_exists() {
        // Test that we can create an Html component
        // This test verifies the component compiles and can be instantiated
        let _html = crate::Html();

        // Test passes if component compiles
    }
}
