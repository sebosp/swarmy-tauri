use reactive_stores::Store;
use leptos::prelude::*;
use super::*;
use reactive_graph::traits::Read;

#[component]
pub fn ReplayScanTable(dir_stats_data: Store<SC2ReplaysDirStatsTable>) -> impl IntoView {
    view! {
        <div class="flex flex-row">
            <div class="flex-item basis-128">
                <h3 class="text-neutral-content">
                    "Replays: "
                    <div class="badge badge-sm badge-ghost">
                        {move || dir_stats_data.total_files().get()}
                    </div>
                </h3>
            </div>
            <div class="flex-item basis-128">
                <h3
                    class="text-info"
                    title="Replays successfully parsed for GameEvents and TrackerEvents"
                >
                    "Basic: "
                    <div class="badge badge-sm badge-info">
                        {move || dir_stats_data.total_supported_replays().get()}
                    </div>
                </h3>
            </div>
            <div class="flex-item basis-128">
                <h3 class="text-success" title="Replays with balance data available for abilities">
                    "Enhanced Support: "
                    <div class="badge badge-sm badge-success">
                        {move || dir_stats_data.ability_supported_replays().get()}
                    </div>
                </h3>
            </div>
        </div>
        <div class="flex gap-4">
            <div class="flex-item grow">
                <h2 class="text-neutral-content flex justify-center bg-gray-800">
                    "Top 10 Players"
                </h2>
                <table class="table bg-gray-500 table-xs table-zebra rounded-box">
                    <thead class="bg-gray-700">
                        <tr>
                            <th></th>
                            <th>Clan</th>
                            <th>Name</th>
                            <th>Total Games</th>
                        </tr>
                    </thead>
                    <tbody>
                        <For
                            each=move || dir_stats_data.top_10_players()
                            key=|row| row.read().name.clone()
                            children=|child| {
                                let idx = child.clone().idx();
                                let clan = child.clone().clan().clone();
                                let name = child.clone().name().clone();
                                let count = child.clone().count();
                                view! {
                                    <tr>
                                        <th>{move || idx.get()}</th>
                                        <td>{move || clan.get()}</td>
                                        <td>{move || name.get()}</td>
                                        <td>{move || count.get()}</td>
                                    </tr>
                                }
                            }
                        />
                    </tbody>
                </table>
            </div>
            <div class="flex-item grow">
                <h2 class="text-neutral-content flex justify-center bg-gray-800">"Top 10 Maps"</h2>
                <table class="table bg-gray-500 table-xs table-zebra rounded-box">
                    <thead class="bg-gray-700">
                        <tr>
                            <th></th>
                            <th>Map Title</th>
                            <th>Total Games</th>
                        </tr>
                    </thead>
                    <tbody>
                        <For
                            each=move || dir_stats_data.top_10_maps()
                            key=|row| row.read().title.clone()
                            children=|child| {
                                let idx = child.clone().idx();
                                let title = child.clone().title().clone();
                                let count = child.clone().count();
                                view! {
                                    <tr>
                                        <th>{move || idx.get()}</th>
                                        <td>{move || title.get()}</td>
                                        <td>{move || count.get()}</td>
                                    </tr>
                                }
                            }
                        />
                    </tbody>
                </table>
            </div>
        </div>
    }
}
