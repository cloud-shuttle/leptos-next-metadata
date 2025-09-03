//! JSON-LD structured data for leptos-next-metadata
//! 
//! This module provides type-safe JSON-LD generation for Schema.org types,
//! enabling rich snippets and better search engine understanding.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// JSON-LD structured data (uses serde_json::Value)
pub type JsonLd = serde_json::Value;

/// Schema.org types for structured data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SchemaOrg {
    /// Web page
    WebPage(WebPage),
    
    /// Article
    Article(Article),
    
    /// Blog post
    BlogPosting(BlogPosting),
    
    /// Product
    Product(Product),
    
    /// Organization
    Organization(Organization),
    
    /// Person
    Person(Person),
    
    /// FAQ page
    FAQPage(FAQPage),
    
    /// Breadcrumb list
    BreadcrumbList(BreadcrumbList),
    
    /// Custom/raw value
    Custom(serde_json::Value),
}

/// Web page schema
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WebPage {
    #[serde(rename = "@context")]
    pub context: Option<String>,
    
    #[serde(rename = "@type")]
    pub type_: Option<String>,
    
    pub name: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    
    #[serde(flatten)]
    pub additional: HashMap<String, serde_json::Value>,
}

/// Article schema
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Article {
    #[serde(rename = "@context")]
    pub context: Option<String>,
    
    #[serde(rename = "@type")]
    pub type_: Option<String>,
    
    pub headline: Option<String>,
    pub description: Option<String>,
    pub author: Option<serde_json::Value>,
    pub datePublished: Option<String>,
    pub dateModified: Option<String>,
    pub image: Option<serde_json::Value>,
    pub url: Option<String>,
    
    #[serde(flatten)]
    pub additional: HashMap<String, serde_json::Value>,
}

/// Blog posting schema
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BlogPosting {
    #[serde(rename = "@context")]
    pub context: Option<String>,
    
    #[serde(rename = "@type")]
    pub type_: Option<String>,
    
    pub headline: Option<String>,
    pub description: Option<String>,
    pub author: Option<serde_json::Value>,
    pub datePublished: Option<String>,
    pub dateModified: Option<String>,
    pub image: Option<serde_json::Value>,
    pub url: Option<String>,
    pub wordCount: Option<i32>,
    
    #[serde(flatten)]
    pub additional: HashMap<String, serde_json::Value>,
}

/// Product schema
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Product {
    #[serde(rename = "@context")]
    pub context: Option<String>,
    
    #[serde(rename = "@type")]
    pub type_: Option<String>,
    
    pub name: Option<String>,
    pub description: Option<String>,
    pub image: Option<serde_json::Value>,
    pub brand: Option<serde_json::Value>,
    pub offers: Option<serde_json::Value>,
    pub aggregateRating: Option<serde_json::Value>,
    
    #[serde(flatten)]
    pub additional: HashMap<String, serde_json::Value>,
}

/// Organization schema
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Organization {
    #[serde(rename = "@context")]
    pub context: Option<String>,
    
    #[serde(rename = "@type")]
    pub type_: Option<String>,
    
    pub name: Option<String>,
    pub url: Option<String>,
    pub logo: Option<serde_json::Value>,
    pub sameAs: Option<Vec<String>>,
    pub contactPoint: Option<serde_json::Value>,
    
    #[serde(flatten)]
    pub additional: HashMap<String, serde_json::Value>,
}

/// Person schema
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Person {
    #[serde(rename = "@context")]
    pub context: Option<String>,
    
    #[serde(rename = "@type")]
    pub type_: Option<String>,
    
    pub name: Option<String>,
    pub url: Option<String>,
    pub image: Option<serde_json::Value>,
    pub sameAs: Option<Vec<String>>,
    pub jobTitle: Option<String>,
    pub worksFor: Option<serde_json::Value>,
    
    #[serde(flatten)]
    pub additional: HashMap<String, serde_json::Value>,
}

