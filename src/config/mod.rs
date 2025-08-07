use config::ConfigError;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub request_timeout: u64,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            request_timeout: 1,
        }
    }
}

pub fn load_config() -> Result<AppConfig, ConfigError> {
    // 简化配置加载，只返回默认配置
    Ok(AppConfig::default())
}