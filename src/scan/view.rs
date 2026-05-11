//! Swarmy Tauri UI - Scan View

use super::arrow_ipc_stats::ArrowIpcStats;
use super::mpq_file_scan::ReplayScanTable;
use crate::scan::*;
use crate::*;
use leptos::ev::MouseEvent;
use leptos::leptos_dom::logging::console_log;
use leptos::prelude::*;
use leptos::task::spawn_local;
use phosphor_leptos::{Icon, IconWeight, BARCODE, DATABASE, HOURGLASS, SHIPPING_CONTAINER};
use reactive_graph::traits::Write;
use reactive_stores::{Patch, Store};
use s2protocol::SC2ReplaysDirStats;
use swarmy_tauri_common::*;

/// Step 1, a user selects a directory to scan.
/// This may contain multiple subdirectories.
/// TODO: We could store a swarmy-stats.json in the root directory to prevent re-scanning.
/// This would also allow this step to be marked as completion when the UI is loaded.
pub fn trigger_basic_scan_replay_path(
    ev: MouseEvent,
    app_settings: ReadSignal<AppSettings>,
    set_backend_response: WriteSignal<ApiResponse>,
    data: Store<SC2ReplaysDirStatsTable>,
    set_activity_stage: WriteSignal<ActivityStage>,
) {
    ev.prevent_default();
    *set_activity_stage.write() = ActivityStage::ScanInit;
    *set_backend_response.write() = ApiResponse::new_incomplete();
    if app_settings.get().replay_path.is_empty() {
        console_log("Replay path is empty.");
        return;
    }

    let app_settings_cp = app_settings.get();

    spawn_local(async move {
        let args = serde_wasm_bindgen::to_value(&app_settings_cp).unwrap();
        // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
        match serde_wasm_bindgen::from_value::<SC2ReplaysDirStats>(
            invoke("basic_scan_replay_path", args).await,
        ) {
            Ok(stats) => {
                let mut stats_table: SC2ReplaysDirStatsTable = stats.into();
                console_log(&format!("New data = {:?}", stats_table));
                data.top_10_players().write().retain(|_| false);
                data.top_10_players()
                    .write()
                    .append(&mut stats_table.top_10_players);
                data.top_10_maps().write().retain(|_| false);
                data.top_10_maps()
                    .write()
                    .append(&mut stats_table.top_10_maps);
                data.total_files().patch(stats_table.total_files);
                data.total_supported_replays()
                    .patch(stats_table.total_supported_replays);
                data.ability_supported_replays()
                    .patch(stats_table.ability_supported_replays);
            }
            Err(e) => {
                console_log(&format!("Error invoking basic_scan_replay_path: {:?}", e));
            }
        }
        *set_activity_stage.write() = ActivityStage::ScanDone;
    });
}

/// Step 2: Once a directory has gone through the basic scan, the process of generating the
/// ArrowIpc "snapshot" can be done.
/// TODO: When an `ipc` directory already exists, we could mark this step as done, but provide the
/// user the possibility to re-run the optimization.
pub fn trigger_optimize_replay_path(
    app_settings: ReadSignal<AppSettings>,
    set_backend_response: WriteSignal<ApiResponse>,
    set_activity_stage: WriteSignal<ActivityStage>,
) {
    *set_activity_stage.write() = ActivityStage::OptimizeInit;
    // Reset backend response status.
    *set_backend_response.write() = ApiResponse::new_incomplete();

    if app_settings.get().replay_path.is_empty() {
        console_log("Replay path is empty.");
        return;
    }

    let app_settings_cp = app_settings.get();
    spawn_local(async move {
        let args = serde_wasm_bindgen::to_value(&app_settings_cp).unwrap();
        console_log(&format!(
            "Invoking optimize_replay_path with args: {:?}",
            args
        ));
        // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
        let response = invoke("optimize_replay_path", args).await;
        console_log(&format!("optimize_replay_path response: {:?}", response));
        match serde_wasm_bindgen::from_value::<ApiResponse>(response) {
            Ok(res) => {
                if res.meta.success {
                    console_log("Optimize replay path succeeded.");
                } else {
                    console_log(&format!("Optimize replay path failed: {:?}", res.message));
                }
                set_backend_response.set(res);
            }
            Err(e) => {
                console_log(&format!("Error invoking optimize_replay_path: {:?}", e));
                set_backend_response.set(ApiResponse {
                    meta: ResponseMeta {
                        success: false,
                        duration_ms: 0,
                        is_complete: true,
                    },
                    message: format!("Error invoking optimize_replay_path: {:?}", e),
                });
            }
        }
        *set_activity_stage.write() = ActivityStage::OptimizeDone;
    });
}

