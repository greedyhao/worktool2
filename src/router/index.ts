import { createRouter, createWebHistory, RouteRecordRaw } from 'vue-router';
import Home from '../views/Home.vue';
import ExceptionLog from '../views/ExceptionLog.vue';
import AnalyzeThread from '../views/AnalyzeThread.vue';
import HciLog from '../views/HciLog.vue';
import MemoryTrace from '../views/MemoryTrace.vue';
import NetTool from '../views/NetTool.vue';
import Settings from '../views/Settings.vue';
import BinaryConverter from '@/views/BinaryConverter.vue';

const routes: Array<RouteRecordRaw> = [
  {
    path: '/',
    name: 'Home',
    component: Home,
  },
  {
    path: '/ExceptionLog',
    name: 'ExceptionLog',
    component: ExceptionLog,
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
  {
    path: '/NetTool',
    name: 'NetTool',
    component: NetTool,
  },
  {
    path: '/Settings',
    name: 'Settings',
    component: Settings,
  },
  {
    path: '/BinaryConverter',
    name: 'BinaryConverter',
    component: BinaryConverter,
  }
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;