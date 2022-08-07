import sh from "../../create-tauri-app.sh";
import ps from "../../create-tauri-app.ps1";
import index from "./index.html";

export default {
  async fetch(request) {
    const { pathname, searchParams } = new URL(request.url);

    if (pathname.startsWith("/sh")) {
      return new Response(sh, {
        headers: { "Content-Type": "text/plain" },
      });
    }

    if (pathname.startsWith("/ps")) {
      return new Response(ps, {
        headers: { "Content-Type": "text/plain" },
      });
    }

    if (pathname.startsWith("/download/bin")) {
      const tag = searchParams.get("tag");
      const arch = searchParams.get("arch");
      const ext = searchParams.get("ext");
      const res = await fetch(
        `https://github.com/tauri-apps/create-tauri-app/releases/download/${tag}/create-tauri-app-${arch}${ext}`
      );

      return new Response(await res.arrayBuffer(), {
        headers: {
          "Content-Type": "application/octet-stream",
          "Content-disposition": `attachment; filename=create-taui-app${searchParams.get(
            "ext"
          )}`,
        },
      });
    }

    return new Response(index, {
      headers: { "Content-Type": "text/html" },
    });
  },
};
