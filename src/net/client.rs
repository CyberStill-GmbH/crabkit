use reqwest::{Client, redirect, header};
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::Semaphore;
use std::time::Duration;

/// Set up the HTTP client 
pub struct HttpClientScanner {
    pub client: Client,
    pub base_url: String,
}

impl HttpClientScanner {
    pub fn new(base_url: String) -> Self {
        // Define headers for the client(Simulate a real navigator)
        let mut headers = header::HeaderMap::new();
        headers.insert(header::ACCEPT, "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8".parse().unwrap());
        headers.insert(header::CONNECTION, "keep-alive".parse().unwrap());
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
            .default_headers(headers)
            .redirect(redirect::Policy::limited(10))
            .build()
            .expect("Error while creating HTTP client");

        Self { client, base_url }
    }
    /// Chain target + path correctly
    pub fn build_url(&self, target: &str, path: &str) -> Result<String, url::ParseError> {
        let mut base = Url::parse(target)?;

        let full_url = base.join(path)?;

        Ok(full_url.to_string())
    }

    // Implement a GET method to make the requests
    pub async fn get_data(&self, endpoint: &str) -> reqwest::Result<reqwest::Response, Box<dyn std::error::Error>> {
        // Use the Url::join logic to enhance security and avoid path traversal issues
        let target_url = Self::build_url(self.base_url.clone(), endpoint)?;

        let response = self.client.get(target_url).send().await?;
        Ok(response)
    }

    // Obtains status code, content length and worth data from the response
    pub async fn extract_results(&self, response: reqwest::Response) -> Result<(u16, usize, String), Box<dyn std::error::Error>> {
        let status_code = response.status().as_u16();
        let content_length = response.content_length().unwrap_or(0) as usize;
        let latency = response.elapsed().as_millis();

        let body = response.text().await?;

        Ok((status_code, content_length, body, latency))
    }

    // Orchestrate the request and response processing
    pub async fn scan_path(&self, target: &str, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let url = self.build_url(target, path)?;
        let response = self.get_data(&url).await?;
        let (status_code, content_length, body, latency) = self.extract_results(response).await?;
        println!("{} - Status: {}, Length: {}, Latency: {} ms", url, status_code, content_length, latency);

        Ok(())
    }
}
