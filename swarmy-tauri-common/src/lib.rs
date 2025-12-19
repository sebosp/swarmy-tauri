pub mod settings;
pub use settings::*;
pub mod error;
pub use error::*;
pub mod response;
pub use response::*;

pub const DETAILS_IPC: &str = "details.ipc";
pub const INIT_DATA_IPC: &str = "init_data.ipc";
pub const UNIT_BORN_IPC: &str = "unit_born.ipc";
