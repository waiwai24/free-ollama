use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Target {
    pub host: String,
    pub port: u16,
    pub source: String,
    pub country: Option<String>,
    pub is_https: bool,
}

impl Target {
    pub fn endpoint(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
    
    pub fn base_url(&self) -> String {
        let protocol = if self.is_https { "https" } else { "http" };
        format!("{}://{}:{}", protocol, self.host, self.port)
    }
}

impl Default for Target {
    fn default() -> Self {
        Self {
            host: String::new(),
            port: 80,
            source: String::new(),
            country: None,
            is_https: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaService {
    pub target: Target,
    pub is_active: bool,
    pub version: Option<String>,
    pub models: Vec<ModelInfo>,
    pub scan_time: DateTime<Utc>,
    pub response_time: Option<u64>,
    pub confidence_score: Option<f64>,
    #[serde(default)]
    pub detection_details: DetectionDetails,
}

impl Default for OllamaService {
    fn default() -> Self {
        Self {
            target: Default::default(),
            is_active: false,
            version: None,
            models: vec![],
            scan_time: chrono::Utc::now(),
            response_time: None,
            confidence_score: None,
            detection_details: Default::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub name: String,
    pub size: Option<u64>,
    pub modified_at: Option<DateTime<Utc>>,
    pub digest: Option<String>,
    pub details: Option<ModelDetails>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelDetails {
    pub format: Option<String>,
    pub family: Option<String>,
    pub families: Option<Vec<String>>,
    pub parameter_size: Option<String>,
    pub quantization_level: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DetectionDetails {
    #[serde(default)]
    pub endpoints_checked: Vec<EndpointResult>,
    #[serde(default)]
    pub http_headers: HashMap<String, String>,
    #[serde(default)]
    pub response_patterns: Vec<String>,
    #[serde(default)]
    pub authenticity_indicators: Vec<AuthenticityIndicator>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointResult {
    pub path: String,
    pub status_code: Option<u16>,
    pub response_time: Option<u64>,
    pub success: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticityIndicator {
    pub indicator_type: String,
    pub value: String,
    pub confidence: f64,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub target: Target,
    pub test_type: TestType,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub average_response_time: f64,
    pub min_response_time: u64,
    pub max_response_time: u64,
    pub requests_per_second: f64,
    pub error_details: Vec<ErrorDetail>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TestType {
    HealthCheck,
    PerformanceTest,
    TpsTest,
    AuthenticityTest,
    AntiDisguiseTest,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorDetail {
    pub error_type: String,
    pub count: u64,
    pub percentage: f64,
    pub examples: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanReport {
    pub scan_id: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub total_targets: usize,
    pub active_services: usize,
    pub suspicious_services: usize,
    pub performance_summary: PerformanceSummary,
    pub services: Vec<OllamaService>,
    pub performance_metrics: Vec<PerformanceMetrics>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceSummary {
    pub fastest_service: Option<Target>,
    pub slowest_service: Option<Target>,
    pub most_reliable_service: Option<Target>,
    pub average_response_time: f64,
    pub total_models_found: usize,
    pub unique_model_names: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaAssetCsv {
    pub country: String,
    pub link: String,
}