use crate::{merge_classes, util::Frame};
use dioxus::prelude::*;

#[component]
pub fn AccordionItem(
    button_class: Option<String>,
    content_class: Option<String>,
    header: Option<Element>,
    arrow_up: Option<Element>,
    arrow_down: Option<Element>,
    children: Element,
) -> Element {
    let mut open = use_signal(|| false);

    let curr_open = *open.read();

    let upside_down = if curr_open { "rotate-180" } else { "" };

    let arrow_up_svg = rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 10 6",
            "aria-hidden": "true",
            fill: "none",
            class: "w-3 h-3 text-gray-800 dark:text-white transition-transform transform {upside_down}",
            path {
                stroke: "currentColor",
                stroke_linejoin: "round",
                stroke_width: "2",
                stroke_linecap: "round",
                d: "M9 5 5 1 1 5"
            }
        }
    };

    let classes = &[
        if curr_open {
            "max-h-full opacity-100"
        } else {
            "max-h-0 opacity-0 overflow-hidden duration-250"
        },
        &content_class.unwrap_or_default(),
    ];

    let content = merge_classes(classes);

    let btn_class = format!("{} {}", "flex items-center justify-between w-full font-medium text-left group-first:rounded-t-xl border-gray-200 dark:border-gray-700", button_class.unwrap_or_default());

    rsx! {
        h2 { class: "group",
            button {
                onclick: move |_| open.toggle(),
                class: "{btn_class}",
                aria_expanded: "{open}",
                if let Some(header) = header {
                    {header}
                }

                if curr_open {
                    if let Some(arrow_up) = arrow_up {
                        {arrow_up}
                    } else {
                        {&arrow_up_svg}
                    }
                } else {
                    if let Some(arrow_down) = arrow_down {
                        {arrow_down}
                    } else {
                        {&arrow_up_svg}
                    }
                }
            }

            if curr_open {
                div { class: "w-full",
                    div { class: "{content}", {children} }
                }
            }
        }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct AccordionProps {
    frame_class: Option<String>,
    class: Option<String>,
    #[props(default = false)]
    multiple: bool,
    #[props(default = false)]
    flush: bool,
    children: Element,
}

pub fn Accordion(props: AccordionProps) -> Element {
    let AccordionProps {
        frame_class,
        class,
        children,
        ..
    } = props;

    let classes = &[
        &frame_class.unwrap_or_default(),
        &class.unwrap_or_default(),
        &"text-gray-500 dark:text-gray-400 transition-all".into(),
    ];

    let class = merge_classes(classes);

    rsx! {
        Frame { class: "{class}", {children} }
    }
}
