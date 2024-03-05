use dioxus::prelude::*;

#[cfg(feature = "desktop")]
use dioxus::desktop::{use_window, use_wry_event_handler, DesktopContext};
#[cfg(feature = "web")]
use web_sys::wasm_bindgen::{closure::Closure, JsCast};

#[derive(Clone)]
pub struct FullScreenManager {
    fullscreen: Signal<bool>,
    toggle: Signal<bool>,
    #[cfg(feature = "desktop")]
    window: DesktopContext,
    #[cfg(feature = "web")]
    document: Signal<web_sys::Document>,
    #[cfg(feature = "web")]
    body: Signal<web_sys::HtmlElement>,
    #[cfg(feature = "web")]
    _closure: Signal<Closure<dyn FnMut()>>,
}

impl FullScreenManager {
    #[cfg(feature = "desktop")]
    pub fn set(&mut self, val: bool) {
        self.window.set_maximized(val);
        self.fullscreen.set(val);
    }

    #[cfg(feature = "web")]
    pub fn set(&mut self, val: bool) {
        self.fullscreen.set(if val {
            self.body.read().request_fullscreen().is_ok()
        } else {
            self.document.read().exit_fullscreen();
            false
        });
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

    #[cfg(feature = "web")]
    {
        let mut document = use_signal(move || {
            web_sys::window()
                .expect("Need a window to get document from")
                .document()
                .expect("Need a document")
        });
        let _closure = use_signal(|| {
            let closure = Closure::new(move || {
                log::info!("Fullscreen changed");
                fullscreen.set(document.read().fullscreen());
            });
            document
                .write()
                .set_onfullscreenchange(Some(closure.as_ref().unchecked_ref()));
            closure
        });

        let body = use_signal(move || document.read().body().expect("Need a body"));

        use_effect(move || {
            if toggle() {
                fullscreen.set(if fullscreen() {
                    document.read().exit_fullscreen();
                    false
                } else {
                    body.read().request_fullscreen().is_ok()
                });
                toggle.set(false);
            }
        });

        FullScreenManager {
            fullscreen,
            toggle,
            document,
            body,
            _closure,
        }
    }

    #[cfg(not(any(feature = "desktop", feature = "web")))]
    {
        FullScreenManager { fullscreen, toggle }
    }
}
