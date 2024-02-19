use dioxus::prelude::*;
use syntect::{highlighting::{Theme, ThemeSet}, parsing::SyntaxSet};

#[derive(Clone)]
struct SyntaxHighlightContext {
  ps: Signal<SyntaxSet>,
  ts: Signal<ThemeSet>,
}

fn use_syntax_highlighting() {
  let ps = use_signal(SyntaxSet::new);
  let ts = use_signal(|| ThemeSet::load_from_folder("/src/syntax_themes").expect("Should have folder of themes available"));
  
  use_root_context(|| SyntaxHighlightContext {ps, ts});
}

#[component]
pub fn CodeEditor(
    value: Signal<String>,
    onchange: Option<EventHandler<FormEvent>>,
    highlight: Option<EventHandler<String>>,
    #[props(default)] insert_spaces: bool,
    #[props(default)] ignore_tab: bool,
    class: Option<String>,
    text_area_class: Option<String>,
    pre_class: Option<String>,
    placeholder: Option<String>,
    #[props(default)] read_only: bool,
    #[props(default)] required: bool,
    onclick: Option<EventHandler<MouseEvent>>,
    onfocus: Option<EventHandler<FocusEvent>>,
    onkeyup: Option<EventHandler<KeyboardEvent>>,
    onkeydown: Option<EventHandler<KeyboardEvent>>,
) -> Element {

  use_syntax_highlighting();
  let SyntaxHighlightContext { ps, ts } = use_context();

    let re = ps.read();
    let syntax = re.find_syntax_by_extension("html")?;

    let inner_html = syntect::html::highlighted_html_for_string(&value.read(), &ps.read(), syntax, &ts.read().themes["base16-ocean.dark"]).expect("NOOOOOOOOOO");

    let oninput = move |evt: FormEvent| value.set(evt.value());

    let onclick = move |evt: MouseEvent| {
        if let Some(onclick) = &onclick {
            onclick.call(evt)
        }
    };
    let onfocus = move |evt: FocusEvent| {
        if let Some(onfocus) = &onfocus {
            onfocus.call(evt)
        }
    };
    let onkeyup = move |evt: KeyboardEvent| {
        if let Some(onkeyup) = &onkeyup {
            onkeyup.call(evt)
        }
    };
    let onkeydown = move |evt: KeyboardEvent| {
        if let Some(onkeydown) = &onkeydown {
            onkeydown.call(evt)
        }
    };

    let pre_val = value.read();
    let addon = if pre_val.ends_with("\n") {" "} else {""};

    rsx! {
        pre {
          class: "{pre_class.unwrap_or_default()} bg-transparent z-1 leading-relaxed 
          text-sm font-mono border-0 h-full w-full resize-none font-semibold cursor-none 
          fixed top-0 left-0 overflow-auto whitespace-nowrap",
          // aria_hidden: true,
          // children: highlighted
          dangerous_inner_html: inner_html
          // "{pre_val}{addon}"
        }

        textarea {
          class: "{class.unwrap_or_default()} bg-transparent z-0 leading-relaxed text-sm 
          font-mono border-0 h-full w-full resize-none overflow-auto truncate antialiased 
          fixed top-0 left-0 whitespace-nowrap",
          spellcheck: false,
          oninput,
          onclick,
          onfocus,
          onkeyup,
          onkeydown,
          onselect: move |evt| {
            log::info!("{evt:?}");
          },
        }
    }
}
