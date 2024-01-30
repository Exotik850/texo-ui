use dioxus::prelude::*;
use manganis::classes;

use crate::{bg_color, border_color, merge_classes, text_color, TexoColor};

#[derive(Clone, PartialEq, Debug, Default)]
pub struct TableContext {
    pub striped: bool,
    pub hoverable: bool,
    pub noborder: bool,
    pub color: TexoColor,
}

#[component]
pub fn Table(
    #[props(default)] shadow: bool,
    #[props(default)] striped: bool,
    #[props(default)] noborder: bool,
    #[props(default)] hoverable: bool,
    #[props(default)] color: TexoColor,
    class: Option<String>,
    div_class: Option<String>,
    children: Element,
    #[props(extends=table)] rest_attributes: Vec<Attribute>,
) -> Element {
    use_context_provider(|| TableContext {
        noborder,
        striped,
        hoverable,
        color,
    });

    let div_class = merge_classes(&[
        if let Some(div_class) = &div_class {
            div_class
        } else {
            "relative overflow-x-auto"
        },
        if shadow {
            "shadow-md sm:rounded-lg"
        } else {
            ""
        },
    ]);

    let class = merge_classes(&[
        if let Some(class) = &class {
            class
        } else {
            "w-full text-left text-sm"
        },
        text_color(color),
    ]);

    rsx! {
      div {
        class: div_class,
        table {
          ..rest_attributes,
          class,
          {children}
        }
      }
    }
}

fn table_head_bg<'a>(color: TexoColor, default: &'a str) -> &'a str {
    match color {
        TexoColor::Alternative => classes!(""),
        TexoColor::Blue => classes!("bg-blue-600"),
        TexoColor::Dark => classes!("bg-gray-400"),
        TexoColor::Default => default,
        TexoColor::Gray => classes!("bg-gray-200"),
        TexoColor::Green => classes!("bg-green-600"),
        TexoColor::Light => classes!("bg-gray-100"),
        TexoColor::Primary => classes!(""),
        TexoColor::Purple => classes!("bg-purple-600"),
        TexoColor::Red => classes!("bg-red-600"),
        TexoColor::Yellow => classes!("bg-yellow-700"),
        TexoColor::None => classes!(""),
    }
}

#[component]
pub fn TableHead(
    class: Option<String>,
    #[props(default = true)] default_row: bool,
    #[props(default)] color: TexoColor,
    #[props(extends=thead)] rest_attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let TableContext {
        striped,
        noborder,
        color,
        ..
    } = use_context();

    let col = if striped {
        "".into()
    } else {
        format!("border-{color}-400")
    };

    let bg_col = if noborder || striped {
        ""
    } else {
        classes!("bg-gray-50 dark:bg-gray-700")
    };

    let classes = &[
        &class.unwrap_or_else(|| "text-xs uppercase".into()),
        text_color(color),
        // border_color(color),
        &col,
        table_head_bg(color, bg_col),
    ];

    let final_class = merge_classes(classes);

    rsx! {
      thead {
        ..rest_attributes,
        class: "{final_class}",
        if default_row {
          tr {
            {children}
          }
        } else {
          {children}
        }
      }
    }
}

#[component]
pub fn TableHeadCell(
    #[props(default)] class: String,
    #[props(default="px-6 py-3".into())] padding: String,
    #[props(extends=th)] rest_attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
      th {
        ..rest_attributes,
        class: "{class} {padding}",
        {children}
      }
    }
}

#[component]
pub fn TableBody(class: Option<String>, children: Element) -> Element {
    rsx!(
    tbody {
      class: "{class.unwrap_or_default()}",
        {children}
      }
    )
}

