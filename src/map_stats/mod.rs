//! Map stats module.

pub mod view;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapListEntry {
    pub title: String,
    pub count: usize,
    pub cache_stats: String,
}
