use dioxus::prelude::*;
use manganis::classes;

use crate::util::{CloseButton, Frame};
use crate::{TexoColor, TexoSize};

#[component]
pub fn Modal(
    open: Signal<bool>,
    #[props(default=Default::default())] title: String,
    #[props(default=TexoSize::Medium)] size: TexoSize,
    #[props(default=TexoColor::Gray)] color: TexoColor,
    #[props[default=false]] autoclose: bool,
    #[props[default=true]] dismissable: bool,
    #[props[default=false]] outside_close: bool,
    #[props[default=false]] rounded: bool,
    #[props[default=false]] shadow: bool,
    backdrop_class: Option<String>,
    body_class: Option<String>,
    dialog_class: Option<String>,
    header: Option<Element>,
    footer: Option<Element>,
    onkeydown: Option<EventHandler<KeyboardEvent>>,
    children: Element,
) -> Element {
    let backdrop = backdrop_class.unwrap_or_else(|| {
        classes!("fixed inset-0 z-40 bg-gray-900 bg-opacity-50 dark:bg-opacity-80").into()
    });
    let class = body_class.unwrap_or_else(|| classes!("relative flex flex-col mx-auto p-6 space-y-6 flex-1 overflow-y-auto overscroll-contain").into());
    let dialog_class = dialog_class.unwrap_or_else(|| {
        classes!("fixed top-0 start-0 end-0 h-modal md:inset-0 md:h-full z-50 w-full p-4 flex")
            .into()
    });
    let handle_key = move |evt: Event<KeyboardData>| if let Key::Escape = evt.data.key() {
      evt.stop_propagation();
      open.set(false)
    };

    if !open() {
      return None;
    }

    rsx!(
          div { class: backdrop }
          div {
              onkeydown: handle_key,
              onclick: move |_| open.toggle(),
              role: "dialog",
              aria_modal: true,
              tabindex: -1,
              class: dialog_class,

              div { class: classes!("flex relative w-full max-h-full max-w-2xl"),
                Frame {
                  rounded: true,
                  shadow: true,
                  class: classes!("w-full divide-y relative flex flex-col mx-auto")

                  if !title.is_empty() || (&header).is_some() {
                      Frame { 
                        color,
                        class: classes!("flex justify-between items-center p-4 rounded-t-lg"),
                          if let Some(header) = &header {
                              {header}
                          } else {
                              h3 { class: classes!("text-xl font-semibold text-gray-900 dark:text-white p-0"), "{title}" }
                          }
                          if dismissable {
                              CloseButton { onclick: move |_| open.set(false), color }
                          }
                      }
                  }

                  div { onkeydown: handle_key, role: "document", class,
                      if dismissable && title.is_empty() && header.is_none() {
                          CloseButton {
                              class: classes!("absolute top-3 end-2.5"),
                              onclick: move |_| open.set(false),
                              color
                          }
                      }
                      {children}
                  }
                  if let Some(footer) = footer {
                      Frame {
                          color,
                          class: classes!("flex items-center p-6 space-x-2 rtl:space-x-reverse rounded-b-lg"),
                          {footer}
                      }
                  }
                }
              }
          }
    )
}
