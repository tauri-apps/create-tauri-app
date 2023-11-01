// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

{% if not_mobile %}// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}{% endif %}

fn main() {
    {% if mobile %}{% lib_name %}::run(){% else %}tauri::Builder::default()
        {% if alpha %}.plugin(tauri_plugin_window::init())
        .plugin(tauri_plugin_shell::init())
        {% endif %}.invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");{% endif %}
}
