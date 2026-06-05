//! Shared types used by both the build (SSR) and the blog-filter island (CSR).
//! `BlogPostSummary` is serialized into the blog page and deserialized by the island.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Axes {
    pub physician: i64,
    pub engineer: i64,
    pub life: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlogPostSummary {
    pub slug: String,
    pub title: String,
    pub date: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub axes: Option<Axes>,
    pub formatted_date: String,
}
