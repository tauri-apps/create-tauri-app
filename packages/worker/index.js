import sh from "./scripts/create-tauri-app.sh";
import ps from "./scripts/create-tauri-app.ps1";
import index from "./index.html";

export default {
  async fetch(request) {
    const { pathname, searchParams } = new URL(request.url);

    if (
      pathname.startsWith("/sh") ||
      pathname.startsWith("/create-tauri-app.sh")
    ) {
      return new Response(sh, {
        headers: { "Content-Type": "text/plain" },
      });
    }

    if (
      pathname.startsWith("/ps") ||
      pathname.startsWith("/create-tauri-app.ps1")
    ) {
      return new Response(ps, {
        headers: { "Content-Type": "text/plain" },
      });
    }

    if (pathname.startsWith("/bin")) {
      const res = await fetch(
        `https://github.com/tauri-apps/create-tauri-app/releases/download/${searchParams.get(
          "tag"
        )}/create-tauri-app-${searchParams.get("arch")}${searchParams.get(
          "ext"
        )}`
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
