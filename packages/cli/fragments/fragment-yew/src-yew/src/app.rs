use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[function_component(App)]
pub fn app() -> Html {
    let greet_input_ref = use_ref(|| NodeRef::default());

    let name = use_state(|| String::new());

    let greet_msg = use_state(|| String::new());
    {
        let greet_msg = greet_msg.clone();
        let name = name.clone();
        let name2 = name.clone();
        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    if name.is_empty() {
                        return;
                    }

                    let new_msg = invoke(
                        "greet",
                        JsValue::from_serde(&GreetArgs { name: &*name }).unwrap(),
                    )
                    .await;
                    log(&new_msg.as_string().unwrap());
                    greet_msg.set(new_msg.as_string().unwrap());
                });

                || {}
            },
            name2,
        );
    }

    let greet = {
        let name = name.clone();
        let greet_input_ref = greet_input_ref.clone();
        Callback::from(move |_| {
            name.set(greet_input_ref.cast::<web_sys::HtmlInputElement>().unwrap().value());
        })
    };

    html! {
        <main id="app">
            <div>
                <a href="https://tauri.app" target="_blank">
                    <img src="public/tauri.svg" class="logo" alt="Tauri logo"/>
                </a>
                <a href="https://yew.rs" target="_blank">
                    <img src="public/yew.png" class="logo yew" alt="Yew logo"/>
                </a>
            </div>

            <div class="card">
                <input id="greet-input" ref={&*greet_input_ref} placeholder="Enter a name..." />
                <button type="button" onclick={greet}>{"Greet"}</button>
            </div>

            <p>{ &*greet_msg }</p>


            <p>
                {"Edit "}
                <code>{"src-yew/src/app.rs"}</code> {" to test yew hot reload."}
            </p>
            <p>
                {"Edit "}
                <code>{"src-tauri/src/main.rs"}</code> {" to test app hot reload."}
            </p>

            <p>
                {"Recommended IDE setup: "}
                <a href="https://code.visualstudio.com/" target="_blank">{"VS Code"}</a>
                {" + "}
                <a href="https://github.com/tauri-apps/tauri-vscode" target="_blank">{"Tauri"}</a>
                {" + "}
                <a href="https://github.com/rust-lang/rust-analyzer" target="_blank">{"rust-analyzer"}</a>
            </p>
            <p class="read-the-docs">{"Click on the Tauri and Yew logos to learn more"}</p>
        </main>
    }
}
