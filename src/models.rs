use serde::Deserialize;
use std::collections::HashMap;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Deserialize)]
pub struct Crate {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub license: Option<String>,
    pub documentation: Option<String>,
    pub homepage: Option<String>,
    pub repository: Option<String>,
    pub downloads: u64,
    pub recent_downloads: Option<u64>,
    pub categories: Option<Vec<String>>,
    pub keywords: Option<Vec<String>>,
    pub versions: Option<Vec<u64>>,
    pub max_version: String,
    pub max_stable_version: Option<String>,
    pub links: HashMap<String, String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub exact_match: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ApiResponse {
    pub crates: Vec<Crate>,
}

#[derive(Debug)]
pub struct Length {
    pub id: usize,
    pub name: usize,
    pub max_version: usize,
    pub max_stable_version: usize,
    pub updated_at: usize,
}
