//! Dataframe for map statistics.

use polars::prelude::*;
use swarmy_tauri_common::*;

pub fn try_query_map_stats(
    replay_path: String,
    query: MapStatsQuery,
) -> Result<Vec<MapStats>, SwarmyTauriError> {
    if replay_path.is_empty() {
        return Err(SwarmyTauriError::Other(
            "Replay path is not set, please set it in Scan tab.".to_string(),
        ));
    }
    let replay_path = format!("{}/ipcs", replay_path.trim_end_matches('/'),);

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
    let mut details_query = LazyFrame::scan_ipc(
        PlPath::new(&format!("{}/{}", replay_path, DETAILS_IPC)),
        Default::default(),
        Default::default(),
    )?;

    if !query.map_title.is_empty() {
        details_query =
            details_query.filter(col("title").str().contains(lit(query.map_title), false));
    }
    if !query.player_name.is_empty() {
        details_query = details_query.filter(
            col("player_name")
                .str()
                .contains(lit(query.player_name), false),
        );
    }
    let res = details_query
        .group_by([col("title"), col("cache_handles")])
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
            len().alias("num_games"),
        ])
        .sort(
            ["num_games"],
            SortMultipleOptions::default()
                .with_order_descending(true)
                .with_nulls_last(true),
        )
        .limit(1000)
        .collect()?;
    println!("{res}");
    let res: Vec<MapStats> = (0..res.height())
        .map(|idx| extract_map_stats_from_df_row(&res.slice(idx as i64, 1)))
        .collect::<Result<_, _>>()?;
    Ok(res)
}

fn extract_map_stats_from_df_row(row: &DataFrame) -> Result<MapStats, SwarmyTauriError> {
    let min_date = col_ymd_to_naive_date(row, "min_date")?;
    let max_date = col_ymd_to_naive_date(row, "max_date")?;
    let title = row
        .column("title")?
        .str()?
        .get(0)
        .unwrap_or("Empty Title")
        .to_string();
    let cache_handles = row
        .column("cache_handles")?
        .str()?
        .get(0)
        .unwrap_or("")
        .to_string();
    let num_games = row.column("num_games")?.u32()?.get(0).unwrap_or(0);
    // The cache_handles have start information for the:
    // - "s2ma" filetype
    // - 2 letter region, i.e. US, EU, KR, etc.
    // - then the file hash, not sure this is unique across regions.
    /*❯ echo "73326d6100004555"|xxd -r -ps
    s2maEU⏎

    ❯ echo "73326d6100005553"|xxd -r -ps
    s2maUS⏎

    ❯ echo "73326d6100004b52"|xxd -r -ps
    s2maKR⏎

    ❯ echo "73326d6100004b52"|xxd -r -ps|xxd
    00000000: 7332 6d61 0000 4b52                      s2ma..KR
    */
    println!("Title: {}", title);
    println!("Cache handles: {}", cache_handles);
    Ok(MapStats {
        max_date,
        min_date,
        num_games,
        title,
        cache_handles,
    })
}
