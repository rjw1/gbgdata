use leptos::prelude::*;

#[component]
pub fn About() -> impl IntoView {
    view! {
        <div class="about-container">
            <h1>"About GBG Data Explorer"</h1>

            <section class="about-section">
                <h2>"The Project"</h2>
                <p>
                    "GBG Data Explorer is a tool designed to visualize and analyze historical data from the CAMRA Good Beer Guide.
                    It allows beer enthusiasts and researchers to track pub inclusions, streaks, and geographic trends over several decades."
                </p>
            </section>

            <section class="about-section highlight">
                <h2>"The 1972 Trial Year"</h2>
                <p>
                    <strong>"Note on Data Integrity:"</strong>
                    " The 1972 edition of the Good Beer Guide was a trial run. While we have included the data for historical completeness,
                    it is excluded from all calculated statistics, including 'Total Appearances' and 'Current Streak'."
                </p>
                <p>
                    "Appearances in 1972 are marked as 'Trial' on pub detail pages and do not count towards all-time rankings."
                </p>
            </section>

            <section class="about-section">
                <h2>"Credits"</h2>
                <p>
                    "Data curated and provided by Duncan (2025). Geocoding is performed via automated services and may contain inaccuracies."
                </p>
            </section>

            <section class="about-section">
                <h2>"Technology"</h2>
                <p>
                    "Built with Rust using the Leptos framework, Axum, and Postgres/PostGIS."
                </p>
            </section>
        </div>
    }
}
