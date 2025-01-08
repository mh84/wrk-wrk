use leptos::{prelude::*};
use serde::Serialize;
use serde_wasm_bindgen::to_value;
use shared::TaskKind;
use wasm_bindgen_futures::spawn_local;

use crate::{icons::{ArrowLeft, FloppyDisk}, invoke};

#[component]
pub fn AddTask() -> impl IntoView {
    let navigate = leptos_router::hooks::use_navigate();
    view! {
        <a href="/tasks" class="flex flex-row items-center space-x-2">
            <ArrowLeft />
            <p>{"Back"}</p>
        </a>
        <div>
            {"TBD"}
        </div>
        <div
            on:click=move |_| {
                spawn_local(async move {
                    add_task(String::from("wrk5-wrk5"), TaskKind::Development).await;
                });

                navigate("/tasks", Default::default());
            }
            class="absolute right-8 bottom-28 h-10 rounded-lg bg-green-400 shadow-md flex flex-row px-2 space-x-2 justify-center items-center"
        >
            <FloppyDisk />
            <p>Save task</p>
        </div>
    }
}

#[derive(Serialize)]
struct Args {
    name: String,
    taskkind: TaskKind,
}

async fn add_task(name: String, taskkind: TaskKind) {
    if let Ok(args) = to_value(&Args { name, taskkind }) {
        let _ = invoke("add_task", args).await;
    }
}
