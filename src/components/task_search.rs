use leptos::prelude::*;

use crate::icons::*;

#[component]
pub fn TaskSearch(
    pattern: ReadSignal<String>,
    set_pattern: WriteSignal<String>,
) -> impl IntoView {
    view! {
        <div class="flex flex-row my-4 items-center">
            <input
                type="text"
                bind:value=(pattern, set_pattern)
                class="border-0 focus:border-0 focus:ring-transparent border-b-2 focus:border-b-2 border-gray-400 focus:border-yellow-500 grow"
            />
            <div class="absolute right-6" on:click=move |_| set_pattern.set(String::new())>
                <XCircle />
            </div>
        </div>
    }
}