pub fn trigger_download_replay_caches(
    app_settings: ReadSignal<AppSettings>,
    set_backend_response: WriteSignal<ApiResponse>,
    set_activity_stage: WriteSignal<ActivityStage>,
) {
    *set_activity_stage.write() = ActivityStage::DownloadingCachesInit;
    // Reset backend response status.
    *set_backend_response.write() = ApiResponse::new_incomplete();

    if app_settings.get().replay_path.is_empty() {
        console_log("Replay path is empty.");
        return;
    }

    let app_settings_cp = app_settings.get();
    spawn_local(async move {
        let args = serde_wasm_bindgen::to_value(&app_settings_cp).unwrap();
        console_log(&format!(
            "Invoking download_replay_caches with args: {:?}",
            args
        ));
        // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
        let response = invoke("download_replay_caches", args).await;
        console_log(&format!("download_replay_caches response: {:?}", response));
        match serde_wasm_bindgen::from_value::<ApiResponse>(response) {
            Ok(res) => {
                if res.meta.success {
                    console_log("Download replay caches succeeded.");
                } else {
                    console_log(&format!("Download replay caches failed: {:?}", res.message));
                }
                set_backend_response.set(res);
            }
            Err(e) => {
                console_log(&format!("Error invoking download_replay_caches: {:?}", e));
                set_backend_response.set(ApiResponse {
                    meta: ResponseMeta {
                        success: false,
                        duration_ms: 0,
                        is_complete: true,
                    },
                    message: format!("Error invoking download_replay_caches: {:?}", e),
                });
            }
        }
        *set_activity_stage.write() = ActivityStage::DownloadingCachesDone;
    });
}

