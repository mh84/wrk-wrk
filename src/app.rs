use crate::components::*;

use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn App() -> impl IntoView {
    let current_path = RwSignal::new("/");

    view! {
        <Router>
            <main class="w-full h-full flex flex-col">
                <Header />
                <div class="px-4 grow shadow-lg">
                    <Routes fallback=NotFound>
                        <Route path=path!("/") view=Current />
                        <Route path=path!("/day") view=Day />
                        <Route path=path!("/week") view=Week />
                        <Route path=path!("/tasks") view=Tasks />
                    </Routes>
                </div>
                <Footer current_path=current_path />
            </main>
        </Router>
    }
}
