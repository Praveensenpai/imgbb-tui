use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct ImagePayload {
    pub bytes: Vec<u8>,
    pub filename: String,
}

impl ImagePayload {
    pub fn from_file(path: &PathBuf) -> Result<Self, crate::domain::error::AppError> {
        use crate::domain::error::AppError;
        if !path.exists() {
            return Err(AppError::FileNotFound(path.display().to_string()));
        }
        let bytes = std::fs::read(path)?;
        let filename = path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "upload.png".to_string());
        Ok(Self { bytes, filename })
    }

    pub fn size(&self) -> u64 {
        self.bytes.len() as u64
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadResult {
    pub url: String,
    pub viewer_url: String,
    pub thumb_url: String,
    pub delete_url: String,
    pub filename: String,
    pub size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub url: String,
    pub filename: String,
    pub uploaded_at: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UploadState {
    Idle,
    Uploading,
    Success,
    Failed,
}
