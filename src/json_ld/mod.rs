//! JSON-LD structured data for leptos-next-metadata
//! 
//! This module provides type-safe JSON-LD generation for Schema.org types,
//! enabling rich snippets and better search engine understanding.

use crate::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// JSON-LD structured data
pub type JsonLd = Value;

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
    
    /// Event
    Event(Event),
    
    /// Recipe
    Recipe(Recipe),
    
    /// Review
    Review(Review),
    
    /// Local business
    LocalBusiness(LocalBusiness),
    
    /// FAQ page
    FAQPage(FAQPage),
    
    /// How-to
    HowTo(HowTo),
    
    /// Custom schema
    Custom(CustomSchema),
}

/// Web page schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebPage {
    #[serde(rename = "@context")]
    pub context: String,
    
    #[serde(rename = "@type")]
    pub r#type: String,
    
    pub name: String,
    
    pub description: Option<String>,
    
    pub url: Option<String>,
    
    pub mainEntity: Option<Box<SchemaOrg>>,
    
    pub breadcrumb: Option<BreadcrumbList>,
    
    #[serde(flatten)]
    pub additional: std::collections::HashMap<String, Value>,
}

/// Article schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Article {
    #[serde(rename = "@context")]
    pub context: String,
    
    #[serde(rename = "@type")]
    pub r#type: String,
    
    pub headline: String,
    
    pub description: Option<String>,
    
    pub image: Option<Vec<String>>,
    
    pub author: Option<Person>,
    
    pub publisher: Option<Organization>,
    
    pub datePublished: Option<String>,
    
    pub dateModified: Option<String>,
    
    pub articleBody: Option<String>,
    
    pub articleSection: Option<String>,
    
    pub keywords: Option<Vec<String>>,
    
    #[serde(flatten)]
    pub additional: std::collections::HashMap<String, Value>,
}

/// Blog post schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogPosting {
    #[serde(rename = "@context")]
    pub context: String,
    
    #[serde(rename = "@type")]
    pub r#type: String,
    
    pub headline: String,
    
    pub description: Option<String>,
    
    pub image: Option<Vec<String>>,
    
    pub author: Option<Person>,
    
    pub publisher: Option<Organization>,
    
    pub datePublished: Option<String>,
    
    pub dateModified: Option<String>,
    
    pub articleBody: Option<String>,
    
    pub blogPost: Option<Vec<BlogPosting>>,
    
    #[serde(flatten)]
    pub additional: std::collections::HashMap<String, Value>,
}

/// Product schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    #[serde(rename = "@context")]
    pub context: String,
    
    #[serde(rename = "@type")]
    pub r#type: String,
    
    pub name: String,
    
    pub description: Option<String>,
    
    pub image: Option<Vec<String>>,
    
    pub brand: Option<Brand>,
    
    pub offers: Option<Offer>,
    
    pub aggregateRating: Option<AggregateRating>,
    
    pub review: Option<Vec<Review>>,
    
    pub category: Option<String>,
    
    pub sku: Option<String>,
    
    pub mpn: Option<String>,
    
    pub gtin: Option<String>,
    
    #[serde(flatten)]
    pub additional: std::collections::HashMap<String, Value>,
}

/// Organization schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Organization {
    #[serde(rename = "@context")]
    pub context: String,
    
    #[serde(rename = "@type")]
    pub r#type: String,
    
    pub name: String,
    
    pub description: Option<String>,
    
    pub url: Option<String>,
    
    pub logo: Option<String>,
    
    pub sameAs: Option<Vec<String>>,
    
    pub contactPoint: Option<Vec<ContactPoint>>,
    
    pub address: Option<PostalAddress>,
    
    #[serde(flatten)]
    pub additional: std::collections::HashMap<String, Value>,
}

/// Person schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Person {
    #[serde(rename = "@context")]
    pub context: String,
    
    #[serde(rename = "@type")]
    pub r#type: String,
    
    pub name: String,
    
    pub description: Option<String>,
    
    pub image: Option<String>,
    
    pub url: Option<String>,
    
    pub sameAs: Option<Vec<String>>,
    
    pub jobTitle: Option<String>,
    
    pub worksFor: Option<Organization>,
    
    pub alumniOf: Option<Organization>,
    
    #[serde(flatten)]
    pub additional: std::collections::HashMap<String, Value>,
}

