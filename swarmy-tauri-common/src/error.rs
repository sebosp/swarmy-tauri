use thiserror::Error;

#[derive(Error, Debug)]
pub enum SwarmyTauriError {
    // #[error("Store Error")]
    // TauriPluginStore(#[from] tauri_plugin_store::Error),
    #[error("StdError")]
    StdErr(#[from] Box<dyn std::error::Error>),
    #[error("S2proto Error")]
    S2ProtoErr(#[from] s2protocol::error::S2ProtocolError),
    #[error(transparent)]
    StdIo(#[from] std::io::Error),

    #[cfg(not(target_arch = "wasm32"))]
    #[error("Polars Error: {0}")]
    Polars(#[from] polars::error::PolarsError),

    #[error("UTF8 Error: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),
    #[error("Other Error: {0}")]
    Other(String),
}

impl From<SwarmyTauriError> for String {
    fn from(err: SwarmyTauriError) -> Self {
        match err {
            // SwarmyTauriError::TauriPluginStore(e) => format!("Store Error: {}", e),
            SwarmyTauriError::StdErr(e) => format!("StdError: {}", e),
            SwarmyTauriError::StdIo(e) => format!("StdIoError: {}", e),
            SwarmyTauriError::S2ProtoErr(e) => format!("S2proto Error: {}", e),

            #[cfg(not(target_arch = "wasm32"))]
            SwarmyTauriError::Polars(e) => format!("Polars Error: {}", e),

            SwarmyTauriError::Utf8(e) => format!("UTF8 Error: {}", e),
            SwarmyTauriError::Other(e) => format!("Other Error: {}", e),
        }
    }
}

#[derive(serde::Serialize)]
#[serde(tag = "kind", content = "message")]
#[serde(rename_all = "camelCase")]
enum ErrorKind {
    StdErr(String),
    Io(String),
    Utf8(String),
    S2Proto(String),

    #[cfg(not(target_arch = "wasm32"))]
    Polars(String),

    Other(String),
}

impl serde::Serialize for SwarmyTauriError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let error_message = self.to_string();
        let error_kind = match self {
            Self::StdErr(_) => ErrorKind::StdErr(error_message),
            Self::StdIo(_) => ErrorKind::Io(error_message),
            Self::Utf8(_) => ErrorKind::Utf8(error_message),
            Self::S2ProtoErr(_) => ErrorKind::S2Proto(error_message),

            #[cfg(not(target_arch = "wasm32"))]
            Self::Polars(_) => ErrorKind::Polars(error_message),

            Self::Other(_) => ErrorKind::Other(error_message),
        };
        error_kind.serialize(serializer)
    }
}
