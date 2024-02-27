mod use_slider;
#[cfg(feature="tween")]
mod use_tween;
#[cfg(feature="clipboard")]
mod use_clipboard;
mod use_fullscreen;
pub use use_fullscreen::*;
// #[cfg(feature="controller")]
// mod use_controller;

/// This module contains the implementation of a toast notification system.
/// 
/// The `ToastType` enum represents the different types of toast notifications that can be displayed.
/// 
/// The `ToastPosition` enum represents the different positions where toast notifications can be displayed.
/// 
/// The `ToastInfo` struct represents the information of a toast notification, including its title, description, height, and options.
/// 
/// The `ToastOptions` struct represents the options for a toast notification, including its type, icon, duration, action, and position.
/// 
/// The `ToastManager` struct represents the manager for toast notifications, including the list of toasts and their heights.
/// 
/// The `toast` function is used to display a toast notification with the specified title, description, and options.
pub mod use_toast;

mod use_timeout;
pub use use_timeout::*;


#[cfg(feature="desktop")] 
pub use use_fullscreen::*;
// #[cfg(feature="controller")] 
// pub use use_controller::*;
#[cfg(feature="clipboard")]
pub use use_clipboard::*;
pub use use_slider::*;
#[cfg(feature="tween")]
pub use use_tween::*;
