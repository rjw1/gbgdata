use anyhow::Result;
use reqwest::Client;
use serde::Deserialize;
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

impl Default for Geocoder {
    fn default() -> Self {
        Self::new()
    }
}

impl Geocoder {
    pub fn new() -> Self {
        let client = Client::builder()
            .user_agent("gbgdata-importer (bob@example.com)")
            .build()
            .unwrap();

        let url = std::env::var("NOMINATIM_URL").unwrap_or_default();
        let is_local = url.contains("localhost") || url.contains("127.0.0.1");

        Self {
            client,
            url,
            is_local,
        }
    }

    pub async fn geocode(
        &self,
        name: &str,
        _address: &str,
        town: &str,
        postcode: &str,
        region: &str,
    ) -> Result<Option<(f64, f64)>> {
        if self.url.is_empty() {
            return Ok(None);
        }
        // Fallback strategies in order of reliability
        let queries = vec![
            format!("{}, {}", name, town),
            format!("{}, {}, {}", name, town, region),
            format!("{}, {}", name, postcode),
            format!("{}, {}, {}", name, town, postcode),
            format!("{}", postcode), // Final hail mary: just the postcode
        ];

        for query in queries {
            let resp = self
                .client
                .get(&self.url)
                .query(&[
                    ("q", query.clone()),
                    ("format", "json".to_string()),
                    ("limit", "1".to_string()),
                ])
                .send()
                .await?;

            if resp.status().as_u16() == 403 {
                return Err(anyhow::anyhow!(
                    "Nominatim 403 Forbidden - Rate limited or blocked"
                ));
            }

            let results: Vec<NominatimResponse> = resp.json().await?;

            // Only sleep if NOT local (Nominatim policy: 1 request per second)
            if !self.is_local {
                sleep(Duration::from_secs(1)).await;
            }

            if let Some(res) = results.first() {
                if let (Ok(lat), Ok(lon)) = (res.lat.parse::<f64>(), res.lon.parse::<f64>()) {
                    return Ok(Some((lat, lon)));
                }
            }
        }

        Ok(None)
    }
}
