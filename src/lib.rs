/// Swarmy Tauri Library
pub mod scan;
pub use scan::*;
use wasm_bindgen::prelude::*;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ReplaysDirectory<'a> {
    pub path: &'a str,
    pub serial: bool,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    pub async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}
