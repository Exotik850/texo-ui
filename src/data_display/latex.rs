use base64::Engine;
use dioxus::prelude::*;
use latex2mathml::DisplayStyle;

#[component]
pub fn Latex(value: String) -> Element {
  match latex2mathml::replace(&value) {
    Ok(ml) => rsx!(
      div {
        dangerous_inner_html: ml,
      }
    ),
    Err(e) => rsx!(
      "Could not render latex! {e}"
    )
  }

}

pub fn latex2element(
  value: &str,
  display: DisplayStyle
) -> Element {
  let ml = match latex2mathml::latex_to_mathml(value, display) {
    Ok(ml) => ml,
    Err(e) => {
      return rsx!(
        "Could not render latex! {e}"
      )
    }
  };
  rsx!(

    div {
      // "{value}"
      dangerous_inner_html: ml,
    }

    // img {
    //   src: "data:application/pdf;base64,{pdf_encoded}",
    //   alt: "Rendered latex",
    // }
  )
}