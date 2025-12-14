//! Swarmy Tauri UI - Scan View

use crate::*;
use leptos::ev::MouseEvent;
use leptos::leptos_dom::logging::console_log;
use leptos::prelude::*;
use leptos::task::spawn_local;
use phosphor_leptos::{Icon, IconWeight, CPU};
use reactive_graph::traits::Read;
use reactive_graph::traits::Write;
use reactive_stores::{Patch, Store};
use s2protocol::SC2ReplaysDirStats;
use swarmy_tauri_common::*;

pub fn trigger_optimize_replay_path(
    ev: MouseEvent,
    replay_path: ReadSignal<String>,
    disable_parallel_scans: ReadSignal<bool>,
    set_optimize_button_enabled: WriteSignal<bool>,
) {
    ev.prevent_default();
    let name = replay_path.get_untracked();
    let disable_parallel = disable_parallel_scans.get_untracked();
    if name.is_empty() {
        console_log("Replay path is empty.");
        return;
    }
    set_optimize_button_enabled.set(false);

    spawn_local(async move {
        let args = serde_wasm_bindgen::to_value(&AppSettings {
            disable_parallel_scans: disable_parallel,
            replay_path: name,
            has_arrow_ipc_export: false,
        })
        .unwrap();
        // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
        match serde_wasm_bindgen::from_value::<String>(invoke("optimize_replay_path", args).await) {
            Ok(stats) => {
                set_optimize_button_enabled.set(true);
                console_log(&format!("Optimize completed: {}", stats));
            }
            Err(e) => {
                console_log(&format!("Error invoking optimize_replay_path: {:?}", e));
                set_optimize_button_enabled.set(true);
                return;
            }
        }
    });
}

pub fn trigger_basic_scan_replay_path(
    ev: MouseEvent,
    replay_path: ReadSignal<String>,
    disable_parallel_scans: ReadSignal<bool>,
    set_scan_button_enabled: WriteSignal<bool>,
    data: Store<SC2ReplaysDirStatsTable>,
) {
    ev.prevent_default();
    let name = replay_path.get_untracked();
    let disable_parallel = disable_parallel_scans.get_untracked();
    if name.is_empty() {
        console_log("Replay path is empty.");
        return;
    }
    set_scan_button_enabled.set(false);

    spawn_local(async move {
        let args = serde_wasm_bindgen::to_value(&AppSettings {
            disable_parallel_scans: disable_parallel,
            replay_path: name,
            has_arrow_ipc_export: false,
        })
        .unwrap();
        // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
        match serde_wasm_bindgen::from_value::<SC2ReplaysDirStats>(
            invoke("basic_scan_replay_path", args).await,
        ) {
            Ok(stats) => {
                set_scan_button_enabled.set(true);
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
                set_scan_button_enabled.set(true);
            }
        }
    });
}

#[component]
pub fn ScanDirectory() -> impl IntoView {
    let (replay_path, set_replay_path) = signal(String::new());
    let (scan_button_enabled, set_scan_button_enabled) = signal(false);
    let (optimize_button_enabled, set_optimize_button_enabled) = signal(false);
    let (disable_parallel_scans, set_disable_parallel_scans) = signal(false);
    let (has_arrow_ipc_export, set_has_arrow_ipc_export) = signal(false);

    spawn_local(async move {
        // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
        match serde_wasm_bindgen::from_value::<AppSettings>(
            invoke_without_args("get_current_app_config").await,
        ) {
            Ok(config) => {
                console_log(&format!("Loaded app config: {:?}", config));
                set_disable_parallel_scans.set(config.disable_parallel_scans);
                set_replay_path.set(config.replay_path);
                set_has_arrow_ipc_export.set(config.has_arrow_ipc_export);
            }
            Err(e) => {
                console_log(&format!("Error invoking get_current_app_config: {:?}", e));
            }
        }
        set_scan_button_enabled.set(true);
        set_optimize_button_enabled.set(true);
    });
    let tx_update_replay_dir = move |ev| {
        let v = event_target_value(&ev);
        set_scan_button_enabled.set(v.len() > 0);
        set_optimize_button_enabled.set(v.len() > 0);
        set_replay_path.set(v);
    };
    let data = Store::new(SC2ReplaysDirStatsTable::from(SC2ReplaysDirStats::default()));

    view! {
        <div class="grid grid-cols-8 grid-rows-1 gap-1">
            <div class="col-span-5">
                <label class="input input-sm w-full">
                    "Path"
                    <input
                        class="input input-sm my-0 mx-0"
                        id="scan-directory-input"
                        value=move || replay_path.get()
                        on:input=tx_update_replay_dir
                        type="text"
                    />
                </label>
            </div>
            <div class="col-span-2 justify-start">
                <button
                    class={move || if replay_path.get().len() > 0 { "btn btn-primary btn-sm m-0" } else { "btn btn-disabled btn-sm m-0" }}
                    on:click=move |ev: MouseEvent| trigger_basic_scan_replay_path(
                        ev,
                        replay_path.clone(),
                        disable_parallel_scans.clone(),
                        set_scan_button_enabled.clone(),
                        data.clone(),
                    )
                    disabled= {move || !scan_button_enabled.get() }
                    title="Initial scan for StarCraft II replays">
                    {
                        move || if replay_path.get().len() > 0{
                            "Scan"
                        } else if scan_button_enabled.get() {
                            "Scanning..."
                        } else {
                            "Scan"
                        }
                    }
                </button>
                <button
                    class={move || if replay_path.get().len() > 0 { "btn btn-success btn-sm m-0" } else { "btn btn-disabled btn-sm m-0" }}
                    on:click=move |ev: MouseEvent| trigger_optimize_replay_path(
                        ev,
                        replay_path.clone(),
                        disable_parallel_scans.clone(),
                        set_optimize_button_enabled.clone(),
                    )
                    disabled= {move || !optimize_button_enabled.get() }
                    title="Optimize the replay generating Arrow files (may take some time)">
                    {
                        move || if replay_path.get().len() > 0 && !optimize_button_enabled.get() {
                            "Optimizing..."
                        } else {
                            "Optimize"
                        }
                    }
                </button>
            </div>
            <div class="col-span-1 flex justify-end">
                <label class="btn btn-sm btn-circle swap swap-rotate"
                    title=move || if disable_parallel_scans.get() {
                        "Enable Parallel Processing"
                    } else {
                        "Disable Parallel Processing"
                    }
                >
                    <input type="checkbox"
                        checked=move || disable_parallel_scans.get()
                        on:click=move |_| set_disable_parallel_scans.set(!disable_parallel_scans.get()) />
                    <Icon icon=CPU weight=IconWeight::Bold
                        prop:title=move || if disable_parallel_scans.get() {
                            "Parallel Processing Disabled"
                        } else {
                            "Parallel Processing Enabled"
                        }
                        prop:class=move || if disable_parallel_scans.get() {
                            "swap-on fill-current"
                        } else {
                            "swap-off fill-current"
                        }
                        color=move || if disable_parallel_scans.get() {
                            "orange"
                        } else {
                            "green"
                        }
                        />
                </label>
            </div>
        </div>
        <Show when={move || data.total_files().get() > 0}>
            <ReplayScanTable
                data
            />
         </Show>
    }
}

