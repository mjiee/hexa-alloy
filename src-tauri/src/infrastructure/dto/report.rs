use serde::{Deserialize, Serialize};

use crate::infrastructure::dto::{news::NewsDetail, stock::StockDetail, stock_price::StockPrice};

// ReportDetail, report detail
#[derive(Serialize, Deserialize)]
pub struct ReportDetail {
    id: i64,                         // report id
    category: String,                // report category
    report_time: String,             // report time
    stock_ticker: String,            // stock ticker
    market_impact: String,           // market impact
    completed: Option<bool>,         // report completed
    news: Option<NewsDetail>,        // news
    prices: Option<Vec<StockPrice>>, // stock prices
    stock: Option<StockDetail>,      // stock
}

// ReportWinRate, win rate
#[derive(Serialize, Deserialize)]
pub struct ReportWinRate {
    total: Option<i64>,         // total
    day1_win_rate: Option<f64>, // win rate
    day2_win_rate: Option<f64>, // win rate
    day3_win_rate: Option<f64>, // win rate
    day4_win_rate: Option<f64>, // win rate
    day5_win_rate: Option<f64>, // win rate
    day6_win_rate: Option<f64>, // win rate
    day7_win_rate: Option<f64>, // win rate
}

#[derive(Serialize, Deserialize)]
pub struct QueryReportsRequest {
    page: Option<i64>,             // page
    size: Option<i64>,             // size
    category: Option<String>,      // report category
    news_id: Option<i64>,          // news id
    stock_ticker: Option<String>,  // stock ticker
    start_date: Option<String>,    // start date
    end_date: Option<String>,      // end date
    market_impact: Option<String>, // market impact
    winning_streak: Option<i64>,   // winning streak
}

#[derive(Serialize, Deserialize)]
pub struct GetReportRequest {
    id: i64, // report id
}
