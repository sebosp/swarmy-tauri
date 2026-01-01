//! Dataframe for map statistics.

use polars::prelude::*;
use swarmy_tauri_common::*;

#[tauri::command(rename_all = "snake_case")]
pub async fn get_map_list(replay_path: String, player_name: String) -> ApiResponse {
    let t = std::thread::spawn(move || {
        let init_time = std::time::Instant::now();
        match try_get_map_list(replay_path, player_name) {
            Ok(val) => ApiResponse::new(
                ResponseMetaBuilder::new(true)
                    .duration_ms(init_time.elapsed().as_millis() as u64)
                    .build(),
                serde_json::to_string(&val).unwrap_or_default(),
            ),
            Err(e) => {
                log::error!("Error getting snapshot metadata: {}", e);
                ApiResponse::new(
                    ResponseMetaBuilder::new(false)
                        .duration_ms(init_time.elapsed().as_millis() as u64)
                        .build(),
                    format!("Error getting snapshot metadata: {:?}", e),
                )
            }
        }
    });
    t.join().unwrap()
}

/// Gets the list of maps from the details.ipc file
pub fn try_get_map_list(replay_path: String, player_name: String) -> Result<MapStats, SwarmyTauriError> {
    let replay_path = format!("{}/ipcs/", replay_path);
    log::info!("Getting map list from: {}", replay_path);
    // Add the size of all the files in state.source_dir
    let mut directory_size = 0;
    for entry in std::fs::read_dir(&replay_path)? {
        let entry = entry?;
        let path = entry.path();
        let metadata = std::fs::metadata(path)?;
        let size = metadata.len();
        directory_size += size;
    }
    // get the date_modified of the details.ipc file
    let details_ipc_filename = format!("{}/{}", replay_path, DETAILS_IPC);
    let date_modified = std::fs::metadata(details_ipc_filename)?.modified()?;
    let mut details_query = LazyFrame::scan_ipc(
        PlPath::new(&format!("{}/{}", replay_path, DETAILS_IPC)),
        Default::default(),
        Default::default(),
    )?;

    if !player_name.is_empty() {
        details_query = details_query.filter(col("player").str().contains_literal(lit(player_name)));
    }

    let res = details_query
        .group_by([col("ext_datetime"), col("ext_fs_id"), col("title"), col("cache_handles")])
        .agg([
            col("ext_datetime")
                .min()
                .dt()
                .to_string("%Y-%m-%d")
                .alias("min_date"),
            col("ext_datetime")
                .max()
                .dt()
                .to_string("%Y-%m-%d")
                .alias("max_date"),
            col("title")
                .unique()
                .dt()
                .to_string("%Y-%m-%d")
                .alias("max_date"),
            col("ext_fs_id").count().alias("num_games"),
        ])
        .collect()?;

    let min_date_str = res
        .column("min_date")?
        .str()?
        .get(0)
        .unwrap_or("1970-01-01");
    let max_date_str = res
        .column("max_date")?
        .str()?
        .get(0)
        .unwrap_or("1970-01-01");
    let num_games = res.column("num_games")?.u64()?.get(0).unwrap_or(0) + 1;
    let min_date = chrono::NaiveDate::parse_from_str(min_date_str, "%Y-%m-%d")
        .unwrap_or(chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap());
    let max_date = chrono::NaiveDate::parse_from_str(max_date_str, "%Y-%m-%d")
        .unwrap_or(chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap());
    let title = res
        .column("title")?
        .str()?
        .get(0)
        .unwrap_or("Unknown Map")
        .to_string();
    let cache_handles = res
        .column("cache_handles")?
        .str()?
        .get(0)
        .unwrap_or("")
        .to_string();

    Ok(MapStats {
        directory_size,
        date_modified,
        max_date,
        min_date,
        num_games,
        title,
        cache_handles,
    })
}
