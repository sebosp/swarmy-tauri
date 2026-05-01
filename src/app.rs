//! Swarmy Tauri Application

use bevy::prelude::*;
use leptos::leptos_dom::logging::console_log;
use leptos::prelude::*;
use leptos_bevy_canvas::prelude::*;
use swarmy_tauri_ui::components::left_sidebar_menu::LeftSideBarMenu;
use swarmy_tauri_ui::config::view::Config;
use swarmy_tauri_ui::map_stats::view::StatsByMap;
use swarmy_tauri_ui::scan::view::ScanDirectory;

#[derive(Message, Clone)]
pub struct TextMessage {
    pub text: String,
}

fn on_input_test(evt: leptos::ev::Event, text_message_sender: LeptosMessageSender<TextMessage>) {
    // send the message over to Bevy
    text_message_sender
        .send(TextMessage {
            text: event_target_value(&evt),
        })
        .ok();
}

#[component]
pub fn Main() -> impl IntoView {
    let active_page = RwSignal::new("Home".to_string());

    let (text_message_sender, bevy_text_receiver) = message_l2b::<TextMessage>();
    let text_message_sender = StoredValue::new(text_message_sender);
    let bevy_text_receiver = StoredValue::new(bevy_text_receiver);
    console_log("Main component initialized");

    view! {
        <BevyCanvas
            init=move || {
                let bevy_text_receiver = bevy_text_receiver.read_value().into_inner().clone();
                init_bevy_app(bevy_text_receiver)
            }

            {..}
            width="300"
            height="500"
        />
        <div id="swarmy-tauri-window" class="flex w-screen h-screen bg-gray-800 rounded">
            <LeftSideBarMenu active_page=active_page />
            <div
                id="swarmy-tauri-content"
                class="flex grow w-full h-full overflow-auto text-gray-400 bg-gray-900 rounded"
            >
                <Show when=move || active_page.get() == "Home">
                    <div
                        id="swarmy-tauri-scan-directory-content"
                        class="flex flex-col grow p-2 rounded"
                    >
                        <input
                            type="text"
                            on:input=move |evt| {
                                let text_message_sender = text_message_sender
                                    .read_value()
                                    .into_inner()
                                    .clone();
                                on_input_test(evt, text_message_sender)
                            }
                        />
                    </div>
                </Show>
                <Show when=move || active_page.get() == "Scan">
                    <div
                        id="swarmy-tauri-scan-directory-content"
                        class="flex flex-col grow p-2 rounded"
                    >
                        <ScanDirectory />
                    </div>
                </Show>
                <Show when=move || active_page.get() == "Stats By Map">
                    <div
                        id="swarmy-tauri-stats-by-map-content"
                        class="flex flex-col grow p-2 rounded"
                    >
                        <StatsByMap />
                    </div>
                </Show>
                <Show when=move || active_page.get() == "Config">
                    <div id="swarmy-tauri-config-content">
                        <Config />
                    </div>
                </Show>
            </div>
        </div>
    }
}

// In Bevy it ends up just as a normal message
pub fn set_text(mut message_reader: MessageReader<TextMessage>) {
    for message in message_reader.read() {
        console_log(&format!("Received message from Leptos: {}", message.text));
        // do something with the message
    }
}

// This initializes a normal Bevy app
fn init_bevy_app(text_receiver: BevyMessageReceiver<TextMessage>) -> App {
    let mut app = App::new();
    console_log("Initializing Bevy App");
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            // "#bevy_canvas" is the default and can be
            // changed in the <BevyCanvas> component
            canvas: Some("#bevy_canvas".into()),
            ..default()
        }),
        ..default()
    }))
    // import the message here into Bevy
    .import_message_from_leptos(text_receiver)
    .add_systems(Update, set_text);

    app
}
