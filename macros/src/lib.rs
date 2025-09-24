//! Procedural macros for leptos-next-metadata

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parse, parse::ParseStream, parse_macro_input, Expr, Ident, Result, Token};

/// Generate metadata tags for Leptos applications
///
/// This macro generates the appropriate leptos_meta components based on the provided metadata.
///
/// # Examples
///
/// ```rust
/// use leptos_next_metadata_macros::metadata;
///
/// metadata! {
///     title: "My Page",
///     description: "This is my page description",
///     keywords: ["rust", "leptos", "web"],
///     openGraph: {
///         title: "My Page",
///         type: "website",
///         url: "https://example.com"
///     },
///     twitter: {
///         card: "summary",
///         title: "My Page"
///     }
/// }
/// ```
#[proc_macro]
pub fn metadata(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as MetadataInput);

    match generate_metadata_code(input) {
        Ok(tokens) => tokens.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

/// Generate dynamic metadata at runtime
///
/// This macro accepts an async closure that returns a Metadata struct and generates
/// the appropriate leptos_meta components reactively.
///
/// # Examples
///
/// ```rust
/// use leptos_next_metadata_macros::generate_metadata;
///
/// generate_metadata! {
///     async || {
///         // Fetch data from API
///         let data = fetch_page_data().await;
///
///         Metadata {
///             title: Title::Template {
///                 template: "%s | My Blog".into(),
///                 default: "My Blog".into(),
///             },
///             description: Some(data.excerpt),
///             ..Default::default()
///         }
///     }
/// }
/// ```
#[proc_macro]
pub fn generate_metadata(input: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(input as GenerateMetadataInputParser);
    let input = GenerateMetadataInput {
        closure: parsed.closure,
    };

    generate_dynamic_metadata_code(input).into()
}

/// Parsed input for the generate_metadata macro
struct GenerateMetadataInput {
    /// The async closure that generates metadata
    closure: Expr,
}

/// Parser for the generate_metadata macro input
struct GenerateMetadataInputParser {
    closure: Expr,
}

impl Parse for GenerateMetadataInputParser {
    fn parse(input: ParseStream) -> Result<Self> {
        // Parse the async closure: async || { ... }
        let closure: Expr = input.parse()?;

        Ok(GenerateMetadataInputParser { closure })
    }
}

/// Generate the dynamic metadata code
fn generate_dynamic_metadata_code(input: GenerateMetadataInput) -> proc_macro2::TokenStream {
    let closure = &input.closure;

    quote! {
        {
            use leptos::*;
            use leptos::prelude::*;
            use leptos_next_metadata::metadata::Metadata;

            // Create a signal to hold the metadata
            let (metadata, set_metadata) = signal(Metadata::default());

            // Execute the async closure and update the metadata
            leptos::task::spawn_local(async move {
                let result = (#closure)().await;
                set_metadata.set(result);
            });

            // Generate metadata components reactively
            move || {
                let meta = metadata.get();

                // Basic metadata
                let title_view = meta.title.as_ref().map(|title| {
                    match title {
                        leptos_next_metadata::metadata::Title::Static(s) => {
                            view! { <Title text=s.to_string()/> }
                        },
                        leptos_next_metadata::metadata::Title::Template { template: _, default } => {
                            view! { <Title text=default.to_string()/> }
                        }
                    }
                });

                let description_view = meta.description.as_ref().map(|desc| {
                    view! { <Meta name="description" content=desc.to_string()/> }
                });

                let keywords_view = meta.keywords.as_ref().map(|keywords| {
                    match keywords {
                        leptos_next_metadata::metadata::Keywords::Single(k) => {
                            view! { <Meta name="keywords" content=k.to_string()/> }
                        },
                        leptos_next_metadata::metadata::Keywords::Multiple(ks) => {
                            let keywords_str = ks.join(", ");
                            view! { <Meta name="keywords" content=keywords_str/> }
                        }
                    }
                });

                let canonical_view = meta.canonical.as_ref().map(|url| {
                    view! { <Link rel="canonical" href=url.to_string()/> }
                });

                // OpenGraph metadata
                let og_view = meta.open_graph.as_ref().map(|og| {
                    view! {
                        <>
                            {og.title.as_ref().map(|title| {
                                view! { <Meta property="og:title" content=title.to_string()/> }
                            })}

                            {og.description.as_ref().map(|desc| {
                                view! { <Meta property="og:description" content=desc.to_string()/> }
                            })}

                            {og.r#type.as_ref().map(|og_type| {
                                view! { <Meta property="og:type" content=og_type.to_string()/> }
                            })}

                            {og.url.as_ref().map(|url| {
                                view! { <Meta property="og:url" content=url.to_string()/> }
                            })}

                            {og.site_name.as_ref().map(|name| {
                                view! { <Meta property="og:site_name" content=name.to_string()/> }
                            })}

                            // OG images iteration removed for now - will add back later
                        </>
                    }
                });

                // Twitter metadata
                let twitter_view = meta.twitter.as_ref().map(|twitter| {
                    view! {
                        <>
                            {twitter.card.as_ref().map(|card| {
                                let card_str = match card {
                                    leptos_next_metadata::metadata::TwitterCard::Summary => "summary",
                                    leptos_next_metadata::metadata::TwitterCard::SummaryLargeImage => "summary_large_image",
                                    leptos_next_metadata::metadata::TwitterCard::App => "app",
                                    leptos_next_metadata::metadata::TwitterCard::Player => "player",
                                };
                                view! { <Meta name="twitter:card" content=card_str.to_string()/> }
                            })}

                            {twitter.title.as_ref().map(|title| {
                                view! { <Meta name="twitter:title" content=title.to_string()/> }
                            })}

                            {twitter.description.as_ref().map(|desc| {
                                view! { <Meta name="twitter:description" content=desc.to_string()/> }
                            })}
                        </>
                    }
                });

                // Additional metadata iteration removed for now - will add back later

                // Return all views
                view! {
                    <>
                        {title_view.map(|view| view)}
                        {description_view.map(|view| view)}
                        {keywords_view.map(|view| view)}
                        {canonical_view.map(|view| view)}
                        {og_view.map(|view| view)}
                        {twitter_view.map(|view| view)}
                    </>
                }
            }
        }
    }
}

