use dioxus::prelude::*;

#[derive(Clone, PartialEq, Debug)]
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

    let active = if is_active {
        "bg-gray-100"
    } else {
        ""
    };

    let id = action.id;
    rsx! (
      button {
        class: "flex w-full items-center cursor-pointer p-2 {active} rounded-lg",
        onclick: move |_| {action.run.call(manager); manager.visible.set(false)},
        onmouseenter: move |_| manager.active.set(Some(id)),
        if let Some(icon) = &action.icon {
          span {
            class: "mr-2",
            {icon}
          }
        }
        span {
          class: "mr-2",
          "{action.title}"
        },
        span {
          class: "text-gray-500",
          "{action.description}"
        }
      }
    )
}

#[component]
pub fn CommandPalette(
    actions: Signal<Vec<CommandAction>>,
    visible: Signal<bool>,
    #[props(default)] placeholder: String,
) -> Element {
    let mut active = use_signal(|| None);
    let mut input = use_signal(String::new);
    let mut input_el = use_signal(|| None);

    let handle_keys = move |ev: KeyboardEvent| {
      match ev.key() {
        Key::Escape => visible.set(false),
        Key::ArrowUp => {},
        Key::ArrowDown => {},
        _ => {},
      }
    };

    // let visible = use_signal(|| visible);
    use_effect(move || {
        if visible() {
            input_el
                .write()
                .as_ref()
                .map(|f: &MountedEvent| f.set_focus(true));
        } else {
            input.write().clear();
            active.set(None);
        }
    });

    let mut manager = CommandPaletteManager {
        visible,
        input,
        actions,
        active,
    };
    use_root_context(|| manager);

    visible().then(|| ())?;

    rsx!(
      div {
        class: "fixed top-0 left-0 w-full h-full bg-black bg-opacity-50 z-50",
        onclick: move |_| manager.visible.set(false),
        onkeydown: handle_keys,
        div {
          class: "w-96 m-auto bg-white rounded-lg shadow-lg",
          onmouseleave: move |_| manager.active.set(None),
          onmouseenter: move |_| manager.active.set(None),
          onkeydown: handle_keys,
          div {
            class: "p-2",
            input {
              class: "w-full",
              placeholder,
              oninput: move |e| input.set(e.value()),
              onmounted: move |el| input_el.set(Some(el)),
              onkeydown: handle_keys,
            },
          }
          div {
            class: "p-2",
            onkeydown: handle_keys,
            // {actions.iter().map(|action| command(action.clone(), manager))}
            for action in actions.iter().filter(|f| f.title.contains(input.read().as_str())) {
              Command {
                action: action.clone(),
              }
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
