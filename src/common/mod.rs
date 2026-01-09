/// Common views.
use leptos::{leptos_dom::logging::console_log, prelude::*};
use phosphor_leptos::{Icon, IconWeight, X_CIRCLE};

#[component]
pub fn DisplayBackendStatus(
    backend_response: ReadSignal<swarmy_tauri_common::ApiResponse>,
) -> impl IntoView {
    let error_lines = move || {
        backend_response
            .get()
            .message
            .replace("\\n", "\n")
            .lines()
            .map(|line| line.to_string())
            .collect::<Vec<String>>()
    };
    view! {
        <Show when=move || {
            !backend_response.get().meta.success && backend_response.get().meta.is_complete
        }>
            <div role="alert" class="alert alert-error alert-soft m-1 p-1">
                <Icon icon=X_CIRCLE weight=IconWeight::Bold prop:class="stroke-current" />
                <p class="text-ellipsis overflow-hidden">
                    <For
                        // a function that returns the items we're iterating over; a signal is fine
                        each=move || error_lines()
                        key=|line| line.clone()
                        // renders each item to a view
                        children=move |line: String| {
                            console_log(&format!("Error line: {:?}", line));
                            view! { <span>{move || line.clone()}<br /></span> }
                        }
                    />
                </p>
            </div>
        </Show>
    }
}
