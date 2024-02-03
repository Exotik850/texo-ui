#![feature(async_closure)]
use dioxus::prelude::*;
use manganis::classes;
use texo_ui::elements::*;
use texo_ui::util::Card;
#[cfg(feature="clipboard")]
use texo_ui::hooks::use_clipboard;

pub fn main() {
  dioxus_logger::init(log::LevelFilter::Info).unwrap();
  #[cfg(feature="clipboard")]
  launch(App)
}

#[cfg(feature="clipboard")]
#[component]
fn App() -> Element {

    let mut clip = use_clipboard().unwrap();
    let val = clip.value();

    let get = use_callback(|| move |_| async move {
      clip.get().await;
    });

    rsx!(
        link {
            href: "https://unpkg.com/tailwindcss@^2/dist/tailwind.min.css",
            rel: "stylesheet"
        }

        Accordion {
          {(0..10).map(|i| rsx! {
            AccordionItem {
              header: rsx!("Hold on! {i}"),
              "I'm down here! {i}"
            }
          })}
          AccordionItem {
            header: rsx!("One more!"),
            Button {
              onclick: get,
              "Click me!"
            }
          }
        }


        span {
          "{val}"
        }
        // FileTreeView {
        //   path: "./"
        // }
    )
}
