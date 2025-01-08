use leptos::{prelude::*, task::spawn_local};
use serde::Serialize;
use serde_wasm_bindgen::{from_value, to_value};
use shared::Task;

use crate::{components::*, icons::PlusCircle, invoke};

#[derive(Default, PartialEq, Eq, Copy, Clone)]
pub enum TasksTab {
    #[default]
    Active = 1,
    Finished = 2,
}

impl From<TasksTab> for &'static str {
    fn from(value: TasksTab) -> Self {
        match value {
            TasksTab::Active => "Active",
            TasksTab::Finished => "Finished",
        }
    }
}

#[component]
pub fn Tasks() -> impl IntoView {
    let active = TasksTab::Active.into();
    let finished = TasksTab::Finished.into();

    let tabs = vec![active, finished];
    let current_tab = RwSignal::<&'static str>::new(active);

    let (pattern, set_pattern) = signal(String::new());

    let (tasks, set_tasks) = signal(Vec::<Task>::new());

    Effect::new(move |_| {
        let pattern = pattern.get();
        let current_tab = current_tab.get();
        spawn_local(async move {
            let tasks = get_tasks(pattern, current_tab.eq(finished)).await;

            set_tasks.set(tasks);
        });
    });

    view! {
        <TabList tabs=tabs current_tab=current_tab />
        <TaskSearch pattern=pattern set_pattern=set_pattern />
        <TaskList tasks=tasks />
        {
            move || if current_tab.get().eq(active) {
                view! {
                    <a
                        class="absolute right-8 bottom-28 h-10 rounded-lg bg-green-400 shadow-md flex flex-row px-2 space-x-2 justify-center items-center"
                        href="/add_task"
                    >
                        <PlusCircle />
                        <p>Add task</p>
                    </a>
                }.into_any()
            } else {
                "".into_any()
            }
        }
    }
}

#[derive(Serialize)]
struct Args {
    pattern: String,
    finished: bool,
}

async fn get_tasks(pattern: String, finished: bool) -> Vec<Task> {
    if let Ok(args) = to_value(&Args { pattern, finished }) {
        from_value(invoke("get_tasks", args).await).unwrap_or_default()
    } else {
        vec![]
    }
}
