import { createRouter, createWebHistory } from "vue-router";
import { AppPage } from "./page";

import Welcome from "../views/Welcome.vue";
const Create = () => import("../views/Create.vue");
const Join = () => import("../views/Join.vue");
const WaitingRoom = () => import("../views/WaitingRoom.vue");
const Voting = () => import("../views/Voting.vue");
const Results = () => import("../views/Results.vue");

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: "/welcome",
      name: AppPage.Welcome,
      component: Welcome,
    },
    {
      path: "/create",
      name: AppPage.Create,
      component: Create,
    },
    {
      path: "/join",
      name: AppPage.Join,
      component: Join,
    },
    {
      path: "/waiting-room",
      name: AppPage.WaitingRoom,
      component: WaitingRoom,
    },
    {
      path: "/voting",
      name: AppPage.Voting,
      component: Voting,
    },
    {
      path: "/results",
      name: AppPage.Results,
      component: Results,
    },
    {
      path: "/:unknown*",
      name: "unknown",
      redirect: "/welcome",
    },
  ],
});

export default router;
