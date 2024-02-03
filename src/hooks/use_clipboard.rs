use dioxus::prelude::*;
#[cfg(not(target_arch="wasm"))]
use arboard::Clipboard;
#[cfg(target_arch="wasm")]
use web_sys::{Window, Clipboard};

pub enum ClipboardError {
  #[cfg(not(target_arch="wasm"))]
  ArboardError(arboard::Error),
  #[cfg(target_arch="wasm")]
  WebSysError(web_sys::Error)
}

pub struct ClipboardManager {
  pub value: Signal<String>,
  #[cfg(not(target_arch="wasm"))]
  clipboard: Clipboard,
}

impl ClipboardManager {
  /// This will poll the system to retrieve the clipboard value and update 
  #[cfg(target_arch="wasm")]
  pub fn get(&mut self) {
    
  }
  
  #[cfg(not(target_arch="wasm"))]
  pub fn get(&mut self) -> Result<(), ClipboardError> {
    if let Some(clip) = self.clipboard.get_text().ok() {
      self.value.set(clip);
    }
    Ok(())
  }

  /// Get the current value of the signal
  pub fn read(&self) -> Ref<String> {
    self.value.read()
  }

  /// Copy the internal signal
  pub fn signal(&self) -> Signal<String> {
    self.value
  }

}

/// Returns a struct that allows for the retrieval and setting of the clipboard
/// value. This detects what platform you are using (WASM / Windows / Mac / Linux)
/// and uses the correct implementation accordingly. Note: due to security restrictions
/// there is no way to automatically update the value with the user's clipboard,
/// the `get` function must be manually called to check the value of the clipboard
/// and update the signal
pub fn use_clipboard() -> Result<ClipboardManager, ClipboardError> {
  let value = use_signal(String::new);
  Ok(ClipboardManager {
      value,
      #[cfg(not(target_arch="wasm"))]
      clipboard: arboard::Clipboard::new()?
    })
}