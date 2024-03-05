use dioxus::prelude::*;

use crate::merge_classes;

#[derive(Clone, PartialEq)]
pub struct PaginationInfo {
    pub active: bool,
    pub name: String,
    pub href: Option<String>,
    pub onclick: Option<EventHandler<MouseEvent>>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct PaginationContext {
    table: bool,
}

#[component]
pub fn Pagination(
    pages: Vec<PaginationInfo>,
    #[props(default = "text-blue-600 border border-gray-300 bg-blue-50 hover:bg-blue-100 hover:text-blue-700 dark:border-gray-700 dark:bg-gray-700 dark:text-white".into(), into)]
    active_class: String,
    #[props(default = "text-gray-500 bg-white hover:bg-gray-100 hover:text-gray-700 dark:bg-gray-800 dark:border-gray-700 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-white".into(), into)]
    normal_class: String,
    #[props(default = "inline-flex -space-x-px rtl:space-x-reverse items-center".into(), into)]
    ul_class: String,
    #[props(default)] table: bool,
    #[props(default)] large: bool,
    #[props(default = "Page navigation".into(), into)] aria_label: String,
    #[props(default, into)] class: String,
    previous: Option<Element>,
    next: Option<Element>,
    onprevious: Option<EventHandler<MouseEvent>>,
    onnext: Option<EventHandler<MouseEvent>>,
) -> Element {
    use_context_provider(|| PaginationContext { table });

    let ul_classes = merge_classes(&[
        if table {
            "divide-x rtl:divide-x-reverse dark divide-gray-700 dark:divide-gray-700"
        } else {
            ""
        },
        &ul_class,
        &class,
    ]);

    rsx! {
        nav { aria_label: aria_label,
            ul { class: ul_classes,
                li {
                    PaginationItem {
                        large,
                        class: if table { "rounded-l" } else { "rounded-s-lg" },
                        normal_class: normal_class.clone(),
                        onclick: move |evt| {
                            if let Some(prev) = &onprevious {
                                prev.call(evt)
                            }
                        },
                        {previous.unwrap_or_else(|| rsx!("Previous"))}
                    }
                }

                for PaginationInfo { active , name , href , onclick } in pages.into_iter() {
                    li {
                        PaginationItem {
                            large,
                            active,
                            active_class: active_class.clone(),
                            normal_class: normal_class.clone(),
                            href,
                            onclick: move |evt| {
                                if let Some(oc) = &onclick {
                                    oc.call(evt);
                                }
                            },
                            "{name}"
                        }
                    }
                }

                li {
                    PaginationItem {
                        large,
                        normal_class,
                        class: if table { "rounded-r" } else { "rounded-e-lg" },
                        onclick: move |evt| {
                            if let Some(prev) = &onnext {
                                prev.call(evt)
                            }
                        },
                        {next.unwrap_or_else(|| rsx!("Next"))}
                    }
                }
            }
        }
    }
}

#[component]
pub fn PaginationItem(
    href: Option<String>,
    #[props(default)] active: bool,
    #[props(default = "text-blue-600 border border-gray-300 bg-blue-50 hover:bg-blue-100 hover:text-blue-700 dark:border-gray-700 dark:bg-gray-700 dark:text-white".into(), into)]
    active_class: String,
    #[props(default = "text-gray-500 bg-white hover:bg-gray-100 hover:text-gray-700 dark:bg-gray-800 dark:border-gray-700 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-white".into(), into)]
    normal_class: String,
    #[props(default)] large: bool,
    #[props(default)] group: bool,
    #[props(default, into)] class: String,
    onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    let PaginationContext { table } = use_context();

    let class = merge_classes(&[
        "flex items-center font-medium",
        if large {
            "h-10 px-4 text-base"
        } else {
            "h-8 px-3 text-sm"
        },
        if group {
            ""
        } else if table {
            "rounded"
        } else {
            "rounded-lg"
        },
        if table { "" } else { "border" },
        if active { &active_class } else { &normal_class },
        &class,
    ]);

    let onclick  = move |evt| if let Some(oc) = &onclick {
      oc.call(evt)
    };

    rsx!(
        if let Some(href) = href {
            a { href: href, class: class, onclick: onclick, {children} }
        } else {
            button { class: class, onclick: onclick, {children} }
        }
    )
}
