import { createRouter, createWebHistory, RouteRecordRaw } from 'vue-router';
import Home from '../views/Home.vue';
import HardfaultLog from '../views/HardfaultLog.vue';
import AnalyzeThread from '../views/AnalyzeThread.vue';
import HciLog from '../views/HciLog.vue';
import MemoryTrace from '../views/MemoryTrace.vue';

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
    path: '/HciLog',
    name: 'HciLog',
    component: HciLog,
  },
  {
    path: '/MemoryTrace',
    name: 'MemoryTrace',
    component: MemoryTrace,
  },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;