use crate::{impl_context::ImplContext, lore_html::LoreHtml};
use lore_web_collections_core::LineType;

pub struct Parser<'a> {
    pub impl_context: &'a ImplContext<'a>,
}

impl<'a> Parser<'a> {
    pub fn parse_ir_node(&self, ir_node: &LineType) -> String {
        match ir_node {
            LineType::Empty => {
                "".to_string()
            },
            LineType::UrlLink(name, url) => {
                format!(
                    r##"<p style="margin-left: 2rem"><a href="{}" class="link_html">{}</a></p>"##,
                    url,
                    name
                )
            },
            LineType::LoreLink(name, lore) => {
                format!(
                    r#"<p style="margin-left: 2rem"><a href="{}/index/{}.html" class="link_lore">{}</a></p>"#,
                    self.impl_context.link_base,
                    lore,
                    name
                )
            },
            LineType::Placeholder => {
                "<br>".to_string()
            },
            LineType::Comment(comment) => {
                format!(
                    "<!-- {} -->",
                    comment
                )
            },
            LineType::DomainTitle(title) => {
                format!(
                    r#"<p class="title">{}</p>"#,
                    title
                )
            },
            LineType::Atom(atom) => {
                atom.to_string()
            }
        }
    }

    pub fn parse(&self, ir_nodes: &[LineType], impl_context: &ImplContext) -> LoreHtml {
        let content: String = ir_nodes
            .iter()
            .map(|node| self.parse_ir_node(node))
            .collect::<Vec<_>>()
            .join("\n");

        LoreHtml {
            title: impl_context.title.to_string(),
            css_url: impl_context.css_url.to_string(),
            html_content: content,
        }
    }
}