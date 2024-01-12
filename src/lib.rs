use dioxus::prelude::*;

pub mod elements;


pub trait TexoComponent {
  fn get_size(&self, size: TexoSize) -> &str;
  fn get_class(&self, color: TexoColor) -> &str;
}

#[derive(PartialEq)]
pub enum TexoColor {
  Alternative,
  Blue,
  Dark,
  Green,
  Light,
  Primary,
  Purple,
  Red,
  Yellow,
  None,
}

#[derive(PartialEq)]
pub enum TexoSize {
  ExtraSmall,
  Small,
  Medium,
  Large,
  ExtraLarge,
}
