use leptos::prelude::*;

use shared::{TaskKind, TaskKindValue};

#[component]
pub fn Tag(task_kind: TaskKind) -> impl IntoView {
    let task_kind_value = TaskKindValue::from(task_kind);
    let mut class = String::from("rounded text-sm px-2 items-center ");
    class.push_str(&task_kind_value.color);

    view! {
        <div class=class>
            {task_kind_value.name}
        </div>
    }
}
