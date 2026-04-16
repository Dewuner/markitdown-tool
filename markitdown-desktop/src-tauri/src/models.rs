use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ConversionResult {
    pub success: bool,
    pub data: Option<ConversionData>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConversionData {
    pub filename: String,
    pub source_path: String,
    pub output_path: Option<String>,
    pub markdown_content: Option<String>,
    pub image_paths: Option<Vec<String>>,
    pub file_size: Option<i64>,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchResult {
    pub success: bool,
    pub data: Option<BatchData>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchData {
    pub results: Vec<ConversionResult>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub id: i64,
    pub filename: String,
    pub source_path: String,
    pub output_path: Option<String>,
    pub status: String,
    pub error_message: Option<String>,
    pub markdown_content: Option<String>,
    pub image_paths: Option<String>,
    pub file_size: Option<i64>,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
pub struct IpcResponse<T: Serialize> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T: Serialize> IpcResponse<T> {
    pub fn ok(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn err(msg: impl Into<String>) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(msg.into()),
        }
    }
}
