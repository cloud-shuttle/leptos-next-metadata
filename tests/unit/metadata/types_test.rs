use leptos_next_metadata::metadata::*;
use rstest::*;
use pretty_assertions::assert_eq;
use insta::assert_yaml_snapshot;

#[rstest]
#[case::static_title(Title::Static("Test".into()), Some("Page"), "Test")]
#[case::template_with_value(
    Title::Template { 
        template: "%s | Site".into(), 
        default: "Site".into() 
    },
    Some("Home"),
    "Home | Site"
)]
#[case::template_without_value(
    Title::Template { 
        template: "%s | Site".into(), 
        default: "Site".into() 
    },
    None,
    "Site"
)]
#[case::absolute(Title::Absolute("Override".into()), Some("Ignored"), "Override")]
fn test_title_resolution(
    #[case] title: Title,
    #[case] segment: Option<&str>,
    #[case] expected: &str,
) {
    assert_eq!(title.resolve(segment), expected);
}

#[test]
fn test_metadata_default() {
    let meta = Metadata::default();
    assert!(meta.title.is_none());
    assert!(meta.description.is_none());
    assert!(meta.keywords.is_empty());
    assert_yaml_snapshot!(meta, @r###"
    title: ~
    description: ~
    keywords: []
    open_graph: ~
    twitter: ~
    robots: ~
    alternates: ~
    icons: ~
    manifest: ~
    other: {}
    "###);
}

#[rstest]
#[case::valid_description("A good description with proper length", true)]
#[case::too_short("Short", false)]
#[case::too_long(&"x".repeat(200), false)]
#[case::empty("", false)]
#[case::whitespace_only("   ", false)]
fn test_description_validation(
    #[case] description: &str,
    #[case] is_valid: bool,
) {
    let validator = DescriptionValidator::new(20, 160);
    assert_eq!(validator.is_valid(description), is_valid);
}

#[test]
fn test_open_graph_types() {
    let og_types = vec![
        "website", "article", "book", "profile",
        "music.song", "music.album", "music.playlist", "music.radio_station",
        "video.movie", "video.episode", "video.tv_show", "video.other",
    ];
    
    for og_type in og_types {
        let og = OpenGraph {
            og_type: Some(og_type.into()),
            ..Default::default()
        };
        assert_eq!(og.og_type.as_deref(), Some(og_type));
    }
}

#[test]
fn test_twitter_card_types() {
    let card_types = vec![
        "summary", "summary_large_image", "app", "player"
    ];
    
    for card_type in card_types {
        let twitter = Twitter {
            card: Some(card_type.into()),
            ..Default::default()
        };
        assert_eq!(twitter.card.as_deref(), Some(card_type));
    }
}

#[test]
fn test_robots_directives() {
    let robots = Robots {
        index: Some(true),
        follow: Some(false),
        noarchive: Some(true),
        nosnippet: Some(false),
        noimageindex: Some(true),
        nocache: Some(false),
        ..Default::default()
    };
    
    let directives = robots.to_directives();
    assert!(directives.contains("index"));
    assert!(directives.contains("nofollow"));
    assert!(directives.contains("noarchive"));
    assert!(directives.contains("snippet"));
    assert!(directives.contains("noimageindex"));
    assert!(directives.contains("cache"));
}

#[test]
fn test_viewport_default() {
    let viewport = Viewport::default();
    assert_eq!(viewport.width, Some("device-width".into()));
    assert_eq!(viewport.initial_scale, Some(1.0));
    assert_eq!(viewport.maximum_scale, None);
    assert_eq!(viewport.user_scalable, None);
}

#[test]
fn test_icon_rel_attribute() {
    let icon = Icon {
        url: "/favicon.ico".into(),
        sizes: Some("16x16".into()),
        icon_type: Some("image/x-icon".into()),
        rel: Some("icon".into()),
        ..Default::default()
    };
    
    assert_eq!(icon.get_rel(), "icon");
    
    let apple_icon = Icon {
        url: "/apple-touch-icon.png".into(),
        rel: Some("apple-touch-icon".into()),
        ..Default::default()
    };
    
    assert_eq!(apple_icon.get_rel(), "apple-touch-icon");
}

#[test]
fn test_alternate_languages() {
    let mut alternates = Alternates::default();
    alternates.languages.insert("en".into(), "/en".into());
    alternates.languages.insert("es".into(), "/es".into());
    alternates.languages.insert("fr".into(), "/fr".into());
    
    assert_eq!(alternates.languages.len(), 3);
    assert_eq!(alternates.languages.get("en"), Some(&"/en".to_string()));
}

#[test]
fn test_metadata_builder_pattern() {
    let meta = Metadata::builder()
        .title("Test Page")
        .description("A test page description")
        .keywords(vec!["test", "rust", "leptos"])
        .og_title("OG Test Page")
        .og_description("OG description")
        .twitter_card("summary_large_image")
        .build();
    
    match &meta.title {
        Some(Title::Static(title)) => assert_eq!(title, "Test Page"),
        _ => panic!("Expected static title"),
    }
    
    assert_eq!(meta.description.as_deref(), Some("A test page description"));
    assert_eq!(meta.keywords.len(), 3);
    
    let og = meta.open_graph.as_ref().unwrap();
    assert_eq!(og.title.as_deref(), Some("OG Test Page"));
    assert_eq!(og.description.as_deref(), Some("OG description"));
    
    let twitter = meta.twitter.as_ref().unwrap();
    assert_eq!(twitter.card.as_deref(), Some("summary_large_image"));
}

#[test]
fn test_metadata_validation() {
    let meta = Metadata {
        title: Some(Title::Static("".into())), // Invalid: empty
        description: Some("x".repeat(200)), // Invalid: too long
        keywords: vec!["valid".into(), "".into()], // Mixed validity
        ..Default::default()
    };
    
    let validation_result = meta.validate();
    assert!(!validation_result.is_valid);
    assert!(validation_result.errors.len() >= 2);
    
    assert!(validation_result.errors.iter().any(|e| e.contains("title")));
    assert!(validation_result.errors.iter().any(|e| e.contains("description")));
}

#[test] 
fn test_og_image_url_resolution() {
    let base_url = "https://example.com";
    
    let relative = OgImage::new("/image.jpg");
    assert_eq!(relative.resolve_url(base_url), "https://example.com/image.jpg");
    
    let absolute = OgImage::new("https://cdn.example.com/image.jpg");
    assert_eq!(absolute.resolve_url(base_url), "https://cdn.example.com/image.jpg");
    
    let protocol_relative = OgImage::new("//cdn.example.com/image.jpg");
    assert_eq!(protocol_relative.resolve_url(base_url), "https://cdn.example.com/image.jpg");
}