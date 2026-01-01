//! Leptos view for map stats.
use leptos::prelude::*;
use reactive_stores::Store;
use swarmy_tauri_common::*;
use crate::*;
use leptos::task::spawn_local;
use serde::{Deserialize, Serialize};
use leptos::leptos_dom::logging::console_log;
use phosphor_leptos::{Icon, IconWeight, X_CIRCLE};

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


async fn fetch_query_map_stats(query: MapStatsQuery) -> Result<MapStatsDataFrame, SwarmyTauriError> {
    let args = serde_wasm_bindgen::to_value(&query).unwrap();
    console_log(&format!(
        "Invoking fetch_query_map_stats with args: {:?}",
        args
    ));
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    let response = serde_wasm_bindgen::from_value::<ApiResponse>(
        invoke("query_map_stats", args).await,
    )?;
            Ok(serde_json::from_str(&response.message)?)
}

fn trigger_fetch_query_map_stats(data: Store<MapStatsDataFrame>, query: ReadSignal<MapStatsQuery>) {
    let query_cp = query.get_untracked();
    spawn_local(async move {
        *data.write() = fetch_query_map_stats(query_cp).await.unwrap_or_default();
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
    view! {
        <div class="grid grid-cols-8 grid-rows-1 gap-1">
            <div class="col-span-3">
                <label class="input input-sm">
                    <span class="label">"Map"</span>
                    <input
                        class="input input-sm my-0 mx-0"
                        value=move || map_title.get()
                        on:input=move |_| {
                            trigger_fetch_query_map_stats(map_stats_data, query);
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
                            trigger_fetch_query_map_stats(map_stats_data, query);
                        }
                        type="text"
                    />
                </label>
            </div>
            <div class="col-span-3"></div>
            <Show when=move || {
                !backend_response.get().meta.success && backend_response.get().meta.is_complete
            }>
                <div role="alert" class="alert alert-error shadow-lg m-1 p-1">
                    <Icon icon=X_CIRCLE weight=IconWeight::Bold prop:class="stroke-current" />
                    <span>{backend_response.get().message.clone()}</span>
                </div>
            </Show>
            <Show when=move || { map_stats.get().total > 0 }>
                <MapStatsDataTable map_stats_data />
            </Show>
        </div>
    }
}

#[component]
pub fn MapStatsDataTable(map_stats_data: Store<MapStatsDataFrame>) -> impl IntoView {
    view! {}
}
