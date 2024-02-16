use dioxus::prelude::*;
use manganis::classes;
use texo_ui::elements::*;
use texo_ui::TexoSize;
use texo_ui::util::Card;

pub fn main() {
  dioxus_logger::init(log::LevelFilter::Info).unwrap();
  launch(App)
}

#[component]
fn App() -> Element {

    let pages = vec![
      PaginationInfo { active: false, name: "1".into(), href: None, onclick: None },
      PaginationInfo { active: false, name: "Middle".into(), href: None, onclick: None },
      PaginationInfo { active: false, name: "3".into(), href: None, onclick: None },
    ];

    let mut open = use_signal(|| false);

    rsx!(
        link {
            href: "https://unpkg.com/tailwindcss@^2/dist/tailwind.min.css",
            rel: "stylesheet"
        }

        Button {
          onclick: move |_| open.toggle(),
          "Click me!"
        }

        Modal {
          open,
          size: TexoSize::ExtraSmall,
          title: "Something dumb",
          QrCode {
            data: "https://unpkg.com/tailwindcss@^2/dist/tailwind.min.css",
            width: "100px",
            height: "100px",
          }
        }

        // Pagination {
        //   pages,
        //   table: true,
        //   onprevious: move |_| println!("UGH"),
        // }

        // Breadcrumb {
        //   solid: true,
        //   BreadcrumbItem {
        //     home: true,
        //     "Home"
        //   }
        //   BreadcrumbItem {
        //     "Projects"
        //   }
        //   BreadcrumbItem {
        //     "TexoUI"
        //   }
        // }

        // Accordion {
        //   {(0..10).map(|i| rsx! {
        //     AccordionItem {
        //       header: rsx!("Hold on! {i}"),
        //       "I'm down here! {i}"
        //     }
        //   })}
        //   AccordionItem {
        //     header: rsx!("One more!"),
        //     Button {
        //       onclick: move |_| clip.get(),
        //       "Click me!"
        //     }
        //   }
        // }


        // span {
        //   "{val}"
        // }
        // FileTreeView {
        //   path: "./"
        // }
    )
}
