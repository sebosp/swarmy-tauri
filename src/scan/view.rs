//! Swarmy Tauri UI - Scan View

use crate::*;
use leptos::ev::MouseEvent;
use leptos::leptos_dom::logging::console_log;
use leptos::prelude::*;
use leptos::task::spawn_local;
use phosphor_leptos::{Icon, IconWeight, BARCODE, CPU, DATABASE, FOLDERS, HOURGLASS, X_CIRCLE};
use reactive_graph::traits::Write;
use reactive_stores::{Patch, Store};
use s2protocol::SC2ReplaysDirStats;
use swarmy_tauri_common::*;
use super::mpq_file_scan::ReplayScanTable;
use super::arrow_ipc_stats::ArrowIpcStats;

pub fn trigger_optimize_replay_path(
    ev: MouseEvent,
    app_settings: ReadSignal<AppSettings>,
    set_optimize_button_enabled: WriteSignal<bool>,
    backend_response: WriteSignal<ApiResponse>,
) {
    ev.prevent_default();
    // Reset backend response status.
    *backend_response.write() = ApiResponse::new_incomplete();

    if app_settings.get_untracked().replay_path.is_empty() {
        console_log("Replay path is empty.");
        return;
    }
    set_optimize_button_enabled.set(false);

    let app_settings_cp = app_settings.get_untracked();
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
                backend_response.set(res);
                set_optimize_button_enabled.set(true);
            }
            Err(e) => {
                console_log(&format!("Error invoking optimize_replay_path: {:?}", e));
                set_optimize_button_enabled.set(true);
                backend_response.set(ApiResponse {
                    meta: ResponseMeta {
                        success: false,
                        duration_ms: 0,
                        is_complete: true,
                    },
                    message: format!("Error invoking optimize_replay_path: {:?}", e),
                });
            }
        }
    });
}

pub fn trigger_basic_scan_replay_path(
    ev: MouseEvent,
    app_settings: ReadSignal<AppSettings>,
    backend_response: WriteSignal<ApiResponse>,
    data: Store<SC2ReplaysDirStatsTable>,
) {
    ev.prevent_default();
    *backend_response.write() = ApiResponse::new_incomplete();
    if app_settings.get().replay_path.is_empty() {
        console_log("Replay path is empty.");
        return;
    }

    let app_settings_cp = app_settings.get_untracked();

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
    });
}

