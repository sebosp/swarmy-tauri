//! Dataframe for map statistics.

use polars::prelude::*;
use swarmy_tauri_common::*;

pub fn try_query_map_stats(
    replay_path: String,
    query: MapStatsQuery,
) -> Result<MapStats, SwarmyTauriError> {
    let replay_path = format!("{}ipcs", replay_path.trim_end_matches('/').to_string(),);

    log::info!(
        "Querying map stats from replay path: {} for map_title: {} and player_name: {}",
        replay_path,
        query.map_title,
        query.player_name
    );
    let ipc_path = std::path::Path::new(&replay_path);
    if !ipc_path.exists() {
        return Err(SwarmyTauriError::Other(
            "Directory not Optimized yet, go to Scan first.".to_string(),
        ));
    }
    let details_ipc_filename = format!("{}/{}", replay_path, DETAILS_IPC);
    let date_modified = std::fs::metadata(details_ipc_filename)?.modified()?;
    let mut details_query = LazyFrame::scan_ipc(
        PlPath::new(&format!("{}/{}", replay_path, DETAILS_IPC)),
        Default::default(),
        Default::default(),
    )?;

    if !query.map_title.is_empty() {
        details_query = details_query.filter(col("map_title").eq(lit(query.map_title)));
    }
    if !query.player_name.is_empty() {
        details_query = details_query.filter(
            col("player_name")
                .str()
                .contains(lit(query.player_name), false),
        );
    }
    let res = details_query
        .group_by([
            col("ext_datetime"),
            col("ext_fs_id"),
            col("title"),
            col("cache_handles"),
        ])
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
    let min_date = col_ymd_to_naive_date(&res, "min_date")?;
    let max_date = col_ymd_to_naive_date(&res, "max_date")?;
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
    let num_games = res.column("num_games")?.u64()?.get(0).unwrap_or(0) + 1;
    Ok(MapStats {
        date_modified,
        max_date,
        min_date,
        num_games,
        title,
        cache_handles,
    })
}
