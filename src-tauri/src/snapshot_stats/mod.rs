//! Provides information about the analyzed game collection.
use polars::prelude::*;
use swarmy_tauri_common::*;

#[tauri::command(rename_all = "snake_case")]
pub async fn get_snapshot_metadata(replay_path: String) -> ApiResponse {
    // create a thread to get the metadata in the background:
    let t = std::thread::spawn(move || {
        let init_time = std::time::Instant::now();
        match try_get_snapshot_metadata(replay_path) {
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
pub fn try_get_snapshot_metadata(replay_path: String) -> Result<SnapshotStats, SwarmyTauriError> {
    let replay_path = format!("{}/ipcs/", replay_path);
    log::info!("Getting snapshot metadata from: {}", replay_path);
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
    let details_query = LazyFrame::scan_ipc(
        PlPath::new(&format!("{}/{}", replay_path, DETAILS_IPC)),
        Default::default(),
        Default::default(),
    )?;

    // Date range from the details.ipc file
    let res = details_query
        .select([
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
            col("ext_fs_id").max().alias("num_games"),
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
    // let data_str = crate::common::convert_df_to_json_data(&res)?;

    Ok(SnapshotStats {
        directory_size,
        date_modified,
        max_date,
        min_date,
        num_games,
        num_maps: 0,
    })
}