#[component]
pub fn ReplayScanTable(data: Store<SC2ReplaysDirStatsTable>) -> impl IntoView {
    view! {
        <div class="flex flex-row">
            <div class="flex-item basis-128">
                <h3 class="text-neutral-content">
                    "Replays: "
                    <div class="badge badge-sm badge-ghost">{move || data.total_files().get()}</div>
                </h3>
            </div>
            <div class="flex-item basis-128">
                <h3 class="text-info" title="Replays successfully parsed for GameEvents and TrackerEvents">
                    "Basic: "
                    <div class="badge badge-sm badge-info">{move || data.total_supported_replays().get()}</div>
                </h3>
            </div>
            <div class="flex-item basis-128">
                <h3 class="text-success" title="Replays with balance data available for abilities">
                    "Enhanced Support: "
                    <div class="badge badge-sm badge-success">{move || data.ability_supported_replays().get()}</div>
                </h3>
            </div>
        </div>
        <div class="flex gap-4">
            <div class="flex-item grow">
                <h2 class="text-neutral-content flex justify-center bg-gray-800">"Top 10 Players"</h2>
                <table class="table bg-gray-500 table-sm table-zebra">
                    <thead class="bg-gray-700">
                    <tr>
                        <th></th>
                        <th>Clan</th>
                        <th>Name</th>
                        <th>Total Games</th>
                    </tr>
                    </thead>
                    <tbody>
                        <For
                            each=move || data.top_10_players()
                            key=|row| row.read().name.clone()
                            children=|child| {
                                let idx = child.clone().idx();
                                let clan = child.clone().clan().clone();
                                let name = child.clone().name().clone();
                                let count = child.clone().count();
                                view! {
                                    <tr>
                                        <th>{move || idx.get()}</th>
                                        <td>{move || clan.get()}</td>
                                        <td>{move || name.get()}</td>
                                        <td>{move || count.get()}</td>
                                    </tr>
                                }
                            }
                        />
                    </tbody>
                </table>
            </div>
           <div class="flex-item grow">
                <h2 class="text-neutral-content flex justify-center bg-gray-800">"Top 10 Maps"</h2>
                <table class="table bg-gray-500 table-sm table-zebra">
                    <thead class="bg-gray-700">
                    <tr>
                        <th></th>
                        <th>Map Title</th>
                        <th>Total Games</th>
                    </tr>
                    </thead>
                    <tbody>
                        <For
                            each=move || data.top_10_maps()
                            key=|row| row.read().title.clone()
                            children=|child| {
                                let idx = child.clone().idx();
                                let title = child.clone().title().clone();
                                let count = child.clone().count();
                                view! {
                                    <tr>
                                        <th>{move || idx.get()}</th>
                                        <td>{move || title.get()}</td>
                                        <td>{move || count.get()}</td>
                                    </tr>
                                }
                            }
                        />
                    </tbody>
                </table>
                </div>
        </div>
    }
}
