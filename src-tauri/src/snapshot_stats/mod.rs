//! Provides information about the analyzed game collection.
use serde::{Deserialize, Serialize};
use swarmy_tauri_common::*;

/// Contains metadata information related to the minimun, maximum date of the snapshot taken, the number of
/// files analyzed, the number of maps and the number of players in the analyzed collection
#[derive(Debug, Serialize, Deserialize)]
pub struct SnapshotStats {
    /// The size of the IPC files
    pub directory_size: u64,
    /// The time of modification of the details IPC file.
    pub date_modified: std::time::SystemTime,
}

impl Default for SnapshotStats {
    fn default() -> Self {
        SnapshotStats {
            directory_size: 0,
            date_modified: std::time::SystemTime::UNIX_EPOCH,
        }
    }
}

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
                serde_json::to_string(&val).unwrap_or_default() ,
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
fn try_get_snapshot_metadata(replay_path: String) -> Result<SnapshotStats, SwarmyTauriError> {
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
    Ok(SnapshotStats {
        directory_size,
        date_modified,
    })
}
