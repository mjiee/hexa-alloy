import { createWebHistory, createRouter, RouteRecordRaw } from "vue-router";
import { AppLayout } from "@/components";
import {
  HomePage,
  MarketPage,
  MessageListPage,
  MessageDetailPage,
  ProfilePage,
  AssetsPage,
  TransactionsPage,
  StockPage,
} from "@/pages";

const routes: RouteRecordRaw[] = [
  {
    path: "/",
    component: AppLayout,
    children: [
      { path: "", component: HomePage },
      { path: "assets", component: AssetsPage },
      { path: "transactions", component: TransactionsPage },
      { path: "market", component: MarketPage },
      { path: "stock", component: StockPage },
      { path: "message", component: MessageListPage },
      { path: "message/detail", component: MessageDetailPage },
      { path: "profile", component: ProfilePage },
    ],
  },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;
