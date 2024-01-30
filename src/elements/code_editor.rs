use dioxus::prelude::*;

#[component]
pub fn CodeEditor(
    value: Signal<String>,
    onchange: Option<EventHandler<FormEvent>>,
    highlight: Option<EventHandler<String>>,
    #[props(default = false)] insert_spaces: bool,
    #[props(default = false)] ignore_tab: bool,
    class: Option<String>,
    text_area_class: Option<String>,
    pre_class: Option<String>,
    placeholder: Option<String>,
    #[props(default = false)] read_only: bool,
    #[props(default = false)] required: bool,
    onclick: Option<EventHandler<MouseEvent>>,
    onfocus: Option<EventHandler<FocusEvent>>,
    onkeyup: Option<EventHandler<KeyboardEvent>>,
    onkeydown: Option<EventHandler<KeyboardEvent>>,
) -> Element {
    let capture = use_signal(|| false);
    let selection_start = use_signal(|| 0);
    let selection_end = use_signal(|| 0);

    let selection = use_signal(|| (0, 0));
    // let mut js = eval(include_str!("./code_editor.js")).unwrap();

    // use_future(move || async move {
    //   while let Ok(msg) = js.recv().await {
    //     log::info!("{msg:?}")
    //   }
    // });

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

    rsx! {
      div {
        pre {
          class: "{pre_class.unwrap_or_default()} font-light cursor-none relative",
          aria_hidden: true,
          // children: highlighted
          "{value.read()}"
        }

        textarea {
          class: "{class.unwrap_or_default()} absolute top-0 left-0 h-full w-full resize-none truncate antialiased",
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
}
