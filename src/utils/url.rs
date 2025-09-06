//! URL handling utilities

use crate::Result;

/// Join URL paths safely
pub fn join_url_paths(base: &str, path: &str) -> String {
    let base = base.trim_end_matches('/');
    let path = path.trim_start_matches('/');

    if path.is_empty() {
        base.to_string()
    } else if base.is_empty() {
        format!("/{}", path)
    } else {
        format!("{}/{}", base, path)
    }
}

/// Ensure URL has protocol
pub fn ensure_protocol(url: &str, default_protocol: &str) -> String {
    if url.starts_with("http://") || url.starts_with("https://") {
        url.to_string()
    } else if url.starts_with("//") {
        format!("{}{}", default_protocol, url)
    } else {
        format!("{}://{}", default_protocol, url)
    }
}

/// Check if URL is absolute
pub fn is_absolute_url(url: &str) -> bool {
    url.starts_with("http://") || url.starts_with("https://") || url.starts_with("//")
}

/// Extract domain from URL
pub fn extract_domain(url: &str) -> Result<String> {
    let url = url::Url::parse(url).map_err(|e| {
        crate::Error::UrlError(url::ParseError::from(e))
    })?;

    url.host_str()
        .map(|host| host.to_string())
        .ok_or_else(|| crate::Error::ValidationError("No host found in URL".to_string()))
}

/// Build URL with query parameters
pub fn build_url_with_params(base: &str, params: &[(&str, &str)]) -> String {
    if params.is_empty() {
        return base.to_string();
    }

    let query_string = params
        .iter()
        .map(|(k, v)| format!("{}={}", urlencoding::encode(k), urlencoding::encode(v)))
        .collect::<Vec<_>>()
        .join("&");

    if base.contains('?') {
        format!("{}&{}", base, query_string)
    } else {
        format!("{}?{}", base, query_string)
    }
}

/// Normalize URL by removing trailing slashes and fragments
pub fn normalize_url(url: &str) -> Result<String> {
    let mut parsed = url::Url::parse(url).map_err(|e| {
        crate::Error::UrlError(url::ParseError::from(e))
    })?;

    // Remove fragment
    parsed.set_fragment(None);

    // Normalize path
    let path = parsed.path().trim_end_matches('/');
    if path.is_empty() {
        parsed.set_path("/");
    } else {
        parsed.set_path(path);
    }

    Ok(parsed.to_string())
}