/// FAQ page schema
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FAQPage {
    #[serde(rename = "@context")]
    pub context: Option<String>,
    
    #[serde(rename = "@type")]
    pub type_: Option<String>,
    
    pub mainEntity: Option<Vec<Question>>,
    
    #[serde(flatten)]
    pub additional: HashMap<String, serde_json::Value>,
}

/// Question schema for FAQ
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Question {
    #[serde(rename = "@type")]
    pub type_: Option<String>,
    
    pub name: Option<String>,
    pub acceptedAnswer: Option<Answer>,
    
    #[serde(flatten)]
    pub additional: HashMap<String, serde_json::Value>,
}

/// Answer schema for FAQ
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Answer {
    #[serde(rename = "@type")]
    pub type_: Option<String>,
    
    pub text: Option<String>,
    
    #[serde(flatten)]
    pub additional: HashMap<String, serde_json::Value>,
}

/// Breadcrumb list schema
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BreadcrumbList {
    #[serde(rename = "@context")]
    pub context: Option<String>,
    
    #[serde(rename = "@type")]
    pub type_: Option<String>,
    
    pub itemListElement: Option<Vec<ListItem>>,
    
    #[serde(flatten)]
    pub additional: HashMap<String, serde_json::Value>,
}

/// List item for breadcrumbs
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListItem {
    #[serde(rename = "@type")]
    pub type_: Option<String>,
    
    pub position: Option<i32>,
    pub name: Option<String>,
    pub item: Option<String>,
    
    #[serde(flatten)]
    pub additional: HashMap<String, serde_json::Value>,
}

impl SchemaOrg {
    /// Create a web page schema
    pub fn web_page(name: &str, description: Option<&str>, url: Option<&str>) -> Self {
        SchemaOrg::WebPage(WebPage {
            context: Some("https://schema.org".to_string()),
            type_: Some("WebPage".to_string()),
            name: Some(name.to_string()),
            description: description.map(|s| s.to_string()),
            url: url.map(|s| s.to_string()),
            additional: HashMap::new(),
        })
    }
    
    /// Create an article schema
    pub fn article(
        headline: &str,
        author: &str,
        date_published: &str,
        description: Option<&str>,
    ) -> Self {
        let author_value = serde_json::json!({
            "@type": "Person",
            "name": author
        });
        
        SchemaOrg::Article(Article {
            context: Some("https://schema.org".to_string()),
            type_: Some("Article".to_string()),
            headline: Some(headline.to_string()),
            author: Some(author_value),
            datePublished: Some(date_published.to_string()),
            description: description.map(|s| s.to_string()),
            ..Default::default()
        })
    }
    
    /// Create an organization schema
    pub fn organization(name: &str, url: Option<&str>, logo: Option<&str>) -> Self {
        SchemaOrg::Organization(Organization {
            context: Some("https://schema.org".to_string()),
            type_: Some("Organization".to_string()),
            name: Some(name.to_string()),
            url: url.map(|s| s.to_string()),
            logo: logo.map(|s| serde_json::Value::String(s.to_string())),
            sameAs: None,
            contactPoint: None,
            additional: HashMap::new(),
        })
    }
    
    /// Create a person schema
    pub fn person(name: &str, job_title: Option<&str>, url: Option<&str>) -> Self {
        SchemaOrg::Person(Person {
            context: Some("https://schema.org".to_string()),
            type_: Some("Person".to_string()),
            name: Some(name.to_string()),
            jobTitle: job_title.map(|s| s.to_string()),
            url: url.map(|s| s.to_string()),
            image: None,
            sameAs: None,
            worksFor: None,
            additional: HashMap::new(),
        })
    }
    
