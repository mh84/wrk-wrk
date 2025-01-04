mod app;
mod components;
mod icons;

use app::*;
use leptos::prelude::*;

fn main() {
    mount_to_body(|| {
        view! {
            <App/>
        }
    })
}
