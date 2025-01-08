mod app;
mod components;
mod icons;
mod pages;

use leptos::prelude::*;

use app::*;

fn main() {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    mount_to_body(|| {
        view! {
            <App/>
        }
    })
}
