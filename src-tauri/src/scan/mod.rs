//! Swarmy Tauri UI - SC2Replay Directory Scan and Export to Arrow IPC Module

use s2protocol::arrow_store::ArrowIpcTypes;
use s2protocol::cli::WriteArrowIpcProps;
use s2protocol::game_events::read_balance_data_from_json_dir;
use s2protocol::SC2ReplaysDirStats;
use std::path::PathBuf;
use swarmy_tauri_common::*;
use tauri_plugin_store::StoreBuilder;

use crate::settings::load_app_settings_from_store;

#[tauri::command]
pub fn get_current_app_config(app_handle: tauri::AppHandle) -> Result<AppSettings, String> {
    let store = StoreBuilder::new(&app_handle, "settings.json")
        .build()
        .map_err(|e| {
            log::error!("Error building store: {}", e);
            format!("Error building store: {:?}", e)
        })?;

    // If there are no saved settings yet, this will return an error so we ignore the return value.
    let _ = store.reload();

    let app_settings = load_app_settings_from_store(&store).map_err(|e| {
        log::error!("Error loading app settings: {}", e);
        format!("Error loading app settings: {:?}", e)
    })?;
    Ok(app_settings)
}

#[tauri::command]
pub async fn basic_scan_replay_path(
    app_handle: tauri::AppHandle,
    path: String,
    serial: bool,
) -> Result<SC2ReplaysDirStats, String> {
    let store = StoreBuilder::new(&app_handle, "settings.json")
        .build()
        .map_err(|e| {
            log::error!("Error building store: {}", e);
            format!("Error building store: {:?}", e)
        })?;

    // If there are no saved settings yet, this will return an error so we ignore the return value.
    let _ = store.reload();

    store.set("disable_parallel_scans", serial);
    store.set("replay_paths", vec![path.clone()]);
    // create a thread to scan the directory in the background:
    let t = std::thread::spawn(move || {
        log::info!("Scanning replays directory: {}", path);
        match SC2ReplaysDirStats::from_directory(&path, serial) {
            Ok(s) => {
                log::info!(
                    "Finished scanning replays directory: {} with res: {:?}",
                    path,
                    s
                );
                Ok(s)
            }
            Err(e) => {
                log::error!("Error scanning replays directory: {}", e);
                Err(format!("Error scanning replays directory: {:?}", e))
            }
        }
    });
    t.join().unwrap()
}

#[tauri::command]
pub async fn optimize_replay_path(path: String, serial: bool) -> Result<(), String> {
    // create a thread to scan the directory in the background:
    let t = std::thread::spawn(move || {
        let path = PathBuf::from(&path);
        let destination = path.join("ipcs");
        log::info!(
            "Optimizing replays directory: {} and storing into {}",
            path.display(),
            destination.display()
        );
        let versioned_abilities = read_balance_data_from_json_dir(&path).map_err(|e| {
            log::error!("Error reading balance data: {}", e);
            format!("Error reading balance data: {:?}", e)
        })?;
        // TODO: Move from cli on s2protocol and create a leptos view to configure this.
        let props = WriteArrowIpcProps {
            scan_max_files: 10000,
            process_max_files: 10000,
            traverse_max_depth: 8,
            min_version: None,
            max_version: None,
        };
        match ArrowIpcTypes::handle_arrow_ipc_cmd(
            path,
            destination,
            &props,
            &versioned_abilities,
            serial,
        ) {
            Ok(s) => {
                log::info!("Finished optimizing replays directory.",);
                Ok(s)
            }
            Err(e) => {
                log::error!("Error scanning replays directory: {}", e);
                Err(format!("Error scanning replays directory: {:?}", e))
            }
        }
    });
    t.join().unwrap()
}
