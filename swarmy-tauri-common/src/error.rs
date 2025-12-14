use thiserror::Error;

#[derive(Error, Debug)]
pub enum SwarmyTauriError {
    // #[error("Store Error")]
    // TauriPluginStore(#[from] tauri_plugin_store::Error),
    #[error("StdError")]
    StdErr(#[from] Box<dyn std::error::Error>),
}

impl From<SwarmyTauriError> for String {
    fn from(err: SwarmyTauriError) -> Self {
        match err {
            // SwarmyTauriError::TauriPluginStore(e) => format!("Store Error: {}", e),
            SwarmyTauriError::StdErr(e) => format!("StdError: {}", e),
        }
    }
}