/// Event schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    #[serde(rename = "@context")]
    pub context: String,
    
    #[serde(rename = "@type")]
    pub r#type: String,
    
    pub name: String,
    
    pub description: Option<String>,
    
    pub startDate: String,
    
    pub endDate: Option<String>,
    
    pub location: Option<Place>,
    
    pub organizer: Option<Organization>,
    
    pub performer: Option<Vec<Person>>,
    
    pub offers: Option<Offer>,
    
    pub eventStatus: Option<String>,
    
    #[serde(flatten)]
    pub additional: std::collections::HashMap<String, Value>,
}

/// Recipe schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recipe {
    #[serde(rename = "@context")]
    pub context: String,
    
    #[serde(rename = "@type")]
    pub r#type: String,
    
    pub name: String,
    
    pub description: Option<String>,
    
    pub image: Option<Vec<String>>,
    
    pub author: Option<Person>,
    
    pub datePublished: Option<String>,
    
    pub prepTime: Option<String>,
    
    pub cookTime: Option<String>,
    
    pub totalTime: Option<String>,
    
    pub recipeYield: Option<String>,
    
    pub recipeCategory: Option<Vec<String>>,
    
    pub recipeCuisine: Option<Vec<String>>,
    
    pub recipeIngredient: Option<Vec<String>>,
    
    pub recipeInstructions: Option<Vec<RecipeInstruction>>,
    
    pub nutrition: Option<NutritionInformation>,
    
    #[serde(flatten)]
    pub additional: std::collections::HashMap<String, Value>,
}

/// Review schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Review {
    #[serde(rename = "@context")]
    pub context: String,
    
    #[serde(rename = "@type")]
    pub r#type: String,
    
    pub reviewRating: Rating,
    
    pub author: Option<Person>,
    
    pub reviewBody: Option<String>,
    
    pub datePublished: Option<String>,
    
    pub itemReviewed: Option<Box<SchemaOrg>>,
    
    #[serde(flatten)]
    pub additional: std::collections::HashMap<String, Value>,
}

/// Local business schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalBusiness {
    #[serde(rename = "@context")]
    pub context: String,
    
    #[serde(rename = "@type")]
    pub r#type: String,
    
    pub name: String,
    
    pub description: Option<String>,
    
    pub url: Option<String>,
    
    pub telephone: Option<String>,
    
    pub address: PostalAddress,
    
    pub geo: Option<GeoCoordinates>,
    
    pub openingHours: Option<Vec<String>>,
    
    pub priceRange: Option<String>,
    
    pub servesCuisine: Option<Vec<String>>,
    
    #[serde(flatten)]
    pub additional: std::collections::HashMap<String, Value>,
}

/// FAQ page schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FAQPage {
    #[serde(rename = "@context")]
    pub context: String,
    
    #[serde(rename = "@type")]
    pub r#type: String,
    
    pub mainEntity: Vec<Question>,
    
    #[serde(flatten)]
    pub additional: std::collections::HashMap<String, Value>,
}

/// How-to schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HowTo {
    #[serde(rename = "@context")]
    pub context: String,
    
    #[serde(rename = "@type")]
    pub r#type: String,
    
    pub name: String,
    
    pub description: Option<String>,
    
    pub image: Option<Vec<String>>,
    
    pub step: Vec<HowToStep>,
    
    pub totalTime: Option<String>,
    
    pub tool: Option<Vec<String>>,
    
    pub supply: Option<Vec<String>>,
    
    #[serde(flatten)]
    pub additional: std::collections::HashMap<String, Value>,
}

/// Custom schema for flexible use
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomSchema {
    #[serde(rename = "@context")]
    pub context: String,
    
    #[serde(rename = "@type")]
    pub r#type: String,
    
    #[serde(flatten)]
    pub properties: std::collections::HashMap<String, Value>,
}

