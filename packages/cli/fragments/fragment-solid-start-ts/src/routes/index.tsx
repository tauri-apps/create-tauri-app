import {Title} from "solid-start";
import Greet from "~/components/Greet";

export default function Home() {
    return (
        <main>
            <Title>Welcome to Tauri!</Title>
            <h1>Welcome to Tauri!</h1>

            <div class="row">
                <a href="https://vitejs.dev" target="_blank">
                    <img src="/vite.svg" class="logo vite" alt="Vite logo"/>
                </a>
                <a href="https://tauri.app" target="_blank">
                    <img src="/tauri.svg" class="logo tauri" alt="Tauri logo"/>
                </a>
                <a href="https://start.solidjs.com" target="_blank">
                    <img src="/solid.svg" class="logo solid-start" alt="Solid-Start logo"/>
                </a>
            </div>

            <p>Click on the Tauri, Vite, and Solid-Start logos to learn more.</p>
            <Greet/>
        </main>
    );
}