    /// Create an FAQ page schema
    pub fn faq_page(questions: &[(&str, &str)]) -> Self {
        let main_entity: Vec<Question> = questions
            .iter()
            .map(|(question, answer)| Question {
                type_: Some("Question".to_string()),
                name: Some(question.to_string()),
                acceptedAnswer: Some(Answer {
                    type_: Some("Answer".to_string()),
                    text: Some(answer.to_string()),
                    ..Default::default()
                }),
                ..Default::default()
            })
            .collect();
        
        SchemaOrg::FAQPage(FAQPage {
            context: Some("https://schema.org".to_string()),
            type_: Some("FAQPage".to_string()),
            mainEntity: Some(main_entity),
            additional: HashMap::new(),
        })
    }
    
    /// Create breadcrumb navigation
    pub fn breadcrumbs(items: &[(&str, &str)]) -> Self {
        let item_list: Vec<ListItem> = items
            .iter()
            .enumerate()
            .map(|(index, (name, url))| ListItem {
                type_: Some("ListItem".to_string()),
                position: Some((index + 1) as i32),
                name: Some(name.to_string()),
                item: Some(url.to_string()),
                ..Default::default()
            })
            .collect();
        
        SchemaOrg::BreadcrumbList(BreadcrumbList {
            context: Some("https://schema.org".to_string()),
            type_: Some("BreadcrumbList".to_string()),
            itemListElement: Some(item_list),
            additional: HashMap::new(),
        })
    }
    
    /// Convert to JSON-LD value
    pub fn to_json_ld(&self) -> crate::Result<JsonLd> {
        serde_json::to_value(self)
            .map_err(|e| crate::Error::SerializationError(e.to_string()))
    }
}

