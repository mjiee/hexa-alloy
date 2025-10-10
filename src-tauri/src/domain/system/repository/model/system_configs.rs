use sea_orm::entity::prelude::*;

use crate::domain::system::entity;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "system_configs")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
    pub key: String,
    #[sea_orm(column_type = "Text")]
    pub value: serde_json::Value,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl From<entity::system_config::SystemConfig> for Model {
    fn from(entity: entity::system_config::SystemConfig) -> Self {
        Self {
            key: entity.key,
            value: entity.value,
        }
    }
}

impl Into<entity::system_config::SystemConfig> for Model {
    fn into(self) -> entity::system_config::SystemConfig {
        entity::system_config::SystemConfig {
            key: self.key,
            value: self.value,
        }
    }
}
