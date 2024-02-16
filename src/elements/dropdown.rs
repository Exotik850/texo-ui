use dioxus::prelude::*;

use crate::TexoTrigger;

#[component]
pub fn Dropdown(
    #[props(default)] open: bool,
    lead: Element,
    children: Element,
    #[props(default)] class: String,
    #[props(default)] child_class: String,
    #[props(default)] trigger: TexoTrigger,
    #[props(extends=button)] rest_attributes: Vec<Attribute>,
) -> Element {
    let mut open = use_signal(|| open);
    rsx!(
      button {
        ..rest_attributes,
        class: "dropdown {class}",

        onmouseover: move |_| if TexoTrigger::Hover == trigger {open.set(true)},
        onmouseleave: move |_| if TexoTrigger::Hover == trigger {open.set(false)},
        onclick: move |_| if TexoTrigger::Click == trigger {open.toggle()},

        {lead}

        if open() {
          div { class: "{child_class}", style: "position: absolute;
          z-index: 1;", {children} }
        }
      }
    )
}
