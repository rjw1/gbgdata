use leptos::prelude::*;
use crate::models::SortMode;

#[component]
pub fn SortSelector(
    #[prop(into)] sort: Signal<SortMode>,
    #[prop(into)] set_sort: WriteSignal<SortMode>,
    #[prop(default = false)] show_distance: bool,
) -> impl IntoView {
    view! {
        <div class="sort-selector">
            <span class="sort-label">"Sort by: "</span>
            <select 
                on:change=move |ev| {
                    let val = event_target_value(&ev);
                    let mode = match val.as_str() {
                        "streak" => SortMode::Streak,
                        "total" => SortMode::TotalEntries,
                        "distance" => SortMode::Distance,
                        _ => SortMode::Name,
                    };
                    set_sort.set(mode);
                }
                prop:value=move || match sort.get() {
                    SortMode::Name => "name",
                    SortMode::Streak => "streak",
                    SortMode::TotalEntries => "total",
                    SortMode::Distance => "distance",
                }
            >
                <option value="name">"Name (A-Z)"</option>
                <option value="streak">"Current Streak"</option>
                <option value="total">"Total Entries"</option>
                {if show_distance {
                    view! { <option value="distance">"Distance"</option> }.into_any()
                } else {
                    ().into_any()
                }}
            </select>
        </div>
    }
}
