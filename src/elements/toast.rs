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
        .filter_map(|(i, f)| (i < 3).then(|| toast_helper(&f)));

    rsx!({ toasts })
}

fn toast_helper(ToastInfo { options, .. }: &ToastInfo) -> Element {
    let ToastOptions { .. } = options;

    rsx! {
        
    }
}
