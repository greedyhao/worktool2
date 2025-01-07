import { createRouter, createWebHistory, RouteRecordRaw } from 'vue-router';
import Home from '../views/Home.vue';
import HardfaultLog from '../views/HardfaultLog.vue';
import AnalyzeThread from '../views/AnalyzeThread.vue';
import App3 from '../views/App3.vue';

const routes: Array<RouteRecordRaw> = [
  {
    path: '/',
    name: 'Home',
    component: Home,
  },
  {
    path: '/HardfaultLog',
    name: 'HardfaultLog',
    component: HardfaultLog,
  },
  {
    path: '/AnalyzeThread',
    name: 'AnalyzeThread',
    component: AnalyzeThread,
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