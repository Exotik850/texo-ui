use dioxus::prelude::*;
use manganis::classes;
use texo_ui::elements::*;
use texo_ui::util::Card;

pub fn main() {
    dioxus_logger::init(log::LevelFilter::Info).unwrap();
    launch(App)
}

#[component]
fn App() -> Element {

    let mut open = use_signal(|| true);
    let value = use_signal(String::new);

    rsx!(
        link {
            href: "https://unpkg.com/tailwindcss@^2/dist/tailwind.min.css",
            rel: "stylesheet"
        }

        Table {
          striped: true,
          TableHead {
            TableHeadCell {
              "Product Name"
            }
            TableHeadCell {
              "Color"
            }
            TableHeadCell {
              "Category"
            }
          }
          TableBody {
            TableBodyRow {
              TableBodyCell {
              "Apple Something"
              }
              TableBodyCell {
                "Shit brown"
              }
              TableBodyCell {
                "Way too fucking expensive"
              }
            }
            TableBodyRow {
              TableBodyCell {
                "Apple Something 2"
              }
              TableBodyCell {
                "Shittier brown"
              }
              TableBodyCell {
                "Way too fucking expensive"
              }
            }
          }

        }
        // FileTreeView {
        //   path: "./"
        // }
    )
}
