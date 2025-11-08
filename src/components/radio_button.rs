use leptos::prelude::*;

use shared::{TaskKind, TaskKindValue};

#[component]
pub fn RadioButton(
    name: String,
    task_kind: TaskKind,
    set_taskkind: WriteSignal<String>,
) -> impl IntoView {
    let task_kind_value = TaskKindValue::from(task_kind.clone());
    let task_kind_value_clone = task_kind_value.clone();
    let class = String::from("rounded text-sm flex items-center px-2 ") + &task_kind_value.color;

    view! {
        <div class="flex flex-row space-x-6 items-center">
            <div class="w-20 text-right pr-2">
                <input
                    type="radio"
                    id={task_kind_value.name.clone()}
                    name=name
                    value={task_kind_value.name.clone()}
                    checked=task_kind.eq(&TaskKind::Development)
                    on:input=move |ev| {
                        if event_target_checked(&ev) {
                            set_taskkind.set(task_kind_value.name.clone());
                        }
                    }
                />
            </div>
            <label for={task_kind_value_clone.name} class=class>
                {task_kind_value_clone.name.clone()}
            </label>
        </div>
    }
}
