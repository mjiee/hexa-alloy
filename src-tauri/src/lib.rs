use crate::{
    adapter::{news, stock, stock_report, system_config},
    infrastructure::app::setup_app,
};

mod adapter;
mod application;
mod domain;
mod infrastructure;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(setup_app)
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            stock::query_stocks,
            stock::get_stock,
            stock::get_stock_prices,
            stock::add_stock,
            news::query_news,
            news::get_news,
            stock_report::query_reports,
            stock_report::get_report,
            stock_report::get_report_win_rate,
            system_config::save_config,
            system_config::get_config,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
