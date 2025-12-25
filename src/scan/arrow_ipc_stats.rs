use chrono::DateTime;
use chrono::{NaiveDateTime, Utc};
use leptos::prelude::*;
use phosphor_leptos::{
    Icon, IconWeight, BOXING_GLOVE, CALENDAR_DOT, CALENDAR_STAR, CIRCUITRY, HARD_DRIVE,
};
use si_scale::helpers::bibytes2;
use std::time::UNIX_EPOCH;
use swarmy_tauri_common::SnapshotStats;

pub fn time_ago(date: NaiveDateTime) -> String {
    let now = Utc::now().naive_utc();
    let duration = now.signed_duration_since(date);
    let num_days = duration.num_days();
    let num_hours = duration.num_hours();
    let num_minutes = duration.num_minutes();

    if num_days > 1 {
        match num_days {
            0 => "today".to_string(),
            1 => "yesterday".to_string(),
            2..=6 => format!("{} days ago", duration.num_days()),
            7..=13 => "1 week ago".to_string(),
            14..=20 => "2 weeks ago".to_string(),
            21..=27 => "3 weeks ago".to_string(),
            28..=30 => "1 month ago".to_string(),
            31..=59 => format!("{} months ago", duration.num_days() / 30),
            60..=364 => format!("{} months ago", duration.num_days() / 30),
            365..=729 => "1 year ago".to_string(),
            _ => format!("{} years ago", duration.num_days() / 365),
        }
    } else if num_hours > 1 {
        match num_hours {
            1 => "1 hour ago".to_string(),
            _ => format!("{} hours ago", duration.num_hours()),
        }
    } else if num_minutes > 1 {
        match num_minutes {
            1 => "1 minute ago".to_string(),
            _ => format!("{} minutes ago", duration.num_minutes()),
        }
    } else {
        "just now".to_string()
    }
}

#[component]
pub fn ArrowIpcStats(arrow_ipc_stats: ReadSignal<SnapshotStats>) -> impl IntoView {
    let (modif_dt, _set_modif_dt) = signal(
        DateTime::from_timestamp(
            arrow_ipc_stats
                .get()
                .date_modified
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs() as i64,
            0,
        )
        .unwrap_or_default(),
    );
    view! {
        <div class="grid grid-cols-1">
            <div class="col-span-1">
                <p class="flex justify-center text-sky-400">"Snapshot statistics"</p>
            </div>
        </div>
        <div class="grid grid-cols-7">
            <div class="col-span-3 stats">
                <div class="stat shadow">
                    <div class="stat-figure text-primary">
                        <Icon
                            icon=CALENDAR_STAR
                            weight=IconWeight::Bold
                            size="24px"
                            prop:class="stroke-current"
                        />
                    </div>
                    <div class="stat-title">"Optimized"</div>
                    <div
                        class="stat-value text-primary"
                        title=move || modif_dt.get().format("%H:%M:%S %Z").to_string()
                    >
                        {move || time_ago(modif_dt.get().naive_utc())}
                    </div>
                    <div class="stat-desc">
                        {move || modif_dt.get().format("%Y-%m-%d %H:%M:%S %Z").to_string()}
                    </div>
                </div>
            </div>
            <div class="col-span-1"></div>
            <div class="col-span-3 stats">
                <div class="stat shadow">
                    <div class="stat-figure text-primary">
                        <Icon
                            icon=BOXING_GLOVE
                            weight=IconWeight::Bold
                            size="24px"
                            prop:class="stroke-current"
                        />
                    </div>
                    <div class="stat-title">"Processed"</div>
                    <div class="stat-value text-primary">
                        {move || arrow_ipc_stats.get().num_games}
                    </div>
                    <div class="stat-desc">"SC2Replay files"</div>
                </div>
            </div>
        </div>
        <div class="grid grid-cols-7">
            <div class="col-span-3 stats">
                <div class="stat shadow">
                    <div class="stat-figure text-primary">
                        <Icon
                            icon=HARD_DRIVE
                            weight=IconWeight::Bold
                            size="24px"
                            prop:class="stroke-current"
                        />
                    </div>
                    <div class="stat-title">"Snapshot size"</div>
                    <div class="stat-value text-primary">
                        {move || bibytes2(arrow_ipc_stats.get().directory_size as u32)}
                    </div>
                    <div class="stat-desc">
                        <code>"ipcs/"</code>
                        " directory"
                    </div>
                </div>
            </div>
            <div class="col-span-1"></div>
            <div class="col-span-3 stats">
                <div class="stat shadow">
                    <div class="stat-figure text-primary">
                        <Icon
                            icon=CIRCUITRY
                            weight=IconWeight::Bold
                            size="24px"
                            prop:class="stroke-current"
                        />
                    </div>
                    <div class="stat-title">"maps"</div>
                    <div class="stat-value text-primary">
                        {move || arrow_ipc_stats.get().num_maps}
                    </div>
                    <div class="stat-desc">"(unique maps)"</div>
                </div>
            </div>
        </div>
        <div class="grid grid-cols-7">
            <div class="col-span-3 stats">
                <div class="stat shadow">
                    <div class="stat-figure text-primary">
                        <Icon
                            icon=CALENDAR_DOT
                            weight=IconWeight::Bold
                            size="24px"
                            prop:class="stroke-current"
                        />
                    </div>
                    <div class="stat-title">"From:"</div>
                    <div class="stat-value text-primary">
                        {move || arrow_ipc_stats.get().min_date.format("%Y-%m-%d").to_string()}
                    </div>
                    <div class="stat-desc">"Minimum replay time"</div>
                </div>
            </div>
            <div class="col-span-1"></div>
            <div class="col-span-3 stats">
                <div class="stat shadow">
                    <div class="stat-figure text-primary">
                        <Icon
                            icon=CALENDAR_DOT
                            weight=IconWeight::Bold
                            size="24px"
                            prop:class="stroke-current"
                        />
                    </div>
                    <div class="stat-title">"To:"</div>
                    <div class="stat-value text-primary">
                        {move || arrow_ipc_stats.get().max_date.format("%Y-%m-%d").to_string()}
                    </div>
                    <div class="stat-desc">"Maximum replay time"</div>
                </div>
            </div>
        </div>
    }
}
