//! Leptos view for map stats.
use leptos::prelude::*;
use reactive_stores::Store;

use s2protocol::details::PlayerLobbyDetails;
use serde::{Deserialize, Serialize};

#[derive(Store, Debug, Default, Clone, Serialize, Deserialize)]
pub struct MapStatsTable {
    pub total: usize,
    #[store(key: String = |row| row.title.clone())]
    pub res: Vec<PlayerLobbyDetails>,
    pub start: usize,
    pub end: usize,
    pub page: usize,
    pub per_page: usize,
}

#[component]
pub fn StatsByMap() -> impl IntoView {
    let (map_stats, set_map_stats) = signal(MapStatsTable::default());
    view! {
        <div>
            <h2 class="text-2xl font-bold mb-4">"Stats by Map"</h2>
            // Content for Stats by Map goes here
            <p>"This is where the map statistics will be displayed."</p>
        </div>
    }
}

#[component]
pub fn ReplayScanTable(map_stats_data: Store<MapStatsTable>) -> impl IntoView {
    view! {}
}
