//! Swarmy Tauri Application

use leptos::ev::MouseEvent;
use leptos::leptos_dom::logging::console_log;
use leptos::prelude::*;
use leptos::task::spawn_local;
use phosphor_leptos::{Icon, IconWeight, CPU};
use reactive_graph::traits::Write;
use reactive_stores::{Patch, Store};
use s2protocol::SC2ReplaysDirStats;
use swarmy_tauri_ui::*;

#[component]
pub fn ScanDirectory() -> impl IntoView {
    let (replay_path, set_replay_path) = signal(String::new());
    let (scan_button_enabled, set_scan_button_enabled) = signal(false);
    let (enable_serial_processing, set_enable_serial_processing) = signal(false);

    let tx_update_replay_dir = move |ev| {
        let v = event_target_value(&ev);
        set_scan_button_enabled.set(v.len() > 0);
        set_replay_path.set(v);
    };
    let fake_data = SC2ReplaysDirStats {
        total_files: 0,
        total_supported_replays: 0,
        ability_supported_replays: 0,
        top_10_maps: vec![],
        top_10_players: vec![],
    };
    let fake_data_table: SC2ReplaysDirStatsTable = fake_data.into();
    let data = Store::new(fake_data_table);

    let trigger_basic_scan_replay_path = move |ev: MouseEvent| {
        ev.prevent_default();
        let name = replay_path.get_untracked();
        let enable_serial = enable_serial_processing.get_untracked();
        if name.is_empty() {
            console_log("Replay path is empty.");
            return;
        }
        set_scan_button_enabled.set(false);

        spawn_local(async move {
            let args = serde_wasm_bindgen::to_value(&ReplaysDirectory {
                path: &name,
                serial: enable_serial,
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
                    return;
                }
            }
        });
    };

    view! {
        <div class="grid grid-cols-8 grid-rows-1 gap-1">
            <div class="col-span-5">
                <label class="input input-sm w-full">
                    "Path"
                    <input
                        class="input input-sm my-0 mx-0"
                        id="scan-directory-input"
                        on:input=tx_update_replay_dir
                        type="text"
                    />
                </label>
            </div>
            <div class="col-span-2 justify-start">
                <button
                    class={move || if replay_path.get().len() > 0 { "btn btn-primary btn-sm m-0" } else { "btn btn-disabled btn-sm m-0" }}
                    on:click=trigger_basic_scan_replay_path
                    disabled= {move || !scan_button_enabled.get() }
                    title="Initial scanfor StarCraft II replays">
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
                    on:click=trigger_basic_scan_replay_path
                    disabled= {move || !scan_button_enabled.get() }
                    title="Optimize the replay generating Arrow files (may take some time)">
                    {
                        move || if replay_path.get().len() > 0 && !scan_button_enabled.get() {
                            "Optimizing..."
                        } else {
                            "Optimize"
                        }
                    }
                </button>
            </div>
            <div class="col-span-1 flex justify-end">
                <label class="label cursor-pointer" title="Enable serial processing (less CPU usage)">
                    <input type="checkbox"
                        prop:class=move || if enable_serial_processing.get() {
                            "btn btn-soft my-0 mx-0 btn-active"
                        } else {
                            "btn btn-soft my-0 mx-0"
                        }
                        checked=move || enable_serial_processing.get()
                        on:click=move |_| set_enable_serial_processing.set(!enable_serial_processing.get()) />
                        <span
                        title=move || if enable_serial_processing.get() {
                            "Serial Processing Enabled"
                        } else {
                            "Serial Processing Disabled"
                        }
                        prop:class=move || if enable_serial_processing.get() {
                            "label-text text-success mx-1"
                        } else {
                            "label-text text-warning-content mx-1"
                        } >
                        <Icon icon=CPU weight=IconWeight::Bold
                            color=move || if enable_serial_processing.get() {
                                "red"
                            } else {
                                "green"
                            }
                            />
                        </span>
                </label>
            </div>
        </div>
        <ReplayScanTable
            data
         />
    }
}

#[component]
pub fn Main() -> impl IntoView {
    view! {
    <div class="flex flex-row h-screen bg-gray-800">
      <div class="flex flex-col items-center w-16 h-full overflow-hidden text-gray-400 bg-gray-900 rounded">
        <a class="flex items-center justify-center mt-3" href="#">
            <svg class="w-8 h-8 fill-current" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 32 32" fill="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M 3.04,10.7C 4.74,7.81 12.18,5.92 10.99,11.19 9.19,15.57 7.14,19.85 5.35,24.25 5.03,25.35 2.41,30.52 3.77,28.76 9.17,21.67 14.58,14.58 19.99,7.49 17.84,5.42 15.69,3.35 13.55,1.27 8.35,1.81 5.92,7.1 3.04,10.7Z" />
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="m 30.44,10.7c -1.71,-2.89 -9.15,-4.78 -7.95,0.48 1.8,4.38 3.85,8.67 5.64,13.07 0.32,1.1 2.94,6.27 1.58,4.5C 24.3,21.67 18.89,14.58 13.49,7.49 15.63,5.42 17.78,3.35 19.93,1.27c 5.2,0.53 7.63,5.82 10.51,9.43z" />
            </svg>
        </a>
        <div class="flex flex-col items-center mt-3 border-t border-purple-700">
            <a class="flex items-center justify-center w-12 h-12 mt-2 rounded hover:bg-gray-700 hover:text-gray-300" href="#">
                <svg class="w-6 h-6 stroke-current" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                     <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6" />
                </svg>
            </a>
            <a class="flex items-center justify-center w-12 h-12 mt-2 rounded hover:bg-gray-700 hover:text-gray-300" href="#">
                <svg class="w-6 h-6 stroke-current" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
                </svg>
            </a>
            <a class="flex items-center justify-center w-12 h-12 mt-2 text-gray-200 bg-gray-700 rounded" href="#">
                <svg class="w-6 h-6 stroke-current" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 8v8m-4-5v5m-4-2v2m-2 4h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z" />
                </svg>
            </a>
            <a class="flex items-center justify-center w-12 h-12 mt-2 rounded hover:bg-gray-700 hover:text-gray-300" href="#">
                <svg class="w-6 h-6 stroke-current" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7v8a2 2 0 002 2h6M8 7V5a2 2 0 012-2h4.586a1 1 0 01.707.293l4.414 4.414a1 1 0 01.293.707V15a2 2 0 01-2 2h-2M8 7H6a2 2 0 00-2 2v10a2 2 0 002 2h8a2 2 0 002-2v-2" />
                </svg>
            </a>
        </div>
        <div class="flex flex-col items-center mt-2 border-t border-purple-700">
            <a class="flex items-center justify-center w-12 h-12 mt-2 rounded hover:bg-gray-700 hover:text-gray-300" href="#">
                <svg class="w-6 h-6 stroke-current"  xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6V4m0 2a2 2 0 100 4m0-4a2 2 0 110 4m-6 8a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4m6 6v10m6-2a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4" />
                </svg>
            </a>
            <a class="relative flex items-center justify-center w-12 h-12 mt-2 rounded hover:bg-gray-700 hover:text-gray-300" href="#">
                <svg class="w-6 h-6 stroke-current" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 8h10M7 12h4m1 8l-4-4H5a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v8a2 2 0 01-2 2h-3l-4 4z" />
                </svg>
                <span class="absolute top-0 left-0 w-2 h-2 mt-2 ml-2 bg-indigo-500 rounded-full"></span>
            </a>
        </div>
      </div>
      <div class="flex-grow p-2 overflow-auto">
        <ScanDirectory />
      </div>
    </div>
    }
}
