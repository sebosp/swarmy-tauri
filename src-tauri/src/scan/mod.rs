//! Swarmy Tauri UI - SC2Replay Directory Scan and Export to Arrow IPC Module

use crate::settings::load_app_settings_from_store;
use s2protocol::arrow_store::ArrowIpcTypes;
use s2protocol::cli::WriteArrowIpcProps;
use s2protocol::game_events::read_balance_data_from_json_dir;
use s2protocol::SC2ReplaysDirStats;
use std::path::PathBuf;
use swarmy_tauri_common::*;
use tauri_plugin_store::StoreBuilder;

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

#[tauri::command(rename_all = "snake_case")]
pub async fn basic_scan_replay_path(
    app_handle: tauri::AppHandle,
    replay_path: String,
    disable_parallel_scans: bool,
) -> Result<SC2ReplaysDirStats, String> {
    let store = StoreBuilder::new(&app_handle, "settings.json")
        .build()
        .map_err(|e| {
            log::error!("Error building store: {}", e);
            format!("Error building store: {:?}", e)
        })?;

    // If there are no saved settings yet, this will return an error so we ignore the return value.
    let _ = store.reload();

    store.set("disable_parallel_scans", disable_parallel_scans);
    store.set("replay_path", replay_path.clone());
    // create a thread to scan the directory in the background:
    let t = std::thread::spawn(move || {
        log::info!("Scanning replays directory: {}", replay_path);
        match SC2ReplaysDirStats::from_directory(&replay_path, disable_parallel_scans) {
            Ok(s) => {
                log::info!(
                    "Finished scanning replays directory: {} with res: {:?}",
                    replay_path,
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

#[tauri::command(rename_all = "snake_case")]
pub async fn optimize_replay_path(
    _app_handle: tauri::AppHandle,
    replay_path: String,
    disable_parallel_scans: bool,
) -> ApiResponse {
    let initial_duration = std::time::Instant::now();
    // create a thread to scan the directory in the background:
    let t = std::thread::spawn(move || {
        match try_optimize_replay_path(replay_path, disable_parallel_scans).map_err(|e| {
            log::error!("Error optimizing replays: {}", e);
            e
        }) {
            Ok(()) => Ok(String::from("Optimization completed successfully.")),
            Err(e) => Err(format!("Error optimizing replays: {:?}", e)),
        }
    });
    match t.join().unwrap() {
        Ok(val) => ApiResponse::new(
            ResponseMetaBuilder::new(true)
                .duration_ms(initial_duration.elapsed().as_millis() as u64)
                .build(),
            val,
        ),
        Err(e) => ApiResponse::new(
            ResponseMetaBuilder::new(false)
                .duration_ms(initial_duration.elapsed().as_millis() as u64)
                .build(),
            format!("Error optimizing replays: {:?}", e),
        ),
    }
}

fn try_optimize_replay_path(
    replay_path: String,
    disable_parallel_scans: bool,
) -> Result<(), SwarmyTauriError> {
    let path = PathBuf::from(&replay_path);
    let destination = path.join("ipcs");
    if !destination.exists() {
        std::fs::create_dir_all(&destination)?;
    }
    log::info!(
        "Optimizing replays directory: {} and storing into {}",
        path.display(),
        destination.display()
    );
    let versioned_abilities = read_balance_data_from_json_dir(&path)?;
    // TODO: Move from cli on s2protocol and create a leptos view to configure this.
    let props = WriteArrowIpcProps {
        scan_max_files: 10000,
        process_max_files: 10000,
        traverse_max_depth: 8,
        min_version: None,
        max_version: None,
    };
    Ok(ArrowIpcTypes::handle_arrow_ipc_cmd(
        path,
        destination,
        &props,
        &versioned_abilities,
        disable_parallel_scans,
    )?)
}