// Supporting types

/// Brand schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Brand {
    #[serde(rename = "@type")]
    pub r#type: String,
    
    pub name: String,
    
    pub logo: Option<String>,
}

/// Offer schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Offer {
    #[serde(rename = "@type")]
    pub r#type: String,
    
    pub price: f64,
    
    pub priceCurrency: String,
    
    pub availability: String,
    
    pub url: Option<String>,
    
    pub seller: Option<Organization>,
}

/// Aggregate rating schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregateRating {
    #[serde(rename = "@type")]
    pub r#type: String,
    
    pub ratingValue: f64,
    
    pub reviewCount: u32,
    
    pub bestRating: Option<f64>,
    
    pub worstRating: Option<f64>,
}

/// Rating schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rating {
    #[serde(rename = "@type")]
    pub r#type: String,
    
    pub ratingValue: f64,
    
    pub bestRating: Option<f64>,
    
    pub worstRating: Option<f64>,
}

/// Contact point schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactPoint {
    #[serde(rename = "@type")]
    pub r#type: String,
    
    pub telephone: Option<String>,
    
    pub email: Option<String>,
    
    pub contactType: Option<String>,
}

/// Postal address schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostalAddress {
    #[serde(rename = "@type")]
    pub r#type: String,
    
    pub streetAddress: String,
    
    pub addressLocality: String,
    
    pub addressRegion: Option<String>,
    
    pub postalCode: String,
    
    pub addressCountry: String,
}

/// Place schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Place {
    #[serde(rename = "@type")]
    pub r#type: String,
    
    pub name: String,
    
    pub address: PostalAddress,
    
    pub geo: Option<GeoCoordinates>,
}

/// Geo coordinates schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoCoordinates {
    #[serde(rename = "@type")]
    pub r#type: String,
    
    pub latitude: f64,
    
    pub longitude: f64,
}

/// Breadcrumb list schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreadcrumbList {
    #[serde(rename = "@context")]
    pub context: String,
    
    #[serde(rename = "@type")]
    pub r#type: String,
    
    pub itemListElement: Vec<ListItem>,
}

/// List item schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListItem {
    #[serde(rename = "@type")]
    pub r#type: String,
    
    pub position: u32,
    
    pub name: String,
    
    pub item: String,
}

/// Question schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Question {
    #[serde(rename = "@type")]
    pub r#type: String,
    
    pub name: String,
    
    pub acceptedAnswer: Answer,
}

/// Answer schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Answer {
    #[serde(rename = "@type")]
    pub r#type: String,
    
    pub text: String,
}

/// How-to step schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HowToStep {
    #[serde(rename = "@type")]
    pub r#type: String,
    
    pub name: String,
    
    pub text: String,
    
    pub image: Option<String>,
    
    pub url: Option<String>,
}

/// Recipe instruction schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipeInstruction {
    #[serde(rename = "@type")]
    pub r#type: String,
    
    pub text: String,
}

/// Nutrition information schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NutritionInformation {
    #[serde(rename = "@type")]
    pub r#type: String,
    
    pub calories: Option<String>,
    
    pub carbohydrateContent: Option<String>,
    
    pub proteinContent: Option<String>,
    
    pub fatContent: Option<String>,
}

impl Default for WebPage {
    fn default() -> Self {
        Self {
            context: "https://schema.org".to_string(),
            r#type: "WebPage".to_string(),
            name: String::new(),
            description: None,
            url: None,
            mainEntity: None,
            breadcrumb: None,
            additional: std::collections::HashMap::new(),
        }
    }
}

impl Default for Article {
    fn default() -> Self {
        Self {
            context: "https://schema.org".to_string(),
            r#type: "Article".to_string(),
            headline: String::new(),
            description: None,
            image: None,
            author: None,
            publisher: None,
            datePublished: None,
            dateModified: None,
            articleBody: None,
            articleSection: None,
            keywords: None,
            additional: std::collections::HashMap::new(),
        }
    }
}

