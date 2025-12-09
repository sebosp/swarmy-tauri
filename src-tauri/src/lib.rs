//! Swarmy Tauri UI Library
use tauri::Manager;
use tauri_plugin_store::StoreBuilder;

pub mod scan;
pub use scan::*;
pub mod settings;
use settings::AppSettings;

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
            basic_scan_replay_path,
            optimize_replay_path
        ])
        .plugin(tauri_plugin_store::Builder::default().build())
        .setup(|app| {
            let mut store = StoreBuilder::new(app.handle(), "settings.json").build()?;

            // If there are no saved settings yet, this will return an error so we ignore the return value.
            let _ = store.reload();

            let app_settings = AppSettings::load_from_store(&store)?;

            let disable_parallel_scans = app_settings.disable_parallel_scans;
            let replay_paths = app_settings.replay_paths;

            println!("disable_parallel_scans {}", disable_parallel_scans);
            println!("replay_paths {:?}", replay_paths);
            println!("Path: {:?}", app.handle().path().app_data_dir());

            store.save()?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
