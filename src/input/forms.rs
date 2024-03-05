use dioxus::prelude::*;

use crate::{border_color, merge_classes, text_color, TexoColor, TexoInputType, TexoSize};

#[component]
pub fn Checkbox(
    #[props(default = TexoColor::Gray)] color: TexoColor,
    #[props(default = false)] custom: bool,
    #[props(default = false)] inline: bool,
    #[props(default = "".into())] class: String,
    checked: Signal<bool>,
    #[props(extends=input)] input_attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let class = merge_classes(&[
        if inline { "inline-flex" } else { "flex" }.to_string(),
        class,
    ]);

    rsx!(
        Label { class: class.clone(), input { ..input_attributes, r#type: "checkbox", class: class, checked: checked } }
    )
}

#[component]
pub fn Label(
    #[props(default = TexoColor::Gray)] color: TexoColor,
    #[props(default = "text-sm rtl:text-right font-medium block items-center".into())]
    class: String,
    #[props(default = true)] show: bool,
    #[props(extends=label)] rest_attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let class = merge_classes(&[class, text_color(color).to_string()]);

    rsx! {
        if show {
            label { ..rest_attributes, class: class, {children} }
        } else {
            {children}
        }
    }
}

#[component]
pub fn Helper(
    #[props(default = "text-xs font-normal text-gray-500 dark:text-gray-300".into())] class: String,
    color: TexoColor,
    children: Element,
    #[props(extends=p)] rest_attributes: Vec<Attribute>,
) -> Element {
    let class = merge_classes(&[text_color(color), &class]);

    rsx! { p { ..rest_attributes, class: class } }
}

#[component]
pub fn Input(
    #[props(default=Default::default())] typ: TexoInputType,
    #[props(default = false)] group: bool,
    value: Option<Signal<String>>,
    #[props(default = "block w-full disabled:cursor-not-allowed disabled:opacity-50 rtl:text-right".into())]
    class: String,
    #[props(default=TexoColor::Gray)] color: TexoColor,
    #[props(default=TexoSize::Medium)] size: TexoSize,
    #[props(default = "flex absolute inset-y-0 items-center text-gray-500 dark:text-gray-400".into())]
    float_class: String,
    #[props(extends=input)] rest_attributes: Vec<Attribute>,
    left: Option<Element>,
    right: Option<Element>,
    children: Option<Element>,
) -> Element {
    let class = merge_classes(&[
        border_color(color),
        &class,
        if group {
            "first:rounded-s-lg last:rounded-e-lg border-s-0 first:border-s last:border-e"
        } else {
            "rounded-lg"
        },
    ]);

    rsx!(
        if let Some(left) = left {
            div { class: float_class.clone(), {left} }
        }

        if let Some(children) = children {
            {children}
        } else {
            input {
                ..rest_attributes,
                oninput: move |evt| {
                    if let Some(value) = value.as_mut() {
                        value.set(evt.data.value())
                    }
                },
                class: class,
                r#type: "{typ}"
            }
        }

        if let Some(right) = right {
            div { class: float_class, {right} }
        }
    )
}
