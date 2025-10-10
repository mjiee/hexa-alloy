use serde::{Deserialize, Serialize};

/// StockPrice, stock price
#[derive(Serialize, Deserialize)]
pub struct StockPrice {
    ticker: Option<String>,        // stock ticker
    trade_date: Option<String>,    // trade date
    trade_time: Option<String>,    // trade time
    open_price: Option<f64>,       // open price
    high_price: Option<f64>,       // high price
    low_price: Option<f64>,        // low price
    close_price: Option<f64>,      // close price
    current_price: Option<f64>,    // current price
    prev_close_price: Option<f64>, // previous close price
    change: Option<f64>,           // price change
    change_percent: Option<f64>,   // price change percent
    volume: Option<i64>,           // volume
    turnover: Option<f64>,         // turnover
}

#[derive(Serialize, Deserialize)]
pub struct GetStockPricesRequest {
    ticker: String,    // stock ticker
    days: Option<i64>, // days
}
