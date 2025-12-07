//! Swarmy Tauri UI - SC2Replay Directory Scan and Export to Arrow IPC Module

use s2protocol::arrow_store::ArrowIpcTypes;
use s2protocol::cli::WriteArrowIpcProps;
use s2protocol::game_events::read_balance_data_from_json_dir;
use s2protocol::SC2ReplaysDirStats;
use std::path::PathBuf;

#[tauri::command]
pub async fn basic_scan_replay_path(
    path: String,
    serial: bool,
) -> Result<SC2ReplaysDirStats, String> {
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
