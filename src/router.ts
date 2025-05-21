import { createWebHistory, createRouter, RouteRecordRaw } from "vue-router";
import { AppLayout } from "@/components";
import { HomePage, MarketPage, MessageListPage, ProfilePage } from "@/pages";

const routes: RouteRecordRaw[] = [
  {
    path: "/",
    component: AppLayout,
    children: [
      { path: "", component: HomePage },
      { path: "market", component: MarketPage },
      { path: "message", component: MessageListPage },
      { path: "profile", component: ProfilePage },
    ],
  },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;
