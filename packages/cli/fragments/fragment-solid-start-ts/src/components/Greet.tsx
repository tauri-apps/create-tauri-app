import {createSignal} from "solid-js";

export default function Greet() {
    const [greetMsg, setGreetMsg] = createSignal("");
    const [name, setName] = createSignal("");

    async function greet() {
        const {invoke} = await import("@tauri-apps/api/tauri");
        // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
        setGreetMsg(await invoke("greet", {name: name()}));
    }


    return (
        <>
            <div class="row">
                <div>
                    <input
                        id="greet-input"
                        onChange={(e) => setName(e.currentTarget.value)}
                        placeholder="Enter a name..."/>
                    <button type="button" onClick={() => greet()}>
                        Greet
                    </button>
                </div>
            </div>
            <p>{greetMsg}</p>
        </>
    );
}