#[component]
pub fn ScanDirectory() -> impl IntoView {
    let (app_settings, set_app_settings) = signal(AppSettings::default());
    let (optimize_button_enabled, set_optimize_button_enabled) = signal(false);
    let (disable_parallel_scans, set_disable_parallel_scans) = signal(false);
    let (backend_response, set_backend_response) = signal(ApiResponse {
        meta: ResponseMeta::incomplete(),
        message: String::new(),
    });
    let (arrow_ipc_stats, set_arrow_ipc_stats) = signal(SnapshotStats::default());

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
    });
    let tx_update_replay_dir = move |ev| {
        let v = event_target_value(&ev);
        set_optimize_button_enabled.set(!v.is_empty());
        set_app_settings.update(|settings| {
            settings.replay_path = v;
        });
    };
    let dir_stats_data = Store::new(SC2ReplaysDirStatsTable::from(SC2ReplaysDirStats::default()));

    view! {
        <div class="grid grid-cols-8 grid-rows-1 gap-1">
            <div class="col-span-5">
                <label class="input input-sm w-full">
                    "Path"
                    <input
                        class="input input-sm my-0 mx-0"
                        id="scan-directory-input"
                        value=move || app_settings.get().replay_path
                        on:input=tx_update_replay_dir
                        type="text"
                    />
                </label>
            </div>
            <div class="col-span-2 justify-start">
                <button
                    class=move || {
                        if !app_settings.get().replay_path.is_empty() {
                            "btn btn-primary btn-sm m-0"
                        } else {
                            "btn btn-disabled btn-sm m-0"
                        }
                    }
                    on:click=move |ev: MouseEvent| trigger_basic_scan_replay_path(
                        ev,
                        app_settings,
                        set_backend_response,
                        dir_stats_data,
                    )
                    disabled=move || app_settings.get().replay_path.is_empty()
                    title="Initial scan for StarCraft II replays"
                >
                    <Icon
                        icon=if app_settings.get_untracked().replay_path.is_empty() {
                            BARCODE
                        } else {
                            HOURGLASS
                        }
                        weight=IconWeight::Light
                        prop:class="stroke-current"
                    />
                    "Scan"
                </button>
                <button
                    class=move || {
                        if !app_settings.get().replay_path.is_empty() {
                            "btn btn-success btn-sm m-0"
                        } else {
                            "btn btn-disabled btn-sm m-0"
                        }
                    }
                    on:click=move |ev: MouseEvent| trigger_optimize_replay_path(
                        ev,
                        app_settings,
                        set_optimize_button_enabled,
                        set_backend_response,
                    )
                    disabled=move || !optimize_button_enabled.get()
                    title="Optimize the replay generating Arrow files (may take some time)"
                >
                    <Icon icon=DATABASE weight=IconWeight::Light prop:class="stroke-current" />
                    {move || {
                        if !app_settings.get().replay_path.is_empty()
                            && !optimize_button_enabled.get()
                        {
                            "Optimizing..."
                        } else {
                            "Optimize"
                        }
                    }}
                </button>
            </div>
            <div class="col-span-1 flex justify-end">
                <label
                    class="btn btn-sm btn-circle swap swap-rotate"
                    title=move || {
                        if disable_parallel_scans.get() {
                            "Enable Parallel Processing"
                        } else {
                            "Disable Parallel Processing"
                        }
                    }
                >
                    <input
                        type="checkbox"
                        checked=move || disable_parallel_scans.get()
                        on:click=move |_| {
                            set_disable_parallel_scans.set(!disable_parallel_scans.get())
                        }
                    />
                    <Icon
                        icon=CPU
                        weight=IconWeight::Bold
                        prop:title=move || {
                            if disable_parallel_scans.get() {
                                "Parallel Processing Disabled"
                            } else {
                                "Parallel Processing Enabled"
                            }
                        }
                        prop:class=move || {
                            if disable_parallel_scans.get() {
                                "swap-on fill-current"
                            } else {
                                "swap-off fill-current"
                            }
                        }
                        color=move || if disable_parallel_scans.get() { "orange" } else { "green" }
                    />
                </label>
            </div>
        </div>
        <Show when=move || {
            !backend_response.get().meta.success && backend_response.get().meta.is_complete
        }>
            <div role="alert" class="alert alert-error shadow-lg m-1 p-1">
                <Icon icon=X_CIRCLE weight=IconWeight::Bold prop:class="stroke-current" />
                <span>{backend_response.get().message.clone()}</span>
            </div>
        </Show>
        <Show when=move || {
            dir_stats_data.total_files().get() > 0
                && !app_settings.get().arrow_ipc_stats.directory_size > 0
        }>
            <div role="alert" class="alert alert-warning alert-soft m-1 p-1">
                <Icon icon=FOLDERS weight=IconWeight::Bold prop:class="stroke-current" />
                <span>
                    "Directory is not optimized, click on Optimize to generate the optimized snapshot, this may take a while. "
                    "A subdirectory named "<b>
                        <code>"ipc"</code>
                    </b>" will be created in the chosen folder with the optimized snapshot."
                </span>
            </div>
            <ReplayScanTable dir_stats_data />
        </Show>
        <Show when=move || { app_settings.get().arrow_ipc_stats.directory_size > 0 }>
            <div role="alert" class="alert alert-success alert-soft m-1 p-1">
                <Icon icon=DATABASE weight=IconWeight::Bold prop:class="stroke-current" />
                <span>"Directory is optimized."</span>
            </div>
            <ArrowIpcStats arrow_ipc_stats />
        </Show>
    }
}
