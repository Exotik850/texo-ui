use dioxus::{desktop::{use_window, use_wry_event_handler, DesktopContext}, prelude::*};

#[cfg(feature="desktop")]
#[derive(Clone)]
pub struct FullScreenManager {
  fullscreen: Signal<bool>,
  toggle: Signal<bool>,
  window: DesktopContext
}

#[cfg(feature="desktop")]
impl FullScreenManager {
  pub fn set(&mut self, val: bool) {
    self.fullscreen.set(val);
    self.window.set_maximized(val);
  }

  pub fn toggle(&mut self) {
    self.toggle.set(true);
  }

  pub fn value(&self) -> bool {
    *self.fullscreen.read()
  }
}

#[cfg(feature="desktop")]
pub fn use_fullscreen() -> FullScreenManager {
  let mut fullscreen = use_signal(|| false);
  let mut toggle = use_signal(|| false);
  let window = use_window();

  let window_c = window.clone();

  use_wry_event_handler(move |evt, window| {
    match evt {
        dioxus::desktop::tao::event::Event::WindowEvent { window_id, event, .. } if *window_id == window_c.id() => {
          match event {
            dioxus::desktop::WindowEvent::Resized(_) if window_c.is_maximized() => {
              fullscreen.set(true);
            },
            dioxus::desktop::WindowEvent::Resized(_) => {
              fullscreen.set(false);
            },
            _ => (),
        }
        },
        _ => (),
    }
  });

  let window_c = window.clone();

  use_effect(move || if toggle() {
    if fullscreen() {
      window_c.set_maximized(false);
      fullscreen.set(false);
    } else {
      window_c.set_maximized(true);
      fullscreen.set(true);
    }
    toggle.set(false);
  });

  FullScreenManager {
    fullscreen, toggle, window
  }
}