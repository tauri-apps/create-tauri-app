// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  ssr: false, // Tauri does not support SSR
  srcDir: "src",
  css: ["~/assets/css/style.css"],
});
