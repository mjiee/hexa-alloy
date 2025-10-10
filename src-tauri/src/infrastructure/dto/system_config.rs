use serde::{Deserialize, Serialize};

use crate::domain::system::entity;

#[derive(Serialize, Deserialize)]
pub struct GetConfigRequest {
    pub key: String,
}

#[derive(Serialize, Deserialize)]
pub struct SystemConfigDto {
    pub key: String,
    pub value: serde_json::Value,
}

impl From<entity::system_config::SystemConfig> for SystemConfigDto {
    fn from(entity: entity::system_config::SystemConfig) -> Self {
        Self {
            key: entity.key,
            value: entity.value,
        }
    }
}

impl Into<entity::system_config::SystemConfig> for SystemConfigDto {
    fn into(self) -> entity::system_config::SystemConfig {
        entity::system_config::SystemConfig {
            key: self.key,
            value: self.value,
        }
    }
}
