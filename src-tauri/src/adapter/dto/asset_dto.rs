// Asset
pub(crate) struct Asset {
    pub(crate) id: u32,
    pub(crate) name: String,
}

// Get user assets request
#[derive(serde::Deserialize)]
pub(crate) struct GetAssetsRequest {
    pub(crate) page: u64,
    pub(crate) size: u64,
}
