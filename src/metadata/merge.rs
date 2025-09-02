//! Metadata merging functionality for leptos-next-metadata
//! 
//! This module provides the logic for merging metadata from different
//! levels in the component hierarchy, following Next.js-style inheritance rules.

use super::*;
use crate::Result;

impl Metadata {
    /// Merge this metadata with another metadata instance
    /// 
    /// This follows Next.js-style merging rules:
    /// - Primitive fields (title, description) are replaced
    /// - Objects (openGraph, twitter) are replaced entirely, not deep merged
    /// - Arrays are replaced, not concatenated
    /// 
    /// # Arguments
    /// 
    /// * `other` - The metadata to merge with
    /// 
    /// # Returns
    /// 
    /// A new `Metadata` instance with the merged values
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use leptos_next_metadata::metadata::Metadata;
    /// 
    /// let base = Metadata::with_title("Base Title");
    /// let override_meta = Metadata::with_title("Override Title");
    /// 
    /// let merged = base.merge(&override_meta).unwrap();
    /// assert_eq!(merged.title.unwrap().to_string(), "Override Title");
    /// ```
    pub fn merge(&self, other: &Metadata) -> Result<Metadata> {
        let mut merged = self.clone();
        
        // Merge primitive fields (replace if present)
        if other.title.is_some() {
            merged.title = other.title.clone();
        }
        
        if other.description.is_some() {
            merged.description = other.description.clone();
        }
        
        if other.keywords.is_some() {
            merged.keywords = other.keywords.clone();
        }
        
        if other.authors.is_some() {
            merged.authors = other.authors.clone();
        }
        
        if other.robots.is_some() {
            merged.robots = other.robots.clone();
        }
        
        if other.canonical.is_some() {
            merged.canonical = other.canonical.clone();
        }
        
        if other.viewport.is_some() {
            merged.viewport = other.viewport.clone();
        }
        
        if other.theme_color.is_some() {
            merged.theme_color = other.theme_color.clone();
        }
        
        if other.color_scheme.is_some() {
            merged.color_scheme = other.color_scheme.clone();
        }
        
        if other.referrer.is_some() {
            merged.referrer = other.referrer.clone();
        }
        
        if other.format_detection.is_some() {
            merged.format_detection = other.format_detection.clone();
        }
        
        // Merge objects (replace entirely, not deep merge)
        if other.open_graph.is_some() {
            merged.open_graph = other.open_graph.clone();
        }
        
        if other.twitter.is_some() {
            merged.twitter = other.twitter.clone();
        }
        
        if other.json_ld.is_some() {
            merged.json_ld = other.json_ld.clone();
        }
        
        // Merge alternates (replace entirely)
        if other.alternates.is_some() {
            merged.alternates = other.alternates.clone();
        }
        
        // Merge additional fields
        for (key, value) in &other.additional {
            merged.additional.insert(key.clone(), value.clone());
        }
        
        Ok(merged)
    }
    
    /// Deep merge this metadata with another metadata instance
    /// 
    /// This provides more sophisticated merging than the standard merge:
    /// - Objects are deep merged where possible
    /// - Arrays can be concatenated or replaced based on configuration
    /// - Nested structures are preserved
    /// 
    /// # Arguments
    /// 
    /// * `other` - The metadata to merge with
    /// * `config` - Merge configuration options
    /// 
    /// # Returns
    /// 
    /// A new `Metadata` instance with the deep merged values
    pub fn deep_merge(&self, other: &Metadata, config: &MergeConfig) -> Result<Metadata> {
        let mut merged = self.clone();
        
        // Deep merge Open Graph
        if let (Some(ref mut self_og), Some(ref other_og)) = (merged.open_graph, other.open_graph.clone()) {
            *self_og = self_og.deep_merge(other_og, config)?;
        } else if other.open_graph.is_some() {
            merged.open_graph = other.open_graph.clone();
        }
        
        // Deep merge Twitter
        if let (Some(ref mut self_twitter), Some(ref other_twitter)) = (merged.twitter, other.twitter.clone()) {
            *self_twitter = self_twitter.deep_merge(other_twitter, config)?;
        } else if other.twitter.is_some() {
            merged.twitter = other.twitter.clone();
        }
        
        // Deep merge JSON-LD
        if let (Some(ref mut self_json_ld), Some(ref other_json_ld)) = (merged.json_ld, other.json_ld.clone()) {
            *self_json_ld = self_json_ld.deep_merge(other_json_ld, config)?;
        } else if other.json_ld.is_some() {
            merged.json_ld = other.json_ld.clone();
        }
        
        // Merge alternates with deep merge strategy
        if let (Some(ref mut self_alternates), Some(ref other_alternates)) = (merged.alternates, other.alternates.clone()) {
            for (key, other_link) in other_alternates {
                if let Some(self_link) = self_alternates.get_mut(&key) {
                    *self_link = self_link.merge(&other_link, config)?;
                } else {
                    self_alternates.insert(key, other_link);
                }
            }
        } else if other.alternates.is_some() {
            merged.alternates = other.alternates.clone();
        }
        
        // Apply standard merge for other fields
        merged = merged.merge(other)?;
        
        Ok(merged)
    }
}

