use dioxus::prelude::*;
mod popover;
mod spinner;
mod toast;
mod toolbar;
mod frame;
mod card;

pub use frame::Frame;
pub use card::Card;
pub use popover::Popover;
pub use spinner::Spinner;
pub use toast::Toaster;
pub use toolbar::{Toolbar, ToolBarButton};

use crate::TexoColor;

#[component]
pub fn CloseButton(
    #[props(default)] class: String,
    #[props(default=TexoColor::Red)] color: TexoColor,
    onclick: Option<EventHandler<MouseEvent>>,
) -> Element {
    let cbtw = "ms-auto";

    rsx!(
        ToolBarButton {
            onclick: move |evt| {
                if let Some(onclick) = &onclick {
                    onclick.call(evt)
                }
            },
            class: "{cbtw} {class}",
            color,
            svg {
                class: "w-5 h-5",
                fill: "currentColor",
                view_box: "0 0 20 20",
                xmlns: "http://www.w3.org/2000/svg",
                path {
                    fill_rule: "evenodd",
                    clip_rule: "evenodd",
                    d: "M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z"
                }
            }
        }
    )
}
