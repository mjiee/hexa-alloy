use std::sync::OnceLock;

use crate::{
    application::system_config_app,
    infrastructure::{
        dto::system_config::{GetConfigRequest, SystemConfigDto},
        shared::{ok, RespResult},
    },
};

static ADAPTER: OnceLock<ConfigAdapter> = OnceLock::new();

fn adapter<'a>() -> &'a ConfigAdapter {
    ADAPTER.get_or_init(ConfigAdapter::new)
}

pub struct ConfigAdapter {
    config_app: system_config_app::SystemConfigApp,
}

impl ConfigAdapter {
    pub fn new() -> Self {
        Self {
            config_app: system_config_app::SystemConfigApp::new(),
        }
    }
}

#[tauri::command]
pub async fn get_config<'a>(req: GetConfigRequest) -> RespResult<'a, Option<SystemConfigDto>> {
    ok(adapter().config_app.get_config(req.key).await?)
}

#[tauri::command]
pub async fn save_config<'a>(req: SystemConfigDto) -> RespResult<'a, ()> {
    ok(adapter().config_app.save_config(req).await?)
}
