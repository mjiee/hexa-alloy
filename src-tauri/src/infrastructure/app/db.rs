use std::path::PathBuf;

use crate::infrastructure::{database, migration};

pub(super) async fn init_database(data_dir: PathBuf) {
    if !data_dir.exists() {
        std::fs::create_dir_all(&data_dir).expect("Failed to create db dir");
    }

    database::init(data_dir.to_str().expect("db path error"))
        .await
        .expect("Failed to set default db connection");

    migration::run().await.expect("Failed to run migration");
}
