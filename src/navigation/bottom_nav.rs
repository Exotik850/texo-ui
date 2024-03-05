use crate::TexoPosition;
use dioxus::prelude::*;
use std::fmt::Display;

#[derive(PartialEq, Clone, Copy, Default)]
pub enum BottomNavType {
    #[default]
    Default,
    Border,
    Application,
    Pagination,
    Group,
    Card,
    Meeting,
    Video,
}

impl BottomNavType {
    fn inner_class(self) -> &'static str {
        match self {
            BottomNavType::Meeting => "flex items-center justify-center mx-auto",
            BottomNavType::Video => "flex items-center w-full",
            _ => "",
        }
    }
    fn button_class(self) -> &'static str {
        match self {
      BottomNavType::Default => "inline-flex flex-col items-center justify-center px-5 hover:bg-gray-50 dark:hover:bg-gray-800 group",
      BottomNavType::Border => "inline-flex flex-col items-center justify-center px-5 border-gray-200 border-x hover:bg-gray-50 dark:hover:bg-gray-800 group dark:border-gray-600",
      BottomNavType::Pagination => "inline-flex flex-col items-center justify-center px-5 hover:bg-gray-50 dark:hover:bg-gray-800 group",
      BottomNavType::Group => "inline-flex flex-col items-center justify-center p-4 hover:bg-gray-50 dark:hover:bg-gray-800 group",
      BottomNavType::Card => "inline-flex flex-col items-center justify-center px-5 hover:bg-gray-50 dark:hover:bg-gray-800 group",
      _ => "",
    }
    }
    fn span_class(self) -> &'static str {
        match self {
        BottomNavType::Default | BottomNavType::Border => "text-sm text-gray-500 dark:text-gray-400 group-hover:text-primary-600 dark:group-hover:text-primary-500'",
        BottomNavType::Application | BottomNavType::Pagination | BottomNavType::Group => "sr-only",
        BottomNavType::Card => "text-sm text-gray-500 dark:text-gray-400 group-hover:text-primary-600 dark:group-hover:text-primary-500",
        _ => ""
    }
    }
}

impl Display for BottomNavType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            BottomNavType::Default => "bottom-0 start-0 h-16 bg-white border-t",
            BottomNavType::Border => "bottom-0 start-0 h-16 bg-white border-t",
            BottomNavType::Application => "h-16 max-w-lg -translate-x-1/2 rtl:translate-x-1/2 bg-white border rounded-full bottom-4 start-1/2",
            BottomNavType::Pagination => "bottom-0 h-16 -translate-x-1/2 rtl:translate-x-1/2 bg-white border-t start-1/2",
            BottomNavType::Group => "bottom-0 -translate-x-1/2 rtl:translate-x-1/2 bg-white border-t start-1/2",
            BottomNavType::Card => "bottom-0 start-0 h-16 bg-white border-t",
            BottomNavType::Meeting => "bottom-0 start-0 grid h-16 grid-cols-1 px-8 bg-white border-t md:grid-cols-3",
            BottomNavType::Video => "bottom-0 start-0 grid h-24 grid-cols-1 px-8 bg-white border-t md:grid-cols-3",
        })
    }
}

#[component]
pub fn BottomNav(
    #[props(default = TexoPosition::default())] position: TexoPosition,
    #[props(default = BottomNavType::default())] nav_type: BottomNavType,
    outer_class: Option<String>,
    inner_class: Option<String>,
    header: Option<Element>,
    #[props(extends=div)] rest_attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let otw = outer_class.unwrap_or_else(|| {
        "w-full z-50 border-gray-200 dark:bg-gray-700 dark:border-gray-600".to_string()
    });
    let itw = inner_class.unwrap_or_else(|| "flex h-full max-w-lg mx-auto".to_string());

    rsx! {
        div { ..rest_attributes, class: "{position} {nav_type} {otw}",
            if let Some(header) = header {
                {header}
            }
            div { class: "{nav_type.inner_class()} {itw}", {children} }
        }
    }
}

#[component]
pub fn BottomNavItem(
    button_name: Option<String>,
    // button_position
    button_class: Option<String>,
    span_class: Option<String>,
    href: Option<String>,
    #[props(default = true)] exact: bool,
    #[props(default = Default::default())] nav_type: BottomNavType,
    onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    let button_class = button_class.unwrap_or_else(|| nav_type.button_class().to_string());
    let span_class = span_class.unwrap_or_else(|| nav_type.span_class().to_string());

    rsx!(
        if let Some(href) = href {
            a {
                href: href,
                role: "link",
                class: "{button_class}",
                onclick: move |evt| {
                    if let Some(oc) = &onclick {
                        oc.call(evt)
                    }
                },
                {children},
                span {
                    class: "{span_class}",
                    aria_label: "{button_name.clone().unwrap_or_default()}",
                    "{button_name.clone().unwrap_or_default()}"
                }
            }
        } else {
            button {
                role: "button",
                class: "{nav_type.button_class()} {button_class} ",
                onclick: move |evt| {
                    if let Some(oc) = &onclick {
                        oc.call(evt)
                    }
                },
                {children},
                span {
                    class: "{nav_type.span_class()} {span_class}",
                    aria_label: "{button_name.clone().unwrap_or_default()}",
                    "{button_name.clone().unwrap_or_default()}"
                }
            }
        }
    )
}

#[component]
pub fn BottomNavHeader(
    #[props(default = "w-full".into())] outer_class: String,
    #[props(default = "grid max-w-xs grid-cols-3 gap-1 p-1 mx-auto my-2 bg-gray-100 rounded-lg dark:bg-gray-600".into())]
    inner_class: String,
    children: Element,
    #[props(extends=div)] rest_attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        div { ..rest_attributes, class: outer_class,
            div { class: inner_class, role: "group", {children} }
        }
    }
}

#[component]
pub fn BottomNavHeaderItem(
    #[props(default = "".into())] item_name: String,
    active: bool,
    #[props(default = "px-5 py-1.5 text-xs font-medium text-gray-900 hover:bg-gray-200 dark:text-white dark:hover:bg-gray-700 rounded-lg".into())]
    class: String,
    #[props(default = "px-5 py-1.5 text-xs font-medium text-white bg-gray-900 dark:bg-gray-300 dark:text-gray-900 rounded-lg".into())]
    active_class: String,
    onclick: Option<EventHandler<MouseEvent>>,
    #[props(extends=button)] rest_attributes: Vec<Attribute>,
) -> Element {
    rsx!(
        button {
            ..rest_attributes,
            onclick: move |evt| {
                if let Some(oc) = &onclick {
                    oc.call(evt)
                }
            },
            class: if active { "{active_class}" },
            class: if !active { "{class}" },
            "{item_name}"
        }
    )
}
