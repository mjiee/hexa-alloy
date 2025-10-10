import { call } from "@/utils/call";
import { NewsDetail } from "./newsApi";
import { StockDetail, StockPrice } from "./stockApi";
import { Pagination, QueryResult } from "@/utils/shared";

export enum MarketImpact {
  All = "",
  Bullish = "Bullish",
  Bearish = "Bearish",
  Neutral = "Neutral",
}

export const marketImpactOptions = [
  { label: "全部", value: MarketImpact.All },
  { label: "利好", value: MarketImpact.Bullish },
  { label: "利空", value: MarketImpact.Bearish },
  { label: "中性", value: MarketImpact.Neutral },
];

export enum BullishCategory {
  PERFORMANCE_IMPROVEMENT = "PerformanceImprovement", // 业绩提升类
  POLICY_SUPPORT = "PolicySupport", // 政策利好类
  MERGER_ACQUISITION = "MergerAcquisition", // 并购重组类
  TECHNOLOGY_INNOVATION = "TechnologyInnovation", // 技术创新类
  MARKET_EXPANSION = "MarketExpansion", // 市场拓展类
  INDUSTRY_PROSPERITY = "IndustryProsperity", // 行业繁荣类
  HOT_CONCEPTS = "HotConcepts", // 热门概念类
  DIVIDENDS = "Dividends", // 分红派息类
  CAPITAL_OPERATION = "CapitalOperation", // 资本运作类
  QUALIFICATIONS_AWARDS = "QualificationsAwards", // 资质荣誉类
}

export enum BearishCategory {
  PERFORMANCE_DECLINE = "PerformanceDecline", // 业绩下降类
  RISK_EVENTS = "RiskEvents", // 风险事件类
  MARKET_PRESSURE = "MarketPressure", // 市场压力类
}

export const categoryOptions = [
  { label: "业绩提升", value: BullishCategory.PERFORMANCE_IMPROVEMENT },
  { label: "政策利好", value: BullishCategory.POLICY_SUPPORT },
  { label: "并购重组", value: BullishCategory.MERGER_ACQUISITION },
  { label: "技术创新", value: BullishCategory.TECHNOLOGY_INNOVATION },
  { label: "市场拓展", value: BullishCategory.MARKET_EXPANSION },
  { label: "行业繁荣", value: BullishCategory.INDUSTRY_PROSPERITY },
  { label: "热门概念", value: BullishCategory.HOT_CONCEPTS },
  { label: "分红派息", value: BullishCategory.DIVIDENDS },
  { label: "资本运作", value: BullishCategory.CAPITAL_OPERATION },
  { label: "资质荣誉", value: BullishCategory.QUALIFICATIONS_AWARDS },
  { label: "业绩下降", value: BearishCategory.PERFORMANCE_DECLINE },
  { label: "风险事件", value: BearishCategory.RISK_EVENTS },
  { label: "市场压力", value: BearishCategory.MARKET_PRESSURE },
];

export interface ReportDetail {
  id: number; // report id
  category: string; // report category
  market_impact: string; // market impact
  report_time: string; // report time
  stock_ticker: string; // stock ticker
  completed?: boolean; // report completed
  news?: NewsDetail; // news
  stock?: StockDetail; // stock
  prices?: StockPrice[]; // stock prices
}

export interface ReportWinRate {
  total?: number; // total win rate
  day1_win_rate?: number; // win rate day 1
  day2_win_rate?: number; // win rate day 2
  day3_win_rate?: number; // win rate day 3
  day4_win_rate?: number; // win rate day 4
  day5_win_rate?: number; // win rate day 5
  day6_win_rate?: number; // win rate day 6
  day7_win_rate?: number; // win rate day 7
}

export interface QueryReportsRequest extends Pagination {
  stock_ticker?: string; // stock ticker
  news_id?: number; // news id
  category?: string; // report category
  market_impact?: string; // market impact
  winning_streak?: number; // winning streak
  start_date?: string; // start date
  end_date?: string; // end date
}

export interface GetReportRequest {
  id: number;
}

export function queryReports(req: QueryReportsRequest) {
  return call<QueryReportsRequest, QueryResult<ReportDetail>>("query_reports", req);
}

export function getReport(req: GetReportRequest) {
  return call<GetReportRequest, ReportDetail>("get_report", req);
}

export function getReportWinRate(req: QueryReportsRequest) {
  return call<QueryReportsRequest, ReportWinRate>("get_report_win_rate", req);
}