impl OpenGraph {
    /// Deep merge this Open Graph metadata with another instance
    pub fn deep_merge(&self, other: OpenGraph, config: &MergeConfig) -> Result<OpenGraph> {
        let mut merged = self.clone();
        
        // Merge primitive fields
        if other.title.is_some() {
            merged.title = other.title;
        }
        
        if other.description.is_some() {
            merged.description = other.description;
        }
        
        if other.url.is_some() {
            merged.url = other.url;
        }
        
        if other.r#type.is_some() {
            merged.r#type = other.r#type;
        }
        
        if other.site_name.is_some() {
            merged.site_name = other.site_name;
        }
        
        if other.locale.is_some() {
            merged.locale = other.locale;
        }
        
        // Merge arrays based on configuration
        match config.array_merge_strategy {
            ArrayMergeStrategy::Replace => {
                merged.images = other.images;
                merged.videos = other.videos;
                merged.audio = other.audio;
            }
            ArrayMergeStrategy::Concat => {
                merged.images.extend(other.images);
                if let Some(other_videos) = other.videos {
                    if let Some(ref mut self_videos) = merged.videos {
                        self_videos.extend(other_videos);
                    } else {
                        merged.videos = Some(other_videos);
                    }
                }
                if let Some(other_audio) = other.audio {
                    if let Some(ref mut self_audio) = merged.audio {
                        self_audio.extend(other_audio);
                    } else {
                        merged.audio = Some(other_audio);
                    }
                }
            }
            ArrayMergeStrategy::Merge => {
                // Merge images by URL, keeping the most recent
                for other_image in other.images {
                    if let Some(existing_index) = merged.images.iter().position(|img| img.url == other_image.url) {
                        merged.images[existing_index] = other_image;
                    } else {
                        merged.images.push(other_image);
                    }
                }
                
                // Similar logic for videos and audio
                if let Some(other_videos) = other.videos {
                    if let Some(ref mut self_videos) = merged.videos {
                        for other_video in other_videos {
                            if let Some(existing_index) = self_videos.iter().position(|v| v.url == other_video.url) {
                                self_videos[existing_index] = other_video;
                            } else {
                                self_videos.push(other_video);
                            }
                        }
                    } else {
                        merged.videos = Some(other_videos);
                    }
                }
                
                if let Some(other_audio) = other.audio {
                    if let Some(ref mut self_audio) = merged.audio {
                        for other_audio_item in other_audio {
                            if let Some(existing_index) = self_audio.iter().position(|a| a.url == other_audio_item.url) {
                                self_audio[existing_index] = other_audio_item;
                            } else {
                                self_audio.push(other_audio_item);
                            }
                        }
                    } else {
                        merged.audio = Some(other_audio);
                    }
                }
            }
        }
        
        // Deep merge nested objects
        if let (Some(ref mut self_article), Some(other_article)) = (merged.article, other.article) {
            *self_article = self_article.merge(&other_article, config)?;
        } else if other.article.is_some() {
            merged.article = other.article;
        }
        
        if let (Some(ref mut self_profile), Some(other_profile)) = (merged.profile, other.profile) {
            *self_profile = self_profile.merge(&other_profile, config)?;
        } else if other.profile.is_some() {
            merged.profile = other.profile;
        }
        
        if let (Some(ref mut self_book), Some(other_book)) = (merged.book, other.book) {
            *self_book = self_book.merge(&other_book, config)?;
        } else if other.book.is_some() {
            merged.book = other.book;
        }
        
