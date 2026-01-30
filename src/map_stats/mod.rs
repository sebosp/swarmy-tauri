//! Map stats module.

pub mod view;

use crate::*;
use leptos::leptos_dom::logging::console_log;
use serde::{Deserialize, Serialize};
use swarmy_tauri_common::*;
use leptos::prelude::*;
use leptos::task::spawn_local;
use reactive_stores::{Patch, Store};

#[derive(Store, Debug, Default, Clone, Serialize, Deserialize)]
pub struct MapStatsDataTable {
    pub total: usize,
    #[store(key: String = |row| format!("{}:{}",row.title.clone(), row.cache_handles.clone()))]
    pub data: Vec<MapStats>,
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
    data: Store<MapStatsDataTable>,
    query: MapStatsQuery,
    set_backend_response: WriteSignal<ApiResponse>,
) {
    *set_backend_response.write() = ApiResponse::new_incomplete();
    spawn_local(async move {
        console_log(&format!("Fetching map stats with query: {:?}", query));
        match fetch_query_map_stats(query).await{
            Err(e) => {
                console_log(&format!("Error fetching map stats: {:?}", e));
                *set_backend_response.write() = ApiResponse {
                    meta: ResponseMeta::incomplete(),
                    message: format!("Error fetching map stats: {:?}", e),
                };
            }
            Ok(response) => {
                console_log(&format!("1. Successfully fetched map stats: {:?}", response));
                *set_backend_response.write() = response.clone();
                let mut rows: Vec<MapStats> = serde_json::from_str(&response.message).unwrap_or_default();
                console_log(&format!("2. Successfully deserialized map stats: {:?}", rows));
                data.data().write().retain(|_| false);
                data.total().patch(rows.len());
                data.data().write().append(&mut rows);
            }
        }
    });
}
