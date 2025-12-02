use s2protocol::cli::SC2ReplaysDirStats;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
async fn set_replays_path(path: String) -> Result<SC2ReplaysDirStats, String> {
    // create a thread to scan the directory in the background:
    let t = std::thread::spawn(move || {
        log::info!("Scanning replays directory: {}", path);
        match SC2ReplaysDirStats::from_directory(&path) {
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(tauri_plugin_log::log::LevelFilter::Info)
                .build(),
        )
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![set_replays_path])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
