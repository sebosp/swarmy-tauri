use swarmy_tauri_common::*;
use std::path::PathBuf;
use crate::data::*;
use polars::prelude::*;

#[tauri::command(rename_all = "snake_case")]
pub async fn download_replay_caches(
    _app_handle: tauri::AppHandle,
    replay_path: String,
) -> ApiResponse {
    // create a thread to scan the directory in the background:
    let t = std::thread::spawn(move || {
        let init_time = std::time::Instant::now();
        match try_download_replay_caches(replay_path)
        {
            Ok(val) => ApiResponse::new(
            ResponseMetaBuilder::new(true)
                .duration_ms(init_time.elapsed().as_millis() as u64)
                .build(),
            val,
        ),
            Err(e) => {
            log::error!("Error download caches: {}", e);
        ApiResponse::new(
            ResponseMetaBuilder::new(true)
                .duration_ms(init_time.elapsed().as_millis() as u64)
                .build(),
            format!("Error download caches: {:?}", e))
        }}
    });
    t.join().unwrap()
}

fn try_download_replay_caches(
    replay_path: String,
) -> Result<String, SwarmyTauriError> {

    let replay_path = sanitize_replay_path(&replay_path)?;
    let ipc_path = build_ipc_path(&replay_path)?;

    let cache_path = PathBuf::from(&replay_path);
    let destination = cache_path.join("caches");
    if !destination.exists() {
        std::fs::create_dir_all(&destination)?;
    }
    log::info!(
        "Downloading replay caches from files in {} and storing into {}",
        cache_path.display(),
        destination.display()
    );
    let mut details_query = LazyFrame::scan_ipc(
        PlPath::new(&format!("{}/{}", ipc_path, DETAILS_IPC)),
        Default::default(),
        Default::default(),
    )?;
    let res = details_query.unique(
        Some(Selector::Matches("cache_handles".into())),
        UniqueKeepStrategy::Any,
    )
        .limit(1000)
    .collect()?;
    // TODO: Maybe we don't need the cache region or s2ma format...
    let mut unique_cache_handles: Vec<String> = vec![];
    for idx in 0..res.height() {
        let slice = res.slice(idx as i64, 1);
        let cache_handles_str = slice.column("cache_handles")?.str()?.get(0).unwrap_or("");
        let cache_handles: Vec<&str> = cache_handles_str.split(',').collect();
        for handle in cache_handles {
            if unique_cache_handles.contains(&handle.to_string()) {
                continue;
            }
            unique_cache_handles.push(handle.to_string());
        }
    }
    Ok(String::from("Download caches finished successfully."))
}
