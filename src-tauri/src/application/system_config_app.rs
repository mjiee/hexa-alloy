use crate::{
    domain::system::service::system_config_service::SystemConfigService,
    infrastructure::{dto::system_config::SystemConfigDto, error::BasicResult},
};

pub struct SystemConfigApp {
    config_svc: SystemConfigService,
}

impl SystemConfigApp {
    pub fn new() -> Self {
        SystemConfigApp {
            config_svc: SystemConfigService::new(),
        }
    }
}

impl SystemConfigApp {
    pub async fn get_config<'a>(&self, key: String) -> BasicResult<'a, Option<SystemConfigDto>> {
        let config = self.config_svc.get_config(&key).await?;

        Ok(config.map(SystemConfigDto::from))
    }

    pub async fn save_config<'a>(&self, data: SystemConfigDto) -> BasicResult<'a, ()> {
        self.config_svc.save_config(data.into()).await
    }
}
