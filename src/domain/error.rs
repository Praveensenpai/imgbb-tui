use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("no image in clipboard: {0}")]
    ClipboardEmpty(String),
    #[error("clipboard error: {0}")]
    Clipboard(String),
    #[error("image file not found: {0}")]
    FileNotFound(String),
    #[error("image too large (max {max} bytes, got {got})")]
    TooLarge { max: u64, got: u64 },
    #[error("upload failed: {0}")]
    Upload(String),
    #[error("config error: {0}")]
    Config(String),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}
