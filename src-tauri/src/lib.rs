mod adapter;
mod application;
mod domain;
mod infrastructure;

struct AppState {
    asset_adapter: adapter::asset_adapter::AssetAdapter,
}

impl AppState {
    fn new() -> Self {
        Self {
            asset_adapter: adapter::asset_adapter::AssetAdapter::new(),
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState::new())
        .invoke_handler(tauri::generate_handler![adapter::asset_adapter::get_assets])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
