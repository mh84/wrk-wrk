use leptos::prelude::*;

#[component]
pub fn CalendarDots(selected: bool) -> impl IntoView {
    let fill_class = ("fill-yellow-500", move || selected);

    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 256 256" class="h-8 w-8" class=("text-yellow-500", move || selected)>
            <rect width="256" height="256" fill="none"/>
            <rect x="40" y="40" width="176" height="176" rx="8" fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="16"/>
            <line x1="176" y1="24" x2="176" y2="56" fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="16"/>
            <line x1="80" y1="24" x2="80" y2="56" fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="16"/>
            <line x1="40" y1="88" x2="216" y2="88" fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="16"/>
            <circle cx="128" cy="132" r="12" class=fill_class />
            <circle cx="172" cy="132" r="12" class=fill_class />
            <circle cx="84" cy="172" r="12" class=fill_class />
            <circle cx="128" cy="172" r="12" class=fill_class />
            <circle cx="172" cy="172" r="12" class=fill_class />
        </svg>
    }
}
