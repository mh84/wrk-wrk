use leptos::prelude::*;

use crate::icons::*;

#[component]
pub fn NavPoint(
    path: &'static str,
    icon: NavPointIcon,
    caption: &'static str,
    current_path: RwSignal<&'static str>,
) -> impl IntoView {
    let selected = move || current_path.get().eq(path);
    let on_click = move |_| current_path.set(path);

    view! {
        <a class="flex flex-col grow items-center" href=path on:click=on_click>
            {move || match icon {
                NavPointIcon::Bulldozer => view! { <Bulldozer selected=selected() /> }.into_any(),
                NavPointIcon::CalenderDot => view! { <CalendarDot selected=selected() /> }.into_any(),
                NavPointIcon::CalenderDots => view! { <CalendarDots selected=selected() /> }.into_any(),
                NavPointIcon::ClipboardText => view! { <ClipboardText selected=selected() /> }.into_any(),
            }}
            <p>{caption}</p>
        </a>
    }
}
