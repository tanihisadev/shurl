use arboard::Clipboard;

#[cfg(target_os = "linux")]
use arboard::SetExtLinux;

#[derive(Debug)]
#[allow(dead_code)]
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
    #[cfg(target_os = "linux")]
    {
        let text = text.to_string();
        std::thread::spawn(move || {
            if let Ok(mut clipboard) = Clipboard::new() {
                let _ = clipboard.set().wait().text(text);
            }
        });

        // Give the thread a moment to initialise before main exits
        std::thread::sleep(std::time::Duration::from_millis(50));
        return Ok(());
    }

    #[cfg(not(target_os = "linux"))]
    {
        let mut clipboard = Clipboard::new()
            .map_err(|e| ClipboardError::Failed(e.to_string()))?;
        clipboard
            .set_text(text)
            .map_err(|e| ClipboardError::Failed(e.to_string()))?;
        Ok(())
    }
}