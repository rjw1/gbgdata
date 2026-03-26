use leptos::prelude::*;

#[component]
pub fn StatRing(
    value: i64,
    max: i64,
    label: String,
) -> impl IntoView {
    let radius = 40.0;
    let circumference = 2.0 * std::f64::consts::PI * radius;
    let offset = circumference - (value as f64 / max as f64) * circumference;

    view! {
        <div class="stat-ring-container">
            <svg width="100" height="100" viewBox="0 0 100 100">
                <circle
                    cx="50" cy="50" r=radius
                    fill="transparent"
                    stroke="#e9ecef"
                    stroke-width="8"
                />
                <circle
                    cx="50" cy="50" r=radius
                    fill="transparent"
                    stroke="var(--amber)"
                    stroke-width="8"
                    stroke-dasharray=format!("{}", circumference)
                    stroke-dashoffset=format!("{}", offset)
                    stroke-linecap="round"
                    transform="rotate(-90 50 50)"
                />
                <text
                    x="50" y="55"
                    text-anchor="middle"
                    font-size="18"
                    font-weight="bold"
                    fill="var(--forest-green)"
                >
                    {value} "/" {max}
                </text>
            </svg>
            <span class="stat-label">{label}</span>
        </div>
    }
}
