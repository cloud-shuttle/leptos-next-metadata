# Static Metadata

Static metadata is metadata that doesn't change based on dynamic data or user input. It's the most performant type of metadata since it can be resolved at compile time.

## Basic Static Metadata

The simplest way to add metadata to a component:

```rust
use leptos::*;
use leptos_next_metadata::prelude::*;

#[component]
fn AboutPage() -> impl IntoView {
    metadata! {
        title: "About Us",
        description: "Learn about our company and mission",
        keywords: ["about", "company", "mission", "team"],
    }
    
    view! {
        <main>
            <h1>"About Us"</h1>
            <p>"We build amazing web applications with Leptos!"</p>
        </main>
    }
}
```

## Complete Static Metadata Example

A comprehensive example showing all available static metadata options:

```rust
#[component]
fn ProductPage() -> impl IntoView {
    metadata! {
        // Basic SEO
        title: "Amazing Product - Best in Class",
        description: "Discover our amazing product that solves all your problems with innovative technology and great design.",
        keywords: ["product", "technology", "innovation", "solution"],
        
        // Authorship
        authors: [
            { name: "John Doe", url: "https://johndoe.com" },
            { name: "Jane Smith" },
        ],
        creator: "Our Company",
        publisher: "Our Company",
        
        // Robots and indexing
        robots: {
            index: true,
            follow: true,
            noarchive: false,
            nosnippet: false,
            noimageindex: false,
            nocache: false,
        },
        
        // Viewport (for responsive design)
        viewport: {
            width: "device-width",
            initial_scale: 1.0,
            maximum_scale: 5.0,
            user_scalable: true,
        },
        
        // Open Graph
        openGraph: {
            title: "Amazing Product",
            description: "The best product you'll ever use",
            type: "product",
            url: "https://mysite.com/product",
            site_name: "My Amazing Site",
            locale: "en_US",
            images: [{
                url: "https://mysite.com/product-image.jpg",
                width: 1200,
                height: 630,
                alt: "Product showcase image",
                type: "image/jpeg",
            }],
            videos: [{
                url: "https://mysite.com/product-video.mp4",
                width: 1280,
                height: 720,
                type: "video/mp4",
            }],
        },
        
        // Twitter Cards
        twitter: {
            card: "summary_large_image",
            site: "@mycompany",
            creator: "@johndoe",
            title: "Amazing Product",
            description: "The best product you'll ever use",
            images: ["https://mysite.com/twitter-image.jpg"],
        },
        
        // Icons and favicons
        icons: {
            icon: [
                { url: "/favicon.ico", sizes: "any" },
                { url: "/icon-192.png", sizes: "192x192", type: "image/png" },
                { url: "/icon-512.png", sizes: "512x512", type: "image/png" },
            ],
            apple: [
                { url: "/apple-touch-icon.png", sizes: "180x180" },
            ],
            shortcut: "/favicon.ico",
        },
        
        // PWA Manifest
        manifest: "/manifest.json",
        
        // Alternate versions
        alternates: {
            canonical: "https://mysite.com/product",
            languages: {
                "en": "https://mysite.com/en/product",
                "es": "https://mysite.com/es/product",
                "fr": "https://mysite.com/fr/product",
            },
        },
        
        // Custom meta tags
        other: {
            "theme-color": "#007bff",
            "color-scheme": "dark light",
            "format-detection": "telephone=no",
        }
    }
    
    view! { /* component content */ }
}
```

## Title Templates

Create reusable title patterns:

```rust
// Root layout with title template
#[component]
fn Layout(children: Children) -> impl IntoView {
    metadata! {
        title: {
            template: "%s | My Amazing Site",
            default: "My Amazing Site - Welcome"
        }
    }
    
    view! {
        <main>{children()}</main>
    }
}

// Child pages use the template
#[component]
fn HomePage() -> impl IntoView {
    metadata! {
        title: "Home"  // Becomes "Home | My Amazing Site"
    }
    
    view! { /* content */ }
}

#[component]
fn AboutPage() -> impl IntoView {
    metadata! {
        title: "About Us"  // Becomes "About Us | My Amazing Site"
    }
    
    view! { /* content */ }
}
```

### Absolute Titles

Override title templates when needed:

```rust
#[component]
fn SpecialPage() -> impl IntoView {
    metadata! {
        title: {
            absolute: "Special Page - No Template Applied"
        }
    }
    
    view! { /* content */ }
}
```

## Metadata Inheritance

Child components inherit and can override parent metadata:

