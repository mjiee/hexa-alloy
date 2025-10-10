import { call } from "@/utils/call";
import { Pagination, QueryResult } from "@/utils/shared";

export const exchangeOptions = [
  { value: "sh", label: "上证" },
  { value: "sz", label: "深证" },
];

export interface StockDetail {
  ticker: string; // stock ticker
  name: string; // stock name
  exchange: string; // stock exchange
  industry?: string; // stock industry
  listing_date?: string; // stock listing date
  quote?: StockPrice; // stock realtime quote
}

export interface StockPrice {
  ticker?: string; // stock ticker
  trade_date?: string; // trade date
  trade_time?: string; // trade time
  current_price?: number; // current price
  open_price?: number; // open price
  close_price?: number; // close price
  prev_close_price?: number; // previous close price
  high_price?: number; // high price
  low_price?: number; // low price
  change?: number; // price change
  change_percent?: number; // price change percentage
  volume?: number; // volume
  turnover?: number; // turnover
}

export interface QueryStocksRequest extends Pagination {
  ticker?: null | string;
}

export interface GetStockRequest {
  ticker: string;
}

export interface GetStockPricesRequest {
  ticker: string; // stock ticker
  days?: number; // number of days
}

export async function queryStocks() {
  return call<QueryStocksRequest, QueryResult<StockDetail>>("query_stocks", { page: 1, size: 25 });
}

export async function getStock(req: GetStockRequest) {
  return call<GetStockRequest, StockDetail>("get_stock", req);
}

export async function getStockPrices(req: GetStockPricesRequest) {
  return call<GetStockPricesRequest, StockPrice[]>("get_stock_prices", req);
}
