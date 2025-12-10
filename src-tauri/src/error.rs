use thiserror::Error;

#[derive(Error, Debug)]
pub enum SwarmyTauriError {
    #[error("Store Error")]
    TauriPluginStore(#[from] tauri_plugin_store::Error),
    #[error("Stdre Error")]
    StdErr(#[from] Box<dyn std::error::Error>),
}
