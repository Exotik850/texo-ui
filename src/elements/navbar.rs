use dioxus::prelude::*;

use crate::{merge_classes, util::Frame, TexoColor};

#[derive(Props, PartialEq)]
pub struct MenuProps<'a> {
  #[props(default = 24)]
  size: u8,
  #[props(default = TexoColor::Primary)]
  color: TexoColor,
  #[props(default = MenuVariation::Solid)]
  variation: MenuVariation,
  #[props(default = "bars 3")]
  aria_label: &'a str,
  class: Option<&'a str>
}

#[derive(PartialEq)]
pub enum MenuVariation {
  Solid,
  Outline,
}

#[component]
pub fn Menu<'a>(cx: Scope<'a, MenuProps<'a>>) -> Element {

  let MenuProps { 
    aria_label, 
    size,
    class,
    variation,
    color, .. } = &cx.props;

  render! {
    svg {
        "aria_label": "{aria_label}",
        fill: "none",
        stroke_width: "2",
        xmlns: "http://www.w3.org/2000/svg",
        role: "button",
        tabindex: 0,
        width: "{size}",
        height: "{size}",
        view_box: "0 0 24 24",
        if let Some(class) = class {
            "{class}"
        }
        match variation {
            MenuVariation::Solid => rsx!{
            path {
                stroke: "{color}",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                d: "M4 6h16M4 12h16M4 18h16"
            }
            },
            MenuVariation::Outline => rsx!{
            path {
                fill: "{color}",
                clip_rule: "evenodd",
                fill_rule: "evenodd",
                d: "M3 5a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zM3 10a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zM3 15a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1z"
            }
            }
        }
    }
}
}

#[derive(Props)]
pub struct NavHamburgerProps<'a> {
  onclick: Option<EventHandler<'a, MouseEvent>>,
  children: Element<'a>,
}

#[component]
pub fn NavHamburger<'a>(cx: Scope<'a, NavHamburgerProps<'a>>) -> Element {
  render! {
    button { onclick: move |evt| {
            if let Some(onclick) = &cx.props.onclick {
                onclick.call(evt.clone())
            }
        },
        &cx.props.children
    }
}
}

#[component]
pub fn NavContainer<'a>(cx: Scope, fluid: Option<bool>, class: &'a str, children: Element<'a>) -> Element {
  let fluid = fluid.unwrap_or(false);

  let classes = &[
    "mx-auto flex flex-wrap justify-between items-center",
    if fluid {
      "w-full"
    } else {
      "container"
    },
    class
  ];

  let class = merge_classes(classes);

  render!{
    div { class: "{class}", children }
}
}

#[component]
pub fn NavBar<'a>(cx: Scope, fluid: Option<bool>, class: Option<&'a str>, children: Element<'a>) -> Element {
    let fluid = fluid.unwrap_or(false);

    let classes = &["px-2 sm:px-4 py-2.5 w-full", class.unwrap_or("")];
    let class = merge_classes(classes);

    render!{
        Frame { class: "{class}",
            NavContainer { fluid: fluid, class: "{class}", children }
        }
    }
}