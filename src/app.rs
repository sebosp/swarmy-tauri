use leptos::task::spawn_local;
use leptos::{ev::SubmitEvent, prelude::*};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct ReplaysDirectory<'a> {
    path: &'a str,
}

#[component]
pub fn App() -> impl IntoView {
    let (replay_path_r, replay_path_w) = signal(String::new());
    let (check_replay_path_r, check_replay_path_w) = signal(String::new());

    let update_name = move |ev| {
        let v = event_target_value(&ev);
        replay_path_w.set(v);
    };

    let set_replays_path = move |ev: SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            let name = replay_path_r.get_untracked();
            if name.is_empty() {
                return;
            }

            let args = serde_wasm_bindgen::to_value(&ReplaysDirectory { path: &name }).unwrap();
            // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
            let new_msg = invoke("greet", args).await.as_string().unwrap();
            check_replay_path_w.set(new_msg);
        });
    };

    view! {
        <main class="container">
            <h1>"Welcome to Tauri + Leptos"</h1>

            <div class="row">
                <a href="https://tauri.app" target="_blank">
                    <img src="public/tauri.svg" class="logo tauri" alt="Tauri logo"/>
                </a>
                <a href="https://docs.rs/leptos/" target="_blank">
                    <img src="public/leptos.svg" class="logo leptos" alt="Leptos logo"/>
                </a>
            </div>
            <p>"Click on the Tauri and Leptos logos to learn more."</p>

            <form class="row" on:submit=set_replays_path>
                <input
                    id="greet-input"
                    placeholder="Enter the directory name..."
                    on:input=update_name
                />
                <button type="submit">"Set Replays Path"</button>
            </form>
            <p>{ move || check_replay_path_r.get() }</p>
        </main>
    }
}
