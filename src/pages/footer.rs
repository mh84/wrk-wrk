use crate::{components::*, icons::NavPointIcon};
use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    let current_path = RwSignal::new("/");

    view! {
        <div class="min-h-20 flex flex-row items-center">
            <NavPoint
                path="/"
                icon=NavPointIcon::Bulldozer
                caption="Current"
                current_path=current_path
            />
            <NavPoint
                path="/day"
                icon=NavPointIcon::CalenderDot
                caption="Day"
                current_path=current_path
            />
            <NavPoint
                path="/week"
                icon=NavPointIcon::CalenderDots
                caption="Week"
                current_path=current_path
            />
            <NavPoint
                path="/tasks"
                icon=NavPointIcon::ClipboardText
                caption="Tasks"
                current_path=current_path
            />
        </div>
    }
}
