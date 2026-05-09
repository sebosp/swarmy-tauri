//! Configuration of the app, stored in filesystem.
use crate::scan::ActivityStage;
use crate::*;
use leptos::leptos_dom::logging::console_log;
use leptos::prelude::*;
use leptos::task::spawn_local;
use swarmy_tauri_common::*;
pub mod view;

pub fn fetch_get_current_app_config(
    set_snapshot_stats: WriteSignal<SnapshotStats>,
    set_app_settings: WriteSignal<AppSettings>,
    set_activity_stage: WriteSignal<ActivityStage>,
) {
    spawn_local(async move {
        // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
        match serde_wasm_bindgen::from_value::<AppSettings>(
            invoke_without_args("get_current_app_config").await,
        ) {
            Ok(config) => {
                console_log(&format!("Loaded app config: {:?}", config));
                *set_snapshot_stats.write() = config.snapshot_stats.clone();
                *set_activity_stage.write() = ActivityStage::from(config.clone());
                *set_app_settings.write() = config;
            }
            Err(e) => {
                console_log(&format!("Error invoking get_current_app_config: {:?}", e));
            }
        }
    });
}
