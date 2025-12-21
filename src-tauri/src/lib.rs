//! Swarmy Tauri UI Library

pub mod mpq_file_scan;
pub use mpq_file_scan::*;
pub mod settings;
pub use settings::*;
pub mod snapshot_stats;
pub use snapshot_stats::*;
pub mod common;
pub use common::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_persisted_scope::init())
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(tauri_plugin_log::log::LevelFilter::Info)
                .build(),
        )
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_current_app_config,
            basic_scan_replay_path,
            optimize_replay_path,
            get_snapshot_metadata,
        ])
        .plugin(tauri_plugin_store::Builder::default().build())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
