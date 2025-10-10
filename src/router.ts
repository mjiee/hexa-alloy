import { createWebHistory, createRouter, RouteRecordRaw } from "vue-router";
import { AppLayout } from "./components";
import {
  NewsListPage,
  NewsDetailPage,
  ReportListPage,
  ReportDetailPage,
  StockDetailPage,
  SystemConfigPage,
} from "@/pages";

const routes: RouteRecordRaw[] = [
  {
    path: "/",
    component: AppLayout,
    children: [
      { path: "", component: NewsListPage },
      { path: "news/:id", component: NewsDetailPage },
      { path: "reports", component: ReportListPage },
      { path: "report/:id", component: ReportDetailPage },
      { path: "stock/:ticker", component: StockDetailPage },
      { path: "system/config", component: SystemConfigPage },
    ],
  },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;
