use arboard::Clipboard;

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

    clipboard.set_text(text).map_err(|e| ClipboardError::Failed(e.to_string()))?;

    Ok(())
}