use dioxus::prelude::*;
use texo_ui::elements::*;
use texo_ui::util::Card;

pub fn main() {
    dioxus_logger::init(log::LevelFilter::Info).unwrap();
    launch(App)
}

#[component]
fn App() -> Element {

    let mut open = use_signal(|| true);

    rsx!(
        link {
            href: "https://unpkg.com/tailwindcss@^2/dist/tailwind.min.css",
            rel: "stylesheet"
        }

        // NavBar { 
        //     NavBrand { 
        //         span { class: classes!("self-center whitespace-nowrap text-xl font-semibold dark:text-white"),
        //             "Dioxus"
        //         }
        //     }

        //     NavHamburger {}

        //     NavUl { 
        //         NavLi { onclick: move |_| open.toggle(), "Home (Click me!)" }
        //         NavLi { "Products" }
        //         NavLi { "Contact" }
        //         if open() {
        //           NavLi {
        //             "SECRET SAUCE"
        //           }
        //         }
        //     }
        // }

        TreeView {
          TreeViewItem {
            lead: rsx! {
              "Item 1"
            },
            TreeViewItem {
              lead: rsx! {
                "Child 1" 
              },
              TreeViewItem {
                "Child of Child 1"
              }
              TreeViewItem {
                "Child of Child 2"
              }
            }
            TreeViewItem {
              lead: rsx! {
                "Child 2 (No Children)"
              }
            }
        } 
        TreeViewItem {
          lead: rsx! {
            "Item 2"
          },
          Card {
            "Help!"
          }
        }
      }
    )
}
