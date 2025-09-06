// use leptos::prelude::*;

/// A component that injects a hashed stylesheet link into the document head.
///
/// This component is designed to work with cargo-leptos file hashing feature.
/// It reads the hash file and constructs the correct URL for the hashed CSS file.
///
/// # Example
///
/// ```rust
/// use leptos_next_metadata::prelude::*;
/// use leptos::prelude::LeptosOptions;
///
/// #[component]
/// fn App() -> impl IntoView {
///     let options = LeptosOptions::builder()
///         .output_name("my-app")
///         .build();
///
///     view! {
///         <main>
///             <HashedStylesheet options=options />
///         </main>
///     }
/// }
/// ```
// Re-export the HashedStylesheet component from leptos_meta
pub use leptos_meta::HashedStylesheet;

mod tests {
    // use super::*;
    // use leptos::prelude::LeptosOptions;

    #[test]
    fn test_hashed_stylesheet_component_exists() {
        // Test that we can create a HashedStylesheet component
        use leptos::prelude::LeptosOptions;

        let options = LeptosOptions::builder()
            .output_name("test-app")
            .build();
        let _stylesheet = view! {
            <HashedStylesheet options=options />
        };

        // Test passes if component compiles
    }

    #[test]
    fn test_hashed_stylesheet_with_id() {
        // Test HashedStylesheet component with id
        use leptos::prelude::LeptosOptions;

        let options = LeptosOptions::builder()
            .output_name("test-app")
            .build();
        let _stylesheet = view! {
            <HashedStylesheet options=options id="main-stylesheet" />
        };

        // Test passes if component compiles with id
    }

    #[test]
    fn test_hashed_stylesheet_with_root() {
        // Test HashedStylesheet component with root URL
        let options = LeptosOptions::builder()
            .output_name("test-app")
            .build();
        let _stylesheet = view! {
            <HashedStylesheet options=options root="/assets" />
        };

        // Test passes if component compiles with root
    }

    #[test]
    fn test_hashed_stylesheet_with_all_props() {
        // Test HashedStylesheet component with all props
        let options = LeptosOptions::builder()
            .output_name("test-app")
            .build();
        let _stylesheet = view! {
            <HashedStylesheet
                options=options
                id="main-stylesheet"
                root="/assets"
            />
        };

        // Test passes if component compiles with all props
    }

    #[test]
    fn test_hashed_stylesheet_with_custom_options() {
        // Test HashedStylesheet component with custom options
        let options = LeptosOptions::builder()
            .output_name("my-app")
            .hash_files(true)
            .build();

        let _stylesheet = view! {
            <HashedStylesheet options=options />
        };

        // Test passes if component compiles with custom options
    }

    #[test]
    fn test_hashed_stylesheet_without_hashing() {
        // Test HashedStylesheet component without file hashing
        let options = LeptosOptions::builder()
            .output_name("test-app")
            .hash_files(false)
            .build();

        let _stylesheet = view! {
            <HashedStylesheet options=options />
        };

        // Test passes if component compiles without hashing
    }
}
