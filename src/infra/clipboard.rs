use crate::domain::error::AppError;
use std::io::Write;
use std::process::{Command, Stdio};

const MAX_IMAGE_BYTES: u64 = 32 * 1024 * 1024;

pub fn read_clipboard_image() -> Result<Vec<u8>, AppError> {
    let bytes = run_paste(&["--type", "image/png"])?;
    if bytes.is_empty() {
        return Err(AppError::ClipboardEmpty(
            "no image/png on clipboard".to_string(),
        ));
    }
    if bytes.len() as u64 > MAX_IMAGE_BYTES {
        return Err(AppError::TooLarge {
            max: MAX_IMAGE_BYTES,
            got: bytes.len() as u64,
        });
    }
    Ok(bytes)
}

pub fn copy_to_clipboard(text: &str) -> Result<(), AppError> {
    let mut child = Command::new("wl-copy")
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| AppError::Clipboard(format!("failed to spawn wl-copy: {e}")))?;

    if let Some(stdin) = child.stdin.as_mut() {
        stdin
            .write_all(text.as_bytes())
            .map_err(|e| AppError::Clipboard(format!("failed to write to wl-copy: {e}")))?;
    }

    let status = child
        .wait()
        .map_err(|e| AppError::Clipboard(format!("wl-copy wait failed: {e}")))?;

    if !status.success() {
        return Err(AppError::Clipboard("wl-copy exited non-zero".to_string()));
    }
    Ok(())
}

fn run_paste(args: &[&str]) -> Result<Vec<u8>, AppError> {
    let output = Command::new("wl-paste")
        .args(args)
        .output()
        .map_err(|e| AppError::Clipboard(format!("failed to spawn wl-paste: {e}")))?;

    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr);
        return Err(AppError::Clipboard(format!("wl-paste failed: {err}")));
    }
    Ok(output.stdout)
}
