use dioxus::prelude::*;

use crate::{bg_color, border_color, merge_classes, text_color, TexoColor};

#[component]
pub fn Frame(
    href: Option<String>,
    #[props(default = TexoColor::Dark)] color: TexoColor,
    #[props(default = false)] rounded: bool,
    #[props(default = false)] border: bool,
    #[props(default = false)] shadow: bool,
    children: Element,
    onmounted: Option<EventHandler<MountedEvent>>,
    onclick: Option<EventHandler<MouseEvent>>,
    #[props(default = "".to_string())] class: String,
    role: Option<String>,
) -> Element {
    let classes = &[
        if rounded { "rounded-lg" } else { "" },
        if border { "border" } else { "" },
        if shadow { "shadow-md" } else { "" },
        border_color(color),
        text_color(color),
        bg_color(color),
        &class,
    ];

    let class = merge_classes(classes);
    let onclick = move |evt| {
        if let Some(oc) = &onclick {
            oc.call(evt)
        }
    };

    if let Some(href) = href {
        rsx! {
            a {
                href: "{href}",
                onmounted: move |evt| {
                    if let Some(onmount) = &onmounted {
                        onmount.call(evt.clone())
                    }
                },
                onclick: onclick,
                class: "{class}",
                {children}
            }
        }
    } else {
        rsx! {
            div {
                onmounted: move |evt| {
                    if let Some(onmount) = &onmounted {
                        onmount.call(evt.clone())
                    }
                },
                onclick: onclick,
                class: "{class}",
                {children}
            }
        }
    }
}