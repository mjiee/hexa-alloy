use crate::{
    domain::system::{entity::system_config::SystemConfig, repository::SystemConfigRepo},
    infrastructure::error::BasicResult,
};

pub struct SystemConfigService {
    config_repo: SystemConfigRepo,
}

impl SystemConfigService {
    pub fn new() -> Self {
        Self {
            config_repo: SystemConfigRepo,
        }
    }
}

impl SystemConfigService {
    pub async fn get_config<'a>(&self, key: &str) -> BasicResult<'a, Option<SystemConfig>> {
        self.config_repo.get(key).await
    }

    pub async fn save_config<'a>(&self, data: SystemConfig) -> BasicResult<'a, ()> {
        self.config_repo.save(data).await
    }
}