impl Default for Product {
    fn default() -> Self {
        Self {
            context: "https://schema.org".to_string(),
            r#type: "Product".to_string(),
            name: String::new(),
            description: None,
            image: None,
            brand: None,
            offers: None,
            aggregateRating: None,
            review: None,
            category: None,
            sku: None,
            mpn: None,
            gtin: None,
            additional: std::collections::HashMap::new(),
        }
    }
}

impl Default for Organization {
    fn default() -> Self {
        Self {
            context: "https://schema.org".to_string(),
            r#type: "Organization".to_string(),
            name: String::new(),
            description: None,
            url: None,
            logo: None,
            sameAs: None,
            contactPoint: None,
            address: None,
            additional: std::collections::HashMap::new(),
        }
    }
}

impl Default for Person {
    fn default() -> Self {
        Self {
            context: "https://schema.org".to_string(),
            r#type: "Person".to_string(),
            name: String::new(),
            description: None,
            image: None,
            url: None,
            sameAs: None,
            jobTitle: None,
            worksFor: None,
            alumniOf: None,
            additional: std::collections::HashMap::new(),
        }
    }
}

impl Default for Event {
    fn default() -> Self {
        Self {
            context: "https://schema.org".to_string(),
            r#type: "Event".to_string(),
            name: String::new(),
            description: None,
            startDate: String::new(),
            endDate: None,
            location: None,
            organizer: None,
            performer: None,
            offers: None,
            eventStatus: None,
            additional: std::collections::HashMap::new(),
        }
    }
}

impl Default for Recipe {
    fn default() -> Self {
        Self {
            context: "https://schema.org".to_string(),
            r#type: "Recipe".to_string(),
            name: String::new(),
            description: None,
            image: None,
            author: None,
            datePublished: None,
            prepTime: None,
            cookTime: None,
            totalTime: None,
            recipeYield: None,
            recipeCategory: None,
            recipeCuisine: None,
            recipeIngredient: None,
            recipeInstructions: None,
            nutrition: None,
            additional: std::collections::HashMap::new(),
        }
    }
}

impl Default for Review {
    fn default() -> Self {
        Self {
            context: "https://schema.org".to_string(),
            r#type: "Review".to_string(),
            reviewRating: Rating::default(),
            author: None,
            reviewBody: None,
            datePublished: None,
            itemReviewed: None,
            additional: std::collections::HashMap::new(),
        }
    }
}

impl Default for LocalBusiness {
    fn default() -> Self {
        Self {
            context: "https://schema.org".to_string(),
            r#type: "LocalBusiness".to_string(),
            name: String::new(),
            description: None,
            url: None,
            telephone: None,
            address: PostalAddress::default(),
            geo: None,
            openingHours: None,
            priceRange: None,
            servesCuisine: None,
            additional: std::collections::HashMap::new(),
        }
    }
}

impl Default for FAQPage {
    fn default() -> Self {
        Self {
            context: "https://schema.org".to_string(),
            r#type: "FAQPage".to_string(),
            mainEntity: Vec::new(),
            additional: std::collections::HashMap::new(),
        }
    }
}

impl Default for HowTo {
    fn default() -> Self {
        Self {
            context: "https://schema.org".to_string(),
            r#type: "HowTo".to_string(),
            name: String::new(),
            description: None,
            image: None,
            step: Vec::new(),
            totalTime: None,
            tool: None,
            supply: None,
            additional: std::collections::HashMap::new(),
        }
    }
}

impl Default for CustomSchema {
    fn default() -> Self {
        Self {
            context: "https://schema.org".to_string(),
            r#type: String::new(),
            properties: std::collections::HashMap::new(),
        }
    }
}

impl Default for Brand {
    fn default() -> Self {
        Self {
            r#type: "Brand".to_string(),
            name: String::new(),
            logo: None,
        }
    }
}

impl Default for Offer {
    fn default() -> Self {
        Self {
            r#type: "Offer".to_string(),
            price: 0.0,
            priceCurrency: "USD".to_string(),
            availability: "https://schema.org/InStock".to_string(),
            url: None,
            seller: None,
        }
    }
}

