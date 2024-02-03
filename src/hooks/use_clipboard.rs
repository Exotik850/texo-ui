use std::cell::Ref;

#[cfg(not(feature="web"))]
use arboard::Clipboard;
use dioxus::prelude::*;
use generational_box::GenerationalRef;
use thiserror::Error;
#[cfg(feature="web")]
use {
    wasm_bindgen_futures::JsFuture,
    web_sys::{Clipboard, Window},
};

#[derive(Error, Debug)]
pub enum ClipboardError {
    #[cfg(not(feature="web"))]
    #[error(transparent)]
    ArboardError(#[from] arboard::Error),
    #[cfg(feature="web")]
    #[error("Could not call web sys clipboard function: {0}")]
    WebSysError(String),
}

#[derive(Copy, Clone)]
pub struct ClipboardManager {
    pub value: Signal<String>,
    #[cfg(not(feature="web"))]
    clipboard: Signal<Clipboard>,
    #[cfg(feature="web")]
    clipboard: Signal<Clipboard>,
}

impl ClipboardManager {
    /// This calls the `web_sys` implementation of the `readText()` method
    /// https://developer.mozilla.org/en-US/docs/Web/API/Clipboard/readText
    #[cfg(feature="web")]
    pub async fn get(&mut self) { // -> Result<(), ClipboardError> {
        let action = self.clipboard.read().read_text();
        let result = JsFuture::from(action).await;

        let clip = result.unwrap().as_string().expect("Should only return string");
        // let clip = match result {
        //     Ok(value) => value
        //         .as_string()
        //         .expect("clipboard call should return a string"),
        //     Err(err) => return Err(ClipboardError::WebSysError(format!("{:?}", err.as_string()))),
        // };

        self.value.set(clip);

        // Ok(())
    }

    #[cfg(not(feature="web"))]
    pub fn get(&mut self) -> Result<(), ClipboardError> {
        let clip = self.clipboard.write().get_text()?;
        self.value.set(clip);

        Ok(())
    }

    /// This calls the `web_sys` implementation of the `writeText()` method
    /// https://developer.mozilla.org/en-US/docs/Web/API/Clipboard/writeText
    #[cfg(feature="web")]
    pub async fn write(&mut self, value: String) -> Result<(), ClipboardError> {
        let action = self.clipboard.write().write_text(&value);
        let result = JsFuture::from(action).await;

        match result {
            Ok(value) => (),
            Err(err) => return Err(ClipboardError::WebSysError(format!("{:?}", err.as_string()))),
        }

        self.value.set(value);

        Ok(())
    }

    #[cfg(not(feature="web"))]
    pub fn write(&mut self, value: String) -> Result<(), ClipboardError> {
        self.clipboard.write().set_text(&value)?;
        self.value.set(value);
        Ok(())
    }

    pub fn value(&self) -> GenerationalRef<Ref<String>> {
        self.value.read()
    }
}

fn maybe() -> Option<i32> {
   None
}

/// Returns a struct that allows for the retrieval and setting of the clipboard
/// value. This detects what platform you are using (WASM32 / Windows / Mac / Linux)
/// and uses the correct implementation accordingly. Note: due to security restrictions
/// there is no way to automatically update the value with the user's clipboard,
/// the `get` function must be manually called to check the value of the clipboard
/// and update the signal
#[cfg(not(feature="web"))]
pub fn use_clipboard() -> Result<ClipboardManager, ClipboardError> {
    let value = use_signal(String::new);
    let other = use_signal(maybe);

    // TODO check where this would panic exactly
    let clipboard = use_signal(|| arboard::Clipboard::new().expect("Platform is not supported"));
    Ok(ClipboardManager {
        value,
        clipboard,
    })
}
#[cfg(feature="web")]
pub fn use_clipboard() -> Result<ClipboardManager, ClipboardError> {
    let value = use_signal(String::new);
    // let window = web_sys::window().ok_or(ClipboardError::WebSysError(
    //     "Could not get window from web sys".into(),
    // ))?;
    // let clipboard = window
    //     .navigator()
    //     .clipboard()
    //     .ok_or(ClipboardError::WebSysError(
    //         "Could not get clipboard from web sys window".into(),
    //     ))?;
    let clipboard = use_signal(|| {
      let window = web_sys::window().expect("Should have a window");
      window.navigator().clipboard().expect("Should have a navigator and clipboard")
    });

    Ok(ClipboardManager { value, clipboard })
}
