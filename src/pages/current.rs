use leptos::prelude::*;

use crate::icons::{MoonStars, PlayCircle};

#[component]
pub fn Current() -> impl IntoView {
    view! {
        <div class="h-full w-full flex flex-col space-y-8 justify-center items-center">
            <MoonStars />
            <p class="text-2xl">{"Idle"}</p>
            <div
                class="absolute right-8 bottom-28 h-10 rounded-lg bg-green-400 shadow-md flex flex-row px-2 space-x-2 justify-center items-center"
            >
                <PlayCircle />
                <p>{"Start task"}</p>
            </div>
        </div>
    }
}
