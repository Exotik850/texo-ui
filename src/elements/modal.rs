use dioxus::prelude::*;

use crate::util::{CloseButton, Frame};
use crate::{merge_classes, TexoColor, TexoSize};

fn modal_size(size: TexoSize) -> &'static str {
  match size {
    TexoSize::ExtraSmall => "max-w-md",
    TexoSize::Small => "max-w-lg",
    TexoSize::Medium => "max-w-2xl",
    TexoSize::Large => "max-w-4xl",
    TexoSize::ExtraLarge => "max-w-7xl",
  }
}

#[derive(PartialEq, Copy, Clone, Default)]
pub enum ModalPlacementType {
    TopLeft,
    TopCenter,
    TopRight,
    CenterLeft,
    #[default]
    Center,
    CenterRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
    None,
}


fn placement_class(placement: ModalPlacementType) -> &'static str {
  match placement {
    ModalPlacementType::TopLeft => "justify-start items-start",
    ModalPlacementType::TopCenter => "justify-center items-start",
    ModalPlacementType::TopRight => "justify-end items-start",
    ModalPlacementType::CenterLeft => "justify-start items-center",
    ModalPlacementType::Center => "justify-center items-center",
    ModalPlacementType::CenterRight => "justify-end items-center",
    ModalPlacementType::BottomLeft => "justify-start items-end",
    ModalPlacementType::BottomCenter => "justify-center items-end",
    ModalPlacementType::BottomRight => "justify-end items-end",
    ModalPlacementType::None => "justift-center items-center",
  }
}

#[component]
pub fn Modal(
    open: Signal<bool>,
    #[props(default=Default::default())] title: String,
    #[props(default=TexoSize::Medium)] size: TexoSize,
    #[props(default=TexoColor::Gray)] color: TexoColor,
    #[props(default)] placement: ModalPlacementType,
    #[props(default=true)] dismissable: bool,
    #[props(default)] autoclose: bool,
    #[props(default)] outside_close: bool,
    #[props(default)] rounded: bool,
    #[props(default)] shadow: bool,
    #[props(default = "fixed inset-0 z-40 bg-gray-900 bg-opacity-50 dark:bg-opacity-80".into(), into)]
    backdrop_class: String,
    #[props(default = "p-6 space-y-6 flex-1 overflow-y-auto overscroll-contain".into(), into)]
    body_class: String,
    #[props(default = "fixed top-0 start-0 end-0 h-modal md:inset-0 md:h-full z-50 w-full p-4 flex".into(), into)]
    dialog_class: String,
    header: Option<Element>,
    footer: Option<Element>,
    onkeydown: Option<EventHandler<KeyboardEvent>>,
    children: Element,
) -> Element {
  if !open() {
    return None;
  }

  let handle_key = move |evt: Event<KeyboardData>| {
      if !dismissable {
        return;
      }

      if let Key::Escape = evt.data.key() {
          evt.stop_propagation();
          open.set(false)
      }
  };

  let dialog_class = merge_classes(&[placement_class(placement), &dialog_class]);

    rsx!(
          div { class: backdrop_class }
          div {
              onkeydown: handle_key,
              onclick: move |evt| open.toggle(),
              role: "dialog",
              aria_modal: true,
              tabindex: -1,
              class: dialog_class,

              div { class: "flex relative w-full max-h-full {modal_size(size)}",
                Frame {
                  rounded: true,
                  shadow: true,
                  class: "w-full divide-y relative flex flex-col mx-auto"

                  if !title.is_empty() || (&header).is_some() {
                      Frame {
                        color,
                        class: "flex justify-between items-center p-4 rounded-t-lg",
                          if let Some(header) = &header {
                              {header}
                          } else {
                              h3 { class: "text-xl font-semibold text-gray-900 dark:text-white p-0", "{title}" }
                          }
                          if dismissable {
                              CloseButton { onclick: move |_| open.set(false), color }
                          }
                      }
                  }

                  div { onkeydown: handle_key, role: "document", class: body_class,
                      if dismissable && title.is_empty() && header.is_none() {
                          CloseButton {
                              class: "absolute top-3 end-2.5",
                              onclick: move |_| open.set(false),
                              color
                          }
                      }
                      {children}
                  }
                  if let Some(footer) = footer {
                      Frame {
                          color,
                          class: "flex items-center p-6 space-x-2 rtl:space-x-reverse rounded-b-lg",
                          {footer}
                      }
                  }
                }
              }
          }
    )
}
