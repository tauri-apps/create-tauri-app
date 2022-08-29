---
"create-tauri-app": patch
---

# Fixed yew template

Changed "beforeDevCommand" from "trunk build" to "trunk server".
Before when you called "tauri dev" infinite loop will occur waiting for dev server to become available at "http://localhost:1420".
This would not happen because "trunk build" only builds artifacts and don't actually start serving them.

Changed "withGlobalTauri" from "false" to "true" so example frontend can actually "invoke" backend methods
