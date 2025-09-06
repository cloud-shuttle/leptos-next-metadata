use leptos_next_metadata::metadata::*;
use insta::assert_yaml_snapshot;
use pretty_assertions::assert_eq;

#[test]
fn test_shallow_merge() {
    let parent = Metadata {
        title: Some(Title::Static("Parent".into())),
        description: Some("Parent description".into()),
        keywords: vec!["parent".into()],
        open_graph: Some(OpenGraph {
            title: Some("Parent OG".into()),
            images: vec![OgImage::new("/parent.jpg")],
            ..Default::default()
        }),
        ..Default::default()
    };

    let child = Metadata {
        title: Some(Title::Static("Child".into())),
        keywords: vec!["child".into()],
        open_graph: Some(OpenGraph {
            description: Some("Child OG description".into()),
            ..Default::default()
        }),
        ..Default::default()
    };

    let merged = child.merge(parent);

    // Child's values should take precedence
    match &merged.title {
        Some(Title::Static(title)) => assert_eq!(title, "Child"),
        _ => panic!("Expected static title"),
    }

    // Child's description should be none, so parent's should be used
    assert_eq!(merged.description.as_deref(), Some("Parent description"));

    // Child's keywords should replace parent's (not extend)
    assert_eq!(merged.keywords, vec!["child"]);

    // Child's OpenGraph should completely replace parent's (shallow merge)
    let og = merged.open_graph.as_ref().unwrap();
    assert_eq!(og.title, None); // Child didn't have title, so it's None
    assert_eq!(og.description.as_deref(), Some("Child OG description"));
    assert!(og.images.is_empty()); // Child didn't have images
}

#[test]
fn test_deep_merge_prevention() {
    // Verify that merge is shallow, not deep
    let parent = Metadata {
        open_graph: Some(OpenGraph {
            title: Some("Parent".into()),
            description: Some("Parent desc".into()),
            images: vec![OgImage::new("/parent.jpg")],
            ..Default::default()
        }),
        ..Default::default()
    };

    let child = Metadata {
        open_graph: Some(OpenGraph {
            title: Some("Child".into()),
            ..Default::default()
        }),
        ..Default::default()
    };

    let merged = child.merge(parent);

    // Child's OpenGraph should completely replace parent's
    let og = merged.open_graph.unwrap();
    assert_eq!(og.title.as_deref(), Some("Child"));
    assert_eq!(og.description, None); // Should not inherit from parent
    assert!(og.images.is_empty()); // Should not inherit from parent
}

#[test]
fn test_merge_with_none_values() {
    let parent = Metadata {
        title: Some(Title::Static("Parent Title".into())),
        description: Some("Parent Description".into()),
        keywords: vec!["parent1".into(), "parent2".into()],
        ..Default::default()
    };

    let child = Metadata {
        title: None,
        description: None,
        keywords: vec![],
        ..Default::default()
    };

    let merged = child.merge(parent);

    // None/empty child values should allow parent values through
    match &merged.title {
        Some(Title::Static(title)) => assert_eq!(title, "Parent Title"),
        _ => panic!("Expected parent title"),
    }

    assert_eq!(merged.description.as_deref(), Some("Parent Description"));
    assert_eq!(merged.keywords, vec!["parent1", "parent2"]);
}

#[test]
fn test_merge_chain() {
    let grandparent = Metadata {
        title: Some(Title::Static("Grandparent".into())),
        description: Some("GP Description".into()),
        keywords: vec!["gp".into()],
        ..Default::default()
    };

    let parent = Metadata {
        title: Some(Title::Static("Parent".into())),
        keywords: vec!["parent".into()],
        ..Default::default()
    };

    let child = Metadata {
        description: Some("Child Description".into()),
        ..Default::default()
    };

    let merged = child.merge(parent.merge(grandparent));

    match &merged.title {
        Some(Title::Static(title)) => assert_eq!(title, "Parent"),
        _ => panic!("Expected parent title"),
    }

    assert_eq!(merged.description.as_deref(), Some("Child Description"));
    assert_eq!(merged.keywords, vec!["parent"]);
}

