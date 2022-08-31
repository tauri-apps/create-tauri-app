import type * as TauriApiTypes from "@tauri-apps/api";

declare global {
  interface Window {
    greet: () => void;
    __TAURI__: typeof TauriApiTypes;
  }
}
