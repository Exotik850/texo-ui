use dioxus::prelude::*;

#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub enum ToastType {
    Action,
    Success,
    Info,
    Warning,
    Error,
    Loading,
    #[default]
    Default,
}

#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub enum ToastPosition {
    TopLeft,
    TopCenter,
    TopRight,
    BottomLeft,
    BottomCenter,
    #[default]
    BottomRight,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ToastInfo {
    pub id: u32,
    pub title: String,
    pub description: Option<Element>,
    pub options: ToastOptions,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ToastOptions {
    pub kind: ToastType,
    pub icon: Option<Element>,
    pub invert: bool,
    pub duration: u32,
    pub action: Option<EventHandler<MouseEvent>>,
    pub cancel: Option<EventHandler>,
    pub ondismiss: Option<EventHandler>,
    pub onautoclose: Option<EventHandler>,
    pub dismissible: bool,
    pub class: String,
    pub unstyled: bool,
    pub position: ToastPosition,
}

impl Default for ToastOptions {
    fn default() -> Self {
        Self {
            dismissible: true,
            duration: 250,
            kind: ToastType::Default,
            icon: None,
            invert: false,
            action: None,
            cancel: None,
            ondismiss: None,
            onautoclose: None,
            class: "".into(),
            unstyled: false,
            position: ToastPosition::BottomRight,
        }
    }
}

#[derive(Clone, Copy)]
pub(crate) struct ToastManager {
    pub toasts: Signal<Vec<ToastInfo>>,
    pub heights: Signal<Vec<u32>>,
}

/// Displays a toast notification with the specified title, description, and options.
///
/// # Arguments
///
/// * `title` - The title of the toast notification.
/// * `description` - The description of the toast notification.
/// * `options` - The options for the toast notification.
pub fn toast(title: impl std::fmt::Display, description: Option<Element>, options: ToastOptions) {
    let ToastManager {
        mut toasts,
        heights,
    } = use_context();

    

    let new_toast = ToastInfo {
        id: toast_id(toasts.read().len()),
        description,
        title: title.to_string(),
        options,
    };
    toasts.push(new_toast);
}

#[inline]
fn toast_id(id: usize) -> u32 {
    let mut id = id as u32;
    id = id.wrapping_mul(0x9E3779B1);
    id = id ^ (id >> 16);
    id
}
