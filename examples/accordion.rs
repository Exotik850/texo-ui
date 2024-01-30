use dioxus::prelude::*;
use manganis::classes;
use texo_ui::elements::*;
use texo_ui::hooks::use_fullscreen;
use texo_ui::util::Card;

pub fn main() {
    dioxus_logger::init(log::LevelFilter::Info).unwrap();
    launch(App)
}

#[component]
fn App() -> Element {
    let mut fullscreen = use_fullscreen();
    let fs = if fullscreen.value() { "" } else { " Not" };

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
              onclick: move |_| fullscreen.toggle(),
              "Click me!"
            }
          }
        }


        span {
          "You are{fs} Fullscreen!"
        }
        // FileTreeView {
        //   path: "./"
        // }
    )
}
