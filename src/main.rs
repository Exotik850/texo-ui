use dioxus::prelude::*;
use texo_ui::{elements::*, TexoSize};

pub fn main() {
  dioxus_logger::init(log::LevelFilter::Info).unwrap();
  dioxus_web::launch(App);
}

#[component]
fn App(cx: Scope) -> Element {
  render!(
    link {
        href: "https://unpkg.com/tailwindcss@^2/dist/tailwind.min.css",
        rel: "stylesheet"
    }

    div {
      class: "w-full",
      div {
        class: "w-1/4 mx-auto",
        Accordion {
          AccordionItem {
            header: render! {
              span { "WHATT" }
            },
            "Hello"
          }
          AccordionItem {
            "Hello"
          }
        }
      }
    }

    
)
}