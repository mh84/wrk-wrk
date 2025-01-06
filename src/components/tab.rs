use leptos::prelude::*;

#[component]
pub fn Tab(tab: &'static str, current_tab: RwSignal<&'static str>) -> impl IntoView {
    let on_click = move |_| {
        current_tab.set(tab);
    };

    view! {
        <div
            on:click=on_click
            class="flex grow justify-center items-center"
            class=(["border-b-2", "border-yellow-500"], move || current_tab.get().eq(tab))
        >{tab}</div>
    }
}
