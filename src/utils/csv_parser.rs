use csv::Reader;
use std::fs::File;
use std::io::{BufReader, Read};
use url::Url;
use crate::{Result, OllamaError};
use crate::storage::{Target, OllamaAssetCsv};

pub struct CsvParser;

impl CsvParser {
    /// 解析ollama资产CSV格式 (country,link)
    pub fn parse_ollama_assets<R: Read>(reader: R) -> Result<Vec<Target>> {
        let mut csv_reader = Reader::from_reader(reader);
        let mut targets = Vec::new();
        
        for (line_num, result) in csv_reader.deserialize::<OllamaAssetCsv>().enumerate() {
            match result {
                Ok(asset) => {
                    match Self::parse_url_to_target(&asset.link, &asset.country, line_num + 1) {
                        Ok(target) => targets.push(target),
                        Err(e) => {
                            log::warn!("Failed to parse URL '{}' at line {}: {}", asset.link, line_num + 1, e);
                            continue;
                        }
                    }
                }
                Err(e) => {
                    log::error!("Failed to parse CSV line {}: {}", line_num + 1, e);
                    return Err(OllamaError::Csv(e));
                }
            }
        }
        
        Ok(targets)
    }
    
    /// 解析URL字符串为Target结构体
    fn parse_url_to_target(url_str: &str, country: &str, line_num: usize) -> Result<Target> {
        let url = Url::parse(url_str).map_err(|e| {
            OllamaError::ParseError(format!("Invalid URL '{}': {}", url_str, e))
        })?;
        
        let host = url.host_str().ok_or_else(|| {
            OllamaError::ParseError(format!("No host found in URL '{}'", url_str))
        })?.to_string();
        
        let port = url.port().unwrap_or(match url.scheme() {
            "https" => 443,
            "http" => 80,
            _ => return Err(OllamaError::ParseError(format!("Unsupported scheme in URL '{}'", url_str)))
        });
        
        let is_https = url.scheme() == "https";
        
        Ok(Target {
            host,
            port,
            source: format!("Ollama-Assets-Line-{}", line_num),
            country: Some(country.to_string()),
            is_https,
        })
    }
    
    pub fn parse_from_file(file_path: &str) -> Result<Vec<Target>> {
        let file = File::open(file_path).map_err(OllamaError::Io)?;
        let reader = BufReader::new(file);
        Self::parse_ollama_assets(reader)
    }
    
    pub fn validate_targets(targets: &[Target]) -> Vec<&Target> {
        targets.iter()
            .filter(|target| !target.host.is_empty() && target.port > 0)
            .collect()
    }
}