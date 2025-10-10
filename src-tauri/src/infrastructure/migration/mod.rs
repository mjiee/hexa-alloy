use sea_orm::ConnectionTrait;

use crate::infrastructure::{database, error::BasicResult};

mod v1;

pub async fn run<'a>() -> BasicResult<'a, ()> {
    let sqls = vec![v1::SQL];
    let db = database::conn()?;

    for sql in sqls {
        db.execute_unprepared(sql).await?;
    }

    Ok(())
}
