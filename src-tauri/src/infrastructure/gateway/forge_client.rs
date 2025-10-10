use std::{sync::OnceLock, time::Duration};

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use url::Url;

use crate::infrastructure::{
    dto::{
        news::{GetNewsRequest, NewsDetail, QueryNewsRequest},
        report::{GetReportRequest, QueryReportsRequest, ReportDetail, ReportWinRate},
        stock::{AddStockRequest, GetStockRequest, QueryStocksRequest, StockDetail},
        stock_price::{GetStockPricesRequest, StockPrice},
    },
    error::BasicResult,
    shared::{QueryResult, Response},
};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub host: String,
    pub enable: bool,
}

static CLIENT: OnceLock<ForgeClient> = OnceLock::new();

pub fn client<'a>() -> Option<&'a ForgeClient> {
    CLIENT.get()
}

pub fn init<'a>(config: Config) -> BasicResult<'a, ()> {
    if !config.enable {
        return Ok(());
    }

    let client = ForgeClient::new(&config.host)?;
    let _ = CLIENT.set(client);
    Ok(())
}

pub struct ForgeClient {
    client: reqwest::Client,
    base_url: Url,
}

impl ForgeClient {
    pub fn new<'a>(host: &str) -> BasicResult<'a, Self> {
        let base_url = Url::parse(host)?;
        let client = reqwest::Client::builder()
            .pool_idle_timeout(Duration::from_secs(60))
            .pool_max_idle_per_host(5)
            .build()?;
        Ok(ForgeClient { client, base_url })
    }
}

impl ForgeClient {
    async fn get<'a, P, T>(&self, path: &str, param: P) -> BasicResult<'a, Response<T>>
    where
        P: Serialize,
        T: DeserializeOwned,
    {
        let url = self.base_url.join(path)?;
        let resp = self.client.get(url).query(&param).send().await?.json::<Response<T>>().await?;
        Ok(resp)
    }

    async fn post<'a, B, T>(&self, path: &str, body: B) -> BasicResult<'a, Response<T>>
    where
        B: Serialize,
        T: DeserializeOwned,
    {
        let url = self.base_url.join(path)?;
        let resp = self.client.post(url).json(&body).send().await?.json::<Response<T>>().await?;
        Ok(resp)
    }
}

impl ForgeClient {
    pub async fn query_stocks<'a>(&self, req: QueryStocksRequest) -> BasicResult<'a, Response<QueryResult<StockDetail>>> {
        self.get("/api/stock/query", req).await
    }

    pub async fn get_stock<'a>(&self, req: GetStockRequest) -> BasicResult<'a, Response<StockDetail>> {
        self.get("/api/stock/detail", req).await
    }

    pub async fn get_stock_prices<'a>(&self, req: GetStockPricesRequest) -> BasicResult<'a, Response<Vec<StockPrice>>> {
        self.get("/api/stock/prices", req).await
    }

    pub async fn add_stock<'a>(&self, req: AddStockRequest) -> BasicResult<'a, Response<()>> {
        self.post("/api/stock/add", req).await
    }

    pub async fn query_news<'a>(&self, req: QueryNewsRequest) -> BasicResult<'a, Response<QueryResult<NewsDetail>>> {
        self.get("/api/news/query", req).await
    }

    pub async fn get_news<'a>(&self, req: GetNewsRequest) -> BasicResult<'a, Response<NewsDetail>> {
        self.get("/api/news/detail", req).await
    }

    pub async fn query_reports<'a>(&self, req: QueryReportsRequest) -> BasicResult<'a, Response<QueryResult<ReportDetail>>> {
        self.get("/api/report/query", req).await
    }

    pub async fn get_report<'a>(&self, req: GetReportRequest) -> BasicResult<'a, Response<ReportDetail>> {
        self.get("/api/report/detail", req).await
    }

    pub async fn get_report_win_rate<'a>(&self, req: QueryReportsRequest) -> BasicResult<'a, Response<ReportWinRate>> {
        self.get("/api/report/win_rate", req).await
    }
}
