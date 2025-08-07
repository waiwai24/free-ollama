pub mod config;
pub mod scanner;
pub mod storage;
pub mod utils;
pub mod error;

// 重新导出常用的类型和错误处理
pub use crate::error::{OllamaError, Result};
pub use crate::storage::{Target, OllamaService, ModelInfo};