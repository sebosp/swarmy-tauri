use reactive_stores::Store;
use s2protocol::cli::SC2ReplaysDirStats;
use serde::{Deserialize, Serialize};

#[derive(Store, Debug, Clone, Serialize, Deserialize)]
pub struct SC2ReplaysDirStatsTable {
    pub total_files: usize,
    pub total_supported_replays: usize,
    pub abily_supported_replays: usize,
    #[store(key: String = |row| row.name.clone())]
    pub top_10_players: Vec<SC2ReplaysDirPlayerEntry>,
}

#[derive(Store, Debug, Clone, Serialize, Deserialize)]
pub struct SC2ReplaysDirPlayerEntry {
    pub idx: usize,
    pub name: String,
    pub count: usize,
}

impl From<SC2ReplaysDirStats> for SC2ReplaysDirStatsTable {
    fn from(stats: SC2ReplaysDirStats) -> Self {
        Self {
            total_files: stats.total_files,
            total_supported_replays: stats.total_supported_replays,
            abily_supported_replays: stats.abily_supported_replays,
            top_10_players: stats
                .top_10_players
                .into_iter()
                .enumerate()
                .map(|(idx, (name, count))| SC2ReplaysDirPlayerEntry {
                    idx: idx + 1,
                    name: name,
                    count: count,
                })
                .collect(),
        }
    }
}

#[derive(Store, Debug, Clone)]
pub struct Data {
    #[store(key: String = |row| row.key.clone())]
    pub rows: Vec<DatabaseEntry>,
}

#[derive(Store, Debug, Clone)]
pub struct DatabaseEntry {
    pub key: String,
    pub value: i32,
}
