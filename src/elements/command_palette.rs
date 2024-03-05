use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct CommandAction {
    id: u16,
    active: bool,
    run: EventHandler<CommandPaletteManager>,
    title: String,
    description: String,
    icon: Option<Element>,
    shortcut: Option<String>,
}

impl CommandAction {
    pub fn new<R>(
        run: R,
        title: String,
        description: String,
        icon: Option<Element>,
        shortcut: Option<String>,
    ) -> Self
    where
        R: FnMut(CommandPaletteManager) -> () + 'static,
    {
        Self {
            id: fastrand::u16(..),
            active: false,
            run: EventHandler::new(run),
            title,
            description,
            icon,
            shortcut,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct CommandPaletteManager {
    visible: Signal<bool>,
    input: Signal<String>,
    actions: Signal<Vec<CommandAction>>,
    active: Signal<Option<u16>>,
}

impl std::fmt::Debug for CommandPaletteManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CommandPaletteManager")
            .field("visible", &self.visible)
            .field("input", &self.input)
            .field("active", &self.active)
            .finish()
    }
}

#[component]
fn Command(action: CommandAction) -> Element {
    let mut manager: CommandPaletteManager = use_context();

    let is_active = manager.active.read().is_some_and(|id| id == action.id);

    let active = if is_active { "bg-gray-100" } else { "" };

    let id = action.id;
    rsx! (
        button {
            class: "flex w-full items-center cursor-pointer p-2 {active} rounded-lg",
            onclick: move |_| {
                action.run.call(manager);
                manager.visible.set(false)
            },
            onmouseenter: move |_| manager.active.set(Some(id)),
            if let Some(icon) = &action.icon {
                span { class: "mr-2", {icon} }
            }
            span { class: "mr-2", "{action.title}" }
            span { class: "text-gray-500", "{action.description}" }
        }
    )
}

#[component]
pub fn CommandPalette(
    actions: Signal<Vec<CommandAction>>,
    visible: Signal<bool>,
    #[props(default)] placeholder: String,
) -> Element {
    let mut active: Signal<Option<u16>> = use_signal(|| None);
    let mut input = use_signal(String::new);
    let mut input_el = use_signal(|| None);
    use_effect(move || {
        input_el
            .write()
            .as_ref()
            .map(|f: &MountedEvent| f.set_focus(visible()));
        if !visible() {
            input.write().clear();
            active.set(None);
        }
    });

    let manager = CommandPaletteManager {
        visible,
        input,
        actions,
        active,
    };
    use_root_context(|| manager);

    if !visible() {
        return None;
    }

    let handle_keys = move |ev: KeyboardEvent| match ev.key() {
        Key::Escape => visible.set(false),
        Key::ArrowUp => {
            let actions = actions.read();
            if let Some(active_id) = active() {
                let index = actions.iter().position(|f| f.id == active_id).unwrap();
                if index > 0 {
                    active.set(Some(actions[index - 1].id));
                }
            } else if actions.len() > 0 {
                active.set(Some(actions[actions.len() - 1].id))
            }
        }
        Key::ArrowDown => {
            let actions = actions.read();
            if let Some(active_id) = active() {
                let index = actions.iter().position(|f| f.id == active_id).unwrap();
                if index < actions.len() - 1 {
                    active.set(Some(actions[index + 1].id));
                }
            } else if actions.len() > 0 {
                active.set(Some(actions[0].id))
            }
        }
        Key::Enter => {
            if let Some(id) = active() {
                actions
                    .read()
                    .iter()
                    .find(|f| f.id == id)
                    .map(|f| f.run.call(manager));
                visible.set(false);
            }
        }
        _ => {}
    };

    rsx!(
        div {
            class: "fixed top-0 left-0 w-full h-full bg-black bg-opacity-50 z-50",
            onclick: move |_| visible.set(false),
            onkeydown: handle_keys,
            div {
                class: "w-96 m-auto bg-white rounded-lg shadow-lg",
                onmouseleave: move |_| active.set(None),
                onkeydown: handle_keys,
                div { class: "p-2",
                    input {
                        class: "w-full",
                        placeholder: placeholder,
                        oninput: move |e| input.set(e.value()),
                        onmounted: move |el| input_el.set(Some(el)),
                        onkeydown: handle_keys
                    }
                }
                div { class: "p-2", onkeydown: handle_keys,
                    for action in actions
    .iter()
    .filter(|f| {
        f.title.contains(input.read().as_str())
            || f.description.contains(input.read().as_str())
    }) {
                        Command { action: action.clone(), key: "{action.id}" }
                    }
                }
            }
        }
    )
}

// Example usage
// ```
// fn main() {
//   let actions = Signal::new(vec![
//     CommandAction::new(|_| println!("Hello"), "Hello".to_string(), "Say hello".to_string(), None, None),
//     CommandAction::new(|_| println!("World"), "World".to_string(), "Say world".to_string(), None, None),
//   ]);
//   let palette = rsx! {
//     CommandPalette {
//       actions
//     }
//   };
//   render(palette, "app");
// }
// ```