impl Default for AggregateRating {
    fn default() -> Self {
        Self {
            r#type: "AggregateRating".to_string(),
            ratingValue: 0.0,
            reviewCount: 0,
            bestRating: Some(5.0),
            worstRating: Some(1.0),
        }
    }
}

impl Default for Rating {
    fn default() -> Self {
        Self {
            r#type: "Rating".to_string(),
            ratingValue: 0.0,
            bestRating: Some(5.0),
            worstRating: Some(1.0),
        }
    }
}

impl Default for ContactPoint {
    fn default() -> Self {
        Self {
            r#type: "ContactPoint".to_string(),
            telephone: None,
            email: None,
            contactType: None,
        }
    }
}

impl Default for PostalAddress {
    fn default() -> Self {
        Self {
            r#type: "PostalAddress".to_string(),
            streetAddress: String::new(),
            addressLocality: String::new(),
            addressRegion: None,
            postalCode: String::new(),
            addressCountry: String::new(),
        }
    }
}

impl Default for Place {
    fn default() -> Self {
        Self {
            r#type: "Place".to_string(),
            name: String::new(),
            address: PostalAddress::default(),
            geo: None,
        }
    }
}

impl Default for GeoCoordinates {
    fn default() -> Self {
        Self {
            r#type: "GeoCoordinates".to_string(),
            latitude: 0.0,
            longitude: 0.0,
        }
    }
}

impl Default for BreadcrumbList {
    fn default() -> Self {
        Self {
            context: "https://schema.org".to_string(),
            r#type: "BreadcrumbList".to_string(),
            itemListElement: Vec::new(),
        }
    }
}

impl Default for ListItem {
    fn default() -> Self {
        Self {
            r#type: "ListItem".to_string(),
            position: 0,
            name: String::new(),
            item: String::new(),
        }
    }
}

impl Default for Question {
    fn default() -> Self {
        Self {
            r#type: "Question".to_string(),
            name: String::new(),
            acceptedAnswer: Answer::default(),
        }
    }
}

impl Default for Answer {
    fn default() -> Self {
        Self {
            r#type: "Answer".to_string(),
            text: String::new(),
        }
    }
}

impl Default for HowToStep {
    fn default() -> Self {
        Self {
            r#type: "HowToStep".to_string(),
            name: String::new(),
            text: String::new(),
            image: None,
            url: None,
        }
    }
}

impl Default for RecipeInstruction {
    fn default() -> Self {
        Self {
            r#type: "HowToStep".to_string(),
            text: String::new(),
        }
    }
}

impl Default for NutritionInformation {
    fn default() -> Self {
        Self {
            r#type: "NutritionInformation".to_string(),
            calories: None,
            carbohydrateContent: None,
            proteinContent: None,
            fatContent: None,
        }
    }
}

/// Utility functions for JSON-LD
pub mod utils {
    use super::*;
    
    /// Create a WebPage schema
    pub fn create_webpage(name: &str, description: Option<&str>, url: Option<&str>) -> WebPage {
        WebPage {
            name: name.to_string(),
            description: description.map(|s| s.to_string()),
            url: url.map(|s| s.to_string()),
            ..Default::default()
        }
    }
    
    /// Create an Article schema
    pub fn create_article(
        headline: &str,
        description: Option<&str>,
        author: Option<Person>,
        publisher: Option<Organization>,
    ) -> Article {
        Article {
            headline: headline.to_string(),
            description: description.map(|s| s.to_string()),
            author,
            publisher,
            ..Default::default()
        }
    }
    
    /// Create a Product schema
    pub fn create_product(
        name: &str,
        description: Option<&str>,
        price: f64,
        currency: &str,
    ) -> Product {
        let offer = Offer {
            price,
            priceCurrency: currency.to_string(),
            ..Default::default()
        };
        
        Product {
            name: name.to_string(),
            description: description.map(|s| s.to_string()),
            offers: Some(offer),
            ..Default::default()
        }
    }
    
    /// Create an Organization schema
    pub fn create_organization(
        name: &str,
        description: Option<&str>,
        url: Option<&str>,
        logo: Option<&str>,
    ) -> Organization {
        Organization {
            name: name.to_string(),
            description: description.map(|s| s.to_string()),
            url: url.map(|s| s.to_string()),
            logo: logo.map(|s| s.to_string()),
            ..Default::default()
        }
    }
    
