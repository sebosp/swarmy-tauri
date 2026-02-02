//! Leptos view for app configuration.

use leptos::prelude::*;

#[component]
pub fn Config() -> impl IntoView {
    let (disable_parallel_scans, set_disable_parallel_scans) = signal(false);
    view! {
        <div class="ml-4 mt-4">
            <h2 class="text-base/7 font-semibold text-white">Scan Settings</h2>
            <p class="mt-1 max-w-2xl text-sm/6 text-gray-400">
                "Configuration for SC2Replays batch processing."
            </p>

            <div class="mt-6 space-y-10 border-b border-white/10 pb-12 sm:space-y-0 sm:divide-y sm:divide-white/10 sm:border-t sm:border-t-white/10 sm:pb-0">
                <fieldset>
                    <legend class="sr-only">"Parallel Scanning"</legend>
                    <div class="sm:grid sm:grid-cols-3 sm:gap-4 sm:py-6">
                        <div aria-hidden="true" class="text-sm/6 font-semibold text-white">
                            <p class="text-sm/6 font-semibold text-white">"Parallel Scanning"</p>
                            <p class="text-sm/6 font-semibold text-white">
                                " Status: "
                                <span class=move || {
                                    if disable_parallel_scans.get() {
                                        "inline-flex items-center rounded-full bg-orange-400/10 px-1.5 py-0.5 text-xs font-medium text-orange-400"
                                    } else {
                                        "inline-flex items-center rounded-full bg-green-400/10 px-1.5 py-0.5 text-xs font-medium text-green-400"
                                    }
                                }>
                                    {move || {
                                        if disable_parallel_scans.get() {
                                            "Disabled"
                                        } else {
                                            "Enabled"
                                        }
                                    }}
                                </span>
                            </p>
                        </div>
                        <div class="mt-4 sm:col-span-2 sm:mt-0">
                            <div class="max-w-lg space-y-6">
                                <div class="flex gap-3">
                                    <div class="flex h-6 shrink-0 items-center">
                                        <div class="group grid size-4 grid-cols-1">
                                            <input
                                                id="paralellism"
                                                type="checkbox"
                                                name="paralellism"
                                                checked=move || disable_parallel_scans.get()
                                                on:click=move |_| {
                                                    set_disable_parallel_scans
                                                        .set(!disable_parallel_scans.get())
                                                }
                                                aria-describedby="comments-description"
                                                class="col-start-1 row-start-1 appearance-none rounded-sm border border-white/10 bg-white/5 checked:border-indigo-500 checked:bg-indigo-500 indeterminate:border-indigo-500 indeterminate:bg-indigo-500 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-500 disabled:border-white/5 disabled:bg-white/10 disabled:checked:bg-white/10 forced-colors:appearance-auto"
                                            />
                                            <svg
                                                viewBox="0 0 14 14"
                                                fill="none"
                                                class="pointer-events-none col-start-1 row-start-1 size-3.5 self-center justify-self-center stroke-white group-has-disabled:stroke-white/25"
                                            >
                                                <path
                                                    d="M3 8L6 11L11 3.5"
                                                    stroke-width="2"
                                                    stroke-linecap="round"
                                                    stroke-linejoin="round"
                                                    class="opacity-0 group-has-checked:opacity-100"
                                                />
                                                <path
                                                    d="M3 7H11"
                                                    stroke-width="2"
                                                    stroke-linecap="round"
                                                    stroke-linejoin="round"
                                                    class="opacity-0 group-has-indeterminate:opacity-100"
                                                />
                                            </svg>
                                        </div>
                                    </div>
                                    <div class="text-sm/6">
                                        <label for="paralellism" class="font-medium text-white">
                                            "Disable Parallel Processing"
                                        </label>
                                        <p id="paralellism-description" class="text-gray-400">
                                            "Decreases CPU usage. Disable if you are on a laptop and you experience heating issues."
                                        </p>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </fieldset>
            </div>
        </div>
    }
}
