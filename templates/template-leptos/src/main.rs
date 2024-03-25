mod app;

use app::*;
use leptos::*;
use console_error_panic_hook;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! {
            <App/>
        }
    })
}