#[test]
fn test_merge_complex_nested_structures() {
    let parent = Metadata {
        open_graph: Some(OpenGraph {
            title: Some("Parent OG".into()),
            description: Some("Parent OG Desc".into()),
            images: vec![
                OgImage::new("/parent1.jpg"),
                OgImage::new("/parent2.jpg"),
            ],
            locale: Some("en_US".into()),
            site_name: Some("Parent Site".into()),
            ..Default::default()
        }),
        twitter: Some(Twitter {
            card: Some("summary".into()),
            site: Some("@parent".into()),
            creator: Some("@parent_creator".into()),
            ..Default::default()
        }),
        ..Default::default()
    };

    let child = Metadata {
        open_graph: Some(OpenGraph {
            title: Some("Child OG".into()),
            images: vec![OgImage::new("/child.jpg")],
            ..Default::default()
        }),
        twitter: Some(Twitter {
            card: Some("summary_large_image".into()),
            ..Default::default()
        }),
        ..Default::default()
    };

    let merged = child.merge(parent);

    // OpenGraph should be completely replaced
    let og = merged.open_graph.as_ref().unwrap();
    assert_eq!(og.title.as_deref(), Some("Child OG"));
    assert_eq!(og.description, None); // Not inherited from parent
    assert_eq!(og.images.len(), 1);
    assert_eq!(og.images[0].url, "/child.jpg");
    assert_eq!(og.locale, None); // Not inherited from parent
    assert_eq!(og.site_name, None); // Not inherited from parent

    // Twitter should be completely replaced
    let twitter = merged.twitter.as_ref().unwrap();
    assert_eq!(twitter.card.as_deref(), Some("summary_large_image"));
    assert_eq!(twitter.site, None); // Not inherited from parent
    assert_eq!(twitter.creator, None); // Not inherited from parent
}

#[test]
fn test_merge_icons() {
    let parent = Metadata {
        icons: Some(Icons {
            icon: vec![Icon::new("/favicon-32.png", "32x32")],
            shortcut: vec![Icon::new("/shortcut.ico", "16x16")],
            apple: vec![Icon::new("/apple-touch-icon.png", "180x180")],
            other: vec![
                Icon {
                    url: "/manifest-icon-192.png".into(),
                    sizes: Some("192x192".into()),
                    icon_type: Some("image/png".into()),
                    rel: Some("icon".into()),
                    ..Default::default()
                }
            ],
        }),
        ..Default::default()
    };

    let child = Metadata {
        icons: Some(Icons {
            icon: vec![Icon::new("/favicon-64.png", "64x64")],
            apple: vec![], // Empty but present - should replace parent
            ..Default::default()
        }),
        ..Default::default()
    };

    let merged = child.merge(parent);

    let icons = merged.icons.as_ref().unwrap();
    assert_eq!(icons.icon.len(), 1);
    assert_eq!(icons.icon[0].url, "/favicon-64.png");
    assert!(icons.shortcut.is_empty()); // Should not inherit from parent
    assert!(icons.apple.is_empty()); // Child explicitly set to empty
    assert!(icons.other.is_empty()); // Should not inherit from parent
}

#[test]
fn test_merge_alternates() {
    let mut parent_languages = std::collections::HashMap::new();
    parent_languages.insert("en".into(), "/en".into());
    parent_languages.insert("es".into(), "/es".into());

    let parent = Metadata {
        alternates: Some(Alternates {
            canonical: Some("https://example.com/page".into()),
            languages: parent_languages,
            media: vec![
                AlternateMedia {
                    url: "/mobile".into(),
                    media: "(max-width: 600px)".into(),
                }
            ],
            types: vec![
                AlternateType {
                    url: "/rss.xml".into(),
                    media_type: "application/rss+xml".into(),
                    title: Some("RSS Feed".into()),
                }
            ],
        }),
        ..Default::default()
    };

    let mut child_languages = std::collections::HashMap::new();
    child_languages.insert("fr".into(), "/fr".into());

    let child = Metadata {
        alternates: Some(Alternates {
            languages: child_languages,
            ..Default::default()
        }),
        ..Default::default()
    };

    let merged = child.merge(parent);

    let alts = merged.alternates.as_ref().unwrap();
    assert_eq!(alts.canonical, None); // Should not inherit from parent
    assert_eq!(alts.languages.len(), 1);
    assert_eq!(alts.languages.get("fr"), Some(&"/fr".to_string()));
    assert!(alts.languages.get("en").is_none()); // Should not inherit from parent
    assert!(alts.media.is_empty()); // Should not inherit from parent
    assert!(alts.types.is_empty()); // Should not inherit from parent
}

#[test]
fn test_merge_snapshot() {
    let parent = Metadata {
        title: Some(Title::Template {
            template: "%s | Parent Site".into(),
            default: "Parent Site".into(),
        }),
        description: Some("Parent site description".into()),
        keywords: vec!["parent".into(), "site".into()],
        robots: Some(Robots {
            index: Some(true),
            follow: Some(true),
            ..Default::default()
        }),
        ..Default::default()
    };

    let child = Metadata {
        title: Some(Title::Static("Child Page".into())),
        keywords: vec!["child".into()],
        robots: Some(Robots {
            index: Some(false),
            ..Default::default()
        }),
        ..Default::default()
    };

    let merged = child.merge(parent);

    assert_yaml_snapshot!(merged, @r###"
    title:
      Static: "Child Page"
    description: "Parent site description"
    keywords:
      - child
    open_graph: ~
    twitter: ~
    robots:
      index: false
      follow: ~
      noarchive: ~
      nosnippet: ~
      noimageindex: ~
      nocache: ~
      notranslate: ~
      max_video_preview: ~
      max_image_preview: ~
      max_snippet: ~
    alternates: ~
    icons: ~
    manifest: ~
    other: {}
    "###);
}