/// Validate JSON-LD structure
pub fn validate_json_ld(json_ld: &JsonLd) -> crate::Result<()> {
    // Basic validation - ensure we have @context and @type
    if let Some(obj) = json_ld.as_object() {
        if !obj.contains_key("@context") {
            return Err(crate::Error::ValidationError(
                "JSON-LD missing @context".to_string()
            ));
        }
        
        if !obj.contains_key("@type") {
            return Err(crate::Error::ValidationError(
                "JSON-LD missing @type".to_string()
            ));
        }
    } else {
        return Err(crate::Error::ValidationError(
            "JSON-LD must be an object".to_string()
        ));
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_web_page_schema() {
        let schema = SchemaOrg::web_page(
            "Test Page",
            Some("A test page"),
            Some("https://example.com/test")
        );
        
        let json_ld = schema.to_json_ld().unwrap();
        assert!(json_ld.get("@context").is_some());
        assert!(json_ld.get("@type").is_some());
        assert_eq!(json_ld.get("name").and_then(|v| v.as_str()), Some("Test Page"));
    }
    
    #[test]
    fn test_article_schema() {
        let schema = SchemaOrg::article(
            "Test Article",
            "John Doe",
            "2023-01-01",
            Some("A test article")
        );
        
        let json_ld = schema.to_json_ld().unwrap();
        assert!(json_ld.get("@context").is_some());
        assert_eq!(json_ld.get("@type").and_then(|v| v.as_str()), Some("Article"));
        assert_eq!(json_ld.get("headline").and_then(|v| v.as_str()), Some("Test Article"));
    }
    
    #[test]
    fn test_faq_schema() {
        let questions = &[
            ("What is this?", "This is a test"),
            ("Why test?", "Testing is important")
        ];
        
        let schema = SchemaOrg::faq_page(questions);
        let json_ld = schema.to_json_ld().unwrap();
        
        assert!(json_ld.get("@context").is_some());
        assert_eq!(json_ld.get("@type").and_then(|v| v.as_str()), Some("FAQPage"));
        assert!(json_ld.get("mainEntity").is_some());
    }
    
    #[test]
    fn test_validation() {
        let valid_json = serde_json::json!({
            "@context": "https://schema.org",
            "@type": "WebPage",
            "name": "Test"
        });
        
        assert!(validate_json_ld(&valid_json).is_ok());
        
        let invalid_json = serde_json::json!({
            "name": "Test"
        });
        
        assert!(validate_json_ld(&invalid_json).is_err());
    }
    
    #[test]
    fn test_organization_schema() {
        let schema = SchemaOrg::organization(
            "Test Corp",
            Some("https://example.com"),
            Some("https://example.com/logo.png")
        );
        
        let json_ld = schema.to_json_ld().unwrap();
        assert!(json_ld.get("@context").is_some());
        assert_eq!(json_ld.get("@type").and_then(|v| v.as_str()), Some("Organization"));
        assert_eq!(json_ld.get("name").and_then(|v| v.as_str()), Some("Test Corp"));
        assert_eq!(json_ld.get("url").and_then(|v| v.as_str()), Some("https://example.com"));
        assert_eq!(json_ld.get("logo").and_then(|v| v.as_str()), Some("https://example.com/logo.png"));
    }
    
    #[test]
    fn test_person_schema() {
        let schema = SchemaOrg::person(
            "Jane Smith",
            Some("Software Engineer"),
            Some("https://example.com/jane")
        );
        
        let json_ld = schema.to_json_ld().unwrap();
        assert!(json_ld.get("@context").is_some());
        assert_eq!(json_ld.get("@type").and_then(|v| v.as_str()), Some("Person"));
        assert_eq!(json_ld.get("name").and_then(|v| v.as_str()), Some("Jane Smith"));
        assert_eq!(json_ld.get("jobTitle").and_then(|v| v.as_str()), Some("Software Engineer"));
        assert_eq!(json_ld.get("url").and_then(|v| v.as_str()), Some("https://example.com/jane"));
    }
    
    #[test]
    fn test_breadcrumbs_schema() {
        let items = &[
            ("Home", "https://example.com"),
            ("Products", "https://example.com/products"),
            ("Category", "https://example.com/products/category")
        ];
        
        let schema = SchemaOrg::breadcrumbs(items);
        let json_ld = schema.to_json_ld().unwrap();
        
        assert!(json_ld.get("@context").is_some());
        assert_eq!(json_ld.get("@type").and_then(|v| v.as_str()), Some("BreadcrumbList"));
        assert!(json_ld.get("itemListElement").is_some());
        
        let item_list = json_ld.get("itemListElement").unwrap().as_array().unwrap();
        assert_eq!(item_list.len(), 3);
        assert_eq!(item_list[0].get("position").and_then(|v| v.as_i64()), Some(1));
        assert_eq!(item_list[0].get("name").and_then(|v| v.as_str()), Some("Home"));
    }
    
    #[test]
    fn test_validation_missing_context() {
        let invalid_json = serde_json::json!({
            "@type": "WebPage",
            "name": "Test"
        });
        
        let result = validate_json_ld(&invalid_json);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("@context"));
    }
    
    #[test]
    fn test_validation_missing_type() {
        let invalid_json = serde_json::json!({
            "@context": "https://schema.org",
            "name": "Test"
        });
        
        let result = validate_json_ld(&invalid_json);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("@type"));
    }
    
    #[test]
    fn test_validation_not_object() {
        let invalid_json = serde_json::json!("not an object");
        
        let result = validate_json_ld(&invalid_json);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("object"));
    }
    
    #[test]
    fn test_schema_org_enum_variants() {
        let web_page = SchemaOrg::web_page("Test", None, None);
        let article = SchemaOrg::article("Test", "Author", "2023-01-01", None);
        let organization = SchemaOrg::organization("Test Corp", None, None);
        let person = SchemaOrg::person("Test Person", None, None);
        let faq = SchemaOrg::faq_page(&[("Q", "A")]);
        let breadcrumbs = SchemaOrg::breadcrumbs(&[("Home", "/")]);
        
        // Test that all variants can be created and converted to JSON-LD
        assert!(web_page.to_json_ld().is_ok());
        assert!(article.to_json_ld().is_ok());
        assert!(organization.to_json_ld().is_ok());
        assert!(person.to_json_ld().is_ok());
        assert!(faq.to_json_ld().is_ok());
        assert!(breadcrumbs.to_json_ld().is_ok());
    }
}