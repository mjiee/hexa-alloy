use serde::{Deserialize, Serialize};

use crate::infrastructure::dto::stock::StockDetail;

#[derive(Serialize, Deserialize)]
pub struct QueryNewsRequest {
    page: u64,                     // Page
    size: u64,                     // Size
    stock_ticker: Option<String>,  // stock ticker
    market_impact: Option<String>, // market impact
    start_date: Option<String>,    // start publish time
    end_date: Option<String>,      // end publish time
}

#[derive(Serialize, Deserialize)]
pub struct NewsDetail {
    id: i64,                       // news id
    title: String,                 // news title
    link: Option<String>,          // news link
    publish_time: Option<String>,  // news publish time
    source: Option<String>,        // news source
    contents: Option<Vec<String>>, // news content
    market_impact: Option<String>, // news market impact
    category: Option<String>,      // news category
    stock: Option<StockDetail>,    // news stock
}

#[derive(Serialize, Deserialize)]
pub struct GetNewsRequest {
    id: i64, // Id
}
