use super::components::*;
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
            <main class="w-full h-full bg-gray-200 flex flex-col">
                <nav class="h-8 pl-6 flex text-black justify-start items-end">
                    <NavPoint path="/" name="Current" current_path=current_path />
                    <NavPoint path="/day" name="Day" current_path=current_path />
                    <NavPoint path="/week" name="Week" current_path=current_path />
                    <NavPoint path="/tasks" name="Tasks" current_path=current_path />
                </nav>
                <div class="mx-4 px-4 pt-2 bg-white rounded-t-lg shadow-lg grow">
                    <Routes fallback=NotFound>
                        <Route path=path!("/") view=Current />
                        <Route path=path!("/day") view=Day />
                        <Route path=path!("/week") view=Week />
                        <Route path=path!("/tasks") view=Tasks />
                    </Routes>
                </div>
            </main>
        </Router>
    }
}
