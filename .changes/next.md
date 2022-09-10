---
"create-tauri-app": "patch"
"create-tauri-app-js": "patch"
---

Fix building in `next` and `next-ts` templates by removing the `experimental` option from `next.config.js` since `images.unoptimized` is now stable.