use dioxus::prelude::*;
use manganis::classes;

use crate::{
    bg_color, border_color, elements::ToolBarButton, merge_classes, text_color, TexoColor,
    TexoSize, TexoTrigger,
};

#[component]
pub fn Frame(
    href: Option<String>,
    #[props(default = TexoColor::Dark)] color: TexoColor,
    #[props(default = false)] rounded: bool,
    #[props(default = false)] border: bool,
    #[props(default = false)] shadow: bool,
    children: Element,
    onmounted: Option<EventHandler<MountedEvent>>,
    onclick: Option<EventHandler<MouseEvent>>,
    #[props(default = "".to_string())] class: String,
    role: Option<String>,
) -> Element {
    let classes = &[
        if rounded { "rounded-lg" } else { "" },
        if border { "border" } else { "" },
        if shadow { "shadow-md" } else { "" },
        border_color(color),
        text_color(color),
        bg_color(color),
        &class,
    ];

    let class = merge_classes(classes);
    let onclick = move |evt| {
        if let Some(oc) = &onclick {
            oc.call(evt)
        }
    };

    if let Some(href) = href {
        rsx! {
            a {
                href: "{href}",
                onmounted: move |evt| {
                    if let Some(onmount) = &onmounted {
                        onmount.call(evt.clone())
                    }
                },
                onclick,
                class: "{class}",
                {children}
            }
        }
    } else {
        rsx! {
            div {
                onmounted: move |evt| {
                    if let Some(onmount) = &onmounted {
                        onmount.call(evt.clone())
                    }
                },
                onclick,
                class: "{class}",
                {children}
            }
        }
    }
}

fn card_size(size: Option<TexoSize>) -> &'static str {
    match size {
        Some(TexoSize::ExtraSmall) => classes!("p-2"),
        Some(TexoSize::Small) => classes!("p-4"),
        Some(TexoSize::Medium) => classes!("p-4 sm:p-5"),
        Some(TexoSize::Large) => classes!("p-4 sm:p-6"),
        Some(TexoSize::ExtraLarge) => classes!("p-4 sm:p-8"),
        None => "",
    }
}
fn card_padding(padding: Option<TexoSize>) -> &'static str {
    match padding {
        Some(TexoSize::ExtraSmall) => classes!("max-w-xs"),
        Some(TexoSize::Small) => classes!("max-w-sm"),
        Some(TexoSize::Medium) => classes!("max-w-xl"),
        Some(TexoSize::Large) => classes!("max-w-2xl"),
        Some(TexoSize::ExtraLarge) => classes!("max-w-screen-xl"),
        None => "",
    }
}

#[component]
pub fn Card(
    #[props(default = TexoColor::Gray)] color: TexoColor,
    #[props(default = false)] horizontal: bool,
    #[props(default = false)] reverse: bool,
    #[props(default = false)] rounded: bool,
    #[props(default = false)] shadow: bool,
    #[props(default = false)] border: bool,
    img: Option<String>,
    href: Option<String>,
    #[props(default = Default::default())] class: String,
    onclick: Option<EventHandler<MouseEvent>>,
    #[props(!optional, default = Some(TexoSize::Large))] padding: Option<TexoSize>,
    #[props(!optional, default = Some(TexoSize::Small))] size: Option<TexoSize>,
    children: Element,
) -> Element {
    let padding = card_padding(padding);
    let card_class = merge_classes(&[
        classes!("flex w-full"),
        card_size(size),
        &class,
        if reverse {
            classes!("flex-col-reverse")
        } else {
            classes!("flex-col")
        },
        if horizontal {
            if reverse {
                "md:flex-row-reverse"
            } else {
                "md:flex-row"
            }
        } else {
            ""
        },
        if (&href).is_some() {
            "hover:bg-gray-100 dark:hover:bg-gray-700"
        } else {
            ""
        },
        if !img.as_ref().is_some() { padding } else { "" },
    ]);
    let img_class = merge_classes(&[
        if reverse {
            "rounded-b-lg"
        } else {
            "rounded-t-lg"
        },
        if horizontal {
            "object-cover w-full h-96 md:h-auto md:w-48 md:rounded-none"
        } else {
            ""
        },
        if horizontal {
            if reverse {
                "md:rounded-e-lg"
            } else {
                "md:rounded-s-lg"
            }
        } else {
            ""
        },
    ]);

    rsx!(
      Frame {
        href,
        color,
        rounded,
        shadow,
        border,
        class: card_class,
        onclick: move |evt| if let Some(oc) = &onclick {oc.call(evt)},
        if let Some(img) = img {
          img {
            src: img,
            alt: "",
            class: img_class
          }
          div {
            class: "{padding}",
            {children}
          }
        } else {
          div {
            class: padding,
            {children}
          }
        }
      }
    )
}

#[component]
pub fn CloseButton(
    #[props(default=Default::default())] class: String,
    #[props(default=TexoColor::Red)] color: TexoColor,
    onclick: Option<EventHandler<MouseEvent>>,
) -> Element {
    let cbtw = classes!("ms-auto");

    rsx!(
      ToolBarButton {
        onclick: move |evt| if let Some(onclick) = &onclick {onclick.call(evt)},
        class: "{cbtw} {class}",
        color,
        svg {
          class: classes!("w-5 h-5"),
          fill: "currentColor",
          view_box: "0 0 20 20",
          xmlns: "http://www.w3.org/2000/svg",
          path {
            fill_rule: "evenodd",
            clip_rule: "evenodd",
            d: "M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z",
          }
        }
      }
    )
}

#[derive(Copy, Clone, Debug)]
pub struct TransitionClose(Signal<bool>);

#[component]
pub fn TransitionFrame(
    open: Signal<bool>,
    #[props(default = false)] dismissable: bool,
    children: Element,
) -> Element {
    use_context_provider(|| TransitionClose(open));

    let close_frame = |evt: MouseEvent| {
        evt.stop_propagation();
        open.toggle()
    };

    rsx!(
      if dismissable {
        if open() {
          div {
            Frame {
              {children}
            }
          }
        }
      } else {
        Frame {
          {children}
        }
      }
    )
}

#[component]
pub fn Popper(
    #[props(default = false)] active: bool,
    #[props(default = true)] arrow: bool,
    #[props(default = 8)] offset: u8,
    #[props(default = TexoTrigger::Hover)] trigger: TexoTrigger,
    #[props(default = "absolute".into())] strategy: String,
    #[props(default = false)] open: bool,
    #[props(default = false)] y_only: bool,
    children: Element,
) -> Element {
    let blocked = use_signal(|| false);

    let clickable = trigger == TexoTrigger::Click;
    let mut arrow_el = use_signal(|| (0.0, 0.0));

    todo!();

    rsx!(
      if open {
        Frame {
          {children}
          if arrow {
            div {
              // Arrow stuff
            }
          }
        }
      }
    )
}
