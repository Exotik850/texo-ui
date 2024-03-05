use dioxus::prelude::*;

use crate::{input::Button, merge_classes, TexoTrigger};

#[component]
pub fn SpeedDial(
    class: Option<String>,
    name: Option<String>,
    popper_class: Option<String>,
    #[props(default = TexoTrigger::Hover)] trigger: TexoTrigger,
    #[props(default = false)] text_outside: bool,
    // #[props(default = false)]
    // gradient: bool,
    #[props(default = false)] pill: bool,
    #[props(default = false)] open: bool,
    #[props(default = false)] outline: bool,
    #[props(default = false)] shadow: bool,
    children: Option<Element>,
) -> Element {
    let final_class = merge_classes(&[
        "group fixed end-6 bottom-6".into(),
        class.unwrap_or_default(),
    ]);

    rsx! {
        div { class: "{final_class} group",
            Button { pill, outline, shadow, class: "!p-3",
                if let Some(children) = children {
                    {children}
                } else {
                    svg {
                        class: "w-8 h-8 transition-transform group-hover:rotate-45",
                        fill: "none",
                        stroke: "currentColor",
                        view_box: "0 0 24 24",
                        xmlns: "http://www.w3.org/2000/svg",
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            stroke_width: "2",
                            d: "M12 6v6m0 0v6m0-6h6m-6 0H6"
                        }
                    }
                }
                if let Some(name) = name {
                    span { class: "sr-only", "{name}" }
                }
            }
        }
    }
}
