use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel};

use crate::{
    domain::system::{
        entity::system_config::SystemConfig,
        repository::model::{system_configs, SystemConfigModel, SystemConfigs},
    },
    infrastructure::{database::conn, error::BasicResult},
};

pub struct SystemConfigRepo;

impl SystemConfigRepo {
    pub async fn get<'a>(&self, key: &str) -> BasicResult<'a, Option<SystemConfig>> {
        let data = SystemConfigs::find_by_id(key).one(conn()?).await?;

        Ok(data.map(|a| a.into()))
    }

    pub async fn save<'a>(&self, data: SystemConfig) -> BasicResult<'a, ()> {
        SystemConfigs::insert(SystemConfigModel::from(data).into_active_model().clone())
            .on_conflict(
                sea_orm::sea_query::OnConflict::column(system_configs::Column::Key)
                    .update_columns(vec![system_configs::Column::Value])
                    .to_owned(),
            )
            .exec(conn()?)
            .await?;

        Ok(())
    }
}
