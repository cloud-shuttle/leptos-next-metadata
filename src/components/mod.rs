//! HTML components for metadata rendering
//!
//! This module provides basic HTML components for rendering metadata tags
//! without depending on leptos_meta.

use leptos::prelude::*;

/// A simple Title component that renders a <title> tag
#[component]
pub fn Title(
    /// The title text
    #[prop(into)]
    text: String,
) -> impl IntoView {
    view! {
        <title>{text}</title>
    }
}

/// A simple Meta component that renders a <meta> tag
#[component]
pub fn Meta(
    /// The meta name attribute
    #[prop(into)]
    name: String,
    /// The meta content attribute
    #[prop(into)]
    content: String,
) -> impl IntoView {
    view! {
        <meta name=name content=content/>
    }
}

/// A simple Meta component with property attribute for Open Graph
#[component]
pub fn MetaProperty(
    /// The meta property attribute
    #[prop(into)]
    property: String,
    /// The meta content attribute
    #[prop(into)]
    content: String,
) -> impl IntoView {
    view! {
        <meta prop:property=property content=content/>
    }
}

/// A simple Link component that renders a <link> tag
#[component]
pub fn Link(
    /// The link rel attribute
    #[prop(into)]
    rel: String,
    /// The link href attribute
    #[prop(into)]
    href: String,
) -> impl IntoView {
    view! {
        <link rel=rel href=href/>
    }
}
