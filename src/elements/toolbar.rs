use dioxus::prelude::*;
use manganis::classes;

use crate::{merge_classes, util::Frame, TexoColor, TexoSize};

#[derive(Clone)]
struct ToolbarSeperation(bool);

#[component]
pub fn Toolbar(
  #[props(default = false)]
  embedded: bool,
  div_class: Option<String>,
  seperator_class: Option<String>,
  color: TexoColor,
  children: Element,
  end: Option<Element>,
) -> Element {

  let seperator = use_context_provider(|| ToolbarSeperation(false));

  let final_div_class = merge_classes(&[
    div_class.unwrap_or_default(),
    "flex justify-between items-center".into(),
    if embedded {
      ""
    } else {
      "py-2 px-3"
    }.into()
  ]);

  let sep_classes = merge_classes(&[
    if seperator.0 {
      "sm:divide-x rtl:divide-x-reverse"
    } else {
      ""
    }.into(),
    seperator_class.unwrap_or_default()
  ]);

  rsx! (
    Frame {
      class: "{final_div_class}",
      color,
      rounded: !embedded,
      Frame { 
        class: "{sep_classes} flex flex-wrap items-center"
        {children}
      }
      {end}
    }
  )

}

fn toolbar_color(color: TexoColor) -> &'static str {
  match color {
    TexoColor::Dark => classes!("text-gray-500 hover:text-gray-900 hover:bg-gray-200 dark:text-gray-400 dark:hover:text-white dark:hover:bg-gray-600"),
    TexoColor::Default => classes!("text-gray-500 focus:ring-gray-400 hover:bg-gray-200 dark:hover:bg-gray-800 dark:hover:text-gray-300"),
    TexoColor::Gray => classes!("text-gray-500 focus:ring-gray-400 hover:bg-gray-200 dark:hover:bg-gray-800 dark:hover:text-gray-300"),
    TexoColor::Red => classes!("text-red-500 focus:ring-red-400 hover:bg-red-200 dark:hover:bg-red-800 dark:hover:text-red-300"),
    TexoColor::Yellow => classes!("text-yellow-500 focus:ring-yellow-400 hover:bg-yellow-200 dark:hover:bg-yellow-800 dark:hover:text-yellow-300"),
    TexoColor::Green => classes!("text-green-500 focus:ring-green-400 hover:bg-green-200 dark:hover:bg-green-800 dark:hover:text-green-300"),
    TexoColor::Purple => classes!("text-purple-500 focus:ring-purple-400 hover:bg-purple-200 dark:hover:bg-purple-800 dark:hover:text-purple-300"),
    TexoColor::Blue => classes!("text-blue-500 focus:ring-blue-400 hover:bg-blue-200 dark:hover:bg-blue-800 dark:hover:text-blue-300"),
    TexoColor::Primary => classes!("text-primary-500 focus:ring-primary-400 hover:bg-primary-200 dark:hover:bg-primary-800 dark:hover:text-primary-300"),
    TexoColor::Alternative => todo!(),
    TexoColor::Light => todo!(),
    TexoColor::None => todo!(),
}
}

#[component]
pub fn ToolBarButton(
  head: Option<String>,
  href: Option<String>,
  #[props(default = false)]
  background: bool,
  #[props(default = Default::default())]
  class: String,
  aria_label: Option<String>,
  #[props(default = TexoColor::Gray)]
  color: TexoColor,
  #[props(default = TexoSize::Medium)]
  size: TexoSize,
  #[props(extends=GlobalAttributes)]
  rest_attributes: Vec<Attribute>,
  onclick: Option<EventHandler<MouseEvent>>,
  children: Element,
) -> Element {
  let button_class = merge_classes(&[
    classes!("focus:outline-none whitespace-normal m-0.5 rounded focus:ring-1 p-0.5"),
    &class,
    toolbar_color(color),
    if background {
      classes!("hover:bg-gray-100 dark:hover:bg-gray-600")
    } else {
      classes!("hover:bg-gray-100 dark:hover:bg-gray-700")
    }
  ]);

  if let Some(href) = href {
    rsx!{
      a { 
        ..rest_attributes,
        class: button_class,
        href,
        onclick: move |evt| if let Some(onclick) = &onclick {onclick.call(evt)},
        if let Some(name) = head {
          "{name}" 
        }
        {children}
      }
    }
  } else {
    rsx!{
      button {
        ..rest_attributes,
        class: button_class,
        if let Some(name) = head {
          "{name}"
        }
        {children}
      }
    }
  }
}