/// Metadata input parser that handles nested structures
struct MetadataInput {
    fields: Vec<MetadataField>,
}

struct MetadataField {
    name: Ident,
    value: MetadataValue,
}

#[allow(clippy::large_enum_variant)]
enum MetadataValue {
    /// Simple string or expression
    Simple(Expr),
    /// Nested struct-like object
    Nested(Vec<MetadataField>),
    /// Array of values
    Array(Vec<MetadataValue>),
}

impl Parse for MetadataInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut fields = Vec::new();

        while !input.is_empty() {
            let name: Ident = input.parse()?;
            input.parse::<Token![:]>()?;
            let value = MetadataValue::parse(input)?;

            fields.push(MetadataField { name, value });

            // Handle optional comma
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(MetadataInput { fields })
    }
}

impl Parse for MetadataValue {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(syn::token::Brace) {
            // Parse nested object: { field: value, ... }
            let content;
            syn::braced!(content in input);

            let mut fields = Vec::new();
            while !content.is_empty() {
                let name: Ident = content.parse()?;
                content.parse::<Token![:]>()?;
                let value = MetadataValue::parse(&content)?;

                fields.push(MetadataField { name, value });

                if content.peek(Token![,]) {
                    content.parse::<Token![,]>()?;
                }
            }

