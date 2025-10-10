use std::sync::OnceLock;

use crate::infrastructure::{
    dto::news::{GetNewsRequest, NewsDetail, QueryNewsRequest},
    error::ERROR_FORGE_CLIENT_ERROR,
    gateway,
    shared::{QueryResult, RespResult},
};

static ADAPTER: OnceLock<NewsAdapter> = OnceLock::new();

fn adapter<'a>() -> &'a NewsAdapter {
    ADAPTER.get_or_init(NewsAdapter::new)
}

/// News adapter
pub struct NewsAdapter {}

impl NewsAdapter {
    pub fn new() -> Self {
        Self {}
    }
}

#[tauri::command]
pub async fn query_news<'a>(req: QueryNewsRequest) -> RespResult<'a, QueryResult<NewsDetail>> {
    if let Some(client) = gateway::client() {
        return Ok(client.query_news(req).await?);
    }

    Err(ERROR_FORGE_CLIENT_ERROR)
}

#[tauri::command]
pub async fn get_news<'a>(req: GetNewsRequest) -> RespResult<'a, NewsDetail> {
    if let Some(client) = gateway::client() {
        return Ok(client.get_news(req).await?);
    }

    Err(ERROR_FORGE_CLIENT_ERROR)
}
