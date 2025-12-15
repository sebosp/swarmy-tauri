#[derive(Debug, Clone)]
pub struct ResponseMeta {
    pub success: bool,
    pub duration_ms: u64,
}

#[derive(Debug, Clone)]
pub struct ApiResponse {
    pub meta: ResponseMeta,
    pub message: String,
}
