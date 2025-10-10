use serde::{Deserialize, Serialize};

use crate::infrastructure::dto::stock_price::StockPrice;

#[derive(Serialize, Deserialize)]
pub struct QueryStocksRequest {
    page: u64,
    size: u64,
    ticker: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct StockDetail {
    ticker: String,               // stock ticker
    name: String,                 // stock name
    exchange: String,             // stock exchange
    listing_date: Option<String>, // stock listing date
    industry: Option<String>,     // stock industry
    quote: Option<StockPrice>,    // stock realtime quote
}

#[derive(Serialize, Deserialize)]
pub struct AddStockRequest {
    ticker: String,
    name: String,
    listing_date: String,
    exchange: String,
    industry: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct GetStockRequest {
    ticker: String, // stock ticker
}
