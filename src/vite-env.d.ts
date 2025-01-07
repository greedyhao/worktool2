/// <reference types="vite/client" />

declare module "*.vue" {
  import type { DefineComponent } from "vue";
  const component: DefineComponent<{}, {}, any>;
  export default component;
}

declare const __APP_VERSION__: string;
declare const __BUILD_MODE__: string;
declare const __BUILD_TIME__: string;
