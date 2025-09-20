//! Twitter Card metadata types
//!
//! This module contains all Twitter Card related types and structures.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Twitter Card metadata
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Twitter {
    /// Twitter card type
    pub card: Option<TwitterCard>,
    /// Twitter site handle
    pub site: Option<String>,
    /// Twitter site ID
    pub site_id: Option<String>,
    /// Twitter creator handle
    pub creator: Option<String>,
    /// Twitter creator ID
    pub creator_id: Option<String>,
    /// Twitter title
    pub title: Option<String>,
    /// Twitter description
    pub description: Option<String>,
    /// Twitter image
    pub image: Option<String>,
    /// Twitter image alt text
    pub image_alt: Option<String>,
    /// Twitter player
    pub player: Option<String>,
    /// Twitter player width
    pub player_width: Option<u32>,
    /// Twitter player height
    pub player_height: Option<u32>,
    /// Twitter player stream
    pub player_stream: Option<String>,
    /// Twitter app name (iPhone)
    pub app_name_iphone: Option<String>,
    /// Twitter app ID (iPhone)
    pub app_id_iphone: Option<String>,
    /// Twitter app URL (iPhone)
    pub app_url_iphone: Option<String>,
    /// Twitter app name (iPad)
    pub app_name_ipad: Option<String>,
    /// Twitter app ID (iPad)
    pub app_id_ipad: Option<String>,
    /// Twitter app URL (iPad)
    pub app_url_ipad: Option<String>,
    /// Twitter app name (Google Play)
    pub app_name_googleplay: Option<String>,
    /// Twitter app ID (Google Play)
    pub app_id_googleplay: Option<String>,
    /// Twitter app URL (Google Play)
    pub app_url_googleplay: Option<String>,
    /// Additional Twitter properties
    pub other: HashMap<String, String>,
}

/// Twitter Card types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TwitterCard {
    /// Summary card
    Summary,
    /// Summary card with large image
    SummaryLargeImage,
    /// App card
    App,
    /// Player card
    Player,
}
