use dioxus::prelude::*;

pub struct SliderManager<T: 'static> {
  value: Signal<T>,
  active: Signal<bool>,
  open: Signal<bool>,
  dragging: Signal<bool>,
}

pub fn use_slider<T: 'static>(f: impl FnOnce() -> T) {
  let value = use_signal(f);
  let active = use_signal(|| false);
  let open = use_signal(|| false);
  let dragging = use_signal(|| false);
}