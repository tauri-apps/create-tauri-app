import { defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";{% if mobile %}
import { internalIpV4 } from "internal-ip";

// @ts-expect-error process is a nodejs global
const mobile = !!/android|ios/.exec(process.env.TAURI_ENV_PLATFORM);{% endif %}

// https://vitejs.dev/config/
export default defineConfig(async () => ({
  plugins: [svelte()],

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,{% if mobile %}
    host: mobile ? "0.0.0.0" : false,
    hmr: mobile
      ? {
          protocol: "ws",
          host: await internalIpV4(),
          port: 1421,
        }
      : undefined,{% endif %}
  }
}));
