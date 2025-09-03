//! Procedural macros for leptos-next-metadata

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse::Parse, parse::ParseStream, Token, Ident, Expr, Result};

/// Macro for setting static metadata in Leptos components
/// 
/// # Example
/// ```rust
/// metadata! {
///     title: "My Page",
///     description: "A great page",
///     keywords: ["rust", "leptos"],
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

/// Macro for generating dynamic metadata
/// 
/// # Example
/// ```rust
/// generate_metadata! {
///     async |params, parent| {
///         let data = fetch_data(&params.id).await?;
///         Metadata {
///             title: Title::Absolute(data.title),
///             ..parent.await
///         }
///     }
/// }
/// ```
#[proc_macro]
pub fn generate_metadata(input: TokenStream) -> TokenStream {
    // Parse the input to validate syntax, even if we don't use it yet
    let _input = parse_macro_input!(input as syn::Block);
    
    // For now, just pass through - will implement full dynamic generation later
    quote! {
        // Placeholder implementation
        leptos::create_effect(move |_| {
            // Dynamic metadata will be implemented here
        });
    }.into()
}

/// Simple metadata input parser
struct MetadataInput {
    fields: Vec<MetadataField>,
}

struct MetadataField {
    name: Ident,
    value: Expr,
}

impl Parse for MetadataInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut fields = Vec::new();
        
        while !input.is_empty() {
            let name: Ident = input.parse()?;
            input.parse::<Token![:]>()?;
            let value: Expr = input.parse()?;
            
            fields.push(MetadataField { name, value });
            
            // Handle optional comma
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }
        
        Ok(MetadataInput { fields })
    }
}

fn generate_metadata_code(input: MetadataInput) -> Result<proc_macro2::TokenStream> {
    let field_setters = input.fields.iter().map(|field| {
        let name = &field.name;
        let value = &field.value;
        
        // Convert field names to metadata setters
        match name.to_string().as_str() {
            "title" => quote! {
                leptos_meta::Title::new(#value);
            },
            "description" => quote! {
                leptos_meta::Meta::new("description", #value);
            },
            "keywords" => quote! {
                leptos_meta::Meta::new("keywords", &format!("{:?}", #value).trim_matches(&['[', ']'][..]));
            },
            _ => quote! {
                leptos_meta::Meta::new(stringify!(#name), &#value.to_string());
            },
        }
    });
    
    Ok(quote! {
        {
            use leptos::*;
            use leptos_meta::*;
            
            #(#field_setters)*
        }
    })
}