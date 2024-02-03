#![feature(let_chains, async_closure)]
#![allow(dead_code, non_snake_case)]
use std::{collections::HashSet, fmt::Display};

#[cfg(all(feature="web", feature="desktiop"))]
compile_error!("Cannot target both web and desktop at the same time!");

use manganis::classes;

pub mod elements;
pub mod forms;
pub mod hooks;
pub mod util;

pub fn merge_classes(input: &[impl AsRef<str>]) -> String {
    let mut class_set = HashSet::new();
    for thing in input {
        for class in thing.as_ref().split_whitespace() {
            class_set.insert(class);
        }
    }

    let mut merged = String::new();
    for class in class_set.iter() {
        merged.push_str(&class);
        merged.push(' ');
    }
    if !class_set.is_empty() {
        merged.truncate(merged.len() - 1);
    }
    merged
}

pub fn border_color(color: TexoColor) -> &'static str {
    match color {
        TexoColor::Alternative => todo!(),
        TexoColor::Blue => {
            classes!("border-blue-300 dark:border-blue-800 divide-blue-300 dark:divide-blue-800")
        }
        TexoColor::Default => classes!("border-gray-700 divide-gray-500"),
        TexoColor::Gray => classes!("border-gray-500 divide-gray-500"),
        TexoColor::Dark => classes!("border-gray-500 divide-gray-500"),
        TexoColor::Green => {
            classes!(
                "border-green-300 dark:border-green-800 divide-green-300 dark:divide-green-800"
            )
        }
        TexoColor::Light => classes!("border-gray-500 divide-gray-500"),
        TexoColor::Primary => {
            classes!("border-primary-500 dark:border-primary-200  divide-primary-500 dark:divide-primary-20")
        }
        TexoColor::Purple => {
            classes!(
                "border-purple-300 dark:border-purple-800 divide-purple-300 dark:divide-purple-800"
            )
        }
        TexoColor::Red => {
            classes!("border-red-300 dark:border-red-800 divide-red-300 dark:divide-red-800")
        }
        TexoColor::Yellow => {
            classes!(
                "border-yellow-300 dark:border-yellow-800 divide-yellow-300 dark:divide-yellow-800"
            )
        }
        TexoColor::None => "",
    }
}

pub fn text_color(color: TexoColor) -> &'static str {
    match color {
        TexoColor::Alternative => todo!(),
        TexoColor::Blue => classes!("text-blue-800 dark:text-blue-400"),
        TexoColor::Dark => classes!("text-gray-700 dark:text-gray-300"),
        TexoColor::Default => classes!("text-gray-700 dark:text-gray-400"),
        TexoColor::Gray => classes!("text-gray-800 dark:text-gray-400"),
        TexoColor::Green => classes!("text-green-800 dark:text-green-400"),
        TexoColor::Light => classes!("text-gray-700 dark:text-gray-300"),
        TexoColor::Primary => classes!("text-primary-800 dark:text-primary-400"),
        TexoColor::Purple => classes!("text-purple-800 dark:text-purple-400"),
        TexoColor::Red => classes!("text-red-800 dark:text-red-400"),
        TexoColor::Yellow => classes!("text-yellow-800 dark:text-yellow-300"),
        TexoColor::None => "",
    }
}

pub fn bg_color(color: TexoColor) -> &'static str {
    match color {
        TexoColor::Alternative => todo!(),
        TexoColor::Blue => classes!("bg-blue-50 dark:bg-gray-800"),
        TexoColor::Dark => classes!("bg-gray-50 dark:bg-gray-800"),
        TexoColor::Default => classes!("bg-gray-50 dark:bg-gray-700"),
        TexoColor::Gray => classes!("bg-gray-50 dark:bg-gray-800"),
        TexoColor::Green => classes!("bg-green-50 dark:bg-gray-800"),
        TexoColor::Light => classes!("bg-gray-50 dark:bg-gray-700"),
        TexoColor::Primary => classes!("bg-primary-50 dark:bg-gray-800"),
        TexoColor::Purple => classes!("bg-purple-50 dark:bg-gray-800"),
        TexoColor::Red => classes!("bg-red-50 dark:bg-gray-800"),
        TexoColor::Yellow => classes!("bg-yellow-50 dark:bg-gray-800"),
        TexoColor::None => "",
    }
}

