/// This file defines the HttpClientScanner struct and its associated methods for making HTTP requests, processing responses, and handling errors. It uses the reqwest crate for HTTP operations and provides a structured way to interact with web resources while managing potential issues that may arise during the scanning process.
/// 
/// The HttpClientScanner struct encapsulates the HTTP client configuration and provides methods for building URLs, making GET requests, extracting results from responses, and orchestrating the scanning process. The error handling is implemented through a custom HttpClientError enum to provide more informative messages and manage different failure scenarios effectively.
/// 
/// #Error
/// 
/// The HttpClientError enum defines various error types that can occur during HTTP client operations, such as invalid URLs, timeouts, request failures, body read failures, and response processing errors. Each variant provides a descriptive message to help identify the nature of the error and facilitate debugging and error handling in the scanning process.
/// 
use reqwest::{header, redirect, Client, Response};
use serde::Serialize;
use tokio::time::Instant;
use std::time::Duration;
use thiserror::Error;
use url::Url;


/// Clean response from the request, this were thought to be used for the output of the results in a structured way, and also for future integration with databases or other output formats.
/// 
///  This will be messaged to reporter
/// 
#[derive(Debug, Clone, Serialize)]
pub struct ScanResult {
    pub url: String,
    pub status_code: u16,
    pub content_length: usize,
    pub latency_ms: u128,
}

/// Simple configuration for HTTP client
#[derive(Debug, Clone)]
pub struct HttpClientConfig {
    pub timeout_secs: u64,
    pub max_redirects: usize,
    pub user_agent: String,
}

impl Default for HttpClientConfig {
    fn default() -> Self {
        Self {
            timeout_secs: 10,
            max_redirects: 5,
            user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36".to_string(),
        }
    }
}

/// Handled error for HTTP module
/// Each failure scenario is defined here to provide more informative messages and manage different error types effectively.
#[derive(Debug, Error)]
pub enum HttpClientError {
    #[error("Invalid target URL: {0}")]
    InvalidTarget(String),

    #[error("Invalid path: {0}")]
    InvalidPath(String),

    #[error("Failed to parse URL: {0}")]
    UrlParseError(#[from] url::ParseError),

    #[error("HTTP request failed: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("Failed to build HTTP client: {0}")]
    ClientBuildError(String),
}

// Principal HTTP client of Crabkit
pub struct HttpClientScanner {
    pub client: Client,
    pub base_url: String,
}

/// Quiero un café pero sé que si lo tomo ahora que son las 3a.m no podré dormir 
/// y mañana tengo que estudiar para el parcial de Comina...terminaré esto y me iré a dormir
/// pucha encima el AWS Solutions Arquitect el próximo mes...jajajaj ojalá sobreviva
impl HttpClientScanner {
    /// Create a scanner with default configuration
    pub fn new(base_url: &str) -> Result<Self, HttpClientError> {
        Self::with_config(base_url, HttpClientConfig::default())
    }

    /// Create a scanner with personalized configuration
    pub fn with_config(
        base_url: &str, 
        config: HttpClientConfig
    ) -> Result<Self, HttpClientError> {
        let normalized_base_url = Self::normalize_target(base_url)?;
        let base_url = Url::parse(&normalized_base_url)?;

        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::ACCEPT, 
            header::HeaderValue::from_static(
                "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8"
            ),
        );

        headers.insert(
            header::CONNECTION,
            header::HeaderValue::from_static("keep-alive"),
        );

        let client = Client::builder()
        .timeout(Duration::from_secs(config.timeout_secs))
        .user_agent(config.user_agent)
        .default_headers(headers)
        .redirect(redirect::Policy::limited(config.max_redirects))
        .build()
        .map_err(|e| HttpClientError::ClientBuildError(e.to_string()))?;
        
        Ok(Self {
            client,
            base_url: base_url.to_string(),
        })
    }

    /// Normalize the target URL to ensure it has a proper scheme and format
    /// 
    /// This avoids problems like:
    /// - Missing scheme (http:// or https://)
    /// - Empty path issues
    /// - Absolute URL's inside the wordlist
    fn normalize_path(path: &str) -> Result<String, HttpClientError> {
        let path = path.trim();

        if path.is_empty() {
            return Err(HttpClientError::InvalidPath(
                "Path cannot be empty".to_string()
            ));
        }
        
        if path.starts_with("http://") || path.starts_with("https://") || path.starts_with("/") {
            return Err(HttpClientError::InvalidPath(
                "Absolute URLs are not allowed in the path".to_string()
            ));
        }

        if path.starts_with("/") {
            return Ok(path.to_string());
        } else {
            Ok(format!("/{}", path))
        }
    }

    /// Union of target and path to create the full URL for the request, this is important to avoid path traversal issues and ensure that the URLs are properly formed.
    pub fn build_url(&self, path: &str) -> Result<Url, HttpClientError> {
        let mut normalized_path = Self::normalize_path(path)?;
        let url = self.base_url.join(&normalized_path)?;

        Ok(url)
    }

    // Implement a GET method to make the requests
    pub async fn send_get(&self, url: Url) -> Result<Response, HttpClientError> {
        let response = self.client.get(url).send().await?;

        Ok(response)
    }

    /// Extract the relevant information from the response, such as status code, content length, and latency. This is crucial for analyzing the results and determining the success of the scan.
    async fn extract_result(
        &self,
        url: Url,
        path: &str,
        response: Response,
        latency_ms: u128,
    ) -> Result<ScanResult, HttpClientError> {
        let status_code = response.status().as_u16();

        let header_content_length = response.content_length();

        let content_length = header_content_length
            .map(|value| value as usize)
            .unwrap_or_else(|| body.len());

        Ok(ScanResult {
            url: url.to_string(),
            path: path.to_string(),
            status_code,
            content_length,
            latency_ms,
        })
    }

    // Orchestate a complete request to scan a specific path
    pub async fn scan_path(&self, path: &str) -> Result<ScanResult, HttpClientError> {
        let url = self.build_url(path)?;

        let start = Instant::now();

        let response = self.send_get(url.clone()).await?;

        let latency_ms = start.elapsed().as_millis();

        let result = self
            .extract_result(url, path, response, latency_ms)
            .await?;

        Ok((result))
    }
}
