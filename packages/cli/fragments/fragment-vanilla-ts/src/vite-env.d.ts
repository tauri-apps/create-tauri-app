export {};

declare global {
  interface Window {
    greet: () => Promise<void>;
  }
}
