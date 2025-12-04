use crate::ScanEvent;
use anyhow::{Context, Result};
use reqwest::Client;

pub struct ApiClient {
    client: Client,
    base_url: String,
}

impl ApiClient {
    pub fn new(base_url: &str) -> Self {
        ApiClient {
            client: Client::new(),
            base_url: base_url.trim_end_matches('/').to_string(),
        }
    }

    pub async fn report_event(&self, event: &ScanEvent) -> Result<()> {
        let url = format!("{}/api/v1/events", self.base_url);
        
        let response = self
            .client
            .post(&url)
            .json(event)
            .send()
            .await
            .context("Failed to send HTTP request")?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            anyhow::bail!("API returned error status {}: {}", status, text);
        }

        Ok(())
    }
}

