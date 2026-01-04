/// Common views.
use leptos::prelude::*;
use phosphor_leptos::{Icon, IconWeight, X_CIRCLE};

#[component]
pub fn DisplayBackendStatus(
    backend_response: ReadSignal<swarmy_tauri_common::ApiResponse>,
) -> impl IntoView {
    view! {
        <Show when=move || {
            !backend_response.get().meta.success && backend_response.get().meta.is_complete
        }>
            <div role="alert" class="alert alert-error shadow-lg m-1 p-1">
                <Icon icon=X_CIRCLE weight=IconWeight::Bold prop:class="stroke-current" />
                <span>{backend_response.get().message.clone()}</span>
            </div>
        </Show>
    }
}
