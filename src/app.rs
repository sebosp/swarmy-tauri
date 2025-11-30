use leptos::ev::SubmitEvent;
use leptos::leptos_dom::logging::console_log;
use leptos::prelude::*;
use leptos::task::spawn_local;
use reactive_graph::traits::{Read, Write};
use reactive_stores::{Patch, Store};
use s2protocol::cli::SC2ReplaysDirStats;
use serde::{Deserialize, Serialize};
use swarmy_tauri_ui::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct ReplaysDirectory<'a> {
    path: &'a str,
}

#[component]
pub fn App() -> impl IntoView {
    let (set_replay_path_r, set_replay_path_w) = signal(String::new());
    let (scan_button_enabled, set_scan_button_enabled) = signal(true);
    let fake_data = SC2ReplaysDirStats {
        total_files: 30,
        total_supported_replays: 20,
        ability_supported_replays: 10,
        top_10_maps: vec![],
        top_10_players: vec![
            (String::from("Player1"), 10),
            (String::from("Player2"), 8),
            (String::from("Player3"), 7),
            (String::from("Player4"), 3),
            (String::from("Player5"), 2),
        ],
    };
    let fake_data_table: SC2ReplaysDirStatsTable = fake_data.into();
    let data = Store::new(fake_data_table);

    let tx_update_replay_dir = move |ev| {
        let v = event_target_value(&ev);
        set_replay_path_w.set(v);
    };

    let set_replays_path = move |ev: SubmitEvent| {
        ev.prevent_default();
        let name = set_replay_path_r.get_untracked();
        if name.is_empty() {
            console_log("Replay path is empty.");
            return;
        }
        set_scan_button_enabled.set(false);

        spawn_local(async move {
            let args = serde_wasm_bindgen::to_value(&ReplaysDirectory { path: &name }).unwrap();
            // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
            match serde_wasm_bindgen::from_value::<SC2ReplaysDirStats>(
                invoke("set_replays_path", args).await,
            ) {
                Ok(stats) => {
                    console_log(&format!("Replays Directory Stats: {:?}", stats));
                    set_scan_button_enabled.set(true);
                    let mut stats_table: SC2ReplaysDirStatsTable = stats.into();
                    console_log(&format!("New data = {:?}", stats_table));
                    data.top_10_players().write().retain(|_| false);
                    data.top_10_players()
                        .write()
                        .append(&mut stats_table.top_10_players);
                    data.total_files().patch(stats_table.total_files);
                    data.total_supported_replays()
                        .patch(stats_table.total_supported_replays);
                    data.ability_supported_replays()
                        .patch(stats_table.ability_supported_replays);
                }
                Err(e) => {
                    console_log(&format!("Error invoking set_replays_path: {:?}", e));
                    set_scan_button_enabled.set(true);
                    return;
                }
            }
        });
    };

    view! {
        <div class="grid grid-cols-3 bg-base-800">
            <div class="col-span-3">
                <form
                    class="m-0"
                    on:submit=set_replays_path>
                    <input
                        class="input input-sm my-0 mx-0"
                        id="scan-directory-input"
                        on:input=tx_update_replay_dir
                    />
                    <button
                        class="btn btn-primary btn-sm m-0"
                        type="submit">
                        {
                            move || if !scan_button_enabled.get() {
                                "Scanning..."
                            } else {
                                "Scan Replays Path"
                            }
                        }
                    </button>
                    <p
                        style:display= {move || if scan_button_enabled.get() { "none" } else { "block" } }
                    >"Scanning..."</p>
                </form>
            </div>
            <div class="col-span-1">
                <h3 class="text-neutral-content">
                    "Total Replays: "
                    <div class="badge badge-sm badge-ghost">{move || data.total_files().get()}</div>
                </h3>
            </div>
            <div class="col-span-1">
                <h3 class="text-info">
                    "Supported Replays: "
                    <div class="badge badge-sm badge-info">{move || data.total_supported_replays().get()}</div>
                </h3>
            </div>
            <div class="col-span-1">
                <h3 class="text-success">
                    "Ability Replays: "
                    <div class="badge badge-sm badge-success">{move || data.ability_supported_replays().get()}</div>
                </h3>
            </div>
            <div class="col-span-3">
                <h2 class="text-neutral-content">"Top 10 Players"</h2>
            </div>
            <div class="col-span-3 rounded-box bg-base-100 border border-base-300">
                <table class="table table-sm table-zebra">
                    <thead>
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
        </div>


    }
}
