use dioxus::prelude::*;
use texo_ui::hooks::*;
use texo_ui::elements::*;

pub fn main() {
    dioxus_logger::init(log::LevelFilter::Info).unwrap();
    launch(App)
}

#[component]
fn App() -> Element {
    let mut fullscreen = use_fullscreen();

    rsx!(
        link {
            href: "https://unpkg.com/tailwindcss@^2/dist/tailwind.min.css",
            rel: "stylesheet"
        }

        Button {
          onclick: move |_| fullscreen.toggle(),
          "Click me!"
        }
    )
}
