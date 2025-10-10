use sea_orm::{ConnectOptions, Database, DbConn};
use std::sync::OnceLock;
use std::time::Duration;

use crate::infrastructure::error::{BasicError, BasicResult};

static DB_CONN: OnceLock<DbConn> = OnceLock::new();

pub fn conn<'a>() -> BasicResult<'a, &'a DbConn> {
    let conn = DB_CONN
        .get()
        .ok_or(BasicError::InternalError(anyhow::anyhow!("database not init")))?;

    Ok(conn)
}

pub async fn init(db_path: &str) -> BasicResult<'_, ()> {
    let db_path = format!("sqlite://{db_path}/database.db?mode=rwc");

    let mut option = ConnectOptions::new(db_path);

    option
        .max_connections(20)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(60))
        .idle_timeout(Duration::from_secs(60))
        .sqlx_logging(false);

    let conn = Database::connect(option).await?;

    let _ = DB_CONN.set(conn);

    Ok(())
}
