use std::path::PathBuf;

use swarmy_tauri_common::*;

pub mod data;

#[tauri::command(rename_all = "snake_case")]
pub async fn query_map_stats(
    _app_handle: tauri::AppHandle,
    query: MapStatsQuery,
) -> ApiResponse {
    // create a thread to scan the directory in the background:
    let t = std::thread::spawn(move || {
        let init_time = std::time::Instant::now();
        match try_query_map_stats(query)
        {
            Ok(val) => ApiResponse::new(
            ResponseMetaBuilder::new(true)
                .duration_ms(init_time.elapsed().as_millis() as u64)
                .build(),
            val,
        ),
            Err(e) => {
            log::error!("Error query maps: {}", e);
        ApiResponse::new(
            ResponseMetaBuilder::new(true)
                .duration_ms(init_time.elapsed().as_millis() as u64)
                .build(),
            format!("Error optimizing replays: {:?}", e))
        }}
    });
    t.join().unwrap()
}

fn try_query_map_stats(
    query: MapStatsQuery,
) -> Result<String, SwarmyTauriError> {
    let path = PathBuf::from(&query.replay_path);
    let destination = path.join("ipcs");
    if !destination.exists() {
        std::fs::create_dir_all(&destination)?;
    }
    log::info!(
        "Querying map stats from replay path: {} for map_title: {} and player_name: {}",
        query.replay_path,
        query.map_title,
        query.player_name
    );
    Ok("Not implemented yet".to_string())
}