        // Merge additional properties
        for (key, value) in other.additional {
            merged.additional.insert(key, value);
        }
        
        Ok(merged)
    }
}

impl Twitter {
    /// Deep merge this Twitter metadata with another instance
    pub fn deep_merge(&self, other: Twitter, _config: &MergeConfig) -> Result<Twitter> {
        let mut merged = self.clone();
        
        // Twitter metadata is simple, just replace non-None values
        if other.card.is_some() {
            merged.card = other.card;
        }
        
        if other.site.is_some() {
            merged.site = other.site;
        }
        
        if other.creator.is_some() {
            merged.creator = other.creator;
        }
        
        if other.title.is_some() {
            merged.title = other.title;
        }
        
        if other.description.is_some() {
            merged.description = other.description;
        }
        
        if other.image.is_some() {
            merged.image = other.image;
        }
        
        if other.image_alt.is_some() {
            merged.image_alt = other.image_alt;
        }
        
        Ok(merged)
    }
}

impl Article {
    /// Merge this Article metadata with another instance
    pub fn merge(&self, other: &Article, _config: &MergeConfig) -> Result<Article> {
        let mut merged = self.clone();
        
        // Use the most recent timestamps
        if let Some(other_published) = other.published_time {
            if let Some(self_published) = merged.published_time {
                if other_published > self_published {
                    merged.published_time = Some(other_published);
                }
            } else {
                merged.published_time = Some(other_published);
            }
        }
        
        if let Some(other_modified) = other.modified_time {
            if let Some(self_modified) = merged.modified_time {
                if other_modified > self_modified {
                    merged.modified_time = Some(other_modified);
                }
            } else {
                merged.modified_time = Some(other_modified);
            }
        }
        
        if let Some(other_expiration) = other.expiration_time {
            if let Some(self_expiration) = merged.expiration_time {
                if other_expiration < self_expiration {
                    merged.expiration_time = Some(other_expiration);
                }
            } else {
                merged.expiration_time = Some(other_expiration);
            }
        }
        
        // Replace other fields if present
        if other.author.is_some() {
            merged.author = other.author.clone();
        }
        
        if other.section.is_some() {
            merged.section = other.section.clone();
        }
        
        if other.tags.is_some() {
            merged.tags = other.tags.clone();
        }
        
        Ok(merged)
    }
}

impl Profile {
    /// Merge this Profile metadata with another instance
    pub fn merge(&self, other: &Profile, _config: &MergeConfig) -> Result<Profile> {
        let mut merged = self.clone();
        
        // Replace non-None values
        if other.first_name.is_some() {
            merged.first_name = other.first_name.clone();
        }
        
        if other.last_name.is_some() {
            merged.last_name = other.last_name.clone();
        }
        
        if other.username.is_some() {
            merged.username = other.username.clone();
        }
        
        if other.gender.is_some() {
            merged.gender = other.gender.clone();
        }
        
        Ok(merged)
    }
}

impl Book {
    /// Merge this Book metadata with another instance
    pub fn merge(&self, other: &Book, _config: &MergeConfig) -> Result<Book> {
        let mut merged = self.clone();
        
        // Replace non-None values
        if other.author.is_some() {
            merged.author = other.author.clone();
        }
        
        if other.isbn.is_some() {
            merged.isbn = other.isbn.clone();
        }
        
        if other.release_date.is_some() {
            merged.release_date = other.release_date;
        }
        
        if other.tags.is_some() {
            merged.tags = other.tags.clone();
        }
        
        Ok(merged)
    }
}

impl AlternateLink {
    /// Merge this AlternateLink with another instance
    pub fn merge(&self, other: &AlternateLink, _config: &MergeConfig) -> Result<AlternateLink> {
        let mut merged = self.clone();
        
        // Replace non-None values
        if other.href != self.href {
            merged.href = other.href.clone();
        }
        
        if other.media.is_some() {
            merged.media = other.media.clone();
        }
        
        Ok(merged)
    }
}

