use crate::{
    domain::system::service::system_config_service::SystemConfigService,
    infrastructure::{gateway::forge_client, shared::config_key},
};

pub(super) async fn init_service<'a>() {
    let config_svc = SystemConfigService::new();

    let config = config_svc
        .get_config(config_key::FORGE_CLIENT)
        .await
        .expect("get forge client config failed");

    if let Some(config) = config {
        let config = config
            .into_value::<forge_client::Config>()
            .expect("init forge client config failed");

        forge_client::init(config).expect("init forge client failed");
    }
}
