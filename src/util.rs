use dioxus::prelude::*;

use crate::{bg_color, border_color, merge_classes, text_color, TexoColor};

#[derive(Props)]
pub struct FrameProps<'a> {
    href: Option<&'a str>,
    #[props(default = TexoColor::Dark)]
    color: TexoColor,
    #[props(default = false)]
    rounded: bool,
    #[props(default = false)]
    border: bool,
    #[props(default = false)]
    shadow: bool,
    children: Element<'a>,
    onmounted: Option<EventHandler<'a, MountedEvent>>,
    #[props(default = "")]
    class: &'a str,
    role: Option<&'a str>,
}

#[component]
pub fn Frame<'a>(cx: Scope<'a, FrameProps<'a>>) -> Element {
    let FrameProps {
        href,
        color,
        rounded,
        border,
        shadow,
        children,
        onmounted,
        class,
        role,
    } = &cx.props;
    let classes = &[
        if *rounded { "rounded-lg" } else { "" },
        if *border { "border" } else { "" },
        if *shadow { "shadow-md" } else { "" },
        border_color(*color),
        text_color(*color),
        bg_color(*color),
        class
    ];

    let class = merge_classes(classes);

    render! {
        div {
            onmounted: move |evt| {
                if let Some(onmount) = onmounted {
                    onmount.call(evt.clone())
                }
            },
            class: "{class}",
            children
        }
    }
}
