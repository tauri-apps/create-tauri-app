import sh from "../create-tauri-app.sh";
import ps1 from "../create-tauri-app.ps1";

const filesList = `
    <h1> Index of create.tauri.app </h1>
    <hr>
        <pre>
<a href="/sh">create-tauri-app.sh</a>
<a href="/ps1">create-tauri-app.ps1</a>
        </pre>
    <hr>
`;

export default {
  async fetch(request) {
    const { pathname } = new URL(request.url);

    if (pathname.startsWith("/sh")) {
      return new Response(sh, {
        headers: { "Content-Type": "text/plain" },
      });
    }

    if (pathname.startsWith("/ps1")) {
      return new Response(ps1, {
        headers: { "Content-Type": "text/plain" },
      });
    }

    return new Response(filesList, {
      headers: { "Content-Type": "text/html" },
    });
  },
};
