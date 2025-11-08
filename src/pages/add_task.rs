use leptos::{prelude::*, task::spawn_local};
use serde::Serialize;
use serde_wasm_bindgen::to_value;
use shared::{TaskKind, TaskKindValue};

use crate::{
    components::RadioButton,
    icons::{ArrowLeft, FloppyDisk},
    invoke,
};

#[component]
pub fn AddTask() -> impl IntoView {
    let navigate = leptos_router::hooks::use_navigate();
    let (name, set_name) = signal(String::new());
    let (taskkind, set_taskkind) = signal::<String>(String::from("Development"));

    view! {
        <a href="/tasks" class="flex flex-row items-center space-x-2">
            <ArrowLeft />
            <p>{"Back"}</p>
        </a>
        <div class="flex flex-col space-y-4">
            <div class="flex flex-row space-x-4 items-center">
                <p class="w-20 text-right">{"Name:"}</p>
                <input
                    type="text"
                    bind:value=(name, set_name)
                    minlength=3
                    required=true
                    class="grow border-0 focus:border-0 focus:ring-transparent border-b-2 focus:border-b-2 border-gray-400 focus:border-yellow-500 grow"
                />
            </div>
            <RadioButton name="TaskKind".into() task_kind=TaskKind::Development set_taskkind=set_taskkind />
            <RadioButton name="TaskKind".into() task_kind=TaskKind::CodeReview set_taskkind=set_taskkind />
            <RadioButton name="TaskKind".into() task_kind=TaskKind::Test set_taskkind=set_taskkind />
            <RadioButton name="TaskKind".into() task_kind=TaskKind::Meeting set_taskkind=set_taskkind />
            <RadioButton name="TaskKind".into() task_kind=TaskKind::DevOps set_taskkind=set_taskkind />
            <RadioButton name="TaskKind".into() task_kind=TaskKind::Support set_taskkind=set_taskkind />
            <RadioButton name="TaskKind".into() task_kind=TaskKind::Consulting set_taskkind=set_taskkind />
            <RadioButton name="TaskKind".into() task_kind=TaskKind::Other set_taskkind=set_taskkind />
        </div>
        <div
            on:click=move |_| {
                let name = name.get();
                if name.len() > 3 {
                    let navigate = navigate.clone();
                    spawn_local(async move {
                        add_task(name, TaskKindValue::from(Into::<TaskKind>::into(&*taskkind.get())).db_value).await;
                        navigate("/tasks", Default::default());
                    });
                }
            }
            class="absolute right-8 bottom-28 h-10 rounded-lg bg-green-400 shadow-md flex flex-row px-2 space-x-2 justify-center items-center"
        >
            <FloppyDisk />
            <p>{"Save task"}</p>
        </div>
    }
}

#[derive(Serialize)]
struct AddTaskArgs {
    name: String,
    taskkind: i64,
}

async fn add_task(name: String, taskkind: i64) {
    if let Ok(args) = to_value(&AddTaskArgs { name, taskkind }) {
        let _ = invoke("add_task", args).await;
    }
}
