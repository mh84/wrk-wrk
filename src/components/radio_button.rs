use leptos::prelude::*;

#[component]
pub fn RadioButton(
    name: String,
    value: String,
    set_taskkind: WriteSignal<String>,
) -> impl IntoView {
    let label_value = value.clone();

    view! {
        <div class="flex flex-row space-x-6 items-center">
            <div class="w-20 text-right pr-2">
                <input
                    type="radio"
                    id={value.clone()}
                    name=name
                    value={value.clone()}
                    on:input=move |ev| {
                        if event_target_checked(&ev) {
                            set_taskkind.set(value.clone());
                        }
                    }
                />
            </div>
            <label for={label_value.clone()} class="rounded text-sm bg-gray-400 flex items-center px-2">
                {label_value.clone()}
            </label>
        </div>
    }
}
