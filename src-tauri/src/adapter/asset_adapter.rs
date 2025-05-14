use crate::adapter::dto::asset_dto::GetAssetsRequest;
use crate::AppState;
use tauri::State;

// Asset Adapter
pub(crate) struct AssetAdapter {}

impl AssetAdapter {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

// Get user assets
#[tauri::command]
pub(crate) fn get_assets(data: GetAssetsRequest, state: State<AppState>) -> Vec<String> {
    println!("Hello, world!");

    vec!["asset1".to_string(), "asset2".to_string()]
}
