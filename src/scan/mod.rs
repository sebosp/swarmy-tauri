//! SC2Replay Directory Scan and Export to Arrow IPC Module

pub mod arrow_ipc_stats;
pub mod mpq_file_scan;
pub mod view;

use leptos::leptos_dom::logging::console_log;
use phosphor_leptos::{
    IconWeightData, BARCODE, DATABASE, FOLDERS, HOURGLASS, KEYBOARD, SHIPPING_CONTAINER, X_CIRCLE,
};
use reactive_stores::Store;
use s2protocol::cli::SC2ReplaysDirStats;
use serde::{Deserialize, Serialize};
use swarmy_tauri_common::AppSettings;

/// The different stages from source selection to snapshot and cache download completion.
#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
pub enum ActivityStage {
    #[default]
    None,
    /// A directory has been entered and the process of Scanning SC2Replays can begin.
    DirectoryEntered,
    /// The basic SC2 scan has began.
    ScanInit,
    /// The basic SC2 scan has finished with failure
    ScanFailure,
    /// The basic SC2 scan has finished.
    ScanDone,
    /// The Optimization has began.
    OptimizeInit,
    /// The Optimization has completed with failure
    OptimizeFailure,
    /// The Optimization has completed.
    OptimizeDone,
    /// Downloading Caches process has began.
    DownloadingCachesInit,
    /// Downloading Caches process has completed with failure.
    DownloadingCachesFailure,
    /// Downloading Caches process has completed.
    DownloadingCachesDone,
}

impl ActivityStage {
    pub fn color(&self) -> &'static str {
        match self {
            Self::None => "gray",
            Self::DirectoryEntered => "blue",
            Self::ScanFailure
            | ActivityStage::OptimizeFailure
            | ActivityStage::DownloadingCachesFailure => "red",
            Self::ScanInit | ActivityStage::OptimizeInit | ActivityStage::DownloadingCachesInit => {
                "teal"
            }
            Self::ScanDone | ActivityStage::OptimizeDone | ActivityStage::DownloadingCachesDone => {
                "green"
            }
        }
    }

    pub fn top_container_class(&self) -> Vec<String> {
        vec![
            "border-l-4".to_string(),
            "mt-1".to_string(),
            "p-2".to_string(),
            format!("border-{}-500", self.color()),
            format!("bg-{}-500/10", self.color()),
        ]
    }
    /// The div containing the message about the current stage status.
    pub fn alert_container_class(&self) -> String {
        format!(
            "shrink-0 size-5 text-green-500 bg/text-{}-500/10",
            self.color()
        )
    }

    /// The text class for the current stage status.
    pub fn text_class(&self) -> Vec<String> {
        vec!["text-sm".to_string(), format!("text-{}-300", self.color())]
    }

    pub fn text_content(&self) -> String {
        match self {
            Self::None => "Please enter a directory to scan".to_string(),
            Self::DirectoryEntered=> "Proceed to Scan.".to_string(),
            Self::ScanInit=> "Scanning in progres...".to_string(),
            Self::ScanFailure => "Scan has finished with failure, please check the directory and permissions or the logs.".to_string(),
            Self::ScanDone => "Scan has finished, proceed to Optimize.".to_string(),
            Self::OptimizeInit=> "Data Optimization for analysis in progress.".to_string(),
            Self::OptimizeFailure=> "Scan has finished with failure, please check the directory and permissions or the logs.".to_string(),
            Self::OptimizeDone => "Data Optimization for analysis completed, proceed to Download Caches.".to_string(),
            Self::DownloadingCachesInit => "Downloading Caches.".to_string(),
            Self::DownloadingCachesFailure=> "Download Cache has finished with failure, please check the directory and permissions or the logs.".to_string(),
            Self::DownloadingCachesDone => "Downloading Caches complete.".to_string()
            }
    }

    pub fn icon_data(&self) -> &'static IconWeightData {
        match self {
            Self::None => KEYBOARD,
            Self::DirectoryEntered => FOLDERS,
            Self::ScanInit | Self::OptimizeInit | Self::DownloadingCachesInit => HOURGLASS,
            Self::ScanFailure | Self::OptimizeFailure | Self::DownloadingCachesFailure => X_CIRCLE,
            Self::ScanDone => BARCODE,
            Self::OptimizeDone => DATABASE,
            Self::DownloadingCachesDone => SHIPPING_CONTAINER,
        }
    }
}

impl From<AppSettings> for ActivityStage {
    fn from(source: AppSettings) -> Self {
        console_log(&format!(
            "Transforming AppSettings into ActivityStage {:?}",
            source
        ));
        if source.snapshot_stats.caches_size > 0 {
            Self::DownloadingCachesDone
        } else if source.snapshot_stats.ipc_dir_size > 0 {
            Self::OptimizeDone
        } else if !source.replay_path.is_empty() {
            Self::DirectoryEntered
        } else {
            Self::None
        }
    }
}
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
