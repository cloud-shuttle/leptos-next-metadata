//! String processing utilities

/// Normalize whitespace in text
pub fn normalize_whitespace(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Convert text to title case
pub fn to_title_case(text: &str) -> String {
    text.split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str().to_lowercase().as_str(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Extract plain text from HTML
pub fn strip_html(html: &str) -> String {
    // Simple HTML stripping - in production would use a proper HTML parser
    let mut result = html.to_string();

    // Remove script and style tags completely
    while let Some(start) = result.find("<script") {
        if let Some(end) = result[start..].find("</script>") {
            result.drain(start..start + end + 9);
        } else {
            break;
        }
    }

    while let Some(start) = result.find("<style") {
        if let Some(end) = result[start..].find("</style>") {
            result.drain(start..start + end + 8);
        } else {
            break;
        }
    }

    // Remove all HTML tags
    let mut in_tag = false;
    let mut clean_text = String::new();

    for ch in result.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => clean_text.push(ch),
            _ => {}
        }
    }

    normalize_whitespace(&clean_text)
}

/// Generate SEO-friendly slug from text
pub fn slugify(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() {
                c
            } else if c.is_whitespace() || c == '-' || c == '_' {
                '-'
            } else {
                // Remove special characters
                ' '
            }
        })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join("-")
        .trim_matches('-')
        .to_string()
}

/// Escape HTML entities
pub fn escape_html(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}