fn table_body_color(color: TexoColor) -> &'static str {
    match color {
        TexoColor::Alternative => classes!(""),
        TexoColor::Blue => classes!("bg-blue-500 border-blue-400"),
        TexoColor::Dark => classes!(""),
        TexoColor::Default => classes!("bg-white dark:bg-gray-800 dark:border-gray-700"),
        TexoColor::Gray => classes!("bg-gray-300 dark:bg-gray-800 dark:border-gray-700"),
        TexoColor::Green => classes!("bg-green-500 border-green-400"),
        TexoColor::Light => classes!(""),
        TexoColor::Primary => classes!("bg-purple-500 border-purple-400"),
        TexoColor::Purple => classes!(""),
        TexoColor::Red => classes!("bg-red-500 border-red-400"),
        TexoColor::Yellow => classes!("bg-yellow-500 border-yellow-400"),
        TexoColor::None => classes!(""),
    }
}
fn table_body_hover(color: TexoColor) -> &'static str {
    match color {
        TexoColor::Alternative => classes!(""),
        TexoColor::Blue => classes!("hover:bg-blue-400"),
        TexoColor::Dark => classes!(""),
        TexoColor::Default => classes!("hover:bg-gray-50 dark:hover:bg-gray-600"),
        TexoColor::Gray => classes!(""),
        TexoColor::Green => classes!("hover:bg-green-400"),
        TexoColor::Light => classes!(""),
        TexoColor::Primary => classes!(""),
        TexoColor::Purple => classes!("hover:bg-purple-400"),
        TexoColor::Red => classes!("hover:bg-red-400"),
        TexoColor::Yellow => classes!("hover:bg-yellow-400"),
        TexoColor::None => classes!(""),
    }
}
fn table_strip_color(color: TexoColor) -> &'static str {
    match color {
        TexoColor::Alternative => classes!(""),
        TexoColor::Blue => {
            classes!("odd:bg-blue-800 even:bg-blue-700 odd:dark:bg-blue-800 even:dark:bg-blue-700")
        }
        TexoColor::Dark => classes!(""),
        TexoColor::Default => {
            classes!("odd:bg-white even:bg-gray-50 odd:dark:bg-gray-800 even:dark:bg-gray-700")
        }
        TexoColor::Gray => classes!(""),
        TexoColor::Green => classes!(
            "odd:bg-green-800 even:bg-green-700 odd:dark:bg-green-800 even:dark:bg-green-700"
        ),
        TexoColor::Light => classes!(""),
        TexoColor::Primary => classes!(""),
        TexoColor::Purple => classes!(
            "odd:bg-purple-800 even:bg-purple-700 odd:dark:bg-purple-800 even:dark:bg-purple-700"
        ),
        TexoColor::Red => {
            classes!("odd:bg-red-800 even:bg-red-700 odd:dark:bg-red-800 even:dark:bg-red-700")
        }
        TexoColor::Yellow => classes!(
            "odd:bg-yellow-800 even:bg-yellow-700 odd:dark:bg-yellow-800 even:dark:bg-yellow-700"
        ),
        TexoColor::None => classes!(""),
    }
}

#[component]
pub fn TableBodyRow(
    class: Option<String>,
    #[props(extends=tr)] rest_attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let TableContext {
        striped,
        hoverable,
        noborder,
        color,
    } = use_context();

    let final_class = merge_classes(&[
        class.as_deref().unwrap_or_default(),
        if noborder {
            ""
        } else {
            "border-b last:border-b-0"
        },
        table_body_color(color),
        hoverable
            .then(|| table_body_hover(color))
            .unwrap_or_default(),
        striped
            .then(|| table_strip_color(color))
            .unwrap_or_default(),
    ]);

    rsx! {
      tr {
        ..rest_attributes,
        class: final_class,
        {children}
      }
    }
}

#[component]
pub fn TableBodyCell(
    class: Option<String>,
    onclick: Option<EventHandler<MouseEvent>>,
    #[props(default = TexoColor::Gray)] color: TexoColor,
    children: Element,
    #[props(extends=GlobalAttributes)] rest_attributes: Vec<Attribute>,
) -> Element {
    let is_button = onclick.is_some();

    let classes = &[
        class.unwrap_or_default(),
        "px-6 py-4 whitespace-nowrap font-medium".into(),
        if let TexoColor::Gray = color {
            "text-gray-900 dark:text-white".into()
        } else {
            "text-blue-50 whitespace-nowrap dark:text-blue-100".into()
        },
    ];

    let final_class = merge_classes(classes);

    rsx! {
      if is_button {
        button {
          ..rest_attributes,
          class: "{final_class}",
          onclick: move |evt| onclick.as_ref().unwrap().call(evt.clone()),
          {children}
        }
      } else {
        td {
          ..rest_attributes,
          class: "{final_class}",
          {children}
        }
      }
    }
}
