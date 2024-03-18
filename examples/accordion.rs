use dioxus::prelude::*;
use texo_ui::data_display::{Markdown, MarkdownType};
use texo_ui::hooks::use_toast::toast;
use texo_ui::hooks::*;
use texo_ui::input::CommandAction;

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
    let tm = use_timeout(
        |_| {
            toast(
                "Hello World?",
                rsx!( div { "Hello." } ),
                Default::default(),
            )
        },
        || (),
    );

    let actions = use_signal(|| {
        vec![
            CommandAction::new(
                |_| toast("HELP", None, Default::default()),
                "Hello".to_string(),
                "Say hello".to_string(),
                None,
                None,
            ),
            CommandAction::new(
                |_| println!("World"),
                "World".to_string(),
                "Say world".to_string(),
                None,
                None,
            ),
        ]
    });

    let mut value = use_signal(|| "# Hello, world!".to_string());

    rsx!(
        // link {
        //     href: "https://unpkg.com/tailwindcss@^2/dist/tailwind.min.css",
        //     rel: "stylesheet"
        // }

        textarea { oninput: move |e| value.set(e.value()), value: value }

        Markdown { value, md_type: MarkdownType::Github }
    )
}
