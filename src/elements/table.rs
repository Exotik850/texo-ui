use dioxus::prelude::*;

use crate::{border_color, merge_classes, text_color, TexoColor};


#[component]
pub fn Table(
  #[props(default = false)]
  shadow: bool,
  #[props(default = false)]
  striped: bool,
  #[props(default = false)]
  noborder: bool,
  #[props(default = false)]
  hoverable: bool,
  #[props(default = TexoColor::Gray)]
  color: TexoColor,
) -> Element {
  rsx! {
    div {
      table {

      }
    }
  }
}


#[component]
pub fn TableHead(
  class: Option<String>,
  #[props(default = true)]
  default_row: bool,
  #[props(default = TexoColor::Gray)]
  color: TexoColor,
  children: Element,
) -> Element {

  let classes = &[
    &class.unwrap_or_default(),
    text_color(color),
    border_color(color),
    "text-xs uppercase"
  ];

  let final_class = merge_classes(classes);

  rsx! {
    thead {
      class: "{final_class}",
      if default_row {
        tr {
          {children}
        }
      } else {
        {children}
      }
    }
  }
}

#[component]
pub fn TableBody(class: Option<String>, children: Element) -> Element {
    rsx!(
      tbody {
        class: "{class.unwrap_or_default()}",
        {children}
      }
    )
}



#[component]
pub fn TableBodyCell(
  class: Option<String>,
  onclick: Option<EventHandler<MouseEvent>>,
  #[props(default = TexoColor::Gray)]
  color: TexoColor,
  children: Element,
  #[props(extends=GlobalAttributes)]
  rest_attributes: Vec<Attribute>
) -> Element {

  let is_button = onclick.is_some();

  let classes = &[
    class.unwrap_or_default(),
    "px-6 py-4 whitespace-nowrap font-medium".into(),
    if let TexoColor::Gray = color {
      "text-gray-900 dark:text-white".into()
    } else {
      "text-blue-50 whitespace-nowrap dark:text-blue-100".into()
    },
  ];

  let final_class = merge_classes(classes);

  rsx! {
    if is_button {
      button {
        ..rest_attributes,
        class: "{final_class}",
        onclick: move |evt| onclick.as_ref().unwrap().call(evt.clone()),
        {children}
      }
    } else {
      td {
        ..rest_attributes,
        class: "{final_class}",
        {children}
      }
    }
  }
}


// #[component]
// pub fn TableBody<'a>(cx: Scope, class: Option<&'a str>, children: Element<'a>) -> Element {
//     render!(
//       tbody {
//         class: "{class.unwrap_or_default()}",
//         {children}
//       }
//     )
// }