    /// Create a Person schema
    pub fn create_person(
        name: &str,
        description: Option<&str>,
        job_title: Option<&str>,
        organization: Option<Organization>,
    ) -> Person {
        Person {
            name: name.to_string(),
            description: description.map(|s| s.to_string()),
            jobTitle: job_title.map(|s| s.to_string()),
            worksFor: organization,
            ..Default::default()
        }
    }
    
    /// Create an Event schema
    pub fn create_event(
        name: &str,
        description: Option<&str>,
        start_date: &str,
        end_date: Option<&str>,
        location: Option<Place>,
    ) -> Event {
        Event {
            name: name.to_string(),
            description: description.map(|s| s.to_string()),
            startDate: start_date.to_string(),
            endDate: end_date.map(|s| s.to_string()),
            location,
            ..Default::default()
        }
    }
    
    /// Create a Recipe schema
    pub fn create_recipe(
        name: &str,
        description: Option<&str>,
        ingredients: Vec<String>,
        instructions: Vec<String>,
        prep_time: Option<&str>,
        cook_time: Option<&str>,
    ) -> Recipe {
        let recipe_instructions: Vec<RecipeInstruction> = instructions
            .into_iter()
            .map(|text| RecipeInstruction {
                text,
                ..Default::default()
            })
            .collect();
        
        Recipe {
            name: name.to_string(),
            description: description.map(|s| s.to_string()),
            recipeIngredient: Some(ingredients),
            recipeInstructions: Some(recipe_instructions),
            prepTime: prep_time.map(|s| s.to_string()),
            cookTime: cook_time.map(|s| s.to_string()),
            ..Default::default()
        }
    }
    
    /// Create a Review schema
    pub fn create_review(
        rating_value: f64,
        review_body: Option<&str>,
        author: Option<Person>,
        item_reviewed: Option<Box<SchemaOrg>>,
    ) -> Review {
        let review_rating = Rating {
            ratingValue: rating_value,
            ..Default::default()
        };
        
        Review {
            reviewRating: review_rating,
            reviewBody: review_body.map(|s| s.to_string()),
            author,
            itemReviewed: item_reviewed,
            ..Default::default()
        }
    }
    
    /// Create a LocalBusiness schema
    pub fn create_local_business(
        name: &str,
        description: Option<&str>,
        telephone: Option<&str>,
        address: PostalAddress,
        opening_hours: Option<Vec<String>>,
    ) -> LocalBusiness {
        LocalBusiness {
            name: name.to_string(),
            description: description.map(|s| s.to_string()),
            telephone: telephone.map(|s| s.to_string()),
            address,
            openingHours: opening_hours,
            ..Default::default()
        }
    }
    
    /// Create an FAQPage schema
    pub fn create_faq_page(questions: Vec<(String, String)>) -> FAQPage {
        let main_entity: Vec<Question> = questions
            .into_iter()
            .map(|(question, answer)| Question {
                name: question,
                acceptedAnswer: Answer {
                    text: answer,
                    ..Default::default()
                },
                ..Default::default()
            })
            .collect();
        
        FAQPage {
            mainEntity,
            ..Default::default()
        }
    }
    
    /// Create a HowTo schema
    pub fn create_how_to(
        name: &str,
        description: Option<&str>,
        steps: Vec<String>,
        total_time: Option<&str>,
    ) -> HowTo {
        let step: Vec<HowToStep> = steps
            .into_iter()
            .map(|text| HowToStep {
                text,
                ..Default::default()
            })
            .collect();
        
        HowTo {
            name: name.to_string(),
            description: description.map(|s| s.to_string()),
            step,
            totalTime: total_time.map(|s| s.to_string()),
            ..Default::default()
        }
    }
    
    /// Convert a SchemaOrg type to JSON-LD
    pub fn to_json_ld(schema: &SchemaOrg) -> Result<JsonLd> {
        serde_json::to_value(schema)
            .map_err(|e| crate::Error::SerializationError(e))
    }
    
