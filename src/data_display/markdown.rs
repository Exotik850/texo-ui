use dioxus::prelude::*;
use markdown::{mdast::Node, to_mdast, ParseOptions};

#[derive(Copy, Clone, Default, PartialEq)]
pub enum MarkdownType {
  Github,
  #[default]
  Normal,
  Mdx,
}

impl MarkdownType {
  fn to_settings(self) -> ParseOptions {
    match self {
        MarkdownType::Github => ParseOptions::gfm(),
        MarkdownType::Normal => ParseOptions::mdx(),
        MarkdownType::Mdx => ParseOptions::default(),
    }
  }
}

#[component(no_case_check)]
pub fn Markdown(
    value: String,
    #[props(default)] class: String,
    #[props(extends=div)] rest_attributes: Vec<Attribute>,
    #[props(default)] md_type: MarkdownType,
) -> Element {
    let start = std::time::Instant::now();
    let nodes = to_mdast(&value, &md_type.to_settings())
        .expect("Normal Markdown parsing has no syntax errors");
    let out = rsx!(
        div { ..rest_attributes, { expand_node(&nodes) } }
    );
    log::info!("Markdown parsing took {:?}", start.elapsed());
    out
}

fn expand_node(node: &Node) -> Element {
    match node {
        // We should only get one root node
        Node::Root(root) => {
            rsx!({ root.children.iter().map(expand_node) })
        }
        Node::BlockQuote(bq) => {
            rsx!(
                blockquote { {bq.children.iter().map(expand_node)} }
            )
        }
        Node::FootnoteDefinition(fnd) => rsx!(
            p { id: if let Some(id) = &fnd.label { "{id}" },
                "[{fnd.identifier}]:"
                {fnd.children.iter().map(expand_node)}
            }
        ),
        Node::MdxJsxFlowElement(_) => None,
        Node::List(list) => {
            let children = list.children.iter().map(expand_node);
            let start = list.start.filter(|start| list.ordered && *start != 1);
            if list.ordered {
                rsx!(
                    ol { start: if let Some(start) = start { "{start}" }, {children} }
                )
            } else {
                rsx!(
                    ul { {children} }
                )
            }
        }
        Node::MdxjsEsm(_) => None,
        Node::Toml(toml) => rsx!( pre { "{toml.value}" } ),
        Node::Yaml(yaml) => rsx!( pre { "{yaml.value}" } ),
        Node::Break(_) => rsx!( br {} ),
        Node::InlineCode(ilc) => rsx!( code { "{ilc.value}" } ),
        Node::InlineMath(ilm) => rsx!( span { "{ilm.value}" } ),
        Node::Delete(del) => rsx!(
            del { {del.children.iter().map(expand_node)} }
        ),
        Node::Emphasis(emp) => rsx!(
            em { {emp.children.iter().map(expand_node)} }
        ),
        Node::MdxTextExpression(_) => None,
        Node::FootnoteReference(fnref) => {
            rsx!( a { href: if let Some(lab) = &fnref.label { "#{lab}" }, "[{fnref.identifier}]" } )
        }
        Node::Html(html) => rsx!( div { dangerous_inner_html: "{html.value}" } ),
        Node::Image(img) => {
            rsx!(
                img {
                    src: "{img.url}",
                    alt: "{img.alt}",
                    title: if let Some(title) = &img.title { "{title}" }
                }
            )
        }
        Node::ImageReference(ir) => rsx!(
            img { src: "{ir.identifier}", alt: "{ir.alt}" }
            if let Some(label) = &ir.label {
                "{label}"
            }
        ),
        Node::MdxJsxTextElement(_) => None,
        Node::Link(link) => rsx!(
            a {
                href: "{link.url}",
                title: if let Some(title) = &link.title { "{title}" },
                {link.children.iter().map(expand_node)}
            }
        ),
        Node::LinkReference(lr) => rsx!(
            a { aria_label: if let Some(label) = &lr.label { "{label}" },
                "{lr.identifier}"
                {lr.children.iter().map(expand_node)}
            }
        ),
        Node::Strong(strong) => rsx!(
            strong { {strong.children.iter().map(expand_node)} }
        ),
        Node::Text(text) => {
            rsx!( span { "{text.value}" } )
        }
        Node::Code(code) => rsx!(
            pre { code { language: if let Some(lang) = &code.lang { "{lang}" }, "{code.value}" } }
        ),
        Node::Math(math) => rsx!( span { "{math.value}" } ),
        Node::MdxFlowExpression(mdxflow) => rsx!( span { "{mdxflow.value}" } ),
        Node::Heading(head) => {
            let children = head.children.iter().map(expand_node);
            match head.depth {
                1 => rsx!(
                    h1 { {children} }
                ),
                2 => rsx!(
                    h2 { {children} }
                ),
                3 => rsx!(
                    h3 { {children} }
                ),
                4 => rsx!(
                    h4 { {children} }
                ),
                5 => rsx!(
                    h5 { {children} }
                ),
                6 => rsx!(
                    h6 { {children} }
                ),
                _ => rsx!(
                    div { {children} }
                ),
            }
        }
        Node::Table(table) => {
            rsx!(
                table { {table.children.iter().map(expand_node)} }
            )
        }
        Node::ThematicBreak(_) => rsx!( hr {} ),
        Node::TableRow(tr) => rsx!(
            tr { {tr.children.iter().map(expand_node)} }
        ),
        Node::TableCell(tc) => rsx!(
            td { {tc.children.iter().map(expand_node)} }
        ),
        Node::ListItem(li) => rsx!(
            li { style: if li.checked.is_some() { "display: flex" },
                if let Some(checked) = li.checked {
                    input {
                        r#type: "checkbox",
                        style: "pointer-events: none; margin-right: 0.5em;",
                        checked: checked
                    }
                }
                {li.children.iter().map(expand_node)}
            }
        ),
        Node::Definition(def) => rsx!(
            a {
                href: "{def.url}",
                title: if let Some(title) = &def.title { "{title}" },
                "{def.identifier}"
            }
        ),
        Node::Paragraph(par) => rsx!(
            p { {par.children.iter().map(expand_node)} }
        ),
    }
}