pub trait TexoComponent {
    fn get_size(&self, size: TexoSize) -> &str;
    fn get_class(&self, color: TexoColor) -> &str;
}

#[derive(PartialEq, Copy, Clone, Default, Debug)]
pub enum TexoColor {
    Alternative,
    Blue,
    Dark,
    #[default]
    Default,
    Gray,
    Green,
    Light,
    Primary,
    Purple,
    Red,
    Yellow,
    None,
}

impl Display for TexoColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bit = match self {
            TexoColor::Alternative => "alternative",
            TexoColor::Blue => "blue",
            TexoColor::Dark => "dark",
            TexoColor::Green => "green",
            TexoColor::Default => "default",
            TexoColor::Gray => "gray",
            TexoColor::Light => "light",
            TexoColor::Primary => "primary",
            TexoColor::Purple => "purple",
            TexoColor::Red => "red",
            TexoColor::Yellow => "yellow",
            TexoColor::None => "none",
        };
        write!(f, "{bit}")
    }
}

#[derive(PartialEq, Copy, Clone)]
pub enum TexoSize {
    ExtraSmall,
    Small,
    Medium,
    Large,
    ExtraLarge,
}

impl TexoSize {
    pub fn icon_size(self) -> &'static str {
        match self {
            TexoSize::ExtraSmall => "5",
            TexoSize::Small => "15",
            TexoSize::Medium => "25",
            TexoSize::Large => "50",
            TexoSize::ExtraLarge => "100",
        }
    }
}

#[derive(PartialEq, Copy, Clone)]
pub enum TexoTrigger {
    Hover,
    Click,
}

#[derive(PartialEq, Copy, Clone, Default)]
pub enum TexoInputType {
    Color,
    Date,
    DatetimeLocal,
    Email,
    File,
    Hidden,
    Image,
    Month,
    Number,
    Password,
    Reset,
    Submit,
    Tel,
    #[default]
    Text,
    Time,
    Url,
    Week,
    Search,
}

impl Display for TexoInputType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bit = match self {
            TexoInputType::Color => "color",
            TexoInputType::Date => "date",
            TexoInputType::DatetimeLocal => "datetime-local",
            TexoInputType::Email => "email",
            TexoInputType::File => "file",
            TexoInputType::Hidden => "hidden",
            TexoInputType::Image => "image",
            TexoInputType::Month => "month",
            TexoInputType::Number => "number",
            TexoInputType::Password => "password",
            TexoInputType::Reset => "reset",
            TexoInputType::Submit => "submit",
            TexoInputType::Tel => "tel",
            TexoInputType::Text => "text",
            TexoInputType::Time => "time",
            TexoInputType::Url => "url",
            TexoInputType::Week => "week",
            TexoInputType::Search => "search",
        };
        write!(f, "{bit}")
    }
}

#[derive(PartialEq, Copy, Clone, Default)]
pub enum TexoPosition {
    Static,
    #[default]
    Fixed,
    Absolute,
    Relative,
    Sticky,
}

impl Display for TexoPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bit = match self {
            TexoPosition::Static => "static",
            TexoPosition::Fixed => "fixed",
            TexoPosition::Absolute => "absolute",
            TexoPosition::Relative => "relative",
            TexoPosition::Sticky => "sticky",
        };
        write!(f, "{bit}")
    }
}
#[derive(PartialEq, Copy, Clone, Default)]
pub enum ModalPlacementType {
    TopLeft,
    TopCenter,
    TopRight,
    CenterLeft,
    #[default]
    Center,
    CenterRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
    None,
}
