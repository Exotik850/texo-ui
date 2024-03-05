use dioxus::prelude::*;

use crate::{merge_classes, utility::Frame, TexoColor, TexoSize};

fn card_size(size: Option<TexoSize>) -> &'static str {
  match size {
      Some(TexoSize::ExtraSmall) => "p-2",
      Some(TexoSize::Small) => "p-4",
      Some(TexoSize::Medium) => "p-4 sm:p-5",
      Some(TexoSize::Large) => "p-4 sm:p-6",
      Some(TexoSize::ExtraLarge) => "p-4 sm:p-8",
      None => "",
  }
}
fn card_padding(padding: Option<TexoSize>) -> &'static str {
  match padding {
      Some(TexoSize::ExtraSmall) => "max-w-xs",
      Some(TexoSize::Small) => "max-w-sm",
      Some(TexoSize::Medium) => "max-w-xl",
      Some(TexoSize::Large) => "max-w-2xl",
      Some(TexoSize::ExtraLarge) => "max-w-screen-xl",
      None => "",
  }
}

#[component]
pub fn Card(
  #[props(default = TexoColor::Gray)] color: TexoColor,
  #[props(default)] horizontal: bool,
  #[props(default)] reverse: bool,
  #[props(default)] rounded: bool,
  #[props(default)] shadow: bool,
  #[props(default)] border: bool,
  img: Option<String>,
  href: Option<String>,
  #[props(default)] class: String,
  onclick: Option<EventHandler<MouseEvent>>,
  #[props(!optional, default = Some(TexoSize::Large))] padding: Option<TexoSize>,
  #[props(!optional, default = Some(TexoSize::Small))] size: Option<TexoSize>,
  children: Element,
) -> Element {
  let padding = card_padding(padding);
  let card_class = merge_classes(&[
      "flex w-full",
      card_size(size),
      &class,
      if reverse {
          "flex-col-reverse"
      } else {
          "flex-col"
      },
      if horizontal {
          if reverse {
              "md:flex-row-reverse"
          } else {
              "md:flex-row"
          }
      } else {
          ""
      },
      if (&href).is_some() {
          "hover:bg-gray-100 dark:hover:bg-gray-700"
      } else {
          ""
      },
      if !img.as_ref().is_some() { padding } else { "" },
  ]);
  let img_class = merge_classes(&[
      if reverse {
          "rounded-b-lg"
      } else {
          "rounded-t-lg"
      },
      if horizontal {
          "object-cover w-full h-96 md:h-auto md:w-48 md:rounded-none"
      } else {
          ""
      },
      if horizontal {
          if reverse {
              "md:rounded-e-lg"
          } else {
              "md:rounded-s-lg"
          }
      } else {
          ""
      },
  ]);

  rsx!(
      Frame {
          href,
          color,
          rounded,
          shadow,
          border,
          class: card_class,
          onclick: move |evt| {
              if let Some(oc) = &onclick {
                  oc.call(evt)
              }
          },
          if let Some(img) = img {
              img { src: img, alt: "", class: img_class }
              div { class: "{padding}", {children} }
          } else {
              div { class: padding, {children} }
          }
      }
  )
}