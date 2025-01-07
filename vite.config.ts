import { defineConfig, loadEnv } from "vite";
import vue from "@vitejs/plugin-vue";
import path from 'path';
import version from 'vite-plugin-package-version';

const host = process.env.TAURI_DEV_HOST;

// https://vitejs.dev/config/
export default defineConfig(async () => {
  const env = loadEnv(process.env.NODE_ENV || 'development', process.cwd());

  return {

    plugins: [vue(), version()],
    resolve: {
      alias: {
        '@': path.resolve(__dirname, './src'), // 确保配置了 @ 别名
      },
    },
    define: {
      __APP_VERSION__: JSON.stringify(process.env.npm_package_version || '未知'), // 注入版本号
      __BUILD_MODE__: JSON.stringify(process.env.NODE_ENV || 'development'), // 注入构建模式
      __BUILD_TIME__: JSON.stringify(new Date().toLocaleString()), // 注入编译时间
    },

    // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
    //
    // 1. prevent vite from obscuring rust errors
    clearScreen: false,
    // 2. tauri expects a fixed port, fail if that port is not available
    server: {
      port: 1420,
      strictPort: true,
      host: host || false,
      hmr: host
        ? {
          protocol: "ws",
          host,
          port: 1421,
        }
        : undefined,
      watch: {
        // 3. tell vite to ignore watching `src-tauri`
        ignored: ["**/src-tauri/**"],
      },
    },
  }
});
