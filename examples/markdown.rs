use dioxus::prelude::*;
use texo_ui::data_display::{Markdown, Latex};

fn main() {
  pretty_env_logger::init();
  launch(App)
}

#[component]
fn App() -> Element {

  let mut value = use_signal(String::new);

  rsx!(

    input {
      value,
      oninput: move |e| value.set(e.value()),
    }

    Markdown {
      value: "# Hello, world!

          This is a test of the markdown component.

          $\\LaTeX$ is also supported.
      "
    }
    Latex {
      value,
      display: la_texer::DisplayStyle::Block,
    }
  )
}