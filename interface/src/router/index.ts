import { createRouter, createWebHistory } from 'vue-router'
import type { RouteRecordRaw } from "vue-router";
import Inventory from "./inventory"
import Home from "../views/HomePage/Home.vue";
import Allrights from "../views/HomePage/Allrights.vue";
import Aboutus from "../views/HomePage/Aboutus.vue";
import Vision from "../views/HomePage/Vision.vue";
import Contactus from "../views/HomePage/Contactus.vue";

import Welcome from "../views/Content/Welcome.vue";
import NewArticle from "../views/Content/NewArticle.vue";
import Protocol from "../views/Content/ProtocolList.vue";
import Login from "../views/Content/Login.vue";
import Register from "../views/Content/Register.vue";
import Logout from "../views/Content/Logout.vue";
import Product from "../views/Content/ProductsList.vue";
import Profile from "../views/Content/Profile.vue";

import openfordev from "./controller"


let routes: Array<RouteRecordRaw> = [
  {
    path: '/',
    name: 'home',
    component: Home,
  },

  {
    path: "/login",
    component: Login,
  },
  {
    path: "/register",
    component: Register,
  },
  {
    path: "/allrights",
    component: Allrights,
  },
  {
    path: "/aboutus",
    component: Aboutus,
  },
  {
    path: "/vision",
    component: Vision,
  },
  {
    path: "/contactus",
    component: Contactus,
  },
  {
    path: "/",
    component: Home,
    meta: {
      requiredLogin: openfordev,
    },
  },
  {
    path: "/welcome",
    component: Welcome,
  },

  {
    path: "/profile",
    component: Profile,
    meta: {
      requiredLogin: openfordev,
    },
  },

];

routes = routes.concat(
  Inventory
)


const router = createRouter({
  history: createWebHistory(),
  routes,
})

router.beforeEach((to, from, next) => {
  const isLogin = false;
  if (to.meta.requiredLogin && !isLogin) {
    next("/login");
  } else if (to.meta.redirectAlreadyLogin && isLogin) {
    next("/");
  } else {
    next();
  }
});

export default router
