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

    let replay_paths = store
        .get("replay_paths")
        .map(|v| {
            v.as_array()
                .map(|arr| {
                    arr.iter()
                        .filter_map(|item| item.as_str().map(|s| s.to_string()))
                        .collect()
                })
                .unwrap_or_else(Vec::new)
        })
        .unwrap_or_else(Vec::new);

    Ok(AppSettings {
        disable_parallel_scans,
        replay_paths,
    })
}