    /// Validate JSON-LD against Schema.org
    pub fn validate_schema_org(json_ld: &JsonLd) -> Result<()> {
        // Basic validation - check for required @context and @type
        if let Some(context) = json_ld.get("@context") {
            if let Some(context_str) = context.as_str() {
                if context_str != "https://schema.org" {
                    return Err(crate::Error::ValidationError(
                        "Invalid @context, must be 'https://schema.org'".to_string()
                    ));
                }
            } else {
                return Err(crate::Error::ValidationError(
                    "@context must be a string".to_string()
                ));
            }
        } else {
            return Err(crate::Error::ValidationError(
                "Missing @context".to_string()
            ));
        }
        
        if let Some(r#type) = json_ld.get("@type") {
            if !r#type.is_string() {
                return Err(crate::Error::ValidationError(
                    "@type must be a string".to_string()
                ));
            }
        } else {
            return Err(crate::Error::ValidationError(
                "Missing @type".to_string()
            ));
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_webpage_creation() {
        let webpage = utils::create_webpage("Test Page", Some("Test Description"), Some("https://example.com"));
        
        assert_eq!(webpage.name, "Test Page");
        assert_eq!(webpage.description, Some("Test Description".to_string()));
        assert_eq!(webpage.url, Some("https://example.com".to_string()));
        assert_eq!(webpage.r#type, "WebPage");
        assert_eq!(webpage.context, "https://schema.org");
    }
    
    #[test]
    fn test_article_creation() {
        let author = utils::create_person("John Doe", None, None, None);
        let publisher = utils::create_organization("Test Org", None, None, None);
        
        let article = utils::create_article("Test Article", Some("Test Description"), Some(author), Some(publisher));
        
        assert_eq!(article.headline, "Test Article");
        assert_eq!(article.description, Some("Test Description".to_string()));
        assert!(article.author.is_some());
        assert!(article.publisher.is_some());
    }
    
    #[test]
    fn test_product_creation() {
        let product = utils::create_product("Test Product", Some("Test Description"), 29.99, "USD");
        
        assert_eq!(product.name, "Test Product");
        assert_eq!(product.description, Some("Test Description".to_string()));
        assert!(product.offers.is_some());
        
        if let Some(offer) = product.offers {
            assert_eq!(offer.price, 29.99);
            assert_eq!(offer.priceCurrency, "USD");
        }
    }
    
    #[test]
    fn test_recipe_creation() {
        let ingredients = vec!["Flour".to_string(), "Sugar".to_string()];
        let instructions = vec!["Mix ingredients".to_string(), "Bake".to_string()];
        
        let recipe = utils::create_recipe(
            "Test Recipe",
            Some("Test Description"),
            ingredients.clone(),
            instructions.clone(),
            Some("10 minutes"),
            Some("30 minutes"),
        );
        
        assert_eq!(recipe.name, "Test Recipe");
        assert_eq!(recipe.recipeIngredient, Some(ingredients));
        assert_eq!(recipe.recipeInstructions.unwrap().len(), 2);
        assert_eq!(recipe.prepTime, Some("10 minutes".to_string()));
        assert_eq!(recipe.cookTime, Some("30 minutes".to_string()));
    }
    
    #[test]
    fn test_json_ld_conversion() {
        let webpage = utils::create_webpage("Test", None, None);
        let json_ld = utils::to_json_ld(&SchemaOrg::WebPage(webpage)).unwrap();
        
        assert!(json_ld.is_object());
        assert_eq!(json_ld["@context"], "https://schema.org");
        assert_eq!(json_ld["@type"], "WebPage");
    }
    
    #[test]
    fn test_schema_validation() {
        let valid_json = serde_json::json!({
            "@context": "https://schema.org",
            "@type": "WebPage",
            "name": "Test"
        });
        
        assert!(utils::validate_schema_org(&valid_json).is_ok());
        
        let invalid_json = serde_json::json!({
            "@type": "WebPage",
            "name": "Test"
        });
        
        assert!(utils::validate_schema_org(&invalid_json).is_err());
    }
}
