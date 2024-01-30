use dioxus::prelude::*;
use std::path::Path;

#[derive(Copy, Clone, Debug)]
pub struct TreeContext {
    expand: Signal<bool>,
    collapse: Signal<bool>,
    multiple_c: bool,
    selection_c: bool,
    disabled_c: bool,
}

#[component]
pub fn TreeView(
    #[props(default = false)] selection: bool,
    #[props(default = false)] multiple: bool,
    #[props(default = false)] disabled: bool,
    #[props(default = "w-full space-y-1 py-4 px-4 ml-4 hover:variant-soft rounded-container-token".into())]
    class: String,
    #[props(default="".into())] aria_label: String,
    children: Element,
) -> Element {
    let mut expand = use_signal(|| false);
    let mut collapse = use_signal(|| false);

    use_effect(move || {
        if expand() {
            expand.set(false);
        }
        if collapse() {
            collapse.set(false);
        }
    });

    use_context_provider(|| TreeContext {
        expand,
        collapse,
        multiple_c: multiple,
        selection_c: selection,
        disabled_c: disabled,
    });

    rsx!(
      div {
        class,
        role: "tree",
        aria_multiselectable: multiple,
        aria_label,
        aria_disabled: disabled,
        {children}
      }

    )
}

#[component]
pub fn TreeViewItem(
    #[props(default = false)] open: bool,
    disabled: Option<bool>,
    multiple: Option<bool>,
    selection: Option<bool>,
    input_name: Option<String>,
    #[props(default = "w-full space-y-1 py-4 px-4 ml-4 hover:variant-soft rounded-container-token".into())]
    class: String,
    value: Option<Signal<String>>,
    checked: Option<Signal<bool>>,
    onclick: Option<EventHandler<MouseEvent>>,
    onkeydown: Option<EventHandler<KeyboardEvent>>,
    lead: Option<Element>,
    children: Option<Element>,
) -> Element {
    let mut open = use_signal(|| open);

    let TreeContext {
        expand,
        collapse,
        multiple_c,
        selection_c,
        disabled_c,
    } = use_context::<TreeContext>();

    let multiple = multiple.unwrap_or(multiple_c);
    let selection = selection.unwrap_or(selection_c);
    let disabled = disabled.unwrap_or(disabled_c);

    use_effect(move || {
        if expand() {
            open.set(true)
        }
        if collapse() {
            open.set(false)
        }
    });

    let c_disabled = if disabled {
        "opacity-50 !cursor-not-allowed"
    } else {
        ""
    };

    let c_caret = if open() { "rotate-180" } else { "" };

    let onchange = move |evt: FormEvent| {
        if let Some(mut value) = value {
            value.set(evt.data.value())
        }
    };

    let has_child = children.as_ref().is_some_and(|f| f.is_some());
    let pointer = if has_child || onclick.is_some() {
        "cursor-pointer"
    } else {
        ""
    };

    rsx!(
      details {
        open: open(),
        class,
        aria_disabled: disabled,
        prevent_default: "onclick",
        summary {
          class: "list-none [&::-webkit-details-marker]:hidden flex items-center {pointer} {c_disabled} space-y-1 py-4 px-4 ml-4 hover:variant-soft rounded-container-token",
          aria_selected: selection,
          aria_expanded: if has_child { open() },
          prevent_default: "onclick",
          onclick: move |evt| {if has_child { open.toggle() }; if let Some(oc) = &onclick {oc.call(evt)}},
          onkeydown: move |evt| if let Some(ok) = &onkeydown {ok.call(evt)} else {
                    match evt.data.key() {
                      Key::ArrowLeft => open.set(false),
                      Key::ArrowRight => open.set(true),
                      _ => ()
                    }
              },
          div {
            class: "fill-current w-3 text-center transition duration-[200ms] {c_caret}",
            if has_child {
              svg { view_box: "0 0 448 512", xmlns: "http://www.w3.org/2000/svg",
                  path { d: "M201.4 374.6c12.5 12.5 32.8 12.5 45.3 0l160-160c12.5-12.5 12.5-32.8 0-45.3s-32.8-12.5-45.3 0L224 306.7 86.6 169.4c-12.5-12.5-32.8-12.5-45.3 0s-12.5 32.8 0 45.3l160 160z" }
              }
            } else {
              svg {
                xmlns: "http://www.w3.org/2000/svg", view_box: "0 0 448 512", class: "w-3",
                  path { d: "M432 256c0 17.7-14.3 32-32 32L48 288c-17.7 0-32-14.3-32-32s14.3-32 32-32l352 0c17.7 0 32 14.3 32 32z" }
              }
            }
          }

          if selection && input_name.is_some() {
            if multiple {
              input {
                class: "checkbox",
                r#type: "checkbox",
                name: input_name.unwrap(),
                value: if let Some(value) = value {value.read().clone()},
                checked: if let Some(checked) = checked {checked()},
                onclick: move |_| if let Some(mut checked) = checked {checked.toggle()},
                onchange,
              }
            } else {
              input {
                class: "radio",
                r#type: "radio",
                name: input_name.unwrap(),
                value: if let Some(value) = value { value.read().clone() },
                onchange,
              }
            }
          }

          if let Some(lead) = lead {
            {lead}
          }

        }

        div {
          class: "ml-4",
          if let Some(Some(children)) = &children {
            {children}
          }
        }
      }
    )
}

#[derive(Clone, Debug, PartialEq)]
pub enum TreeOutput {
    Item(Element, Vec<TreeOutput>),
    ItemNoChildren(Element),
    None,
}

fn build_tree<P: AsRef<Path>>(path: P) -> Vec<TreeOutput> {
    let mut items = vec![];

    let Ok(entries) = std::fs::read_dir(&path) else {
        log::warn!("Could not open: {}", path.as_ref().display());
        return items;
    };

    for entry in entries {
        let Ok(entry) = entry else {
            continue;
        };

        let fullpath = entry.path();
        let Some(name) = fullpath.file_name() else {
            continue;
        };
        let name = name.to_str().unwrap_or("Invalid Name!");
        if fullpath.is_dir() {
            let sub_items = build_tree(&fullpath);
            items.push(TreeOutput::Item(rsx! {"{name}"}, sub_items))
        } else {
            items.push(TreeOutput::ItemNoChildren(rsx! {"{name}"}))
        }
    }

    items
}

#[component]
pub fn FileTreeView(path: String) -> Element {
    rsx! {
      RecursiveTreeView {
        components: build_tree(&path)
      }
    }
}

fn expand(item: TreeOutput, child_class: Option<String>) -> Element {
    rsx!(match item {
        TreeOutput::Item(lead, child) => rsx!(
          TreeViewItem {
            lead,
            class: child_class.as_deref().unwrap_or_default(),
            {child.into_iter().map(|f| expand(f, child_class.clone()))}
          }
        ),
        TreeOutput::ItemNoChildren(lead) => rsx!(TreeViewItem {
            lead,
            class: child_class.unwrap_or_default(),
        }),
        TreeOutput::None => None,
    })
}

#[component]
pub fn RecursiveTreeView(components: Vec<TreeOutput>, child_class: Option<String>) -> Element {
    rsx! (
      TreeView {
        {components.into_iter().map(|f| expand(f, child_class.clone()))}
      }
    )
}
