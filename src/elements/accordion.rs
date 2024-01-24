use dioxus::prelude::*;
use dioxus_signals::use_signal;

use crate::{merge_classes, util::Frame};

#[derive(Props)]
pub struct AccordionItemProps<'a> {
    button_class: Option<&'a str>,
    content_class: Option<&'a str>,
    header: Option<Element<'a>>,
    arrow_up: Option<Element<'a>>,
    arrow_down: Option<Element<'a>>,
    children: Element<'a>,
}

#[component]
pub fn AccordionItem<'a>(cx: Scope<'a, AccordionItemProps<'a>>) -> Element {
    let AccordionItemProps {
        button_class,
        content_class,
        header,
        children,
        arrow_down,
        arrow_up,
    } = &cx.props;

    let open = use_signal(cx, || false);

    let curr_open = *open.read();

    let arrow_up_svg = render! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 10 6",
            "aria-hidden": "true",
            fill: "none",
            class: "w-3 h-3 text-gray-800 dark:text-white",
            path {
                stroke: "currentColor",
                stroke_linejoin: "round",
                stroke_width: "2",
                stroke_linecap: "round",
                d: "M9 5 5 1 1 5"
            }
        }
    };

    let arrow_down_svg = render! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 10 6",
            "aria-hidden": "true",
            fill: "none",
            class: "w-3 h-3 text-gray-800 dark:text-white",
            path {
                stroke_linecap: "round",
                stroke_width: "2",
                d: "m1 1 4 4 4-4",
                stroke: "currentColor",
                stroke_linejoin: "round"
            }
        }
    };


    render! {
        h2 { class: "group",
            button {
                onclick: move |_| open.toggle(),
                class: "flex items-center justify-between w-full font-medium text-left group-first:rounded-t-xl border-gray-200 dark:border-gray-700 transition-all {button_class.unwrap_or(\"\")}",
                aria_expanded: "{open}",
                if let Some(header) = header {
                  header
                }
                if curr_open {
                  if let Some(arrow_up) = arrow_up {
                    arrow_up
                  } else {
                    &arrow_up_svg
                  }
                } else {
                  if let Some(arrow_down) = arrow_down {
                    arrow_down
                  } else {
                    &arrow_down_svg
                  }

                }
            }

            if curr_open {
              rsx! {
                div {
                  div {
                    class: "{content_class.unwrap_or_default()}",
                    children
                  }
                }
              }
            } else {
              rsx! {
                div {
                  class: "hidden",
                  div {
                    class: "{content_class.unwrap_or_default()}",
                    children,
                  }
                }
              }
            }
        }
    }
}

#[derive(Props)]
pub struct AccordionProps<'a> {
    frame_class: Option<&'a str>,
    class: Option<&'a str>,
    #[props(default = false)]
    multiple: bool,
    #[props(default = false)]
    flush: bool,
    children: Element<'a>,
}

#[component]
pub fn Accordion<'a>(cx: Scope<'a, AccordionProps<'a>>) -> Element {
    let AccordionProps {
        frame_class,
        class,
        children,
        ..
    } = &cx.props;

    let classes = &[
        frame_class.unwrap_or_default(),
        class.unwrap_or_default(),
        "text-gray-500 dark:text-gray-400 transition-all",
    ];

    let class = merge_classes(classes);

    render! {
        Frame { class: "{class}", children }
    }
}
