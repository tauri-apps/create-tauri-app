// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  ssr: false, // Tauri does not support SSR
  telemetry: false,
  srcDir: "src",
  css: ["~/assets/css/style.css"],
});
