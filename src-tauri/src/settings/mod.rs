use tauri_plugin_store::Store;

#[derive(Debug, Clone)]
pub struct AppSettings {
    pub disable_parallel_scans: bool,
    pub replay_paths: Vec<String>,
}

impl AppSettings {
    pub fn load_from_store<R: tauri::Runtime>(
        store: &Store<R>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let disable_parallel_scans = store
            .get("appSettings.disableParallelScans")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        let replay_paths = store
            .get("appSettings.replayPaths")
            .map(|v| {
                v.to_string()
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .collect::<Vec<String>>()
            })
            .unwrap_or_else(Vec::new);

        Ok(AppSettings {
            disable_parallel_scans,
            replay_paths,
        })
    }
}
