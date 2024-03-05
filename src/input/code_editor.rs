use dioxus::prelude::*;

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
    both_class: Option<String>,
    placeholder: Option<String>,
    #[props(default)] read_only: bool,
    #[props(default)] required: bool,
    #[props(default = true)] default_class: bool,
    #[props(default = true)] default_pre_class: bool,
    onclick: Option<EventHandler<MouseEvent>>,
    onfocus: Option<EventHandler<FocusEvent>>,
    onkeyup: Option<EventHandler<KeyboardEvent>>,
    onkeypress: Option<EventHandler<KeyboardEvent>>,
    onkeydown: Option<EventHandler<KeyboardEvent>>,
) -> Element {
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
    let onkeypress = move |evt: KeyboardEvent| {
        if let Some(onkeypress) = &onkeypress {
            onkeypress.call(evt)
        }
    };

    let pre_val = value.read();
    let addon = if pre_val.ends_with("\n") { " " } else { "" };

    let default_pre_class = if default_pre_class {
        "bg-transparent z-1 leading-relaxed 
        text-sm font-mono border-0 h-full w-full resize-none font-semibold cursor-none 
        absolute top-0 left-0 overflow-auto whitespace-pre-wrap"
    } else {
        ""
    };

    let default_class = if default_class {
        "bg-transparent z-0 leading-relaxed text-sm 
      font-mono border-0 h-full w-full resize-none overflow-auto truncate antialiased 
      absolute top-0 left-0 whitespace-pre-wrap"
    } else {
        ""
    };

    rsx! {
        div { class: "relative w-full h-full",
            pre { class: "{pre_class.unwrap_or_default()} {both_class.as_deref().unwrap_or_default()} {default_pre_class}",
                "{pre_val}{addon}"
            }

            textarea {
                class: "{class.unwrap_or_default()} {both_class.unwrap_or_default()} {default_class}",
                value: value,
                spellcheck: false,
                oninput: oninput,
                onclick: onclick,
                onfocus: onfocus,
                onkeyup: onkeyup,
                onkeydown: onkeydown,
                onkeypress: onkeypress
            }
        }
    }
}
