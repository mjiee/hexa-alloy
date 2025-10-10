import { call } from "@/utils/call";
import { Pagination, QueryResult } from "@/utils/shared";
import { StockDetail } from "./stockApi";

export interface NewsDetail {
  id: number; // news id
  title: string; // news title
  contents?: string[]; // news content
  link?: string; // news link
  source?: string; // news source
  publish_time?: string; // news publish time
  market_impact?: string; // news market impact
  category?: string; // news category
  stock?: StockDetail; // news stock
}

export interface QueryNewsRequest extends Pagination {
  stock_ticker?: string; // stock ticker
  market_impact?: string; // market impact
  start_date?: string; // start date
  end_date?: string; // end date
}

export interface GetNewsRequest {
  id: number;
}

export async function queryNews(req: QueryNewsRequest) {
  return call<QueryNewsRequest, QueryResult<NewsDetail>>("query_news", req);
}

export async function getNews(req: GetNewsRequest) {
  return call<GetNewsRequest, NewsDetail>("get_news", req);
}
