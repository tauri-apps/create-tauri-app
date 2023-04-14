use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use dioxus::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

pub fn app(cx: Scope) -> Element {
    let greet_msg = use_state(cx, || String::new());

    let onsubmit = move |e: FormEvent| {
        to_owned![greet_msg];
        cx.spawn(async move {
            let name = e.values.get("name").unwrap();
            let args = to_value(&GreetArgs { name }).unwrap();
            // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
            let new_msg = invoke("greet", args).await.as_string().unwrap();
            greet_msg.set(new_msg);
        });
    };

    render! (
        style { include_str!("./styles.css") },
        main {
            class: "container",
            div {
                a {
                    href: "https://tauri.app",
                    target: "_blank",
                    img {
                        src: "tauri.svg",
                        class: "logo tauri",
                        alt: "Tauri logo"
                    }
                 }
                a { 
                    href: "https://dioxuslabs.com/",
                    target: "_blank",
                    img {
                        src: "dioxus.png",
                        class: "logo dioxus",
                        alt: "Dioxus logo"
                    }
                }
            }
            p {
                "Click on the Tauri and Dioxus logos to learn more."
            }
            p {
                "Recommended IDE Setup: "
                a { 
                    href: "https://code.visualstudio.com/",
                    target: "_blank",
                    "VS Code"
                }
                " + ",
                a {
                    href: "https://github.com/tauri-apps/tauri-vscode" ,
                    target: "_blank",
                    "Tauri"
                }
                " + ",
                a {
                    href: "https://github.com/rust-lang/rust-analyzer",
                    target: "_blank",
                    "rust-analyzer"
                }
            }
            form {
                class: "row",
                onsubmit: onsubmit,
                prevent_default: "onsubmit",
                input {
                    name: "name",
                    id: "greet-input",
                    placeholder: "Enter a name...",
                }
                button {
                   "type": "submit",
                   "Submit"
                }
            }
            p {
                b {
                    "{greet_msg}"
                }
            }
        }
    )
}
