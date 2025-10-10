use std::sync::OnceLock;

use crate::infrastructure::{
    dto::{
        stock::{AddStockRequest, GetStockRequest, QueryStocksRequest, StockDetail},
        stock_price::{GetStockPricesRequest, StockPrice},
    },
    error::ERROR_FORGE_CLIENT_ERROR,
    gateway,
    shared::{QueryResult, RespResult},
};

static ADAPTER: OnceLock<StockAdapter> = OnceLock::new();

fn adapter<'a>() -> &'a StockAdapter {
    ADAPTER.get_or_init(StockAdapter::new)
}

/// Stock adapter
pub struct StockAdapter {}

impl StockAdapter {
    pub fn new() -> Self {
        Self {}
    }
}

#[tauri::command]
pub async fn query_stocks<'a>(req: QueryStocksRequest) -> RespResult<'a, QueryResult<StockDetail>> {
    if let Some(client) = gateway::client() {
        return Ok(client.query_stocks(req).await?);
    }

    Err(ERROR_FORGE_CLIENT_ERROR)
}

#[tauri::command]
pub async fn get_stock<'a>(req: GetStockRequest) -> RespResult<'a, StockDetail> {
    if let Some(client) = gateway::client() {
        return Ok(client.get_stock(req).await?);
    }

    Err(ERROR_FORGE_CLIENT_ERROR)
}

#[tauri::command]
pub async fn get_stock_prices<'a>(req: GetStockPricesRequest) -> RespResult<'a, Vec<StockPrice>> {
    if let Some(client) = gateway::client() {
        return Ok(client.get_stock_prices(req).await?);
    }

    Err(ERROR_FORGE_CLIENT_ERROR)
}

#[tauri::command]
pub async fn add_stock<'a>(req: AddStockRequest) -> RespResult<'a, ()> {
    if let Some(client) = gateway::client() {
        return Ok(client.add_stock(req).await?);
    }

    Err(ERROR_FORGE_CLIENT_ERROR)
}
