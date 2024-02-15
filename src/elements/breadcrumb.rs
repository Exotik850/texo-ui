use dioxus::prelude::*;

use crate::merge_classes;

#[component]
pub fn Breadcrumb(
  #[props(default)]
  solid: bool,
  #[props(default, into)]
  class: String,
  #[props(default = "flex".into(), into)]
  nav_class: String,
  #[props(default = "flex px-5 py-3 text-gray-700 border border-gray-200 rounded-lg bg-gray-50 dark:bg-gray-800 dark:border-gray-700".into(), into)]
  solid_class: String,
  #[props(default = "inline-flex items-center space-x-1 rtl:space-x-reverse md:space-x-3 rtl:space-x-reverse".into(), into)]
  ol_class: String,
  #[props(default = "Breadcrumb".into(), into)]
  aria_label: String,
  #[props(extends=nav)]
  rest_attributes: Vec<Attribute>,
  children: Element,
) -> Element {

  let nav_class = if solid {
    nav_class
  } else {
    solid_class
  };
  let nav_class = merge_classes(&[
    &nav_class,
    &class
  ]);

  rsx!(
    nav {
      ..rest_attributes,
      aria_label,
      class: nav_class,
      ol {
        class: ol_class,
        {children}
      }
    } 
  )
}

#[component]
pub fn BreadcrumbItem(
  #[props(default)]
  home: bool,
  #[props(default, into)]
  class: String,
  href: Option<String>,
  #[props(default = "ms-1 text-sm font-medium text-gray-700 hover:text-gray-900 md:ms-2 dark:text-gray-400 dark:hover:text-white".into(), into)]
  link_class: String,
  #[props(default = "ms-1 text-sm font-medium text-gray-500 md:ms-2 dark:text-gray-400".into(), into)]
  span_class: String,
  #[props(default = "inline-flex items-center text-sm font-medium text-gray-700 hover:text-gray-900 dark:text-gray-400 dark:hover:text-white".into(), into)]
  home_class: String,
  icon: Option<Element>,
  children: Option<Element>,
  #[props(extends=li)]
  rest_attributes: Vec<Attribute>,
) -> Element {

  let li_class = merge_classes(&[
    "inline-flex items-center",
    &class,
  ]);

  rsx!(
    li {
      ..rest_attributes,
      class: li_class,
      if home {
        a {
          class: home_class,
          if let Some(icon) = icon {
            {icon}
          } else {
            svg {
              class: "w-4 h-4 me-2",
              fill: "currentColor",
              view_box: "0 0 20 20",
              xmlns: "http://www.w3.org/2000/svg",
              path {
                d: "M10.707 2.293a1 1 0 00-1.414 0l-7 7a1 1 0 001.414 1.414L4 10.414V17a1 1 0 001 1h2a1 1 0 001-1v-2a1 1 0 011-1h2a1 1 0 011 1v2a1 1 0 001 1h2a1 1 0 001-1v-6.586l.293.293a1 1 0 001.414-1.414l-7-7z"
              }
            }
          }
          if let Some(children) = children {
            {children}
          }
        }
      } else {
        if let Some(icon) = icon {
          {icon}
        } else {
          svg {
            class: "w-6 h-6 text-gray-400 rtl:-scale-x-100",
            fill: "currentColor",
            view_box: "0 0 20 20",
            xmlns: "http://www.w3.org/2000/svg",
            path {
              fill_rule: "evenodd",
              d: "M7.293 14.707a1 1 0 010-1.414L10.586 10 7.293 6.707a1 1 0 011.414-1.414l4 4a1 1 0 010 1.414l-4 4a1 1 0 01-1.414 0z",
              clip_rule: "evenodd"
            }
          }
        }
        if let Some(href) = href {
          a {
            class: link_class,
            href,
            if let Some(children) = children {
              {children}
            }
          }
        } else {
          span {
            class: span_class,
            if let Some(children) = children {
              {children}
            }
          }
        }
      }
    }
  )
}