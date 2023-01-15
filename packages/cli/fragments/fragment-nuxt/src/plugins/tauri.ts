import { reactive } from "vue";
import { invoke } from "@tauri-apps/api";
import { defineNuxtPlugin } from "#app";

export default defineNuxtPlugin((nuxtApp) => {
  const tauri = reactive({
    invoke,
  });
  nuxtApp.provide("tauri", tauri);
});
