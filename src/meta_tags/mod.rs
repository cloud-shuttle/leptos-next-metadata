use leptos::prelude::*;

/// A component that injects meta tags into the document head during server-side rendering.
///
/// This component is essential for SSR applications as it ensures that all metadata
/// added by other components (Title, Meta, Link, etc.) is properly injected into
/// the HTML head section.
///
/// # Example
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_meta::{Title, Meta, provide_meta_context};
/// use leptos_next_metadata::prelude::MetaTags;
///
/// #[component]
/// fn App() -> impl IntoView {
///     provide_meta_context();
///
///     view! {
///         <Title text="My App" />
///         <Meta name="description" content="My awesome app" />
///         <MetaTags />
///         <main>
///             <h1>"Welcome to My App"</h1>
///         </main>
///     }
/// }
/// ```
#[component]
pub fn MetaTags() -> impl IntoView {
    // Use the MetaTags component from leptos_meta
    // This will inject all the meta tags that have been added to the context
    leptos_meta::MetaTags()
}

mod tests {
    #[test]
    fn test_meta_tags_component_exists() {
        // Test that we can create a MetaTags component
        // This test verifies the component compiles and can be instantiated
        let _meta_tags = crate::MetaTags();

        // Test passes if component compiles
    }
}
