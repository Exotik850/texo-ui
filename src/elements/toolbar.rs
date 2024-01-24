use dioxus::core_macro::component;
use dioxus::prelude::*;

use crate::{TexoColor, TexoSize};

#[derive(Props)]
pub struct ToolBarButtonProps<'a> {
  color: TexoColor,
  name: Option<&'a str>,
  aria_label: Option<&'a str>,
  size: TexoSize,
  href: Option<&'a str>
}

#[component]
pub fn ToolBarButton<'a>(cx: Scope<'a, ToolBarButtonProps<'a>>) -> Element {
  let ToolBarButtonProps { color, name, aria_label, size, href } = &cx.props;

  if let Some(href) = href {
    render!{
        a { href: "{href}",
            if let Some(name) = name {
            "{name}" 
            }
        }
    }
  } else {
    render!{button {
    }
    }
  }
}