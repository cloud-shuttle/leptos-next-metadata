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
/// use leptos_next_metadata::prelude::*;
///
/// #[component]
/// fn App() -> impl IntoView {
///     let (prefers_dark, set_prefers_dark) = create_signal(false);
///     let body_class = move || {
///         if prefers_dark.get() {
///             "dark".to_string()
///         } else {
///             "light".to_string()
///         }
///     };
///
///     view! {
///         <main>
///             <Body {..} class=body_class id="body"/>
///         </main>
///     }
/// }
/// ```
#[component]
pub fn Body() -> impl IntoView {
    // Use the Body component from leptos_meta
    leptos_meta::Body()
}

mod tests {
    // use super::*;

    #[test]
    fn test_body_component_exists() {
        // Test that we can create a Body component
        let _body = view! {
            <Body />
        };

        // Test passes if component compiles
    }

    #[test]
    fn test_body_component_with_class() {
        // Test Body component with class attribute using spread syntax
        let _body = view! {
            <Body {..} class="dark-theme" />
        };

        // Test passes if component compiles with class
    }

    #[test]
    fn test_body_component_with_lang() {
        // Test Body component with lang attribute using spread syntax
        let _body = view! {
            <Body {..} lang="en" />
        };

        // Test passes if component compiles with lang
    }

    #[test]
    fn test_body_component_with_dir() {
        // Test Body component with dir attribute using spread syntax
        let _body = view! {
            <Body {..} dir="ltr" />
        };

        // Test passes if component compiles with dir
    }

    #[test]
    fn test_body_component_with_all_attributes() {
        // Test Body component with all attributes using spread syntax
        let _body = view! {
            <Body {..} class="dark-theme" lang="en" dir="ltr" />
        };

        // Test passes if component compiles with all attributes
    }

    #[test]
    fn test_body_component_with_id() {
        // Test Body component with id attribute using spread syntax
        let _body = view! {
            <Body {..} id="main-body" />
        };

        // Test passes if component compiles with id
    }
}
