use base64::Engine;
use dioxus::prelude::*;
use typst::{diag::SourceResult, layout::Abs, model::Document, visualize::Color, World};

#[derive(Clone, Copy)]
struct TypstManager {
  document: Signal<Document>,
  world: Signal<Box<dyn World>>,
}

pub fn use_typst<F, W>(world: F)
where F: FnOnce() -> W,
      W: World + 'static
{
  let document = use_signal(Document::default);
  let world = use_signal(|| Box::new(world()) as Box<dyn World>);
  let manager = TypstManager { document, world };
  use_root_context(|| manager);
}

#[component]
pub fn Typst<W: World + Clone + PartialEq + 'static>(
  world: Signal<W>,
  #[props(default = Abs::cm(1.0))] padding: Abs,
) -> Element {

  let document: ReadOnlySignal<SourceResult<String>> = use_memo(move || {
    world.read();
    let mut tracer = typst::eval::Tracer::new();
    let document = world.with(|w| typst::compile(w, &mut tracer))?;
    let image = typst_svg::svg_merged(&document.pages, padding);
    // println!("{:?}", image);
    // let bytes = image.encode_png().expect("TO ENCODE OR NOT TO ENCODE");
    // Ok(base64::engine::general_purpose::STANDARD.encode(bytes))
    Ok(image)
  });
  match document.read().as_ref() {
    Ok(doc) => {
      rsx!(
        div {
          dangerous_inner_html: "{doc}"
        }
          // svg {
          //   xmlns: "http://www.w3.org/2000/svg",
          //   width: "100%",
          //   height: "100%",
          //   view_box: "0 0 100 100",
          //   fill: "none",
          //   stroke: "black",
          //   stroke_width: "0.1",
          //   stroke_linecap: "round",
          //   stroke_linejoin: "round",
          //   path {
          //     d: "{doc}",
          //     fill: "none",
          //     stroke: "black",
          //     stroke_width: "0.1",
          //     stroke_linecap: "round",
          //     stroke_linejoin: "round",
          //   }
          
          // }
      )
    },
    Err(err) => {
      rsx!(
        "Could not render document: {err:?}"
      )
    } 
  }
}