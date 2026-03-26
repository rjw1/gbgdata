use leptos::prelude::*;
use crate::models::{PubSummary, PubDetail};
use uuid::Uuid;

#[server(GetPubs, "/api")]
pub async fn get_pubs(query: String) -> Result<Vec<PubSummary>, ServerFnError> {
    use sqlx::PgPool;
    use leptos::context::use_context;
    
    let pool = use_context::<PgPool>()
        .ok_or_else(|| ServerFnError::new("Pool not found in context"))?;

    let pubs = sqlx::query_as!(
        PubSummary,
        r#"SELECT id, name, 
                  COALESCE(town, '') as "town!", 
                  COALESCE(county, '') as "county!", 
                  COALESCE(postcode, '') as "postcode!", 
                  COALESCE(closed, false) as "closed!"
           FROM pubs 
           WHERE name ILIKE $1 OR town ILIKE $1 OR county ILIKE $1
           ORDER BY name LIMIT 50"#,
        format!("%{}%", query)
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(pubs)
}

#[server(GetPubDetail, "/api")]
pub async fn get_pub_detail(id: Uuid) -> Result<PubDetail, ServerFnError> {
    use sqlx::PgPool;
    use leptos::context::use_context;
    
    let pool = use_context::<PgPool>()
        .ok_or_else(|| ServerFnError::new("Pool not found in context"))?;

    let pub_info = sqlx::query!(
        r#"SELECT id, name, 
                  COALESCE(address, '') as "address!", 
                  COALESCE(town, '') as "town!", 
                  COALESCE(county, '') as "county!", 
                  COALESCE(postcode, '') as "postcode!", 
                  COALESCE(closed, false) as "closed!",
                  untappd_id, google_maps_id, whatpub_id, rgl_id
           FROM pubs WHERE id = $1"#,
        id
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    let years = sqlx::query_scalar!(
        "SELECT year FROM gbg_history WHERE pub_id = $1 ORDER BY year DESC",
        id
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(PubDetail {
        id: pub_info.id,
        name: pub_info.name,
        address: pub_info.address,
        town: pub_info.town,
        county: pub_info.county,
        postcode: pub_info.postcode,
        closed: pub_info.closed,
        untappd_id: pub_info.untappd_id,
        google_maps_id: pub_info.google_maps_id,
        whatpub_id: pub_info.whatpub_id,
        rgl_id: pub_info.rgl_id,
        years,
    })
}
