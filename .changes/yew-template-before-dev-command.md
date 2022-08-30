---
"create-tauri-app": patch
---

Fixed yew template "beforeDevCommand" from "trunk build" to "trunk serve". Before when you called "tauri dev" infinite loop will occur waiting for dev server to become available at "http://localhost:1420".

