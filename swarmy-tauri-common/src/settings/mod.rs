//! Module for application settings management.
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct AppSettings {
    pub disable_parallel_scans: bool,
    pub replay_paths: Vec<String>,
}
