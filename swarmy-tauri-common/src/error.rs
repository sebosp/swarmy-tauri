use thiserror::Error;

#[derive(Error, Debug)]
pub enum SwarmyTauriError {
    // #[error("Store Error")]
    // TauriPluginStore(#[from] tauri_plugin_store::Error),
    #[error("StdError")]
    StdErr(#[from] Box<dyn std::error::Error>),
    #[error("S2proto Error")]
    S2ProtoErr(#[from] s2protocol::error::S2ProtocolError),
    #[error("Std Error")]
    StdIoErr(#[from] std::io::Error),
}

impl From<SwarmyTauriError> for String {
    fn from(err: SwarmyTauriError) -> Self {
        match err {
            // SwarmyTauriError::TauriPluginStore(e) => format!("Store Error: {}", e),
            SwarmyTauriError::StdErr(e) => format!("StdError: {}", e),
            SwarmyTauriError::StdIoErr(e) => format!("StdIoError: {}", e),
            SwarmyTauriError::S2ProtoErr(e) => format!("S2proto Error: {}", e),
        }
    }
}
