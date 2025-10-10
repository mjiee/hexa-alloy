use serde::ser;
use tauri::{App, Manager as _};

use crate::infrastructure::app::log::{init_log, LOG_GUARD};

mod db;
mod log;
mod service;

pub fn setup_app(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let log_dir = app.path().app_log_dir().expect("Failed to get app log dir");
    let data_dir = app.path().app_data_dir().expect("Failed to get app data dir");

    if LOG_GUARD.get().is_none() {
        init_log(log_dir);
    }

    tauri::async_runtime::spawn(async move {
        db::init_database(data_dir).await;
        service::init_service().await;
    });

    Ok(())
}
