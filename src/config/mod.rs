//! Configuration of the app, stored in filesystem.
use crate::*;
use leptos::prelude::*;
use swarmy_tauri_common::*;
use leptos::task::spawn_local;
use leptos::leptos_dom::logging::console_log;
pub mod view;


pub fn fetch_get_current_app_config(
    set_arrow_ipc_stats: WriteSignal<SnapshotStats>,
    set_app_settings: WriteSignal<AppSettings>,
    set_optimize_button_enabled: WriteSignal<bool>,
    set_download_cache_enabled: WriteSignal<bool>,

) {
    spawn_local(async move {
        // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
        match serde_wasm_bindgen::from_value::<AppSettings>(
            invoke_without_args("get_current_app_config").await,
        ) {
            Ok(config) => {
                console_log(&format!("Loaded app config: {:?}", config));
                *set_arrow_ipc_stats.write() = config.arrow_ipc_stats.clone();
                *set_app_settings.write() = config;
            }
            Err(e) => {
                console_log(&format!("Error invoking get_current_app_config: {:?}", e));
            }
        }
        set_optimize_button_enabled.set(true);
        set_download_cache_enabled.set(true);
    });
}
