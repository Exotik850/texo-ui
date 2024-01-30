use dioxus::prelude::*;

use crate::{merge_classes, TexoSize};

#[component]
pub fn Button(
    #[props(default = false)] pill: bool,
    #[props(default = false)] outline: bool,
    #[props(default = false)] shadow: bool,
    #[props(default = "button".to_string())] tag: String,
    #[props(default = TexoSize::Medium)] size: TexoSize,
    href: Option<String>,
    class: Option<String>,
    checked: Option<bool>,
    onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    let class = merge_classes(&[
        &class.unwrap_or_default(),
        &"text-center font-medium".to_string(),
        &if pill { "rounded-full" } else { "" }.to_string(),
        &if outline {
            "border border-gray-500 text-gray-500"
        } else {
            "bg-gray-500 text-white"
        }
        .to_string(),
        &match size {
            TexoSize::ExtraSmall => "text-xs py-1.5 px-3",
            TexoSize::Small => "text-sm py-2 px-4",
            TexoSize::Medium => "text-base py-2.5 px-6",
            TexoSize::Large => "text-lg py-3 px-8",
            TexoSize::ExtraLarge => "text-xl py-4 px-10",
        }
        .to_string(),
        &if shadow { "shadow-md" } else { "" }.to_string(),
    ]);

    rsx!(
        button {
            class: "{class}",
            "href": if let Some(href) = href { href } else { "".into() },
            onclick: move |evt| {
                if let Some(onclick) = &onclick {
                    onclick.call(evt)
                }
            },
            { children }
        }
    )
}
