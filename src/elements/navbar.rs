use dioxus::prelude::*;

use crate::{elements::ToolBarButton, merge_classes, util::Frame, TexoColor};

#[derive(PartialEq, Clone)]
pub enum MenuVariation {
    Solid,
    Outline,
}

#[component]
pub fn Menu(
    #[props(default = 24)] size: u8,
    #[props(default = TexoColor::Primary)] color: TexoColor,
    #[props(default = MenuVariation::Solid)] variation: MenuVariation,
    #[props(default = "bars 3".into())] aria_label: String,
    class: Option<String>,
) -> Element {
    rsx! {
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

#[component]
pub fn NavBrand(
  class: Option<String>, 
  href: Option<String>, 
  onclick: Option<EventHandler<MouseEvent>>,
  children: Element
) -> Element {
    let nv_cl = classes!("flex items-center");

    rsx!(
        a {
            class: "{nv_cl} {class.unwrap_or_default()}",
            onclick: move |evt| {
                if let Some(onclick) = &onclick {
                    onclick.call(evt)
                }
            },
            href: if let Some(href) = href { href },
            {children}
        }
    )
}

#[component]
pub fn NavLi(
    href: Option<String>,
    class: Option<String>,
    onclick: Option<EventHandler<MouseEvent>>,
    #[props(extends=li)] rest_attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let tw_cl = classes!("block py-2 pe-4 ps-3 md:p-0 rounded md:border-0");

    rsx!(
        li {
            ..rest_attributes,
            class: "{class.unwrap_or_default()} {tw_cl}",
            if let Some(href) = href {
                a {
                    href: href,
                    onclick: move |evt| {
                        if let Some(onclick) = &onclick {
                            onclick.call(evt)
                        }
                    },
                    {children}
                }
            } else {
                button {
                    onclick: move |evt| {
                        if let Some(onclick) = &onclick {
                            onclick.call(evt)
                        }
                    },
                    {children}
                }
            }
        }
    )
}

use manganis::classes;

#[component]
pub fn NavUl(
    div_class: Option<String>,
    ul_class: Option<String>,
    children: Element,
    #[props(extends=ul)] rest_attributes: Vec<Attribute>,
) -> Element {
    let dtwc = classes!("w-full md:block md:w-auto");
    let ulc = classes!("flex flex-col p-4 mt-4 md:flex-row md:space-x-8 rtl:space-x-reverse md:mt-0 md:text-sm md:font-medium");

    rsx!(
        div { class: "{dtwc} {div_class.unwrap_or_default()}",
            ul {
                ..rest_attributes,
                class: "{ulc} {ul_class.unwrap_or_default()}",
                {children}
            }
        }
    )
}

#[component]
pub fn NavHamburger(
    button_class: Option<String>,
    menu_class: Option<String>,
    onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    let dtwc = classes!("ms-3 md:hidden");
    let ulc = classes!("h-6 w-6 shrink-0");

    rsx! {
        ToolBarButton {
            class: "{dtwc} {button_class.unwrap_or_default()}",
            onclick: move |evt: MouseEvent| {
                if let Some(onclick_s) = &onclick {
                    onclick_s.call(evt.clone())
                }
            },
            Menu { class: "{ulc} {menu_class.unwrap_or_default()}" }
        }
    }
}

#[component]
pub fn NavContainer(fluid: Option<bool>, class: String, children: Element) -> Element {
    let fluid = fluid.unwrap_or(false);

    let class = merge_classes(&[
        classes!("mx-auto flex flex-wrap justify-between items-center"),
        if fluid { classes!("w-full") } else { classes!("container") },
        &class,
    ]);

    rsx! {
        div { class: "{class}", {children} }
    }
}

#[component]
pub fn NavBar(
    fluid: Option<bool>,
    class: Option<String>,
    frame_class: Option<String>,
    children: Element,
) -> Element {
    let fluid = fluid.unwrap_or(false);
    let class = merge_classes(&[classes!("px-2 sm:px-4 py-2.5 w-full"), &class.unwrap_or("".into())]);
    rsx! {
        Frame { class: "{class}",
            NavContainer { fluid: fluid, class: "{class}", {children} }
        }
    }
}
