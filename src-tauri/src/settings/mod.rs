//! Module for application settings management.
use swarmy_tauri_common::*;
use tauri_plugin_store::Store;

use crate::try_get_snapshot_metadata;

pub async fn load_app_settings_from_store<R: tauri::Runtime>(
    store: &Store<R>,
) -> Result<AppSettings, SwarmyTauriError> {
    let disable_parallel_scans = store
        .get("disable_parallel_scans")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    let replay_path = store
        .get("replay_path")
        .and_then(|v| v.as_str().map(|s| s.to_string()))
        .unwrap_or_default();

    // if the ipc directory do basic scan.
    let ipc_path = std::path::Path::new(&replay_path).join("ipcs");
    let arrow_ipc_stats = if ipc_path.exists() && ipc_path.is_dir() {
        let replay_path_cp = replay_path.clone();
        let t = std::thread::spawn(move || match try_get_snapshot_metadata(replay_path_cp) {
            Ok(val) => val,
            Err(e) => {
                log::error!("Error getting snapshot metadata: {}", e);
                SnapshotStats::default()
            }
        });
        t.join().unwrap()
    } else {
        SnapshotStats::default()
    };

    Ok(AppSettings {
        disable_parallel_scans,
        replay_path,
        arrow_ipc_stats,
    })
}
