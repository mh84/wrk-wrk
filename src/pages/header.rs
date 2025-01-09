use crate::icons::RocketLaunch;
use leptos::prelude::*;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <div class="min-h-12 flex flex-row justify-center items-center shadow-xl mb-4">
            <RocketLaunch />
            <p class=" ml-4 text-2xl">Work work!</p>
        </div>
    }
}
