use std::{marker::PhantomData, rc::Rc};

use dioxus::prelude::*;
use typst::{model::Document, World};

#[derive(Clone, Copy)]
struct TypstManager {
  document: Signal<Document>,
  world: Signal<Box<dyn World>>,
}

pub fn use_typst<F>(world: F)
where F: FnOnce() -> Box<dyn World>,
{
  let document = use_signal(Document::default);
  let world = use_signal(world);
  let manager = TypstManager { document, world };
  use_root_context(|| manager);
}

pub fn Typst(
  value: Signal<String>,
) -> Element {

  let TypstManager { document, world } = use_context();
  
  



  None
}