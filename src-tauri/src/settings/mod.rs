//! Module for application settings management.
use swarmy_tauri_common::AppSettings;
use tauri_plugin_store::Store;

pub fn load_app_settings_from_store<R: tauri::Runtime>(
    store: &Store<R>,
) -> Result<AppSettings, Box<dyn std::error::Error>> {
    let disable_parallel_scans = store
        .get("disable_parallel_scans")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    let replay_path = store
        .get("replay_path")
        .and_then(|v| v.as_str().map(|s| s.to_string()))
        .unwrap_or_default();

    let mut has_arrow_ipc_export = false;

    // if the ipc directory exists we can set has_arrow_ipc_export to true
    let ipc_path = std::path::Path::new(&replay_path).join("ipcs");
    if ipc_path.exists() && ipc_path.is_dir() {
        has_arrow_ipc_export = true;
    }

    Ok(AppSettings {
        disable_parallel_scans,
        replay_path,
        has_arrow_ipc_export,
    })
}
