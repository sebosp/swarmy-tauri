use serde::{Deserialize, Serialize};

/// Contains metadata information related to the minimun, maximum date of the map in the snapshot.
/// The cache_handles contain downloadable assets from blizzard's CDN, even tho two maps may have
/// the same title, if their cache_handles differ, they are considered different, maybe different
/// versions, tests, etc.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MapStats {
    /// The size of the IPC files
    pub directory_size: u64,
    /// The time of modification of the details IPC file.
    pub date_modified: std::time::SystemTime,
    /// The number of games
    pub num_games: u64,
    /// The name of the map.
    pub title: String,
    /// The cache_handles for the map.
    pub cache_handles: String,
    /// The minimum date of the snapshot taken
    pub min_date: chrono::NaiveDate,
    /// The maximum date of the snapshot taken
    pub max_date: chrono::NaiveDate,
}

impl Default for MapStats {
    fn default() -> Self {
        Self {
            directory_size: 0,
            date_modified: std::time::SystemTime::UNIX_EPOCH,
            min_date: chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(),
            max_date: chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(),
            num_games: 0,
            title: String::new(),
            cache_handles: String::new(),
        }
    }
}

/// Initial set of query params for the map stats arrow IPC file.
/// XXX: We need to figure out how to handle multiple players.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct MapStatsQuery {
    /// The location of the arrow IPC files.
    pub replay_path: String,
    /// The name of the map.
    pub map_title: String,
    /// A player that must have played a game in the map.
    pub player_name: String,
}
