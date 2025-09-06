use leptos::*;
use leptos_next_metadata::prelude::*;
use pretty_assertions::assert_eq;
use tokio_test;

#[test]
fn test_ssr_metadata_basic_rendering() {
    let runtime = create_runtime();

    let output = runtime.dispose_with(|| {
        provide_metadata_context();

        render_to_string(move || {
            view! {
                <MetadataProvider>
                    <BasicTestComponent />
                </MetadataProvider>
            }
        })
    });

    assert!(output.contains(r#"<title>Basic Test Title</title>"#));
    assert!(output.contains(r#"<meta name="description" content="Basic test description">"#));
    assert!(output.contains(r#"<meta name="keywords" content="test,rust,leptos">"#));
}

#[component]
fn BasicTestComponent() -> impl IntoView {
    metadata! {
        title: "Basic Test Title",
        description: "Basic test description",
        keywords: ["test", "rust", "leptos"],
    }

    view! {
        <div>"Basic test content"</div>
    }
}

#[test]
fn test_ssr_metadata_with_template_title() {
    let runtime = create_runtime();

    let output = runtime.dispose_with(|| {
        provide_metadata_context();

        render_to_string(move || {
            view! {
                <MetadataProvider>
                    <TemplateTestComponent />
                </MetadataProvider>
            }
        })
    });

    assert!(output.contains(r#"<title>Page Title | My Site</title>"#));
}

#[component]
fn TemplateTestComponent() -> impl IntoView {
    metadata! {
        title: {
            template: "%s | My Site",
            default: "My Site",
        }
    }

    // Set the page-specific title
    use_title("Page Title".to_string());

    view! {
        <div>"Template test content"</div>
    }
}

#[test]
fn test_ssr_metadata_nested_components() {
    let runtime = create_runtime();

    let output = runtime.dispose_with(|| {
        provide_metadata_context();

        render_to_string(move || {
            view! {
                <MetadataProvider>
                    <ParentComponent />
                </MetadataProvider>
            }
        })
    });

    // Child should override parent
    assert!(output.contains(r#"<title>Child Title</title>"#));
    assert!(output.contains(r#"<meta name="description" content="Parent description">"#)); // Parent not overridden
    assert!(output.contains(r#"<meta property="og:title" content="Child OG Title">"#));
}

#[component]
fn ParentComponent() -> impl IntoView {
    metadata! {
        title: "Parent Title",
        description: "Parent description",
        openGraph: {
            title: "Parent OG Title",
        }
    }

    view! {
        <div>
            "Parent content"
            <ChildComponent />
        </div>
    }
}

#[component]
fn ChildComponent() -> impl IntoView {
    metadata! {
        title: "Child Title",
        openGraph: {
            title: "Child OG Title",
        }
    }

    view! { <div>"Child content"</div> }
}

#[tokio::test]
async fn test_ssr_async_metadata_generation() {
    use leptos::ssr::render_to_stream;

    let runtime = create_runtime();

    let stream = runtime.dispose_with(|| {
        provide_metadata_context();

        render_to_stream(move || {
            view! {
                <MetadataProvider>
                    <AsyncTestComponent />
                </MetadataProvider>
            }
        })
    });

    // Collect the stream
    let output = stream.collect::<String>().await;

    assert!(output.contains(r#"<title>Async Loaded Title</title>"#));
    assert!(output.contains(r#"<meta name="description" content="Loaded from async source">"#));
}

#[component]
fn AsyncTestComponent() -> impl IntoView {
    let (metadata, set_metadata) = create_signal(None::<Metadata>);

    // Simulate async loading
    create_effect(move |_| {
        spawn_local(async move {
            // Simulate API call delay
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;

            let loaded_metadata = Metadata {
                title: Some(Title::Static("Async Loaded Title".into())),
                description: Some("Loaded from async source".into()),
                ..Default::default()
            };

            set_metadata(Some(loaded_metadata));
        });
    });

    // Use the loaded metadata
    create_effect(move |_| {
        if let Some(meta) = metadata() {
            use_metadata(meta);
        }
    });

    view! { <div>"Async content"</div> }
}

#[test]
fn test_ssr_open_graph_rendering() {
    let runtime = create_runtime();

    let output = runtime.dispose_with(|| {
        provide_metadata_context();

        render_to_string(move || {
            view! {
                <MetadataProvider>
                    <OpenGraphTestComponent />
                </MetadataProvider>
            }
        })
    });

    assert!(output.contains(r#"<meta property="og:title" content="OG Test Title">"#));
    assert!(output.contains(r#"<meta property="og:description" content="OG test description">"#));
    assert!(output.contains(r#"<meta property="og:type" content="article">"#));
    assert!(output.contains(r#"<meta property="og:image" content="https://example.com/og-image.jpg">"#));
    assert!(output.contains(r#"<meta property="og:image:width" content="1200">"#));
    assert!(output.contains(r#"<meta property="og:image:height" content="630">"#));
    assert!(output.contains(r#"<meta property="og:url" content="https://example.com/test">"#));
}

#[component]
fn OpenGraphTestComponent() -> impl IntoView {
    metadata! {
        openGraph: {
            title: "OG Test Title",
            description: "OG test description",
            type: "article",
            url: "https://example.com/test",
            images: [{
                url: "https://example.com/og-image.jpg",
                width: 1200,
                height: 630,
                alt: "Test OG Image",
            }],
        }
    }

    view! { <div>"OpenGraph test content"</div> }
}

#[test]
fn test_ssr_twitter_card_rendering() {
    let runtime = create_runtime();

    let output = runtime.dispose_with(|| {
        provide_metadata_context();

        render_to_string(move || {
            view! {
                <MetadataProvider>
                    <TwitterCardTestComponent />
                </MetadataProvider>
            }
        })
    });

    assert!(output.contains(r#"<meta name="twitter:card" content="summary_large_image">"#));
    assert!(output.contains(r#"<meta name="twitter:site" content="@example">"#));
    assert!(output.contains(r#"<meta name="twitter:creator" content="@author">"#));
    assert!(output.contains(r#"<meta name="twitter:title" content="Twitter Test Title">"#));
    assert!(output.contains(r#"<meta name="twitter:description" content="Twitter test description">"#));
    assert!(output.contains(r#"<meta name="twitter:image" content="https://example.com/twitter-image.jpg">"#));
}

#[component]
fn TwitterCardTestComponent() -> impl IntoView {
    metadata! {
        twitter: {
            card: "summary_large_image",
            site: "@example",
            creator: "@author",
            title: "Twitter Test Title",
            description: "Twitter test description",
            image: "https://example.com/twitter-image.jpg",
        }
    }

    view! { <div>"Twitter card test content"</div> }
}

#[test]
fn test_ssr_robots_and_viewport() {
    let runtime = create_runtime();

    let output = runtime.dispose_with(|| {
        provide_metadata_context();

        render_to_string(move || {
            view! {
                <MetadataProvider>
                    <RobotsViewportTestComponent />
                </MetadataProvider>
            }
        })
    });

    assert!(output.contains(r#"<meta name="robots" content="index,follow,noarchive">"#));
    assert!(output.contains(r#"<meta name="viewport" content="width=device-width,initial-scale=1.0,maximum-scale=2.0">"#));
}

#[component]
fn RobotsViewportTestComponent() -> impl IntoView {
    metadata! {
        robots: {
            index: true,
            follow: true,
            noarchive: true,
        },
        viewport: {
            width: "device-width",
            initialScale: 1.0,
            maximumScale: 2.0,
        }
    }

    view! { <div>"Robots and viewport test content"</div> }
}

#[test]
fn test_ssr_icons_rendering() {
    let runtime = create_runtime();

    let output = runtime.dispose_with(|| {
        provide_metadata_context();

        render_to_string(move || {
            view! {
                <MetadataProvider>
                    <IconsTestComponent />
                </MetadataProvider>
            }
        })
    });

    assert!(output.contains(r#"<link rel="icon" href="/favicon.ico" type="image/x-icon">"#));
    assert!(output.contains(r#"<link rel="icon" href="/favicon-32x32.png" sizes="32x32" type="image/png">"#));
    assert!(output.contains(r#"<link rel="apple-touch-icon" href="/apple-touch-icon.png" sizes="180x180">"#));
}

#[component]
fn IconsTestComponent() -> impl IntoView {
    metadata! {
        icons: {
            icon: [
                { url: "/favicon.ico", type: "image/x-icon" },
                { url: "/favicon-32x32.png", sizes: "32x32", type: "image/png" }
            ],
            apple: [
                { url: "/apple-touch-icon.png", sizes: "180x180" }
            ]
        }
    }

    view! { <div>"Icons test content"</div> }
}

#[test]
fn test_ssr_canonical_and_alternates() {
    let runtime = create_runtime();

    let output = runtime.dispose_with(|| {
        provide_metadata_context();

        render_to_string(move || {
            view! {
                <MetadataProvider>
                    <AlternatesTestComponent />
                </MetadataProvider>
            }
        })
    });

    assert!(output.contains(r#"<link rel="canonical" href="https://example.com/test">"#));
    assert!(output.contains(r#"<link rel="alternate" hreflang="en" href="https://example.com/en/test">"#));
    assert!(output.contains(r#"<link rel="alternate" hreflang="es" href="https://example.com/es/test">"#));
    assert!(output.contains(r#"<link rel="alternate" type="application/rss+xml" href="/rss.xml" title="RSS Feed">"#));
}

#[component]
fn AlternatesTestComponent() -> impl IntoView {
    metadata! {
        alternates: {
            canonical: "https://example.com/test",
            languages: {
                "en": "https://example.com/en/test",
                "es": "https://example.com/es/test"
            },
            types: [{
                url: "/rss.xml",
                type: "application/rss+xml",
                title: "RSS Feed"
            }]
        }
    }

    view! { <div>"Alternates test content"</div> }
}

#[test]
fn test_ssr_json_ld_rendering() {
    let runtime = create_runtime();

    let output = runtime.dispose_with(|| {
        provide_metadata_context();

        render_to_string(move || {
            view! {
                <MetadataProvider>
                    <JsonLdTestComponent />
                </MetadataProvider>
            }
        })
    });

    assert!(output.contains(r#"<script type="application/ld+json">"#));
    assert!(output.contains(r#""@type": "Article""#));
    assert!(output.contains(r#""headline": "Test Article""#));
    assert!(output.contains(r#""author": {"@type": "Person""#));
}

#[component]
fn JsonLdTestComponent() -> impl IntoView {
    use leptos_next_metadata::json_ld::*;

    let article = Article::builder()
        .headline("Test Article")
        .author(Person::builder().name("John Doe").build())
        .date_published("2024-01-15T10:00:00Z")
        .build();

    use_json_ld(article);

    view! { <div>"JSON-LD test content"</div> }
}

#[test]
fn test_ssr_metadata_inheritance_order() {
    let runtime = create_runtime();

    let output = runtime.dispose_with(|| {
        provide_metadata_context();

        render_to_string(move || {
            view! {
                <MetadataProvider>
                    <InheritanceTestGrandparent />
                </MetadataProvider>
            }
        })
    });

    // Child should override parent, parent should override grandparent
    assert!(output.contains(r#"<title>Child Title</title>"#)); // From child
    assert!(output.contains(r#"<meta name="description" content="Parent Description">"#)); // From parent
    assert!(output.contains(r#"<meta name="keywords" content="grandparent,keywords">"#)); // From grandparent
}

#[component]
fn InheritanceTestGrandparent() -> impl IntoView {
    metadata! {
        title: "Grandparent Title",
        description: "Grandparent Description",
        keywords: ["grandparent", "keywords"],
    }

    view! {
        <div>
            "Grandparent"
            <InheritanceTestParent />
        </div>
    }
}

#[component]
fn InheritanceTestParent() -> impl IntoView {
    metadata! {
        title: "Parent Title",
        description: "Parent Description",
    }

    view! {
        <div>
            "Parent"
            <InheritanceTestChild />
        </div>
    }
}

#[component]
fn InheritanceTestChild() -> impl IntoView {
    metadata! {
        title: "Child Title",
    }

    view! { <div>"Child"</div> }
}

#[test]
fn test_ssr_conditional_metadata() {
    let runtime = create_runtime();

    let output = runtime.dispose_with(|| {
        provide_metadata_context();

        render_to_string(move || {
            view! {
                <MetadataProvider>
                    <ConditionalTestComponent show_og=true show_twitter=false />
                </MetadataProvider>
            }
        })
    });

    // Should have OpenGraph but not Twitter
    assert!(output.contains(r#"<meta property="og:title" content="Conditional Title">"#));
    assert!(!output.contains(r#"<meta name="twitter:card""#));
}

#[component]
fn ConditionalTestComponent(
    #[prop(default = false)] show_og: bool,
    #[prop(default = false)] show_twitter: bool,
) -> impl IntoView {
    let mut meta = Metadata {
        title: Some(Title::Static("Conditional Title".into())),
        ..Default::default()
    };

    if show_og {
        meta.open_graph = Some(OpenGraph {
            title: Some("Conditional Title".into()),
            ..Default::default()
        });
    }

    if show_twitter {
        meta.twitter = Some(Twitter {
            card: Some("summary".into()),
            ..Default::default()
        });
    }

    use_metadata(meta);

    view! { <div>"Conditional content"</div> }
}

#[test]
fn test_ssr_error_handling() {
    let runtime = create_runtime();

    // Should not panic with invalid metadata
    let output = runtime.dispose_with(|| {
        provide_metadata_context();

        render_to_string(move || {
            view! {
                <MetadataProvider>
                    <ErrorTestComponent />
                </MetadataProvider>
            }
        })
    });

    // Should still render content even if metadata has issues
    assert!(output.contains("Error test content"));
}

#[component]
fn ErrorTestComponent() -> impl IntoView {
    // Try to use metadata with potentially problematic values
    metadata! {
        title: "", // Empty title
        description: "x".repeat(500), // Very long description
        openGraph: {
            images: [{ url: "" }] // Empty URL
        }
    }

    view! { <div>"Error test content"</div> }
}
