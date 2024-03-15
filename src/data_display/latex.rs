use base64::Engine;
use dioxus::prelude::*;

#[component]
pub fn Latex(
  value: String,
) -> Element {
  let result = tectonic::latex_to_pdf(&value);
  let Ok(pdf_bytes) = result else {
    return rsx!(
      "Could not render latex! {result.unwrap_err()}"
    )
  };
  let pdf_encoded = base64::engine::general_purpose::STANDARD.encode(&pdf_bytes);
  rsx!(
    img {
      src: "data:application/pdf;base64,{pdf_encoded}",
      alt: "Rendered latex",
    }
  )
}