use leptos::prelude::*;
use shared::Task;

#[component]
pub fn TaskList(tasks: ReadSignal<Vec<Task>>) -> impl IntoView {
    view! {
        <div class="flex flex-col">
            {
                move || tasks.get()
                    .iter()
                    .map(|task| view! {
                        <div class="flex flex-row px-2 py-2 border-t border-gray-400 last:border-b items-center">
                            <div>
                                {task.name.clone()}
                            </div>
                            <div class="rounded text-sm bg-gray-400 flex items-center px-2 absolute right-6">
                                {"Pause"}
                            </div>
                        </div>
                    })
                    .collect_view()
            }
        </div>
    }
}
