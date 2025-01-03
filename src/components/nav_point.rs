use leptos::prelude::*;

#[component]
pub fn NavPoint(
    path: &'static str,
    name: &'static str,
    current_path: RwSignal<&'static str>,
) -> impl IntoView {
    view! {
        <a
            class="mx-2 border-b-2"
            class=("border-black", move || current_path.get().eq(path))
            href=path
            inner_html=name
            on:click=move |_| current_path.set(path)
        />
    }
}
