use base64::Engine;
use dioxus::prelude::*;
use la_texer::{replace_latex, DisplayStyle, Node, Parser, TexNode};

#[component]
pub fn Latex(value: String, display: DisplayStyle) -> Element {
  if value.len() > 1 {
    latex2element(value.as_ref(), display)
  } else {
    None
  }
}

pub fn latex2element(value: &str, display: DisplayStyle) -> Element {
    let ds = match display {
        DisplayStyle::Block => true,
        DisplayStyle::Inline => false,
    };

    rsx!(
      if ds {
        br {}
      }
      math {
        {Parser::new(value).map(node_element)}
      }
      if ds {
        br {}
      }
    )
}

fn node_element(node: Node) -> Element {
    match node {
        Node::Number(n) => rsx!(mn { "{n}" }),
        Node::Operator(op) => {
            if op.is_empty() {
                rsx!(mo { "∂" } )
            } else {
                rsx!(mo { "{op}" })
            }
        }
        Node::Text(text, _) => rsx!(mtext {
          "{text}"
        }),
        Node::Letter(letter, _) => rsx!(
          mi {
            "{letter}"
          }
        ),
        Node::Function(fun, arg) => match arg {
          Some(arg) => {
            rsx!(
              mrow {
                mi { "{fun}" }
                mo { "(" }
                {node_element(*arg)}
                mo { ")" }
              }
            )
          },
          None => rsx!(
            mi { "{fun}" }
          )
        },
        Node::Space(space) => rsx!(mspace {
          width: "{space}",
        }),
        Node::Subscript(a, b) => rsx!(
          msub {
            {node_element(*a)}
            {node_element(*b)}
          }
        ),
        Node::Superscript(a, b) => rsx!(
          msup {
            {node_element(*a)}
            {node_element(*b)}
          }
        ),
        Node::SubSup { target, sub, sup } => rsx!(
          msubsup {
            {node_element(*target)}
            {node_element(*sub)}
            {node_element(*sup)}
          }
        ),
        Node::OverOp(op, acc, target) => rsx!(
          mover {
            {node_element(*target)}
            mo { "accent": "{acc}", "{op}" }
          }
        ),
        Node::UnderOp(op, acc, target) => rsx!(
          munder {
            {node_element(*target)}
            mo { "accent": "{acc}", "{op}" }
          }
        ),
        Node::Overset { over, target } => rsx!(
          mover {
            {node_element(*target)}
            {node_element(*over)}
          }
        ),
        Node::Underset { under, target } => rsx!(
          munder {
            {node_element(*target)}
            {node_element(*under)}
          }
        ),
        Node::UnderOver {
            target,
            under,
            over,
        } => rsx!(
          munderover {
            {node_element(*target)}
            {node_element(*under)}
            {node_element(*over)}
          }
        ),
        Node::Sqrt(degree, content) => match degree {
          Some(deg) => rsx!{
            mroot {
              {node_element(*content)}
              {node_element(*deg)}
            }
          },
          None => rsx!{
            msqrt {
              {node_element(*content)}
            }
          }
        },
        Node::Frac(num, denom, lt) => rsx!(
          mfrac {
            linethickness: "{lt}",
            {node_element(*num)}
            {node_element(*denom)}
          }
        ),
        Node::Row(nodes) => rsx!(
          mrow {
            {nodes.into_iter().map(node_element)}
          }
        ),
        Node::Fenced {
            open,
            close,
            content,
        } => rsx!(
          mrow {
            {node_element(*open)}
            {node_element(*close)}
            {node_element(*content)}
          }
        ),
        Node::StrechedOp(stretchy, op) => rsx!(
          mo {
            stretchy,
            "{op}"
          }
        ),
        Node::OtherOperator(op) => rsx!(
          mo {
            "{op}"
          }
        ),
        Node::SizedParen { size, paren } => rsx!(
          mrow {
            mo {
              maxsize: "{size}",
              "{paren}"
            }
          }
        ),
        Node::Matrix(content, align) => {
            // let inner = match content.as_ref() {
            //     Node::Row(nodes) => {}
            //     node => node_element(node),
            // };

            // rsx!(
            //   mtable {
            //     "align": "{align}",
            //     mtr {
            //       mtd {
            //         {inner}
            //       }
            //     }
            //   }
            // )
            todo!()
        }
        Node::Ampersand => rsx!(mo { "&" }),
        Node::NewLine => rsx!(br {}),
        Node::Slashed(content) => rsx!(
          s {
            {node_element(*content)}
          }
        ),
        Node::Style(display, content) => {
            // TODO Fix displaystyle
            rsx!(
              mstyle {
                {node_element(*content)}
              }
            )
        }
        Node::Undefined(tok) => rsx!(
          merror {
            "{tok:?}"
          }
        ),
        _ => None,
    }
}
