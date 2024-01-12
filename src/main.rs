use dioxus::prelude::*;
use texo_ui::elements::{TexoSize, Button};

pub fn main() {
  dioxus_logger::init(log::LevelFilter::Info).unwrap();
  dioxus_web::launch(App);
}

#[component]
fn App(cx: Scope) -> Element {
  render!(
    Button {
      size: TexoSize::Medium,
      onclick: move |_| log::info!("Clicked!"), 
      "Help!"
    }
  )
}