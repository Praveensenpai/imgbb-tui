use crate::domain::error::AppError;
use crate::domain::model::{ImagePayload, UploadResult};
use base64::Engine;
use serde::Deserialize;
use std::time::Duration;

const UPLOAD_URL: &str = "https://api.imgbb.com/1/upload";
const TIMEOUT_SECS: u64 = 120;

#[derive(Debug, Deserialize)]
struct ApiResponse {
    data: Option<ApiData>,
    error: Option<ApiError>,
    status_code: Option<u16>,
}

#[derive(Debug, Deserialize)]
struct ApiData {
    url: String,
    display_url: String,
    thumb: Option<Thumb>,
    delete_url: Option<String>,
    image: Option<ImageInfo>,
}

#[derive(Debug, Deserialize)]
struct Thumb {
    url: String,
}

#[derive(Debug, Deserialize)]
struct ImageInfo {
    filename: Option<String>,
    size: Option<u64>,
}

#[derive(Debug, Deserialize)]
struct ApiError {
    message: String,
}

pub struct ImgbbClient {
    http: reqwest::Client,
    api_key: String,
}

impl ImgbbClient {
    pub fn new(api_key: String) -> Result<Self, AppError> {
        let http = reqwest::Client::builder()
            .timeout(Duration::from_secs(TIMEOUT_SECS))
            .build()
            .map_err(|e| AppError::Upload(e.to_string()))?;
        Ok(Self { http, api_key })
    }

    pub async fn upload(&self, payload: &ImagePayload) -> Result<UploadResult, AppError> {
        let encoded = base64::engine::general_purpose::STANDARD.encode(&payload.bytes);
        let form = reqwest::multipart::Form::new()
            .text("key", self.api_key.clone())
            .text("image", encoded);

        let response = self
            .http
            .post(UPLOAD_URL)
            .multipart(form)
            .send()
            .await
            .map_err(|e| AppError::Upload(e.to_string()))?;

        let status = response.status();
        let body: ApiResponse = response
            .json()
            .await
            .map_err(|e| AppError::Upload(format!("invalid response: {e}")))?;

        if !status.is_success() {
            return Err(AppError::Upload(Self::describe_error(
                &body,
                status.as_u16(),
            )));
        }
        Self::into_result(body, payload)
    }

    fn into_result(body: ApiResponse, payload: &ImagePayload) -> Result<UploadResult, AppError> {
        let data = body
            .data
            .ok_or_else(|| AppError::Upload("empty response data".to_string()))?;
        let thumb_url = data
            .thumb
            .map(|t| t.url)
            .unwrap_or_else(|| data.url.clone());
        let filename = data
            .image
            .as_ref()
            .and_then(|i| i.filename.clone())
            .unwrap_or_else(|| payload.filename.clone());
        let size = data
            .image
            .as_ref()
            .and_then(|i| i.size)
            .unwrap_or_else(|| payload.size());

        Ok(UploadResult {
            url: data.url,
            viewer_url: data.display_url,
            thumb_url,
            delete_url: data.delete_url.unwrap_or_default(),
            filename,
            size,
        })
    }

    fn describe_error(body: &ApiResponse, status: u16) -> String {
        let code = body.status_code.unwrap_or(status);
        match &body.error {
            Some(err) => format!("[{code}] {}", err.message),
            None => format!("HTTP {status}"),
        }
    }
}
