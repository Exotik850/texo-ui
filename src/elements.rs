use std::collections::HashSet;

use dioxus::prelude::*;
use dioxus_signals::use_signal;

use crate::TexoSize;

#[derive(Props)]
pub struct ButtonProps<'a> {
    #[props(default = false)]
    pill: bool,
    #[props(default = false)]
    outline: bool,
    #[props(default = false)]
    shadow: bool,
    #[props(default = "button")]
    tag: &'a str,
    size: TexoSize,
    href: Option<&'a str>,
    checked: Option<bool>,
    onclick: Option<EventHandler<'a, MouseEvent>>,
    children: Element<'a>,
}

fn merge_classes(input: &[&str]) -> String {
    let mut class_set = HashSet::new();
    for thing in input {
        for class in thing.split_whitespace() {
            class_set.insert(class);
        }
    }

    let mut merged = String::new();
    for class in class_set.iter() {
        merged.push_str(&class);
        merged.push(' ');
    }
    merged.truncate(merged.len() - 1);
    merged
}

#[component]
pub fn Button<'a>(cx: &'a Scoped<'a, ButtonProps<'a>>) -> Element<'a> {
    let &ButtonProps {
        pill,
        outline,
        shadow,
        size,
        href,
        onclick,
        children,
        ..
    } = &cx.props;

    let classes = &[
      "text-center font-medium",
      if *pill { 
        "rounded-full"
      } else {
        ""
      },
      if *outline {
        "border border-gray-500 text-gray-500"
      } else {
        "bg-gray-500 text-white" 
      },
      match size {
        TexoSize::ExtraSmall => "text-xs py-1.5 px-3",
        TexoSize::Small => "text-sm py-2 px-4",
        TexoSize::Medium => "text-base py-2.5 px-6", 
        TexoSize::Large => "text-lg py-3 px-8",
        TexoSize::ExtraLarge => "text-xl py-4 px-10",
      },
      if *shadow {
        "shadow-md"
      } else {
        ""
      },
    ];

    let class = merge_classes(classes);

    cx.render(rsx!(
      button {
        class: "{class}",
        "href": if let Some(href) = href { href } else { "" },
        onclick: move |evt| if let Some(onclick) = onclick {
          onclick.call(evt)
        },
        children
      }
    ))
}
