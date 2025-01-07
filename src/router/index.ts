import { createRouter, createWebHistory, RouteRecordRaw } from 'vue-router';
import Home from '../views/Home.vue';
import ExceptionLog from '../views/App1.vue';
import App2 from '../views/App2.vue';
import App3 from '../views/App3.vue';

const routes: Array<RouteRecordRaw> = [
  {
    path: '/',
    name: 'Home',
    component: Home,
  },
  {
    path: '/app1',
    name: 'ExceptionLog',
    component: ExceptionLog,
  },
  {
    path: '/app2',
    name: 'App2',
    component: App2,
  },
  {
    path: '/app3',
    name: 'App3',
    component: App3,
  },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;