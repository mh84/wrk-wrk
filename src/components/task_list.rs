use leptos::prelude::*;
use shared::Task;

use crate::components::Tag;

#[component]
pub fn TaskList(tasks: ReadSignal<Vec<Task>>) -> impl IntoView {
    view! {
        <div class="flex flex-col overflow-y-auto">
            {move || {
                let tasks = tasks.get();
                if !tasks.is_empty() {
                    tasks
                        .iter()
                        .map(|task| {
                            view! {
                                <div class="flex px-2 py-2 border-t border-gray-400 last:border-b items-center space-x-4">
                                    <p>{task.name.clone()}</p>
                                    <Tag task_kind=task.task_kind.clone() />
                                </div>
                            }
                        })
                        .collect_view()
                        .into_any()
                } else {
                    view! { <p class="px-2">{"No Entries."}</p> }.into_any()
                }
            }}
        </div>
    }
}
