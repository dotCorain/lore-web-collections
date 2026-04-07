use lore_protocol_web_collections::LineTypes;

pub struct Parser;

impl Parser {
    pub fn parse_line(raw_line: &str) -> LineTypes {
        let content = raw_line;
    
        let line = if content.is_empty() {
            LineTypes::Empty
        } else if content.contains(" | ") {
            let (name, url) = content.split_once(" | ").unwrap();
            let name = name.to_string();
            let url = url.to_string();
            LineTypes::UrlLink(
                name,
                url
            )
        } else if content.contains(" = ") {
            let (name, category) = content.split_once(" = ").unwrap();
            let name = name.to_string();
            let category = category.to_string();
            LineTypes::LoreLink(
                name,
                category,
            )
        } else if content.starts_with("+ ") {
            let category = content
                .strip_prefix("+ ")
                .unwrap()
                .to_string();
            LineTypes::DomainTitle(
                category
            )
        } else if content.starts_with("# ") {
            let comment = content
                .strip_prefix("# ")
                .unwrap()
                .to_string();
            LineTypes::Comment(
                comment
            )
        } else if content == "---" {
            LineTypes::Placeholder
        } else {
            LineTypes::Atom(
                content.to_string()
            )
        };
    
        line
    }
}