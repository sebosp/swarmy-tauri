use swarmy_tauri_common::*;
use crate::get_current_app_config;
pub mod data;
use data::try_query_map_stats;

#[tauri::command(rename_all = "snake_case")]
pub async fn query_map_stats(
    app_handle: tauri::AppHandle,
    map_title: String,
    player_name: String,
) -> ApiResponse{
    let app_config = match get_current_app_config(app_handle.clone()).await {
        Ok(config) => config,
        Err(e) => {
            return ApiResponse::new(
                ResponseMetaBuilder::new(false)
                    .duration_ms(0)
                    .build(),
                format!("Error getting current app config: {:?}", e),
            );
            }
    };
    let t = std::thread::spawn(move || {
        let init_time = std::time::Instant::now();
        let query = MapStatsQuery {
            map_title,
            player_name,
        };
        match try_query_map_stats(app_config.replay_path, query)
        {
            Ok(val) => ApiResponse::new(
            ResponseMetaBuilder::new(true)
                .duration_ms(init_time.elapsed().as_millis() as u64)
                .build(),
            serde_json::to_string(&val).unwrap_or_default(),
        ),
            Err(e) => {
            log::error!("Error query maps: {}", e);
        ApiResponse::new(
            ResponseMetaBuilder::new(false)
                .duration_ms(init_time.elapsed().as_millis() as u64)
                .build(),
            format!("Error querying maps: {:?}", e))
        }}
    });
    t.join().unwrap()
}
