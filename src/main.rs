mod app;
mod components;

use app::*;
use leptos::prelude::*;

fn main() {
    mount_to_body(|| {
        view! {
            <App/>
        }
    })
}
