use axum::{
    routing::{get, post},
    response::{Html, Json},
    extract::{Path, Query},
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::fs;
use tower_http::cors::CorsLayer;

#[derive(Debug, Serialize, Deserialize)]
struct MetadataRequest {
    title: Option<String>,
    description: Option<String>,
    keywords: Option<Vec<String>>,
    og_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct MetadataResponse {
    success: bool,
    message: String,
    metadata: Option<HashMap<String, String>>,
}

#[tokio::main]
async fn main() {
    println!("🚀 Starting Metadata Test Server");

    // Build our application with a route
    let app = Router::new()
        .route("/", get(serve_test_page))
        .route("/api/metadata", post(handle_metadata))
        .route("/api/health", get(health_check))
        .route("/test/:test_type", get(serve_test_case))
        .layer(CorsLayer::permissive());

    // Run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("✅ Server ready on http://{}", addr);
    println!("📄 Serving metadata test page at /");
    println!("🔧 API endpoints available at /api/*");
    println!("🧪 Test cases available at /test/*");

    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        app
    ).await.unwrap();
}

async fn serve_test_page() -> Html<String> {
    let html_content = fs::read_to_string("examples/test_server/static/index.html")
        .unwrap_or_else(|_| {
            r#"
            <!DOCTYPE html>
            <html lang="en">
            <head>
                <meta charset="UTF-8">
                <meta name="viewport" content="width=device-width, initial-scale=1">
                <title>Metadata Test Server</title>
                <meta name="description" content="Test server for metadata validation">
            </head>
            <body>
                <h1>Metadata Test Server</h1>
                <p>Server is running successfully!</p>
                <p>Check the console for available endpoints.</p>
            </body>
            </html>
            "#.to_string()
        });

    Html(html_content)
}

async fn health_check() -> Json<MetadataResponse> {
    Json(MetadataResponse {
        success: true,
        message: "Server is healthy".to_string(),
        metadata: None,
    })
}

async fn handle_metadata(
    Json(payload): Json<MetadataRequest>,
) -> Json<MetadataResponse> {
    let mut metadata = HashMap::new();

    if let Some(title) = payload.title {
        metadata.insert("title".to_string(), title);
    }
    if let Some(description) = payload.description {
        metadata.insert("description".to_string(), description);
    }
    if let Some(keywords) = payload.keywords {
        metadata.insert("keywords".to_string(), keywords.join(", "));
    }
    if let Some(og_type) = payload.og_type {
        metadata.insert("og:type".to_string(), og_type);
    }

    Json(MetadataResponse {
        success: true,
        message: "Metadata processed successfully".to_string(),
        metadata: Some(metadata),
    })
}

async fn serve_test_case(
    Path(test_type): Path<String>,
    Query(params): Query<HashMap<String, String>>,
) -> Html<String> {
    let title = params.get("title").cloned().unwrap_or_else(|| "Test Page".to_string());
    let description = params.get("description").cloned().unwrap_or_else(|| "A test page for metadata validation".to_string());

    let html = match test_type.as_str() {
        "basic" => format!(
            r#"
            <!DOCTYPE html>
            <html lang="en">
            <head>
                <meta charset="UTF-8">
                <meta name="viewport" content="width=device-width, initial-scale=1">
                <title>{}</title>
                <meta name="description" content="{}">
            </head>
            <body>
                <h1>{}</h1>
                <p>{}</p>
            </body>
            </html>
            "#,
            title, description, title, description
        ),
        "edge-case" => format!(
            r#"
            <!DOCTYPE html>
            <html lang="en">
            <head>
                <meta charset="UTF-8">
                <meta name="viewport" content="width=device-width, initial-scale=1">
                <title>{}</title>
                <meta name="description" content="{}">
                <meta name="keywords" content="test, edge, case, very, long, description, that, might, cause, issues, with, metadata, validation, and, testing, frameworks">
            </head>
            <body>
                <h1>{}</h1>
                <p>{}</p>
                <p>This is an edge case test with very long content.</p>
            </body>
            </html>
            "#,
            title, description, title, description
        ),
        _ => format!(
            r#"
            <!DOCTYPE html>
            <html lang="en">
            <head>
                <meta charset="UTF-8">
                <meta name="viewport" content="width=device-width, initial-scale=1">
                <title>Unknown Test: {}</title>
            </head>
            <body>
                <h1>Unknown Test Type</h1>
                <p>The test type "{}" is not recognized.</p>
            </body>
            </html>
            "#,
            test_type, test_type
        ),
    };

    Html(html)
}
