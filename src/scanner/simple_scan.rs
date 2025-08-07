use reqwest::Client;
use std::time::Duration;
use crate::storage::{OllamaService, Target, DetectionDetails, ModelInfo, ModelDetails};
use crate::error::Result;
use serde::Deserialize;
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Instant;

pub struct SimpleScanner;

impl SimpleScanner {
    pub async fn scan_services(targets: Vec<Target>, timeout_secs: u64) -> Result<Vec<OllamaService>> {
        let client = Client::builder()
            .timeout(Duration::from_secs(timeout_secs))
            .build()?;
            
        let total = targets.len();
        let pb = ProgressBar::new(total as u64);
        pb.set_style(ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta}) {msg}")
            .unwrap_or_else(|_| ProgressStyle::default_bar())
            .progress_chars("#>-"));
            
        let start_time = Instant::now();
        
        // 创建并发任务
        let handles: Vec<_> = targets
            .into_iter()
            .map(|target| {
                let client = client.clone();
                let pb = pb.clone();
                tokio::spawn(async move {
                    pb.set_message(format!("Scanning {}", target.base_url()));
                    let service = Self::scan_service(&client, target, timeout_secs).await;
                    pb.inc(1);
                    service
                })
            })
            .collect();

        // 等待所有任务完成并收集结果
        let mut services = Vec::with_capacity(total);
        for handle in handles {
            if let Ok(result) = handle.await {
                if let Ok(service) = result {
                    services.push(service);
                }
            }
        }
        
        pb.finish_with_message(format!("Scan completed in {:.2?}", start_time.elapsed()));
        
        Ok(services)
    }
    
    async fn scan_service(client: &Client, target: Target, timeout_secs: u64) -> Result<OllamaService> {
        #[derive(Debug, Deserialize)]
        struct ApiModelInfo {
            name: String,
            modified_at: String,
            size: u64,
            digest: String,
            details: ApiModelDetails,
        }

        #[derive(Debug, Deserialize)]
        struct ApiModelDetails {
            format: String,
            family: String,
            families: Vec<String>,
            parameter_size: String,
            quantization_level: String,
        }

        #[derive(Debug, Deserialize)]
        struct TagsResponse {
            models: Vec<ApiModelInfo>,
        }
        
        let url = format!("{}/api/tags", target.base_url());
        let start_time = std::time::Instant::now();
        
        match client.get(&url)
            .timeout(Duration::from_secs(3))  // 固定3秒超时
            .send()
            .await 
        {
            Ok(response) => {
                let response_time = start_time.elapsed().as_millis() as u64;
                let status = response.status().as_u16();
                
                if status == 200 {
                    match response.json::<TagsResponse>().await {
                        Ok(tags) => {
                            // 成功获取到tags响应，确认是Ollama服务
                            let model_count = tags.models.len();
                            let models = tags.models.iter()
                                .map(|m| ModelInfo {
                                    name: m.name.clone(),
                                    size: Some(m.size),
                                    modified_at: Some(chrono::DateTime::parse_from_rfc3339(&m.modified_at)
                                        .map(|dt| dt.with_timezone(&chrono::Utc))
                                        .unwrap_or_else(|_| chrono::Utc::now())),
                                    digest: Some(m.digest.clone()),
                                    details: Some(ModelDetails {
                                        format: Some(m.details.format.clone()),
                                        family: Some(m.details.family.clone()),
                                        families: Some(m.details.families.clone()),
                                        parameter_size: Some(m.details.parameter_size.clone()),
                                        quantization_level: Some(m.details.quantization_level.clone()),
                                    }),
                                })
                                .collect();

                            let mut detection_details = DetectionDetails::default();
                            detection_details.response_patterns.push(format!(
                                "Found {} models, format: {}", 
                                model_count,
                                tags.models.first()
                                    .map(|m| m.details.format.as_str())
                                    .unwrap_or("unknown")
                            ));

                            Ok(OllamaService {
                                target,
                                is_active: true,
                                version: Some(tags.models.first()
                                    .map(|m| m.details.format.clone())
                                    .unwrap_or_default()),
                                models,
                                scan_time: chrono::Utc::now(),
                                response_time: Some(response_time),
                                confidence_score: Some(1.0),
                                detection_details,
                            })
                        },
                        Err(_) => {
                            // 能访问但不是有效的Ollama服务
                            Ok(OllamaService {
                                target,
                                is_active: false,
                                version: None,
                                models: vec![],
                                scan_time: chrono::Utc::now(),
                                response_time: Some(response_time),
                                confidence_score: Some(0.0),
                                detection_details: Default::default(),
                            })
                        }
                    }
                } else {
                    // HTTP状态码不是200，不是有效的Ollama服务
                    Ok(OllamaService {
                        target,
                        is_active: false,
                        version: None,
                        models: vec![],
                        scan_time: chrono::Utc::now(),
                        response_time: Some(response_time),
                        confidence_score: Some(0.0),
                        detection_details: Default::default(),
                    })
                }
            }
            Err(_) => {
                let response_time = start_time.elapsed().as_millis() as u64;
                // 网络错误，无法连接
                Ok(OllamaService {
                    target,
                    is_active: false,
                    version: None,
                    models: vec![],
                    scan_time: chrono::Utc::now(),
                    response_time: Some(response_time),
                    confidence_score: Some(0.0),
                    detection_details: Default::default(),
                })
            }
        }
    }
}