#[component]
pub fn ScanDirectory() -> impl IntoView {
    let (activity_stage, set_activity_stage) = signal(ActivityStage::None);
    let (app_settings, set_app_settings) = signal(AppSettings::default());
    let (backend_response, set_backend_response) = signal(ApiResponse {
        meta: ResponseMeta::incomplete(),
        message: String::new(),
    });
    let (snapshot_stats, set_snapshot_stats) = signal(SnapshotStats::default());

    crate::config::fetch_get_current_app_config(
        set_snapshot_stats,
        set_app_settings,
        set_activity_stage,
    );
    let tx_update_replay_dir = move |ev| {
        let v = event_target_value(&ev);
        if v.is_empty() {
            *set_activity_stage.write() = ActivityStage::None;
        } else {
            *set_activity_stage.write() = ActivityStage::DirectoryEntered;
        }
        set_app_settings.update(|settings| {
            settings.replay_path = v;
        });
    };
    let dir_stats_data = Store::new(SC2ReplaysDirStatsTable::from(SC2ReplaysDirStats::default()));

    view! {
        <div class="grid grid-cols-10">
            <div class="col-span-5 pl-1">
                <label for="scan_path" class="block text-sm/6 font-medium text-white">
                    Path
                </label>
                <input
                    name="scan_path"
                    class=text_input_tailwind_classes().join(" ")
                    id="scan-directory-input"
                    value=move || app_settings.get().replay_path
                    on:input=tx_update_replay_dir
                    type="text"
                />
            </div>
            <div class="col-span-5 justify-start ml-6 mt-6">
                <button
                    class=move || {
                        if activity_stage.get() > ActivityStage::None {
                            "btn btn-primary btn-sm"
                        } else {
                            "btn btn-disabled btn-sm disabled:cursor-not-allowed disabled:bg-gray-50 disabled:text-gray-500 disabled:outline-gray-200"
                        }
                    }
                    on:click=move |ev: MouseEvent| trigger_basic_scan_replay_path(
                        ev,
                        app_settings,
                        set_backend_response,
                        dir_stats_data,
                        set_activity_stage,
                    )
                    disabled=move || app_settings.get().replay_path.is_empty()
                    title="Initial scan for StarCraft II replays"
                >
                    <Icon
                        icon=(move || {
                            if app_settings.get().replay_path.is_empty() {
                                BARCODE
                            } else {
                                HOURGLASS
                            }
                        })()
                        weight=IconWeight::Light
                        prop:class="stroke-current"
                    />
                    "Scan"
                </button>
                <button
                    class=move || {
                        if !app_settings.get().replay_path.is_empty() {
                            "btn btn-accent btn-sm"
                        } else {
                            "btn btn-disabled btn-sm disabled:cursor-not-allowed disabled:bg-gray-50 disabled:text-gray-500 disabled:outline-gray-200"
                        }
                    }
                    on:click=move |_| trigger_optimize_replay_path(
                        app_settings,
                        set_backend_response,
                        set_activity_stage,
                    )
                    disabled=move || { activity_stage.get() != ActivityStage::ScanDone }
                    title="Optimize the replay generating Arrow files (may take some time)"
                >
                    <Icon icon=DATABASE weight=IconWeight::Light prop:class="stroke-current" />
                    {move || {
                        if activity_stage.get() == ActivityStage::OptimizeInit {
                            "Optimizing..."
                        } else {
                            "Optimize"
                        }
                    }}
                </button>
                <button
                    class=move || {
                        if !app_settings.get().replay_path.is_empty() {
                            "btn btn-info btn-sm"
                        } else {
                            "btn btn-disabled btn-sm disabled:cursor-not-allowed disabled:bg-gray-50 disabled:text-gray-500 disabled:outline-gray-200"
                        }
                    }
                    on:click=move |_| trigger_download_replay_caches(
                        app_settings,
                        set_backend_response,
                        set_activity_stage,
                    )
                    disabled=move || activity_stage.get() != ActivityStage::DownloadingCachesInit
                    title="Downloads the caches from Starcraft II servers that contain map information such as Height Map"
                >
                    <Icon
                        icon=SHIPPING_CONTAINER
                        weight=IconWeight::Light
                        prop:class="stroke-current"
                    />
                    {move || {
                        if !app_settings.get().replay_path.is_empty() {
                            "Downloading Caches..."
                        } else {
                            "Download Caches"
                        }
                    }}
                </button>
            </div>
            <DisplayBackendStatus backend_response />
            <div class="col-span-10">
                <Show when=move || { dir_stats_data.total_files().get() == 0 }>
                    <SnapshotStatusHeader app_settings=app_settings dir_stats_data activity_stage />
                </Show>
                <Show when=move || {
                    dir_stats_data.total_files().get() > 0
                        && app_settings.get().snapshot_stats.ipc_dir_size == 0
                }>
                    <SnapshotStatusHeader app_settings=app_settings dir_stats_data activity_stage />
                    <ReplayScanTable dir_stats_data />
                </Show>
                <Show when=move || {
                    activity_stage.get() == ActivityStage::OptimizeDone
                        && app_settings.get().snapshot_stats.ipc_dir_size > 0
                }>
                    <SnapshotStatusHeader app_settings=app_settings dir_stats_data activity_stage />
                    <ArrowIpcStats snapshot_stats=snapshot_stats />
                </Show>
            </div>
        </div>
    }
}
#[component]
pub fn SnapshotStatusHeader(
    app_settings: ReadSignal<AppSettings>,
    dir_stats_data: Store<SC2ReplaysDirStatsTable>,
    activity_stage: ReadSignal<ActivityStage>,
) -> impl IntoView {
    let has_replay_files = move || dir_stats_data.total_files().get() > 0;
    let has_optimized_snapshot = move || app_settings.get().snapshot_stats.ipc_dir_size > 0;
    let has_caches_downloaded = move || app_settings.get().snapshot_stats.num_caches > 0;
    console_log(&format!(
        "has_replay_files {} has_optimized_snapshot {} has_caches_downloaded {}",
        has_replay_files(),
        has_optimized_snapshot(),
        has_caches_downloaded()
    ));
    view! {
        <div class=move || { activity_stage.get().top_container_class().join(" ") }>
            <div class="flex">
                <div class=move || { activity_stage.get().alert_container_class() }>
                    <Icon
                        icon=(move || { activity_stage.get().icon_data() })()
                        weight=IconWeight::Bold
                        prop:class="stroke-current"
                    />
                </div>
                <div class="ml-3">
                    <p class=move || { activity_stage.get().text_class().join(" ") }>
                        <code>{move || app_settings.get().replay_path}</code>
                        ": "
                        {move || activity_stage.get().text_content()}
                    </p>
                </div>
            </div>
        </div>
    }
}
