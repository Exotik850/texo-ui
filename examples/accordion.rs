
use dioxus::prelude::*;
use texo_ui::hooks::use_toast::toast;
use texo_ui::hooks::*;
use texo_ui::elements::*;

pub fn main() {
    dioxus_logger::init(log::LevelFilter::Info).unwrap();
    launch(App)
}

#[component]
fn App() -> Element {
    let mut fullscreen = use_fullscreen();
    let is_fullscreen = fullscreen.is_fullscreen();
    let mut open = use_signal(|| false);
    let tm = use_timeout(|_| log::info!("RAH"), || ());

    let actions = use_signal(|| vec![
      CommandAction::new(|_| toast("HELP", None, Default::default()), "Hello".to_string(), "Say hello".to_string(), None, None),
      CommandAction::new(|_| println!("World"), "World".to_string(), "Say world".to_string(), None, None),
    ]);

    rsx!(
        link {
            href: "https://unpkg.com/tailwindcss@^2/dist/tailwind.min.css",
            rel: "stylesheet"
        }

        Toaster {}

        Button {
          onclick: move |_| tm.start(1000),
          "Click me!"
        }

        Button {
          onclick: move |_| open.toggle(),
          "Open Command Palette!"
        }

        span {
          if is_fullscreen {
            "I'm fullscreen!"
          } else {
            "I'm not fullscreen!"
          }
        }

        CommandPalette {
          actions,
          placeholder: "Type a command...",
          visible: open
        }
    )
}
