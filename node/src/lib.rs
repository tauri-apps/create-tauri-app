#[napi_derive::napi]
fn run(args: Vec<String>, bin_name: Option<String>) {
  create_tauri_app::run(args, bin_name);
}
