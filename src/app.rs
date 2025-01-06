use leptos::prelude::*;
use leptos_router::components::*;
use wasm_bindgen::prelude::*;

use crate::pages::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    pub async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <main class="w-full h-full flex flex-col">
            <Header />
            <Router>
                <Content />
            </Router>
            <Footer />
        </main>
    }
}
