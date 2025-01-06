use leptos::prelude::*;

use super::*;

#[component]
pub fn TabList(tabs: Vec<&'static str>, current_tab: RwSignal<&'static str>) -> impl IntoView {
    view! {
        <div class="flex flex-row">
            {
                move || tabs.iter()
                    .map(|tab| view! { <Tab tab=tab current_tab=current_tab /> })
                    .collect_view()
            }
        </div>
    }
}
