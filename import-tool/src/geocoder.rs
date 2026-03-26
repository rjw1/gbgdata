use reqwest::Client;
use serde::Deserialize;
use anyhow::Result;
use std::time::Duration;
use tokio::time::sleep;

#[derive(Debug, Deserialize)]
struct NominatimResponse {
    lat: String,
    lon: String,
}

pub struct Geocoder {
    client: Client,
    url: String,
    is_local: bool,
}

impl Geocoder {
    pub fn new() -> Self {
        let client = Client::builder()
            .user_agent("gbgdata-importer (bob@example.com)")
            .build()
            .unwrap();
        
        let url = std::env::var("NOMINATIM_URL").unwrap_or_else(|_| "http://localhost:8080/search".to_string());
        let is_local = url.contains("localhost") || url.contains("127.0.0.1");

        Self { client, url, is_local }
    }

    pub async fn geocode(&self, address: &str, town: &str, postcode: &str) -> Result<Option<(f64, f64)>> {
        let query = format!("{}, {}, {}", address, town, postcode);
        
        let resp = self.client.get(&self.url)
            .query(&[
                ("q", query),
                ("format", "json".to_string()),
                ("limit", "1".to_string()),
            ])
            .send()
            .await?;

        if resp.status().as_u16() == 403 {
            return Err(anyhow::anyhow!("Nominatim 403 Forbidden - Rate limited or blocked"));
        }

        let results: Vec<NominatimResponse> = resp.json().await?;
        
        // Only sleep if NOT local (Nominatim policy: 1 request per second)
        if !self.is_local {
            sleep(Duration::from_secs(1)).await;
        }

        if let Some(res) = results.first() {
            let lat = res.lat.parse::<f64>()?;
            let lon = res.lon.parse::<f64>()?;
            Ok(Some((lat, lon)))
        } else {
            Ok(None)
        }
    }
}