            Ok(MetadataValue::Nested(fields))
        } else if input.peek(syn::token::Bracket) {
            // Parse array: [value1, value2, ...]
            let content;
            syn::bracketed!(content in input);

            let mut values = Vec::new();
            while !content.is_empty() {
                let value = MetadataValue::parse(&content)?;
                values.push(value);

                if content.peek(Token![,]) {
                    content.parse::<Token![,]>()?;
                }
            }

            Ok(MetadataValue::Array(values))
        } else {
            // Parse simple expression
            let expr: Expr = input.parse()?;
            Ok(MetadataValue::Simple(expr))
        }
    }
}

fn generate_metadata_code(input: MetadataInput) -> Result<proc_macro2::TokenStream> {
    let mut meta_tags = Vec::new();

    for field in input.fields {
        let field_name = field.name.to_string();
        let tags = generate_field_meta_tags(&field_name, &field.value)?;
        meta_tags.extend(tags);
    }

    Ok(quote! {
        {
            use leptos::*;
            use leptos::prelude::*;
            use leptos_next_metadata::components::*;

            view! {
                <>
                    #(#meta_tags)*
                </>
            }
        }
    })
}

fn generate_field_meta_tags(
    field_name: &str,
    value: &MetadataValue,
) -> Result<Vec<proc_macro2::TokenStream>> {
    let mut tags = Vec::new();

    match value {
        MetadataValue::Simple(expr) => {
            let tag = match field_name {
                "title" => quote! {
                    <Title text=#expr/>
                },
                "description" => quote! {
                    <Meta name="description" content=#expr/>
                },
                "keywords" => quote! {
                    <Meta name="keywords" content=#expr/>
                },
                "author" => quote! {
                    <Meta name="author" content=#expr/>
                },
                "robots" => quote! {
                    <Meta name="robots" content=#expr/>
                },
                "canonical" => quote! {
                    <Link rel="canonical" href=#expr/>
                },
                "viewport" => quote! {
                    <Meta name="viewport" content=#expr/>
                },
                "themeColor" => quote! {
                    <Meta name="theme-color" content=#expr/>
                },
                "colorScheme" => quote! {
                    <Meta name="color-scheme" content=#expr/>
                },
                "referrer" => quote! {
                    <Meta name="referrer" content=#expr/>
                },
                "formatDetection" => quote! {
                    <Meta name="format-detection" content=#expr/>
                },
                _ => quote! {
                    <Meta name=#field_name content=#expr/>
                },
            };
            tags.push(tag);
        }
        MetadataValue::Nested(fields) => {
            match field_name {
                "openGraph" | "open_graph" => {
                    for field in fields {
                        let field_name = field.name.to_string();
                        let nested_tags = generate_og_meta_tags(&field_name, &field.value)?;
                        tags.extend(nested_tags);
                    }
                }
                "twitter" => {
                    for field in fields {
                        let field_name = field.name.to_string();
                        let nested_tags = generate_twitter_meta_tags(&field_name, &field.value)?;
                        tags.extend(nested_tags);
                    }
                }
                _ => {
                    // Handle other nested structures generically
                    for field in fields {
                        let field_name = field.name.to_string();
                        if let MetadataValue::Simple(value) = &field.value {
                            tags.push(quote! {
                                <Meta name=format!("{}:{}", #field_name, #field_name) content=#value/>
                            });
                        }
                    }
                }
            }
        }
        MetadataValue::Array(values) => {
            let array_tags = generate_array_meta_tags(field_name, values)?;
            tags.extend(array_tags);
        }
    }

    Ok(tags)
}

fn generate_og_meta_tags(
    field_name: &str,
    value: &MetadataValue,
) -> Result<Vec<proc_macro2::TokenStream>> {
    let mut tags = Vec::new();

    match value {
        MetadataValue::Simple(expr) => {
            let tag = match field_name {
                "title" => quote! {
                    <MetaProperty property="og:title" content=#expr/>
                },
                "description" => quote! {
                    <MetaProperty property="og:description" content=#expr/>
                },
                "type" => quote! {
                    <MetaProperty property="og:type" content=#expr/>
                },
                "url" => quote! {
                    <MetaProperty property="og:url" content=#expr/>
                },
                "siteName" | "site_name" => quote! {
                    <MetaProperty property="og:site_name" content=#expr/>
                },
                "locale" => quote! {
                    <MetaProperty property="og:locale" content=#expr/>
                },
                _ => quote! {
                    <Meta property=format!("og:{}", #field_name) content=#expr/>
                },
            };
            tags.push(tag);
        }
        MetadataValue::Array(array_values) => {
            match field_name {
                "images" => {
                    for image_value in array_values {
                        if let MetadataValue::Simple(url) = image_value {
                            tags.push(quote! {
                                <MetaProperty property="og:image" content=#url/>
                            });
                        }
                    }
                }
                "videos" => {
                    for video_value in array_values {
                        if let MetadataValue::Simple(url) = video_value {
                            tags.push(quote! {
                                <MetaProperty property="og:video" content=#url/>
                            });
                        }
                    }
                }
                _ => {
                    // TODO: Handle generic array field for og:{} - need to implement proper array handling
                }
            }
        }
        _ => {
            // TODO: Handle other nested structures - need to implement proper nested handling
        }
    }

    Ok(tags)
}

fn generate_twitter_meta_tags(
    field_name: &str,
    value: &MetadataValue,
) -> Result<Vec<proc_macro2::TokenStream>> {
    let mut tags = Vec::new();

    match value {
        MetadataValue::Simple(expr) => {
            let tag = match field_name {
                "card" => quote! {
                    <Meta name="twitter:card" content=#expr/>
                },
                "site" => quote! {
                    <Meta name="twitter:site" content=#expr/>
                },
                "creator" => quote! {
                    <Meta name="twitter:creator" content=#expr/>
                },
                "title" => quote! {
                    <Meta name="twitter:title" content=#expr/>
                },
                "description" => quote! {
                    <Meta name="twitter:description" content=#expr/>
                },
                "image" => quote! {
                    <Meta name="twitter:image" content=#expr/>
                },
                _ => quote! {
                    <Meta name=format!("twitter:{}", #field_name) content=#expr/>
                },
            };
            tags.push(tag);
        }
        MetadataValue::Array(array_values) => {
            match field_name {
                "images" => {
                    for image_value in array_values {
                        if let MetadataValue::Simple(url) = image_value {
                            tags.push(quote! {
                                <Meta name="twitter:image" content=#url/>
                            });
                        }
                    }
                }
                _ => {
                    // TODO: Handle generic twitter array fields
                }
            }
        }
        _ => {
            // TODO: Handle other nested structures for twitter
        }
    }

    Ok(tags)
}

fn generate_array_meta_tags(
    field_name: &str,
    values: &[MetadataValue],
) -> Result<Vec<proc_macro2::TokenStream>> {
    let mut tags = Vec::new();

    match field_name {
        "keywords" => {
            // Convert array to comma-separated string
            let mut keyword_exprs = Vec::new();
            for value in values {
                if let MetadataValue::Simple(expr) = value {
                    keyword_exprs.push(expr);
                }
            }

            if !keyword_exprs.is_empty() {
                tags.push(quote! {
                    <Meta name="keywords" content={[#(#keyword_exprs),*].join(", ")}/>
                });
            }
        }
        "images" => {
            // Generate image meta tags
            for value in values {
                if let MetadataValue::Simple(url) = value {
                    tags.push(quote! {
                        <Meta name="image" content=#url/>
                    });
                }
            }
        }
        "authors" => {
            // Generate author meta tags
            for value in values {
                if let MetadataValue::Simple(author) = value {
                    tags.push(quote! {
                        <Meta name="author" content=#author/>
                    });
                }
            }
        }
        _ => {
            // Handle other arrays generically
            for value in values {
                if let MetadataValue::Simple(expr) = value {
                    tags.push(quote! {
                        <Meta name=#field_name content=#expr/>
                    });
                }
            }
        }
    }

    Ok(tags)
}
