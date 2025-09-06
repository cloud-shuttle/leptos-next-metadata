#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metadata_macro_parsing() {
        let input = "title: \"Test\", description: \"Test description\"";
        let parsed = syn::parse_str::<MetadataMacroInput>(input);
        assert!(parsed.is_ok());
    }

    #[test]
    fn test_generate_metadata_macro_parsing() {
        let input = "async |params, parent| { Metadata::default() }";
        let parsed = syn::parse_str::<GenerateMetadataMacroInput>(input);
        assert!(parsed.is_ok());
    }

    #[test]
    fn test_og_image_macro_parsing() {
        let input = "size: (1200, 630), template: \"blog_post\"";
        let parsed = syn::parse_str::<OgImageMacroInput>(input);
        assert!(parsed.is_ok());
    }

    #[test]
    fn test_nested_metadata_parsing() {
        let input = "openGraph: { title: \"OG Title\", type: \"website\" }";
        let parsed = syn::parse_str::<MetadataMacroInput>(input);
        assert!(parsed.is_ok());

        if let Ok(metadata) = parsed {
            assert_eq!(metadata.fields.len(), 1);
            if let MetadataValue::Nested(fields) = &metadata.fields[0].value {
                assert_eq!(fields.len(), 2);
            }
        }
    }

    #[test]
    fn test_array_metadata_parsing() {
        let input = "keywords: [\"rust\", \"leptos\", \"metadata\"]";
        let parsed = syn::parse_str::<MetadataMacroInput>(input);
        assert!(parsed.is_ok());

        if let Ok(metadata) = parsed {
            assert_eq!(metadata.fields.len(), 1);
            if let MetadataValue::Array(values) = &metadata.fields[0].value {
                assert_eq!(values.len(), 3);
            }
        }
    }

    #[test]
    fn test_complex_metadata_parsing() {
        let input = r#"
            title: "My Page",
            description: "Page description",
            openGraph: {
                title: "OG Title",
                type: "website",
                images: ["/og-image.png"]
            },
            twitter: {
                card: "summary_large_image",
                site: "@mysite"
            }
        "#;

        let parsed = syn::parse_str::<MetadataMacroInput>(input);
        assert!(parsed.is_ok());

        if let Ok(metadata) = parsed {
            assert_eq!(metadata.fields.len(), 4); // title, description, openGraph, twitter
        }
    }

    #[test]
    fn test_field_mapping() {
        let input = MetadataMacroInput {
            fields: vec![
                MetadataField {
                    name: syn::parse_str::<Ident>("openGraph").unwrap(),
                    value: MetadataValue::Nested(vec![]),
                }
            ],
        };

        let struct_type = input.map_field_to_struct_type(&syn::parse_str::<Ident>("openGraph").unwrap());
        let struct_type_str = struct_type.to_string();
        assert!(struct_type_str.contains("OpenGraph"));
    }

    #[test]
    fn test_async_generate_metadata_parsing() {
        let input = "async |params, parent| {
            let post = fetch_post(&params.slug).await?;
            Metadata {
                title: Some(post.title),
                description: Some(post.excerpt),
                ..parent.await
            }
        }";

        let parsed = syn::parse_str::<GenerateMetadataMacroInput>(input);
        assert!(parsed.is_ok());

        if let Ok(gen_meta) = parsed {
            assert!(gen_meta.async_token.is_some());
            assert_eq!(gen_meta.params.len(), 2); // params, parent
        }
    }

    #[test]
    fn test_og_image_macro_parsing_with_data() {
        let input = r#"
            size: (1200, 630),
            template: "blog_post",
            data: {
                title: post.title,
                author: post.author.name,
                date: post.published_at.format("%B %d, %Y")
            }
        "#;

        let parsed = syn::parse_str::<OgImageMacroInput>(input);
        assert!(parsed.is_ok());

        if let Ok(og_image) = parsed {
            assert!(og_image.size.is_some());
            assert!(og_image.template.is_some());
            assert!(og_image.data.is_some());
        }
    }
}