```rust
// Parent layout
#[component]
fn BlogLayout(children: Children) -> impl IntoView {
    metadata! {
        title: {
            template: "%s | Blog | My Site",
            default: "Blog | My Site"
        },
        openGraph: {
            site_name: "My Site",
            type: "website",
            locale: "en_US",
        },
        twitter: {
            site: "@mysite",
        }
    }
    
    view! {
        <div class="blog-layout">
            {children()}
        </div>
    }
}

// Child page inherits and extends
#[component]
fn BlogHomePage() -> impl IntoView {
    metadata! {
        title: "Latest Posts",  // Becomes "Latest Posts | Blog | My Site"
        description: "Read our latest blog posts about web development",
        openGraph: {
            // Inherits site_name, locale from parent
            // Adds specific description for blog home
            description: "Read our latest blog posts about web development",
            images: ["/blog-home-og.jpg"],
        }
    }
    
    view! { /* blog home content */ }
}
```

## Conditional Metadata

Add metadata based on conditions:

```rust
#[component]
fn ConditionalPage() -> impl IntoView {
    let is_premium = use_context::<PremiumContext>()
        .map(|ctx| ctx.is_premium)
        .unwrap_or(false);
    
    // Conditional metadata based on user status
    if is_premium {
        metadata! {
            title: "Premium Features",
            description: "Access exclusive premium features",
            robots: {
                index: true,  // Index premium content
                follow: true,
            }
        }
    } else {
        metadata! {
            title: "Upgrade to Premium",
            description: "Unlock amazing premium features",
            robots: {
                index: false,  // Don't index upgrade pages
                follow: true,
            }
        }
    }
    
    view! { /* conditional content */ }
}
```

## Performance Considerations

### Compile-Time Optimization

Static metadata is resolved at compile time:

```rust
// ✅ This is optimal - resolved at compile time
metadata! {
    title: "Static Title",
    description: "Static description that never changes",
}

// ❌ This forces runtime resolution
let title = "Dynamic Title".to_string();
metadata! {
    title: title,  // Runtime string allocation
}
```

### Macro Expansion

The `metadata!` macro expands to efficient leptos_meta calls:

```rust
// This macro...
metadata! {
    title: "My Page",
    description: "Page description",
}

// ...expands to something like:
create_effect(move |_| {
    set_title("My Page");
    set_meta("description", "Page description");
});
```

## Best Practices

### SEO Best Practices

```rust
metadata! {
    // Title: 50-60 characters optimal
    title: "Perfect Length Title - Under 60 Characters",
    
    // Description: 150-160 characters optimal
    description: "This description is the perfect length for SEO, providing enough detail to entice clicks while staying under the 160 character limit.",
    
    // Keywords: 5-10 relevant keywords
    keywords: ["seo", "metadata", "leptos", "rust", "web"],
    
    // Always include Open Graph for social sharing
    openGraph: {
        title: "Title for Social Media",
        description: "Description optimized for social sharing",
        images: [{
            url: "/og-image-1200x630.jpg",  // 1.91:1 aspect ratio
            width: 1200,
            height: 630,
            alt: "Descriptive alt text for accessibility",
        }],
    },
}
```

### Accessibility

```rust
metadata! {
    // Proper viewport for accessibility
    viewport: {
        width: "device-width",
        initial_scale: 1.0,
        user_scalable: true,  // Don't disable zoom
    },
    
    // Theme color for better UX
    other: {
        "theme-color": "#007bff",
        "color-scheme": "light dark",  // Support both themes
    },
    
    // Accessible icons
    icons: {
        icon: [
            { url: "/favicon.ico", sizes: "any" },
            { url: "/favicon.svg", type: "image/svg+xml" },  // Vector favicon
        ],
    }
}
```

### Performance

```rust
// ✅ Optimal static metadata
metadata! {
    title: "Static Title",
    description: "Static description",
}

// ✅ Use constants for repeated values
const SITE_NAME: &str = "My Amazing Site";
const DEFAULT_OG_IMAGE: &str = "/default-og.jpg";

metadata! {
    openGraph: {
        site_name: SITE_NAME,
        images: [DEFAULT_OG_IMAGE],
    }
}

// ❌ Avoid runtime string building in static metadata
let site_name = format!("{} v2", "My Site");  // Unnecessary allocation
metadata! {
    openGraph: {
        site_name: site_name,  // Forces runtime resolution
    }
}
```

## Debugging and Validation

### Development Warnings

In development mode, leptos-next-metadata provides helpful warnings:

```rust
metadata! {
    // This will trigger a warning in development
    title: "This title is way too long and exceeds the recommended 60 character limit for optimal SEO performance",
    
    // This will also warn about missing description
    // description: missing!
}
```

### Validation Helpers

```rust
use leptos_next_metadata::validation::*;

#[component]
fn ValidatedPage() -> impl IntoView {
    metadata! {
        title: "My Page",
        description: "Page description",
    }
    
    // In development, validate metadata
    #[cfg(debug_assertions)]
    validate_current_metadata();
    
    view! { /* content */ }
}
```

---

**Next Steps:**
- [Dynamic Metadata](dynamic-metadata.md) - Learn about data-driven metadata
- [OG Images](og-images.md) - Generate custom Open Graph images  
- [SEO Optimization](seo-optimization.md) - Advanced SEO techniques