use crate::hooks::use_toast::{ToastInfo, ToastManager, ToastOptions};
use dioxus::prelude::*;

// Toast with future
// Toast with delayed function

// Default lifetime of a toasts (in ms)
const TOAST_LIFETIME: u64 = 4000;

// Default gap between toasts
const GAP: u32 = 14;

const SWIPE_TRESHOLD: u32 = 20;

const TIME_BEFORE_UNMOUNT: u32 = 200;

#[component]
pub fn Toaster() -> Element {
    let toasts = use_signal(Vec::new);
    let heights = use_signal(Vec::new);
    use_root_context(|| ToastManager { toasts, heights });

    let toasts = toasts
        .iter()
        .enumerate()
        .filter_map(|(_i, f)| toast_helper(&f));

    rsx!({ toasts })
}

fn toast_helper(
    ToastInfo {
        title,
        description,
        options,
        ..
    }: &ToastInfo,
) -> Element {
    let ToastOptions { position, .. } = options;

    let position = match position {
        crate::hooks::use_toast::ToastPosition::TopLeft => "top-0 left-0",
        crate::hooks::use_toast::ToastPosition::TopCenter => "top-0 left-0 right-0",
        crate::hooks::use_toast::ToastPosition::TopRight => "top-0 right-0",
        crate::hooks::use_toast::ToastPosition::BottomLeft => "bottom-0 left-0",
        crate::hooks::use_toast::ToastPosition::BottomCenter => "bottom-0 left-0 right-0",
        crate::hooks::use_toast::ToastPosition::BottomRight => "bottom-0 right-0",
    };

    rsx! {
        div { class: "fixed {position} m-4",
            div { class: "bg-white shadow-lg rounded-lg p-4",
                div { class: "flex items-center justify-between", span { class: "font-bold", "{title}" } }
                if let Some(desc) = &description {
                    span { class: "text-gray-500", {desc} }
                }
            }
        }
    }
}
