use serde::Serialize;
use reqwest::Client;
use crate::scanner::SecretDetection;

#[derive(Serialize, Debug)]
pub struct ScanEvent {
    pub file_path: String,
    pub line_number: usize,
    pub secret_type: String,
    pub confidence_score: f64,
    pub blocked_commit: bool,
    // Note: We don't send the actual secret content for security
}

pub struct ApiClient {
    client: Client,
    base_url: String,
    api_key: Option<String>,
}

impl ApiClient {
    pub fn new(base_url: String, api_key: Option<String>) -> Self {
        Self {
            client: Client::new(),
            base_url,
            api_key,
        }
    }

    pub async fn report_scan_event(
        &self, 
        detection: &SecretDetection, 
        blocked: bool
    ) -> Result<(), Box<dyn std::error::Error>> {
        let event = ScanEvent {
            file_path: detection.file_path.clone(),
            line_number: detection.line_number,
            secret_type: detection.secret_type.clone(),
            confidence_score: detection.confidence_score,
            blocked_commit: blocked,
        };

        let mut request = self.client
            .post(&format!("{}/api/scan-events", self.base_url))
            .json(&event);

        if let Some(api_key) = &self.api_key {
            request = request.bearer_auth(api_key);
        }

        let response = request.send().await?;
        
        if !response.status().is_success() {
            eprintln!("Warning: Failed to report scan event to dashboard");
        }

        Ok(())
    }
}