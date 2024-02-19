use dioxus::prelude::*;

#[component]
pub fn Toaster() -> Element {
  let ToastManager { toasts, .. } = use_context();

  let toasts = toasts.iter().map(|f| rsx!(
    
  ));

  rsx!(
    {toasts}
  )

}


#[derive(Copy, Clone, PartialEq, Default, Debug)]
enum ToastType {
  Action,
  Success,
  Info,
  Warning,
  Error,
  Loading,
  #[default]
  Default,
}

#[derive(Copy, Clone, PartialEq, Default, Debug)]
enum ToastPosition {
  TopLeft,
  TopCenter,
  TopRight,
  BottomLeft,
  BottomCenter,
  #[default]
  BottomRight,
}

struct ToastInfo {
  index: usize,
  title: String,
  toast_type: ToastType,
  icon: Option<Element>,
  invert: bool,
  description: Option<Element>,
  duration: u32,
  action: Option<EventHandler<MouseEvent>>,
  cancel: Option<EventHandler<()>>,
  ondismiss: Option<EventHandler<ToastInfo>>,
  onautoclose: Option<EventHandler<ToastInfo>>,
  dismissible: bool,
  class: String,
  unstyled: bool,
  position: ToastPosition,
  height: u32,
}

#[derive(Clone)]
struct ToastManager {
  count: Signal<usize>,
  toasts: Signal<Vec<ToastInfo>>,
}

fn use_toast_manager() {

  let toasts = use_signal(Vec::new);
  let mut count = use_signal(|| 0);

  use_effect(|| {
    let len = toasts.read().len();
    count.set(len);
    println!("There are {count} toasts!");
  });

  let manager = ToastManager {
    toasts,
    count,
  };
  use_root_context(|| manager);
}

pub fn toast(
  title: String,
  description: String,
  class: String,
  style: String,
  kind: ToastType,
  position: ToastPosition,
  duration: u32,
  unstyled: bool,
  invert: bool,
  dismissible: bool,
  icon: Option<Element>,
  action: Option<EventHandler<MouseEvent>>,
  cancel: Option<EventHandler<()>>,
  ondismiss: Option<EventHandler<ToastInfo>>,
  onautoclose: Option<EventHandler<ToastInfo>>,

) {
  let ToastManager {
    count, toasts
  } = use_context();
  let new_toast = ToastInfo {
    index: count(),
    title,
    toast_type,
    class,
    icon,
    invert,
    description,
    dismissible,
    duration,
    position,
    ondismiss,
    onautoclose,
    cancel,
    action,
    unstyled,
    height: 0
  };
  toasts.push(new_toast);
}