impl serde_json::Value {
    /// Deep merge this JSON value with another
    pub fn deep_merge(&self, other: &serde_json::Value, _config: &MergeConfig) -> Result<serde_json::Value> {
        match (self, other) {
            (serde_json::Value::Object(self_obj), serde_json::Value::Object(other_obj)) => {
                let mut merged = self_obj.clone();
                
                for (key, value) in other_obj {
                    if let Some(existing_value) = merged.get(key) {
                        if existing_value.is_object() && value.is_object() {
                            merged[key] = existing_value.deep_merge(value, _config)?;
                        } else {
                            merged[key.clone()] = value.clone();
                        }
                    } else {
                        merged[key.clone()] = value.clone();
                    }
                }
                
                Ok(serde_json::Value::Object(merged))
            }
            _ => Ok(other.clone()),
        }
    }
}

/// Configuration for metadata merging behavior
#[derive(Debug, Clone)]
pub struct MergeConfig {
    /// Strategy for merging arrays
    pub array_merge_strategy: ArrayMergeStrategy,
    
    /// Whether to preserve parent metadata when merging
    pub preserve_parent: bool,
    
    /// Maximum depth for deep merging
    pub max_depth: usize,
}

/// Strategy for merging arrays during metadata combination
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArrayMergeStrategy {
    /// Replace arrays entirely
    Replace,
    
    /// Concatenate arrays
    Concat,
    
    /// Merge arrays by key (e.g., URL for images)
    Merge,
}

impl Default for MergeConfig {
    fn default() -> Self {
        Self {
            array_merge_strategy: ArrayMergeStrategy::Replace,
            preserve_parent: true,
            max_depth: 10,
        }
    }
}

impl Default for ArrayMergeStrategy {
    fn default() -> Self {
        ArrayMergeStrategy::Replace
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_metadata_merge() {
        let base = Metadata::with_title("Base Title");
        let override_meta = Metadata::with_title("Override Title");
        
        let merged = base.merge(&override_meta).unwrap();
        assert_eq!(merged.title.unwrap().to_string(), "Override Title");
    }
    
    #[test]
    fn test_metadata_merge_preserves_base() {
        let base = Metadata::with_title_and_description("Base Title", "Base Description");
        let override_meta = Metadata::with_title("Override Title");
        
        let merged = base.merge(&override_meta).unwrap();
        assert_eq!(merged.title.unwrap().to_string(), "Override Title");
        assert_eq!(merged.description.unwrap(), "Base Description");
    }
    
    #[test]
    fn test_open_graph_merge() {
        let base_og = OpenGraph {
            title: Some("Base OG Title".to_string()),
            images: vec![OgImage::new("/base-image.png")],
            ..Default::default()
        };
        
        let override_og = OpenGraph {
            title: Some("Override OG Title".to_string()),
            images: vec![OgImage::new("/override-image.png")],
            ..Default::default()
        };
        
        let base = Metadata::default().open_graph(base_og);
        let override_meta = Metadata::default().open_graph(override_og);
        
        let merged = base.merge(&override_meta).unwrap();
        let og = merged.open_graph.unwrap();
        
        assert_eq!(og.title.unwrap(), "Override OG Title");
        assert_eq!(og.images.len(), 1);
        assert_eq!(og.images[0].url, "/override-image.png");
    }
    
    #[test]
    fn test_deep_merge_with_concat_strategy() {
        let config = MergeConfig {
            array_merge_strategy: ArrayMergeStrategy::Concat,
            ..Default::default()
        };
        
        let base_og = OpenGraph {
            images: vec![OgImage::new("/base-image.png")],
            ..Default::default()
        };
        
        let override_og = OpenGraph {
            images: vec![OgImage::new("/override-image.png")],
            ..Default::default()
        };
        
        let base = Metadata::default().open_graph(base_og);
        let override_meta = Metadata::default().open_graph(override_og);
        
        let merged = base.deep_merge(&override_meta, &config).unwrap();
        let og = merged.open_graph.unwrap();
        
        assert_eq!(og.images.len(), 2);
        assert_eq!(og.images[0].url, "/base-image.png");
        assert_eq!(og.images[1].url, "/override-image.png");
    }
    
    #[test]
    fn test_robots_merge() {
        let base = Metadata::default().robots(Robots::all());
        let override_meta = Metadata::default().robots(Robots::noindex());
        
        let merged = base.merge(&override_meta).unwrap();
        let robots = merged.robots.unwrap();
        
        assert!(!robots.index);
        assert!(robots.follow);
    }
}
