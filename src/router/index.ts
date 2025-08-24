import { createRouter, createWebHistory, RouteRecordRaw } from 'vue-router';
import Home from '../views/Home.vue';
import ExceptionLog from '../views/ExceptionLog.vue';
import AnalyzeThread from '../views/AnalyzeThread.vue';
import HciLog from '../views/HciLog.vue';
import MemoryTrace from '../views/MemoryTrace.vue';
import NetworkSpeedTest from '../views/NetworkSpeedTest.vue';
import Settings from '../views/Settings.vue';
import BinaryConverter from '@/views/BinaryConverter.vue';
import AudioConverter from '@/views/AudioConverter.vue';

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
    path: '/NetworkSpeedTest',
    name: 'NetworkSpeedTest',
    component: NetworkSpeedTest,
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
  },
  {
    path: '/AudioConverter',
    name: 'AudioConverter',
    component: AudioConverter,
  }
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;