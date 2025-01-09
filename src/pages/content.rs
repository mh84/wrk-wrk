use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;

use super::*;

#[component]
pub fn Content() -> impl IntoView {
    view! {
        <div class="px-4 pb-4 grow shadow-lg flex flex-col space-y-4 overflow-y-auto">
            <Routes fallback=NotFound>
                <Route path=path!("/") view=Current />
                <Route path=path!("/day") view=Day />
                <Route path=path!("/week") view=Week />
                <Route path=path!("/tasks") view=Tasks />
                <Route path=path!("/add_task") view=AddTask />
            </Routes>
        </div>
    }
}
