use dioxus::prelude::*;

#[cfg(feature="desktop")]
use dioxus::desktop::{use_window, use_wry_event_handler, DesktopContext};
#[cfg(feature="web")]
use web_sys::wasm_bindgen::{closure::Closure, JsCast};

#[derive(Clone)]
pub struct FullScreenManager {
  fullscreen: Signal<bool>,
  toggle: Signal<bool>,
  #[cfg(feature = "desktop")]
  window: DesktopContext,
  #[cfg(feature = "web")]
  document: Signal<(web_sys::Document, web_sys::HtmlElement)>,
}

impl FullScreenManager {
  #[cfg(feature = "desktop")]
  pub fn set(&mut self, val: bool) {
    self.fullscreen.set(val);
    self.window.set_maximized(val);
  }

  #[cfg(feature = "web")]
  pub fn set(&mut self, val: bool) {
    let doc = self.document.read();

    let open = if val {
      let result = doc.1.request_fullscreen();
      result.is_ok()
    } else {
      doc.0.exit_fullscreen();
      false
    };
    self.fullscreen.set(open);
  }

  pub fn toggle(&mut self) {
    self.toggle.set(true);
  }

  pub fn is_fullscreen(&self) -> bool {
    *self.fullscreen.read()
  }
}

pub fn use_fullscreen() -> FullScreenManager {
    let mut fullscreen = use_signal(|| false);
    let mut toggle = use_signal(|| false);

    #[cfg(feature = "desktop")]
    {
        let window = use_window();
        let window_c = window.clone();
        use_wry_event_handler(move |evt, window| match evt {
            dioxus::desktop::tao::event::Event::WindowEvent {
                window_id, event, ..
            } if *window_id == window_c.id() => match event {
                dioxus::desktop::WindowEvent::Resized(_) if window_c.is_maximized() => {
                    fullscreen.set(true);
                }
                dioxus::desktop::WindowEvent::Resized(_) => {
                    fullscreen.set(false);
                }
                _ => (),
            },
            _ => (),
        });
        let window_c = window.clone();
        use_effect(move || {
            if toggle() {
                let not_fullscreen = !fullscreen();
                window_c.set_maximized(not_fullscreen);
                fullscreen.set(not_fullscreen);
                toggle.set(false);
            }
        });

        FullScreenManager {
            fullscreen,
            toggle,
            window,
        }
    }

    #[cfg(feature="web")]
    {
      let document = use_signal(|| {
        let document = web_sys::window().expect("Need a window to get document from").document().expect("Need a document");
        let element = document.body().expect("Need a document body to request fullscreen on");
        (document, element)
      });

      let closure = Closure::new(move || {
        fullscreen.set(document.read().0.fullscreen());
      });
      document.read().0.set_onfullscreenchange(Some(closure.as_ref().unchecked_ref()));

      use_effect(move || {
        if toggle() {
          let full = if fullscreen() {
            document.read().0.exit_fullscreen();
            false
          } else {
            document.read().1.request_fullscreen().is_ok()
          };
          fullscreen.set(full);
          toggle.set(false);
        }
      });

      FullScreenManager {
          fullscreen,
          toggle,
          document,
      }
    }

    #[cfg(not(any(feature = "desktop", feature = "web")))]
    {
      FullScreenManager {
          fullscreen,
          toggle,
      }
    }
}
