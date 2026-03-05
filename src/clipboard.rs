use arboard::Clipboard;

#[cfg(target_os = "linux")]
use arboard::SetExtLinux;

#[derive(Debug)]
pub enum ClipboardError {
    Failed(String),
}

impl std::fmt::Display for ClipboardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClipboardError::Failed(msg) => write!(f, "Clipboard error: {}", msg),
        }
    }
}

pub fn copy_to_clipboard(text: &str) -> Result<(), ClipboardError> {
    let mut clipboard = Clipboard::new().map_err(|e| ClipboardError::Failed(e.to_string()))?;

    // Process needs to stay alive longer on linux
    #[cfg(target_os = "linux")]
    clipboard
        .set()
        .wait()
        .text(text.to_string())
        .map_err(|e| ClipboardError::Failed(e.to_string()))?;

    #[cfg(not(target_os = "linux"))]
    clipboard
        .set_text(text)
        .map_err(|e| ClipboardError::Failed(e.to_string()))?;

    Ok(())
}