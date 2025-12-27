use serde::{Deserialize, Serialize};

/// Contains metadata information related to the minimun, maximum date of the snapshot taken, the number of
/// files analyzed, the number of maps and the number of players in the analyzed collection
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
