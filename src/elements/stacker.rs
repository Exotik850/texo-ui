use dioxus::prelude::*;

#[component]
pub fn Stacker(#[props(default)] style: String, children: Element, #[props(extends=div)] rest_attributes: Vec<Attribute>) -> Element {
  let mut css = String::new();
  for i in 1..=10 {
    let opacity = 1.0 - (i as f32 * 0.1);
    let z_index = 10 - i;
    let scale = 1.0 - (i as f32 * 0.05); // reduce size by 5% for each child
    let move_down = (i as f32 * 5.0) as i32; // move down by 5px for each child
    css.push_str(&format!("*:nth-child({}) {{ position: absolute; opacity: {}; z-index: {}; transform: translateY({}px) scale({}); }} \n ", i, opacity, z_index, move_down, scale));
  }

  rsx!(
    div { ..rest_attributes, style: "{css} {style}", {children} }
)
}
