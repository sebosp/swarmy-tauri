//! Leptos view for map stats.
use crate::*;
use leptos::leptos_dom::logging::console_log;
use leptos::prelude::*;
use leptos::task::spawn_local;
use reactive_stores::Store;
use serde::{Deserialize, Serialize};
use swarmy_tauri_common::*;

use s2protocol::details::PlayerLobbyDetails;

#[derive(Store, Debug, Default, Clone, Serialize, Deserialize)]
pub struct MapStatsDataFrame {
    pub total: usize,
    #[store(key: String = |row| row.title.clone())]
    pub res: Vec<PlayerLobbyDetails>,
    pub start: usize,
    pub end: usize,
    pub page: usize,
    pub per_page: usize,
}

async fn fetch_query_map_stats(
    query: MapStatsQuery,
) -> Result<ApiResponse, SwarmyTauriError> {
    let args = serde_wasm_bindgen::to_value(&query).unwrap();
    console_log(&format!(
        "Invoking fetch_query_map_stats with args: {:?}",
        args
    ));
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    let response =
        serde_wasm_bindgen::from_value::<ApiResponse>(invoke("query_map_stats", args).await)?;
    Ok(response)
}

fn trigger_fetch_query_map_stats(
    data: Store<MapStatsDataFrame>,
    query: ReadSignal<MapStatsQuery>,
    set_backend_response: WriteSignal<ApiResponse>,
) {
    let query_cp = query.get_untracked();
    spawn_local(async move {
        console_log("Fetching map stats...");
        match fetch_query_map_stats(query_cp).await{
            Err(e) => {
                console_log(&format!("Error fetching map stats: {:?}", e));
                *set_backend_response.write() = ApiResponse {
                    meta: ResponseMeta::incomplete(),
                    message: format!("Error fetching map stats: {:?}", e),
                };
            }
            Ok(response) => {
                *set_backend_response.write() = response.clone();
                *data.write() = serde_json::from_str(&response.message).unwrap_or_default();
            }
        }
    });
}

#[component]
pub fn StatsByMap() -> impl IntoView {
    let (query, set_query) = signal(MapStatsQuery::default());
    let (player_name, set_player_name) = signal(String::new());
    let (map_title, set_map_title) = signal(String::new());
    let (map_stats, set_map_stats) = signal(MapStatsDataFrame::default());
    let (backend_response, set_backend_response) = signal(ApiResponse {
        meta: ResponseMeta::incomplete(),
        message: String::new(),
    });
    let map_stats_data = Store::new(MapStatsDataFrame::default());
    trigger_fetch_query_map_stats(map_stats_data, query, set_backend_response);
    view! {
        <div>
            <div class="grid grid-cols-8 grid-rows-1 gap-1">
                <div class="col-span-3">
                    <label class="input input-sm">
                        <span class="label">"Map"</span>
                        <input
                            class="input input-sm my-0 mx-0"
                            value=move || map_title.get()
                            on:input=move |_| {
                                trigger_fetch_query_map_stats(
                                    map_stats_data,
                                    query,
                                    set_backend_response,
                                );
                            }
                            type="text"
                        />
                    </label>
                </div>
                <div class="col-span-1"></div>
                <div class="col-span-3">
                    <label class="input input-sm">
                        <span class="label">"Player"</span>
                        <input
                            class="input input-sm my-0 mx-0"
                            value=move || player_name.get()
                            on:input=move |_| {
                                trigger_fetch_query_map_stats(
                                    map_stats_data,
                                    query,
                                    set_backend_response,
                                );
                            }
                            type="text"
                        />
                    </label>
                </div>
                <div class="col-span-1"></div>
            </div>
            <DisplayBackendStatus backend_response />
            <Show when=move || { map_stats.get().total > 0 }>
                <MapStatsDataTable map_stats_data />
            </Show>
        </div>
    }
}

#[component]
pub fn MapStatsDataTable(map_stats_data: Store<MapStatsDataFrame>) -> impl IntoView {
    view! {
        <div class="overflow-x-auto w-full p-2">
            <table class="table w-full">
                <thead>
                    <tr>
                        <th>"Map Title"</th>
                        <th>"Games Played"</th>
                        <th>"Average Players"</th>
                    </tr>
                </thead>
                <tbody>
                    <For
                        each=move || map_stats_data.res()
                        key=|row| {
                            format!(
                                "{}:{}",
                                row.read().ext_fs_id.clone(),
                                row.read().user_init_data_name.clone(),
                            )
                        }
                        children=|child| {
                            view! {
                                <tr>
                                    <td>{child.read().title.clone()}</td>
                                    <td>{child.read().ext_datetime.to_string()}</td>
                                    <td>{child.read().user_init_data_clan_tag.clone()}</td>
                                    <td>{child.read().user_init_data_name.clone()}</td>
                                </tr>
                            }
                        }
                    />
                </tbody>
            </table>
        </div>
    }
}
