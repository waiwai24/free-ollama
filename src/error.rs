use thiserror::Error;

pub type Result<T> = std::result::Result<T, OllamaError>;

#[derive(Error, Debug)]
pub enum OllamaError {
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("CSV parsing error: {0}")]
    Csv(#[from] csv::Error),
    
    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("Configuration error: {0}")]
    Config(#[from] config::ConfigError),
    
    #[error("Parse error: {0}")]
    ParseError(String),
    
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    
    #[error("Service unavailable: {endpoint}")]
    ServiceUnavailable { endpoint: String },
    
    #[error("Request timeout")]
    Timeout,
    
    #[error("Authentication failed")]
    AuthenticationFailed,
    
    #[error("Service detection failed: {reason}")]
    DetectionFailed { reason: String },
    
    #[error("Performance test failed: {reason}")]
    PerformanceTestFailed { reason: String },
}