
use dioxus::prelude::*;
use texo_ui::hooks::use_toast::toast;
use texo_ui::hooks::*;
use texo_ui::elements::*;

pub fn main() {
    dioxus_logger::init(log::LevelFilter::Info).unwrap();
    // wasm_logger::init(wasm_logger::Config::default());
    launch(App)
}

#[component]
fn App() -> Element {
    // let mut fullscreen = use_fullscreen();
    // let is_fullscreen = fullscreen.is_fullscreen();
    let mut open = use_signal(|| false);

    // let actions = use_signal(|| vec![
    //   CommandAction::new(|_| toast("HELP", None, Default::default()), "Hello".to_string(), "Say hello".to_string(), None, None),
    //   CommandAction::new(|_| log::info!("World"), "World".to_string(), "Say world".to_string(), None, Some("world".to_string())),
    //   CommandAction::new(|_| log::info!("eager"), "weigh".to_string(), "Say explore".to_string(), None, None),
    //   CommandAction::new(|_| log::info!("comfortable"), "atmosphere".to_string(), "Say forty".to_string(), None, None),
    //   CommandAction::new(|_| log::info!("his"), "these".to_string(), "Say syllable".to_string(), None, None),
    //   CommandAction::new(|_| log::info!("chief"), "rich".to_string(), "Say forward".to_string(), None, None),
    // ]);

    rsx!(
        link {
            href: "https://unpkg.com/tailwindcss@^2/dist/tailwind.min.css",
            rel: "stylesheet"
        }

        Toaster {}
 
        Button {
          onclick: move |_| {
            open.toggle();
            log::info!("Open Command Palette!");
          }, 
          "Open Command Palette!"
        }

        Popover {
          open, 
          content: rsx!(
            "Hello, World!"
          ),

          button {
            onclick: move |_| {
              open.toggle();
              log::info!("Open Command Palette!");
            },
            "Open Command Palette!"
          }
        }

        // CommandPalette {
        //   actions,
        //   placeholder: "Type a command...",
        //   visible: open
        // }
    )
}
