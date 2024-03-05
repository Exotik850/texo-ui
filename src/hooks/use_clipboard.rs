use std::cell::Ref;

#[cfg(all(not(web_sys_unstable_apis), feature = "web"))]
compile_error!("Use the environment variable RUSTFLAGS=\"--cfg=web_sys_unstable_apis\" to use clipboard functionality");

#[cfg(feature = "desktop")]
use arboard::Clipboard;
use dioxus::prelude::*;
use generational_box::GenerationalRef;
use thiserror::Error;
#[cfg(feature = "web")]
use {
    wasm_bindgen_futures::JsFuture,
    web_sys::{Clipboard, Window},
};

#[derive(Error, Debug)]
pub enum ClipboardError {
    #[cfg(feature = "desktop")]
    #[error(transparent)]
    ArboardError(#[from] arboard::Error),
    #[cfg(feature = "web")]
    #[error("Could not call web sys clipboard function: {0}")]
    WebSysError(String),
}

#[derive(Copy, Clone)]
pub struct ClipboardManager {
    pub value: Signal<String>,
    #[cfg(any(feature = "web", feature = "desktop"))]
    clipboard: Signal<Clipboard>,
}

impl ClipboardManager {
    /// This calls the `web_sys` implementation of the `readText()` method
    /// https://developer.mozilla.org/en-US/docs/Web/API/Clipboard/readText
    #[cfg(feature = "web")]
    pub fn get(mut self) {
        spawn(async move {
            let action = self.clipboard.read().read_text();
            let result = JsFuture::from(action).await;

            let clip = match result {
                Ok(value) => value.as_string().expect("Should only return string"),
                Err(err) => {
                    log::error!("Could not read from clipboard: {:?}", err.as_string());
                    return;
                }
            };
            self.value.set(clip);
        });
    }

    #[cfg(feature = "desktop")]
    pub fn get(&mut self) -> Result<(), ClipboardError> {
        let clip = self.clipboard.write().get_text()?;
        self.value.set(clip);
        Ok(())
    }

    /// This calls the `web_sys` implementation of the `writeText()` method
    /// https://developer.mozilla.org/en-US/docs/Web/API/Clipboard/writeText
    #[cfg(feature = "web")]
    pub fn write(mut self, value: String) {
        spawn(async move {
            let action = self.clipboard.write().write_text(&value);
            let result = JsFuture::from(action).await;
            if let Err(err) = result {
                log::error!("Could not write to clipboard: {:?}", err.as_string());
                return;
            }
            self.value.set(value);
        });
    }

    #[cfg(feature = "desktop")]
    pub fn write(&mut self, value: String) -> Result<(), ClipboardError> {
        self.clipboard.write().set_text(&value)?;
        self.value.set(value);
        Ok(())
    }

    /// Returns a reference to the current value of the internal signal
    /// and subscribes it to the current scope
    pub fn value(&self) -> GenerationalRef<Ref<String>> {
        self.value.read()
    }
}

/// Returns a struct that allows for the retrieval and setting of the clipboard
/// value. This detects what platform you are using (WASM32 / Windows / Mac / Linux)
/// and uses the correct implementation accordingly.
///
/// Note: due to security restrictions there is no way to automatically update the value with the user's clipboard,
/// the `get` function must be manually called to check the value of the clipboard
#[cfg(feature = "desktop")]
pub fn use_clipboard() -> Result<ClipboardManager, ClipboardError> {
    let value = use_signal(String::new);
    let clipboard = use_signal(|| arboard::Clipboard::new().expect("Platform is not supported"));
    Ok(ClipboardManager { value, clipboard })
}

/// Returns a struct that allows for the retrieval and setting of the clipboard
/// value. This detects what platform you are using (WASM32 / Windows / Mac / Linux)
/// and uses the correct implementation accordingly.
///
/// Note: due to security restrictions there is no way to automatically update the value with the user's clipboard,
/// the `get` function must be manually called to check the value of the clipboard
///
/// This uses the `web_sys` clipboard implementation, and requires the `web_sys_unstable_apis` enabled via the `RUSTFLAGS` environment variable
/// e.g: `RUSTFLAGS="--cfg=web_sys_unstable_apis" cargo run`
#[cfg(feature = "web")]
pub fn use_clipboard() -> Result<ClipboardManager, ClipboardError> {
    let value = use_signal(String::new);
    let clipboard = use_signal(|| {
        let window = web_sys::window().expect("Should have a window");
        window
            .navigator()
            .clipboard()
            .expect("Should have a navigator and clipboard")
    });

    Ok(ClipboardManager { value, clipboard })
}
