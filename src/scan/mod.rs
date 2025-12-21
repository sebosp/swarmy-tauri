//! SC2Replay Directory Scan and Export to Arrow IPC Module

use reactive_stores::Store;
use s2protocol::cli::SC2ReplaysDirStats;
use serde::{Deserialize, Serialize};
pub mod view;
pub use view::*;
pub mod mpq_file_scan;
pub use mpq_file_scan::*;

#[derive(Store, Debug, Clone, Serialize, Deserialize)]
pub struct SC2ReplaysDirStatsTable {
    pub total_files: usize,
    pub total_supported_replays: usize,
    pub ability_supported_replays: usize,
    #[store(key: String = |row| row.name.clone())]
    pub top_10_players: Vec<SC2ReplaysDirPlayerEntry>,
    #[store(key: String = |row| row.title.clone())]
    pub top_10_maps: Vec<SC2ReplaysDirMapEntry>,
}

#[derive(Store, Debug, Clone, Serialize, Deserialize)]
pub struct SC2ReplaysDirPlayerEntry {
    pub idx: usize,
    pub clan: String,
    pub name: String,
    pub count: usize,
}

#[derive(Store, Debug, Clone, Serialize, Deserialize)]
pub struct SC2ReplaysDirMapEntry {
    pub idx: usize,
    pub title: String,
    pub count: usize,
}

impl From<SC2ReplaysDirStats> for SC2ReplaysDirStatsTable {
    fn from(stats: SC2ReplaysDirStats) -> Self {
        Self {
            total_files: stats.total_files,
            total_supported_replays: stats.total_supported_replays,
            ability_supported_replays: stats.ability_supported_replays,
            top_10_players: stats
                .top_10_players
                .into_iter()
                .enumerate()
                .map(|(idx, (name, count))| {
                    let (clan, name) = if let Some((clan, name)) = name.split_once("<sp/>") {
                        let clan = clan.replace("&gt;", "").replace("&lt;", "");
                        (clan.to_string(), name.to_string())
                    } else {
                        (String::new(), name)
                    };
                    SC2ReplaysDirPlayerEntry {
                        idx: idx + 1,
                        clan,
                        name,
                        count,
                    }
                })
                .collect(),
            top_10_maps: stats
                .top_10_maps
                .into_iter()
                .enumerate()
                .map(|(idx, (title, count))| SC2ReplaysDirMapEntry {
                    idx: idx + 1,
                    title,
                    count,
                })
                .collect(),
        }
    }
}
